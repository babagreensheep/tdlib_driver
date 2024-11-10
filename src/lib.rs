use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::ffi::CStr;
use std::ffi::CString;
use std::sync::Arc;
use std::sync::Mutex;
pub use td_api::Api;
pub use td_api::*;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::oneshot::*;
use tokio::task::JoinHandle;

mod td_api;
mod tdlib;

#[no_mangle]
unsafe extern "C" fn log(
    _verbosity_level: ::std::os::raw::c_int,
    _message: *const ::std::os::raw::c_char,
) {
    // let message = CStr::from_ptr(message);
    // let message = message.to_str().unwrap();
    // println!("{message}");
}

pub type Notifier = UnboundedSender<Notification>;
type Tasks = HashMap<usize, Sender<Value>>;
type TasksSync = Arc<Mutex<Tasks>>;

#[allow(dead_code)]
/// # Driver
/// [Driver] wraps the underlying `tdlib` and associated API and provides quality of life public
/// methods:
/// - [Driver::new]: Does all the plumbing to create a td client, a task queue and a notification
/// handler.
/// *Note*: Plumbing for sending is called through the [Api] traitt
/// # Safety
/// The [Driver] needs to be wrapped in an [std::sync::Arc] in order to become thread safe.
pub struct Driver<const TIMEOUT_MS: usize> {
    id: i32,
    reference_point: std::time::Instant,
    tasks: TasksSync,
    listener_handle: JoinHandle<()>,
}

impl<const TIMEOUT_MS: usize> Drop for Driver<TIMEOUT_MS> {
    fn drop(&mut self) {
        self.listener_handle.abort();
    }
}

impl<const TIMEOUT_MS: usize> Driver<TIMEOUT_MS> {
    /// # New
    /// Create a new [Driver].
    /// Arguments:
    /// - `notification_handler`: Option of a notification handler to call
    /// # Example
    /// Note that this example is a hardwired JSON object (which the implementation allows for).
    /// However the td_lib api methods are also hardwired in as a trait.
    /// ```
    /// let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel();
    ///
    /// tokio::task::spawn(async move {
    ///     let update = receiver.recv().await.unwrap();
    ///     println!("{update:#?}");
    /// });
    ///
    /// let driver = Driver::<1000>::new(Some(sender));
    ///
    /// let payload = driver.getAuthorizationState().await.unwrap();
    /// println!("{payload:#?}");
    /// ```
    pub fn new(notifier: Option<Notifier>) -> Self {
        // Create a client
        let id = unsafe { tdlib::td_create_client_id() };

        // Clear tdlib's log stream
        Self::clear_log_stream(id);
        // Implement our own log stream
        unsafe {
            let callback: tdlib::td_log_message_callback_ptr = Some(log);
            tdlib::td_set_log_message_callback(1024, callback);
        };

        // Start a listening loop
        let tasks = Arc::new(Mutex::new(HashMap::new()));
        let listener_handle = tokio::task::spawn(Self::start(tasks.clone(), notifier));

        // Create driver
        Self {
            id,
            tasks,
            reference_point: std::time::Instant::now(),
            listener_handle,
        }
    }

    /// Starts a loop that listens for messages from TDLIB api and sends it to the relevant
    /// [Driver::tasks];
    async fn start(tasks: TasksSync, notifier: Option<Notifier>) {
        loop {
            // Yield to [tokio] executor to prevent blocking
            tokio::task::yield_now().await;

            // Listen for messages
            let timeout = TIMEOUT_MS as f64 / 1000f64;
            let payload = unsafe { tdlib::td_receive(timeout).as_ref() };
            let payload =
                payload.map(|ptr| unsafe { CStr::from_ptr(ptr) }.to_string_lossy().to_string());

            // Payload is an Option:
            // - No payload; continue looping
            let payload = payload.map(|output| serde_json::from_str::<Value>(&output));

            // Message handler
            match payload {
                // If payload is empty continue loop
                None => continue,

                // If paylod validly converts to JSON
                Some(Ok(payload)) => {
                    // Obtain task_id
                    let task_id = payload["@extra"].as_u64();

                    // Check if there is a type ID
                    match task_id {
                        Some(task_id) => {
                            // Attempt to notify the task
                            if let Err(err) = Self::notify_task(tasks.clone(), task_id, payload) {
                                if let Some(handler) = &notifier {
                                    let _ = handler.send(Notification::PacketDropped(task_id, err));
                                }
                            }
                        }
                        None => {
                            if let Some(handler) = &notifier {
                                let _ = handler.send(Notification::Driver(payload));
                            }
                        }
                    }
                }

                // If payload does not convert to JSON: notify the client
                Some(Err(err)) => {
                    if let Some(handler) = &notifier {
                        // NOTE: Sender failure means that the receiver has dropped, we do not need
                        // to parse that event.
                        let _ = handler.send(Notification::ApiError(err));
                    }
                }
            }
        }
    }

