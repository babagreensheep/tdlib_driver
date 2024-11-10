use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock, RwLockReadGuard, RwLockWriteGuard};

/// Compile an item
pub trait Compile<Output> {
    type State;
    async fn compile(self, state: Self::State) -> Result<Output, String>;
}

pub type Meta = HashMap<String, String>;

/// Dictionary of types that exist
pub type Types = HashMap<String, Type>;

/// Declared types in the sytem
pub static TYPES: OnceLock<Arc<RwLock<Types>>> = OnceLock::new();

#[derive(Debug)]
/// Types that exist in the Schema
pub enum Type {
    /// Types that map to rust types
    ///
    /// E.g.:
    /// - `int32 = Int32;`, meaning `int32` inherits from Int32
    /// - `boolFalse = Bool;`, meaning `boolFalse` inherits from Bool
    ///
    /// Both Int32 and Bool are *inherent* types. So we must define the following mappings:
    /// - `int32` maps to a TD type of `Int32`
    /// - `Int32` maps to a rust type of `i32`
    /// - `boolFalse` maps to a TD type of `Bool`
    /// - `Bool` maps to a rust type of `bool`
    Rust(&'static str),

    /// Custom TD type
    TD {
        ///  Meta data of the TD type
        ///
        ///  E.g. description: "some description"
        meta: Meta,

        /// Parent of the Type  
        ///
        /// Applies only if there is a parent type, e.g. `int32 = Int32;`
        /// - `int32` ([Type::TD]) inherits from `Int32` (also a TD type)
        /// - `Int32` ([Type::Rust]) inherits from i32
        /// E.g. bool
        inherits_from: Option<String>,

        /// Children of that type
        children: Vec<(String, usize)>,

        /// Dependants of that type
        depends_on: Vec<String>,
    },
}

pub struct TypeDef<'hash>(pub &'hash String, pub &'hash Type);

impl<'hash> std::fmt::Display for TypeDef<'hash> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self(name, data) = self;

        match data {
            Type::Rust(_) => Ok(()),
            Type::TD { meta, children, .. } => {
                // Remove base types
                if Type::from_schema_type(name).is_some() {
                    return Ok(());
                }

                // If there are no children, it is simply a type mapping that has been optimisd
                // away
                if children.is_empty() {
                    return Ok(());
                }

                // Write permissions
                writeln!(f, "#[allow(non_camel_case_types)]")?;

                // Write derive
                writeln!(
                    f,
                    "#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]"
                )?;
                writeln!(f, "#[serde(tag = \"@type\")]")?;

                // Write metadata
                if let Some(value) = meta.get("description") {
                    writeln!(f, "/// {value}")?;
                }

                writeln!(f, "pub enum {name}{{")?;

                // Write children
                for (child, arg_count) in children {
                    // child description
                    let child_description = if *arg_count == 0 {
                        "".to_string()
                    } else {
                        format!("({child})")
                    };

                    writeln!(f, "   {child}{child_description},",)?;
                }

                writeln!(f, "}}\n\n")?;

                Ok(())
            }
        }
    }
}

impl Type {
    /// Creates a new [Type::Rust] if the provided `schema_type` maps to a valid rust_type
    pub fn from_schema_type<SchemaType: ToString>(schema_type: &SchemaType) -> Option<Self> {
        match schema_type.to_string().as_str() {
            "::std::primitive::f64" | "f64" | "Double" => Some(Type::Rust("::std::primitive::f64")),
            "::std::string::String" | "bytes" | "String" | "Bytes" => {
                Some(Type::Rust("::std::string::String"))
            }
            "::std::primitive::i32" | "i32" | "Int32" => Some(Type::Rust("::std::primitive::i32")),
            "::std::primitive::i64" | "i64" | "Int53" | "Int64" => {
                Some(Type::Rust("::std::primitive::i64"))
            }
            "::std::vec::Vec" | "Vector" => Some(Type::Rust("::std::vec::Vec")),
            "::std::primitive::bool" | "bool" | "Bool" => {
                Some(Type::Rust("::std::primitive::bool"))
            }
            _ => None,
        }
    }

    /// Transforms a key, e.g. `Bool`, to either a rust equivalent (`bool`) or the parent class and
    /// inserts the key into the database if it is not already present
    /// Examples:
    /// - If the supplied type is `AuthenticationCodeType`, this checks to see if
    /// `AuthenticationCodeType` exists already. Since `AuthenticationCodeType` does not map to a
    /// rust base type, `AuthenticationCodeType` is returned
    /// - If the supplied type is `Bool`, this checks to see if `Bool` exists already.
    ///     - If `Bool` already exists, it will trace `Bool` to the rust type __`bool`__ and return
    ///     __`bool`__.
    ///     - If `Bool` does not exist yet, it will insert a new entry for `Bool` that maps to the
    ///     rust type __`bool`__ and then return __`bool`__.
    pub fn transform_type(key: String) -> String {
        // Create an entry first
        // WARNING: This has to be two operations or the guards will smash
        match Self::write().entry(key.clone()) {
            Entry::Occupied(_) => (),
            Entry::Vacant(vacant_entry) => {
                // Try to create a new entry
                let new_entry = Self::from_schema_type(&key).unwrap_or(Type::TD {
                    meta: HashMap::new(),
                    inherits_from: None,
                    children: Vec::new(),
                    depends_on: Vec::new(),
                });

                // Insert new entry for the key
                vacant_entry.insert(new_entry);
            }
        };

        // Search entries
        match Self::is_rust(&key) {
            Some(rust_type) => rust_type.into(),
            None => key,
        }
    }

    /// Check if the specified types map to a rust type and returns the underlying rusttype
    pub fn is_rust<KeyType: ToString>(parent_key: &KeyType) -> Option<&'static str> {
        let read_guard = Self::read();
        let parent = read_guard.get(&parent_key.to_string())?;
        match parent {
            Type::Rust(rust_type) => Some(rust_type),
            Type::TD { inherits_from, .. } => match inherits_from {
                Some(parent_key) => {
                    let parent_key = parent_key.clone();

                    // WARNING: Read guard must be dropped to avoid smashing guards
                    std::mem::drop(read_guard);

                    Self::is_rust(&parent_key)
                }
                None => None,
            },
        }
    }

    /// Obtain reference to the static store of [TYPES]
    fn store() -> &'static Arc<RwLock<Types>> {
        TYPES.get_or_init(|| Arc::new(RwLock::new(HashMap::new())))
    }

    /// Returns mutable reference to existing types
    pub fn write() -> RwLockWriteGuard<'static, HashMap<String, Type>> {
        Self::store().write().unwrap()
    }

    /// Returns mutable reference to existing types
    pub fn read() -> RwLockReadGuard<'static, HashMap<String, Type>> {
        Self::store().read().unwrap()
    }

    /// Insert a new entry
    pub fn insert(key: String, type_entry: Type) -> Result<String, String> {
        match Self::write().entry(key) {
            Entry::Occupied(occupied_entry) => Err(format!(
                "{} has already been declared: {occupied_entry:?}",
                occupied_entry.key()
            )),

            Entry::Vacant(vacant_entry) => {
                let key = vacant_entry.key().clone();
                vacant_entry.insert(type_entry);
                Ok(key)
            }
        }
    }

    /// Check for recursion
    fn recursion_check<const DEPTH: usize>(look_for: &String, in_type_id: &String) -> Option<()> {
        let mut to_search = vec![in_type_id.clone()];
        let mut searched = Vec::new();
        let mut loop_count = 0usize;

        'return_loop: loop {
            // Maximum break
            if loop_count > DEPTH {
                break None;
            }

            // Fetch the next search item, or none if there are no more search terms
            let type_id = to_search.pop()?;

            // Attempt to fetch a type description and break if fails
            let search_array = match Type::read().get(&type_id)? {
                Type::Rust(_) => None?,
                Type::TD {
                    children,
                    depends_on,
                    ..
                } => {
                    if type_id.chars().next()?.is_uppercase() {
                        children
                            .iter()
                            .map(|(child, _)| child.clone())
                            .collect::<Vec<_>>()
                    } else {
                        depends_on.iter().map(|parent| parent.clone()).collect()
                    }
                }
            };

            for search in search_array {
                // Continue this for loop if the search term is a base type
                if Type::from_schema_type(&search).is_some() {
                    continue;
                }

                // Break if found
                // if searched.contains(&search) {
                if search == *look_for {
                    println!("recursion of \"{search}\" found after {loop_count} iterations");
                    break 'return_loop Some(());
                }

                if Type::read().contains_key(&search) {
                    // println!("pushing \"{search}\"");
                    to_search.push(search)
                }
            }

            // Add the type to the list of searched types
            searched.push(type_id);

            // Add loop counter
            loop_count += 1;
        }
    }
}

#[test]
fn base_type() {
    let mut writer = Type::write();
    writer.insert("Int32".to_string(), Type::Rust("i32"));
    writer.insert(
        "int32".to_string(),
        Type::TD {
            meta: HashMap::new(),
            inherits_from: Some("Int32".to_string()),
            children: Vec::new(),
            depends_on: Vec::new(),
        },
    );
    writer.insert(
        "int52".to_string(),
        Type::TD {
            meta: HashMap::new(),
            inherits_from: Some("int32".to_string()),
            children: Vec::new(),
            depends_on: Vec::new(),
        },
    );
    std::mem::drop(writer);

    assert!(Type::is_rust(&"int32").is_some());
    assert!(Type::is_rust(&"int42").is_none());
    assert!(Type::is_rust(&"int52").is_some());
}

/// Objects
pub static OBJECTS: OnceLock<Arc<RwLock<Vec<Object>>>> = OnceLock::new();