    /// Retrieve and notify the [Driver::tasks]
    fn notify_task(tasks: TasksSync, task_id: u64, payload: Value) -> Result<(), Value> {
        let task_id = task_id as usize;

        // Retrieve oneshot sender into hashmap
        // NOTE: safe to unwrap because poison cleared
        while tasks.is_poisoned() {
            tasks.clear_poison();
        }

        // Send value to task
        match tasks.lock().unwrap().remove(&task_id) {
            Some(sender) => sender.send(payload),
            None => Err(payload),
        }
    }

    /// Send raw payload string
    fn send_sync<Payload: Serialize>(id: i32, payload: Payload) {
        let payload = serde_json::to_string(&payload).unwrap();
        let payload = CString::new(payload).unwrap();
        unsafe {
            tdlib::td_send(id, payload.as_ptr());
        }
    }

    /// Clears the internal log stream
    fn clear_log_stream(id: i32) {
        // Create payload for empty log stream
        let log_stream = LogStream::logStreamEmpty;
        let log_stream = serde_json::json!({
            "@type": "setLogStream",
            "log_stream": log_stream
        });

        Self::send_sync(id, log_stream);
    }
}

impl<const TIMEOUT_MS: usize> Api for Driver<TIMEOUT_MS> {
    /// Sends the `payload` to the TDLIB api.
    /// *Arguments*:
    /// - `payload`: payload as a [::std::string::String]
    async fn send<Payload: Serialize, Target: DeserializeOwned>(
        &self,
        payload: Payload,
    ) -> Result<Target, Failure> {
        // Create JSON payload
        let mut payload = serde_json::to_value(payload).unwrap();

        // Create oneshot channel; task number based on time_hash
        let task_id = {
            let task_id = std::time::Instant::now() - self.reference_point;
            task_id.as_nanos() as usize // Same as % operation
        };
        let (sender, receiver) = channel();

        // Insert oneshot sender into hashmap
        // NOTE: safe to unwrap because poison cleared
        while self.tasks.is_poisoned() {
            self.tasks.clear_poison();
        }
        {
            let mut guard = self.tasks.lock().unwrap();
            match guard.entry(task_id) {
                std::collections::hash_map::Entry::Occupied(_) => {
                    Err(Failure::Receiver(ReceiverError::QueueFull))
                }
                std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(sender);
                    Ok(())
                }
            }
        }?;

        // Append task to payload
        // NOTE: unwrap because usize always validly converts
        payload["@extra"] = serde_json::to_value(task_id).unwrap();

        // Send payload
        Self::send_sync(self.id, payload);

        // Wait on receiver
        let timeout = TIMEOUT_MS as f64 / 1000f64;
        let result =
            tokio::time::timeout(std::time::Duration::from_secs_f64(timeout), receiver).await??;

        // Get output
        match serde_json::from_value(result.clone()) {
            Result::Ok(value) => Ok(value),
            Err(_) => match serde_json::from_value(result) {
                Result::Ok(error) => Err(Failure::TdLib(error)),
                Err(err) => Err(err.into()),
            },
        }
    }
}

#[derive(Debug)]
pub enum Notification {
    /// Packet that does not have a task allocated to it
    Driver(Value),

    /// A packet that has dropped becuase the calling function's task did not make it into the
    /// hashmap
    PacketDropped(u64, Value),

    /// The [Driver] has encountered a payload which it is unable to process. This likely means
    /// that the telegram API is issuing new object classes that the existing handler is unable to
    /// process.
    ApiError(serde_json::Error),
}

#[derive(Debug)]
pub enum Failure {
    TdLib(Error),
    Receiver(ReceiverError),
}

#[derive(Debug)]
pub enum ReceiverError {
    Serde(serde_json::Error),
    Channel(tokio::sync::oneshot::error::RecvError),
    QueueFull,
    Timeout(tokio::time::error::Elapsed),
    TdLib(Error),
}

impl From<tokio::time::error::Elapsed> for Failure {
    fn from(value: tokio::time::error::Elapsed) -> Self {
        Self::Receiver(ReceiverError::Timeout(value))
    }
}

impl From<tokio::sync::oneshot::error::RecvError> for Failure {
    fn from(value: tokio::sync::oneshot::error::RecvError) -> Self {
        Self::Receiver(ReceiverError::Channel(value))
    }
}

impl From<serde_json::Error> for Failure {
    fn from(value: serde_json::Error) -> Self {
        Self::Receiver(ReceiverError::Serde(value))
    }
}

#[tokio::test]
/// Run the driver with a handler
async fn driver_with_handler() {
    let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel();

    tokio::task::spawn(async move {
        let update = receiver.recv().await.unwrap();
        println!("{update:#?}");
    });

    let driver = Driver::<1000>::new(Some(sender));

    let payload = driver.getAuthorizationState().await.unwrap();
    println!("{payload:#?}");
}

#[tokio::test]
/// Run the driver and send a couple of requests
async fn driver() {
    let driver = Driver::<1000>::new(None);

    let payload = driver.getAuthorizationState().await.unwrap();
    println!("{payload:#?}");

    let payload = driver.getAuthorizationState().await.unwrap();
    println!("{payload:#?}");

    let payload = driver.getAuthorizationState().await.unwrap();
    println!("{payload:#?}");
}

#[tokio::test]
/// # Test of the driver interface
/// Test to create a [Driver] and send a simple authorization request
async fn driver_handroll() {
    let payload = serde_json::json!({
        "@type": "getAuthorizationState",
    });

    let driver = Driver::<1000>::new(None);

    let payload = driver
        .send::<Value, AuthorizationState>(payload)
        .await
        .unwrap();

    println!("{payload:#?}");
}

#[test]
/// # Test of the C interface
/// Simple test loop to create a client, set a logging callbck, send a request for
/// AuthorizationState and attempt to serialize the output
fn c_json_client() {
    // Create client
    let client = unsafe { tdlib::td_create_client_id() };
    assert_eq!(client, 1);

    // Set logging callback
    let callback: tdlib::td_log_message_callback_ptr = Some(log);
    unsafe { tdlib::td_set_log_message_callback(1024, callback) };

    let send = serde_json::json!({
        "@extra": "1",
        "@type": "getAuthorizationState",
    });

    let send = std::ffi::CString::new(send.to_string()).unwrap();
    unsafe { tdlib::td_send(client, send.as_ptr()) };

    println!("request sent");

    let result = loop {
        println!("looping");
        let result = unsafe { tdlib::td_receive(2.0).as_ref() };
        if let Some(result_ptr) = result {
            let result_string = unsafe { std::ffi::CStr::from_ptr(result_ptr) }
                .to_string_lossy()
                .into_owned();

            match serde_json::from_str::<Update>(&result_string) {
                Ok(value) => {
                    println!("{value:#?}");
                    continue;
                }
                Err(_err) => {
                    break result_string;
                }
            };
        }
    };

    let result = serde_json::from_str::<AuthorizationState>(&result).unwrap();
    println!("{result:#?}");
}

#[test]
/// Test basic serialization of API
fn serialization() {
    let base_string = "{\"@type\": 'updateOption',\"name\":\"version\",\"value\":{\"@type\":\"optionValueString\",\"value\":\"1.8.39\"},\"@client_id\":1}";
    let _test = serde_json::from_str::<Update>(base_string).unwrap();

    let option_value = td_api::OptionValue::optionValueString(optionValueString {
        value: "1.8.39".to_string(),
    });

    let update_option = updateOption {
        name: "version".to_string(),
        value: option_value,
    };
    let update_option = Update::updateOption(update_option);

    let update_string = serde_json::to_string(&update_option).unwrap();
    println!("{update_string}");
    assert_eq!(update_string, base_string);
}