#[derive(Debug)]
/// Objects
pub struct Object {
    pub name: String,
    pub meta: Meta,
    pub fields: Vec<ObjectValue>,
    // pub result_type: String,
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            name, meta, fields, ..
        } = self;

        // Write permissions
        writeln!(f, "#[allow(non_camel_case_types)]")?;

        // Write derive
        writeln!(
            f,
            "#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]"
        )?;
        writeln!(f, "/// # {name}")?;

        // write metadata
        if let Some(value) = meta.get("description") {
            writeln!(f, "/// {value}")?;
        }

        writeln!(f, "pub struct {name} {{")?;

        for ObjectValue { key, type_id } in fields {
            // PERF: This has been optimised based on the current state of recursion (52 levels)
            let recursion = Type::recursion_check::<128>(name, type_id);

            // Check if nullable (and also print description)
            let nullable = if let Some(description) = meta.get(key) {
                writeln!(f, "   /// {description}")?;
                description.contains("null")
            } else {
                false
            };

            // Rewrite the type-id if there is recursion
            let type_id = if recursion.is_some() {
                format!("Box<{type_id}>")
            } else {
                format!("{type_id}")
            };

            // Rewrite the type_id if nullable
            let type_id = if nullable {
                format!("Option<{type_id}>")
            } else {
                format!("{type_id}")
            };

            // Write out
            writeln!(f, "   pub {key}: {type_id},")?;
        }

        writeln!(f, "}}\n\n")?;
        Ok(())
    }
}

impl Object {
    /// Obtain reference to the static store of [OBJECTS]
    fn store() -> &'static Arc<RwLock<Vec<Self>>> {
        OBJECTS.get_or_init(|| Arc::new(RwLock::new(Vec::new())))
    }

    /// Returns mutable reference to existing objects
    pub fn write() -> RwLockWriteGuard<'static, Vec<Object>> {
        Self::store().write().unwrap()
    }

    /// Returns mutable reference to existing types
    pub fn read() -> RwLockReadGuard<'static, Vec<Object>> {
        Self::store().read().unwrap()
    }

    /// Push new object
    pub fn push(object: Object) {
        Self::write().push(object);
    }
}

#[derive(Debug)]
/// Values in an object
pub struct ObjectValue {
    pub key: String,
    pub type_id: String,
}

/// Objects
pub static FUNCTIONS: OnceLock<Arc<RwLock<Vec<Function>>>> = OnceLock::new();

#[derive(Debug)]
/// Function
pub struct Function {
    pub name: String,
    pub meta: Meta,
    pub fields: Vec<ObjectValue>,
    pub result_type: String,
}

impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            name,
            meta,
            fields,
            result_type,
        } = self;

        // Write permissions
        writeln!(
            f,
            "#[allow(async_fn_in_trait, non_snake_case, non_camel_case_types)]"
        )?;
        // write metadata
        let metadata = format!(
            "/// # {name}{}",
            if let Some(value) = meta.get("description") {
                format!("\n/// {value}")
            } else {
                "".to_string()
            }
        );

        // let fields_annotations = meta
        //     .iter()
        //     .filter(|(key, _)| *key != "description")
        //     .map(|(key, value)| format!("/// -`{key}`: {value}"))
        //     .collect::<Vec<_>>()
        //     .join("\n");

        let (comments, arguments) = fields
            .iter()
            .map(|ObjectValue { key, type_id }| {
                let comment = meta
                    .get(key)
                    .map(|desc| format!("    /// -`{key}`: {desc}"))
                    .unwrap_or(String::default());

                let type_id = comment
                    .contains("null")
                    .then(|| format!("Option<{type_id}>"))
                    .unwrap_or(format!("{type_id}"));

                let argument = format!(", {key}: {type_id}");

                (comment, argument)
            })
            .collect::<(Vec<_>, Vec<_>)>();

        let comments = comments.join("\n");
        let arguments = arguments.join("");

        // Compile Value fields
        let value_fields = fields
            .iter()
            .map(|field| {
                let ObjectValue { key, .. } = field;

                // Create a sanitized key with the "r#" prefix removed
                let sanitized_key = key.strip_prefix("r#").unwrap_or(key.as_str());

                format!(r#","{sanitized_key}": {key}"#)
            })
            .collect::<Vec<_>>()
            .join("");

        writeln!(
            f,
            r#"
    {metadata}
{comments}
    async fn {name}(&self{arguments}) -> ::std::result::Result<{result_type}, crate::Failure> {{
        // Create payload
        let payload = json!({{
            "@type": "{name}"{value_fields}
        }});
    
        // Send payload and await reesponse
        <Self as Api>::send::<Value, {result_type}>(self, payload).await
    }}
"#
        )?;

        Ok(())
    }
}

impl Function {
    /// Obtain reference to the static store of [OBJECTS]
    fn store() -> &'static Arc<RwLock<Vec<Self>>> {
        FUNCTIONS.get_or_init(|| Arc::new(RwLock::new(Vec::new())))
    }

    /// Returns mutable reference to existing objects
    pub fn write() -> RwLockWriteGuard<'static, Vec<Self>> {
        Self::store().write().unwrap()
    }

    /// Returns mutable reference to existing types
    pub fn read() -> RwLockReadGuard<'static, Vec<Self>> {
        Self::store().read().unwrap()
    }

    /// Push new object
    pub fn push(function: Function) {
        Self::write().push(function);
    }
}

#[cfg(test)]
pub fn harness<Callback: std::future::Future>(
    future: Callback,
) -> <Callback as std::future::Future>::Output {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(future)
}
