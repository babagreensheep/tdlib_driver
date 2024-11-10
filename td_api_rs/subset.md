double ? = Double;
string ? = String;

int32 = Int32;
int53 = Int53;
int64 = Int64;
bytes = Bytes;

boolFalse = Bool;
boolTrue = Bool;

vector {t:Type} # [ t ] = Vector t;


//@description An object of this type can be returned on every function call, in case of an error
//@code Error code; subject to future changes. If the error code is 406, the error message must not be processed in any way and must not be displayed to the user
//@message Error message; subject to future changes
error code:int32 message:string = Error;


//@description An object of this type is returned on a successful function call for certain functions
ok = Ok;


//@class AuthenticationCodeType @description Provides information about the method by which an authentication code is delivered to the user

//@description A digit-only authentication code is delivered via a private Telegram message, which can be viewed from another active session
//@length Length of the code
authenticationCodeTypeTelegramMessage length:int32 = AuthenticationCodeType;

//@description A digit-only authentication code is delivered via an SMS message to the specified phone number; non-official applications may not receive this type of code
//@length Length of the code
authenticationCodeTypeSms length:int32 = AuthenticationCodeType;

//@description An authentication code is a word delivered via an SMS message to the specified phone number; non-official applications may not receive this type of code
//@first_letter The first letters of the word if known
authenticationCodeTypeSmsWord first_letter:string = AuthenticationCodeType;

//@description An authentication code is a phrase from multiple words delivered via an SMS message to the specified phone number; non-official applications may not receive this type of code
//@first_word The first word of the phrase if known
authenticationCodeTypeSmsPhrase first_word:string = AuthenticationCodeType;

//@description A digit-only authentication code is delivered via a phone call to the specified phone number
//@length Length of the code
authenticationCodeTypeCall length:int32 = AuthenticationCodeType;

//@description An authentication code is delivered by an immediately canceled call to the specified phone number. The phone number that calls is the code that must be entered automatically
//@pattern Pattern of the phone number from which the call will be made
authenticationCodeTypeFlashCall pattern:string = AuthenticationCodeType;

//@description An authentication code is delivered by an immediately canceled call to the specified phone number. The last digits of the phone number that calls are the code that must be entered manually by the user
//@phone_number_prefix Prefix of the phone number from which the call will be made
//@length Number of digits in the code, excluding the prefix
authenticationCodeTypeMissedCall phone_number_prefix:string length:int32 = AuthenticationCodeType;

//@description A digit-only authentication code is delivered to https://fragment.com. The user must be logged in there via a wallet owning the phone number's NFT
//@url URL to open to receive the code
//@length Length of the code
authenticationCodeTypeFragment url:string length:int32 = AuthenticationCodeType;

//@description A digit-only authentication code is delivered via Firebase Authentication to the official Android application
//@device_verification_parameters Parameters to be used for device verification
//@length Length of the code
authenticationCodeTypeFirebaseAndroid device_verification_parameters:FirebaseDeviceVerificationParameters length:int32 = AuthenticationCodeType;

//@description A digit-only authentication code is delivered via Firebase Authentication to the official iOS application
//@receipt Receipt of successful application token validation to compare with receipt from push notification
//@push_timeout Time after the next authentication method is expected to be used if verification push notification isn't received, in seconds
//@length Length of the code
authenticationCodeTypeFirebaseIos receipt:string push_timeout:int32 length:int32 = AuthenticationCodeType;


//@description Information about the authentication code that was sent
//@phone_number A phone number that is being authenticated
//@type The way the code was sent to the user
//@next_type The way the next code will be sent to the user; may be null
//@timeout Timeout before the code can be re-sent, in seconds
authenticationCodeInfo phone_number:string type:AuthenticationCodeType next_type:AuthenticationCodeType timeout:int32 = AuthenticationCodeInfo;

//@description Information about the email address authentication code that was sent
//@email_address_pattern Pattern of the email address to which an authentication code was sent
//@length Length of the code; 0 if unknown
emailAddressAuthenticationCodeInfo email_address_pattern:string length:int32 = EmailAddressAuthenticationCodeInfo;


//@class EmailAddressAuthentication @description Contains authentication data for an email address

//@description An authentication code delivered to a user's email address @code The code
emailAddressAuthenticationCode code:string = EmailAddressAuthentication;

//@description An authentication token received through Apple ID @token The token
emailAddressAuthenticationAppleId token:string = EmailAddressAuthentication;

//@description An authentication token received through Google ID @token The token
emailAddressAuthenticationGoogleId token:string = EmailAddressAuthentication;


//@class EmailAddressResetState @description Describes reset state of an email address

//@description Email address can be reset after the given period. Call resetAuthenticationEmailAddress to reset it and allow the user to authorize with a code sent to the user's phone number
//@wait_period Time required to wait before the email address can be reset; 0 if the user is subscribed to Telegram Premium
emailAddressResetStateAvailable wait_period:int32 = EmailAddressResetState;

//@description Email address reset has already been requested. Call resetAuthenticationEmailAddress to check whether immediate reset is possible
//@reset_in Left time before the email address will be reset, in seconds. updateAuthorizationState is not sent when this field changes
emailAddressResetStatePending reset_in:int32 = EmailAddressResetState;


//@description Represents a part of the text that needs to be formatted in some unusual way @offset Offset of the entity, in UTF-16 code units @length Length of the entity, in UTF-16 code units @type Type of the entity
textEntity offset:int32 length:int32 type:TextEntityType = TextEntity;

//@description Contains a list of text entities @entities List of text entities
textEntities entities:vector<textEntity> = TextEntities;

//@description A text with some entities @text The text @entities Entities contained in the text. Entities can be nested, but must not mutually intersect with each other.
//-Pre, Code and PreCode entities can't contain other entities. BlockQuote entities can't contain other BlockQuote entities. Bold, Italic, Underline, Strikethrough, and Spoiler entities can contain and can be part of any other entities. All other entities can't contain each other
formattedText text:string entities:vector<textEntity> = FormattedText;


//@description Contains Telegram terms of service @text Text of the terms of service @min_user_age The minimum age of a user to be able to accept the terms; 0 if age isn't restricted @show_popup True, if a blocking popup with terms of service must be shown to the user
termsOfService text:formattedText min_user_age:int32 show_popup:Bool = TermsOfService;


//@class AuthorizationState @description Represents the current authorization state of the TDLib client

//@description Initialization parameters are needed. Call setTdlibParameters to provide them
authorizationStateWaitTdlibParameters = AuthorizationState;

//@description TDLib needs the user's phone number to authorize. Call setAuthenticationPhoneNumber to provide the phone number, or use requestQrCodeAuthentication or checkAuthenticationBotToken for other authentication options
authorizationStateWaitPhoneNumber = AuthorizationState;

//@description TDLib needs the user's email address to authorize. Call setAuthenticationEmailAddress to provide the email address, or directly call checkAuthenticationEmailCode with Apple ID/Google ID token if allowed
//@allow_apple_id True, if authorization through Apple ID is allowed
//@allow_google_id True, if authorization through Google ID is allowed
authorizationStateWaitEmailAddress allow_apple_id:Bool allow_google_id:Bool = AuthorizationState;

//@description TDLib needs the user's authentication code sent to an email address to authorize. Call checkAuthenticationEmailCode to provide the code
//@allow_apple_id True, if authorization through Apple ID is allowed
//@allow_google_id True, if authorization through Google ID is allowed
//@code_info Information about the sent authentication code
//@email_address_reset_state Reset state of the email address; may be null if the email address can't be reset
authorizationStateWaitEmailCode allow_apple_id:Bool allow_google_id:Bool code_info:emailAddressAuthenticationCodeInfo email_address_reset_state:EmailAddressResetState = AuthorizationState;

//@description TDLib needs the user's authentication code to authorize. Call checkAuthenticationCode to check the code @code_info Information about the authorization code that was sent
authorizationStateWaitCode code_info:authenticationCodeInfo = AuthorizationState;

//@description The user needs to confirm authorization on another logged in device by scanning a QR code with the provided link @link A tg:// URL for the QR code. The link will be updated frequently
authorizationStateWaitOtherDeviceConfirmation link:string = AuthorizationState;

//@description The user is unregistered and need to accept terms of service and enter their first name and last name to finish registration. Call registerUser to accept the terms of service and provide the data @terms_of_service Telegram terms of service
authorizationStateWaitRegistration terms_of_service:termsOfService = AuthorizationState;

//@description The user has been authorized, but needs to enter a 2-step verification password to start using the application.
//-Call checkAuthenticationPassword to provide the password, or requestAuthenticationPasswordRecovery to recover the password, or deleteAccount to delete the account after a week
//@password_hint Hint for the password; may be empty
//@has_recovery_email_address True, if a recovery email address has been set up
//@has_passport_data True, if some Telegram Passport elements were saved
//@recovery_email_address_pattern Pattern of the email address to which the recovery email was sent; empty until a recovery email has been sent
authorizationStateWaitPassword password_hint:string has_recovery_email_address:Bool has_passport_data:Bool recovery_email_address_pattern:string = AuthorizationState;

//@description The user has been successfully authorized. TDLib is now ready to answer general requests
authorizationStateReady = AuthorizationState;

//@description The user is currently logging out
authorizationStateLoggingOut = AuthorizationState;

//@description TDLib is closing, all subsequent queries will be answered with the error 500. Note that closing TDLib can take a while. All resources will be freed only after authorizationStateClosed has been received
authorizationStateClosing = AuthorizationState;

//@description TDLib client is in its final state. All databases are closed and all resources are released. No other updates will be received after this. All queries will be responded to
//-with error code 500. To continue working, one must create a new instance of the TDLib client
authorizationStateClosed = AuthorizationState;


//@class FirebaseDeviceVerificationParameters @description Describes parameters to be used for device verification

//@description Device verification must be performed with the SafetyNet Attestation API @nonce Nonce to pass to the SafetyNet Attestation API
firebaseDeviceVerificationParametersSafetyNet nonce:bytes = FirebaseDeviceVerificationParameters;

//@description Device verification must be performed with the classic Play Integrity verification (https://developer.android.com/google/play/integrity/classic)
//@nonce Base64url-encoded nonce to pass to the Play Integrity API
//@cloud_project_number Cloud project number to pass to the Play Integrity API
firebaseDeviceVerificationParametersPlayIntegrity nonce:string cloud_project_number:int64 = FirebaseDeviceVerificationParameters;


//@description Represents the current state of 2-step verification
//@has_password True, if a 2-step verification password is set
//@password_hint Hint for the password; may be empty
//@has_recovery_email_address True, if a recovery email is set
//@has_passport_data True, if some Telegram Passport elements were saved
//@recovery_email_address_code_info Information about the recovery email address to which the confirmation email was sent; may be null
//@login_email_address_pattern Pattern of the email address set up for logging in
//@pending_reset_date If not 0, point in time (Unix timestamp) after which the 2-step verification password can be reset immediately using resetPassword
passwordState has_password:Bool password_hint:string has_recovery_email_address:Bool has_passport_data:Bool recovery_email_address_code_info:emailAddressAuthenticationCodeInfo login_email_address_pattern:string pending_reset_date:int32 = PasswordState;

//@description Contains information about the current recovery email address @recovery_email_address Recovery email address
recoveryEmailAddress recovery_email_address:string = RecoveryEmailAddress;


//@description Returns information about the availability of a temporary password, which can be used for payments @has_password True, if a temporary password is available @valid_for Time left before the temporary password expires, in seconds
temporaryPasswordState has_password:Bool valid_for:int32 = TemporaryPasswordState;


//@description Represents a local file
//@path Local path to the locally available file part; may be empty
//@can_be_downloaded True, if it is possible to download or generate the file
//@can_be_deleted True, if the file can be deleted
//@is_downloading_active True, if the file is currently being downloaded (or a local copy is being generated by some other means)
//@is_downloading_completed True, if the local copy is fully available
//@download_offset Download will be started from this offset. downloaded_prefix_size is calculated from this offset
//@downloaded_prefix_size If is_downloading_completed is false, then only some prefix of the file starting from download_offset is ready to be read. downloaded_prefix_size is the size of that prefix in bytes
//@downloaded_size Total downloaded file size, in bytes. Can be used only for calculating download progress. The actual file size may be bigger, and some parts of it may contain garbage
localFile path:string can_be_downloaded:Bool can_be_deleted:Bool is_downloading_active:Bool is_downloading_completed:Bool download_offset:int53 downloaded_prefix_size:int53 downloaded_size:int53 = LocalFile;

//@description Represents a remote file
//@id Remote file identifier; may be empty. Can be used by the current user across application restarts or even from other devices. Uniquely identifies a file, but a file can have a lot of different valid identifiers.
//-If the identifier starts with "http://" or "https://", it represents the HTTP URL of the file. TDLib is currently unable to download files if only their URL is known.
//-If downloadFile/addFileToDownloads is called on such a file or if it is sent to a secret chat, TDLib starts a file generation process by sending updateFileGenerationStart to the application with the HTTP URL in the original_path and "#url#" as the conversion string.
//-Application must generate the file by downloading it to the specified location
//@unique_id Unique file identifier; may be empty if unknown. The unique file identifier which is the same for the same file even for different users and is persistent over time
//@is_uploading_active True, if the file is currently being uploaded (or a remote copy is being generated by some other means)
//@is_uploading_completed True, if a remote copy is fully available
//@uploaded_size Size of the remote available part of the file, in bytes; 0 if unknown
remoteFile id:string unique_id:string is_uploading_active:Bool is_uploading_completed:Bool uploaded_size:int53 = RemoteFile;

//@description Represents a file
//@id Unique file identifier
//@size File size, in bytes; 0 if unknown
//@expected_size Approximate file size in bytes in case the exact file size is unknown. Can be used to show download/upload progress
//@local Information about the local copy of the file
//@remote Information about the remote copy of the file
file id:int32 size:int53 expected_size:int53 local:localFile remote:remoteFile = File;


//@class InputFile @description Points to a file

//@description A file defined by its unique identifier @id Unique file identifier
inputFileId id:int32 = InputFile;

//@description A file defined by its remote identifier. The remote identifier is guaranteed to be usable only if the corresponding file is still accessible to the user and known to TDLib.
//-For example, if the file is from a message, then the message must be not deleted and accessible to the user. If the file database is disabled, then the corresponding object with the file must be preloaded by the application
//@id Remote file identifier
inputFileRemote id:string = InputFile;

//@description A file defined by a local path @path Local path to the file
inputFileLocal path:string = InputFile;

//@description A file generated by the application. The application must handle updates updateFileGenerationStart and updateFileGenerationStop to generate the file when asked by TDLib
//@original_path Local path to a file from which the file is generated. The path doesn't have to be a valid path and is used by TDLib only to detect name and MIME type of the generated file
//@conversion String specifying the conversion applied to the original file; must be persistent across application restarts. Conversions beginning with '#' are reserved for internal TDLib usage
//@expected_size Expected size of the generated file, in bytes; pass 0 if unknown
inputFileGenerated original_path:string conversion:string expected_size:int53 = InputFile;


//@description Describes an image in JPEG format
//@type Image type (see https://core.telegram.org/constructor/photoSize)
//@photo Information about the image file
//@width Image width
//@height Image height
//@progressive_sizes Sizes of progressive JPEG file prefixes, which can be used to preliminarily show the image; in bytes
photoSize type:string photo:file width:int32 height:int32 progressive_sizes:vector<int32> = PhotoSize;

//@description Thumbnail image of a very poor quality and low resolution @width Thumbnail width, usually doesn't exceed 40 @height Thumbnail height, usually doesn't exceed 40 @data The thumbnail in JPEG format
minithumbnail width:int32 height:int32 data:bytes = Minithumbnail;


//@class ThumbnailFormat @description Describes format of a thumbnail

//@description The thumbnail is in JPEG format
thumbnailFormatJpeg = ThumbnailFormat;

//@description The thumbnail is in static GIF format. It will be used only for some bot inline query results
thumbnailFormatGif = ThumbnailFormat;

//@description The thumbnail is in MPEG4 format. It will be used only for some animations and videos
thumbnailFormatMpeg4 = ThumbnailFormat;

//@description The thumbnail is in PNG format. It will be used only for background patterns
thumbnailFormatPng = ThumbnailFormat;

//@description The thumbnail is in TGS format. It will be used only for sticker sets
thumbnailFormatTgs = ThumbnailFormat;

//@description The thumbnail is in WEBM format. It will be used only for sticker sets
thumbnailFormatWebm = ThumbnailFormat;

//@description The thumbnail is in WEBP format. It will be used only for some stickers and sticker sets
thumbnailFormatWebp = ThumbnailFormat;


//@description Represents a thumbnail
//@format Thumbnail format
//@width Thumbnail width
//@height Thumbnail height
//@file The thumbnail
thumbnail format:ThumbnailFormat width:int32 height:int32 file:file = Thumbnail;


//@class MaskPoint @description Part of the face, relative to which a mask is placed

//@description The mask is placed relatively to the forehead
maskPointForehead = MaskPoint;

//@description The mask is placed relatively to the eyes
maskPointEyes = MaskPoint;

//@description The mask is placed relatively to the mouth
maskPointMouth = MaskPoint;

//@description The mask is placed relatively to the chin
maskPointChin = MaskPoint;

//@description Position on a photo where a mask is placed
//@point Part of the face, relative to which the mask is placed
//@x_shift Shift by X-axis measured in widths of the mask scaled to the face size, from left to right. (For example, -1.0 will place the mask just to the left of the default mask position)
//@y_shift Shift by Y-axis measured in heights of the mask scaled to the face size, from top to bottom. (For example, 1.0 will place the mask just below the default mask position)
//@scale Mask scaling coefficient. (For example, 2.0 means a doubled size)
maskPosition point:MaskPoint x_shift:double y_shift:double scale:double = MaskPosition;


//@class StickerFormat @description Describes format of a sticker

//@description The sticker is an image in WEBP format
stickerFormatWebp = StickerFormat;

//@description The sticker is an animation in TGS format
stickerFormatTgs = StickerFormat;

//@description The sticker is a video in WEBM format
stickerFormatWebm = StickerFormat;


//@class StickerType @description Describes type of sticker

//@description The sticker is a regular sticker
stickerTypeRegular = StickerType;

//@description The sticker is a mask in WEBP format to be placed on photos or videos
stickerTypeMask = StickerType;

//@description The sticker is a custom emoji to be used inside message text and caption
stickerTypeCustomEmoji = StickerType;


//@class StickerFullType @description Contains full information about sticker type

//@description The sticker is a regular sticker @premium_animation Premium animation of the sticker; may be null. If present, only Telegram Premium users can use the sticker
stickerFullTypeRegular premium_animation:file = StickerFullType;

//@description The sticker is a mask in WEBP format to be placed on photos or videos @mask_position Position where the mask is placed; may be null
stickerFullTypeMask mask_position:maskPosition = StickerFullType;

//@description The sticker is a custom emoji to be used inside message text and caption. Currently, only Telegram Premium users can use custom emoji
//@custom_emoji_id Identifier of the custom emoji
//@needs_repainting True, if the sticker must be repainted to a text color in messages, the color of the Telegram Premium badge in emoji status, white color on chat photos, or another appropriate color in other places
stickerFullTypeCustomEmoji custom_emoji_id:int64 needs_repainting:Bool = StickerFullType;


//@description Represents a closed vector path. The path begins at the end point of the last command @commands List of vector path commands
closedVectorPath commands:vector<VectorPathCommand> = ClosedVectorPath;


//@description Describes one answer option of a poll
//@text Option text; 1-100 characters. Only custom emoji entities are allowed
//@voter_count Number of voters for this option, available only for closed or voted polls
//@vote_percentage The percentage of votes for this option; 0-100
//@is_chosen True, if the option was chosen by the user
//@is_being_chosen True, if the option is being chosen by a pending setPollAnswer request
pollOption text:formattedText voter_count:int32 vote_percentage:int32 is_chosen:Bool is_being_chosen:Bool = PollOption;


//@class PollType @description Describes the type of poll

//@description A regular poll @allow_multiple_answers True, if multiple answer options can be chosen simultaneously
pollTypeRegular allow_multiple_answers:Bool = PollType;

//@description A poll in quiz mode, which has exactly one correct answer option and can be answered only once
//@correct_option_id 0-based identifier of the correct answer option; -1 for a yet unanswered poll
//@explanation Text that is shown when the user chooses an incorrect answer or taps on the lamp icon; 0-200 characters with at most 2 line feeds; empty for a yet unanswered poll
pollTypeQuiz correct_option_id:int32 explanation:formattedText = PollType;


//@description Describes an animation file. The animation must be encoded in GIF or MPEG4 format
//@duration Duration of the animation, in seconds; as defined by the sender
//@width Width of the animation
//@height Height of the animation
//@file_name Original name of the file; as defined by the sender
//@mime_type MIME type of the file, usually "image/gif" or "video/mp4"
//@has_stickers True, if stickers were added to the animation. The list of corresponding sticker set can be received using getAttachedStickerSets
//@minithumbnail Animation minithumbnail; may be null
//@thumbnail Animation thumbnail in JPEG or MPEG4 format; may be null
//@animation File containing the animation
animation duration:int32 width:int32 height:int32 file_name:string mime_type:string has_stickers:Bool minithumbnail:minithumbnail thumbnail:thumbnail animation:file = Animation;

//@description Describes an audio file. Audio is usually in MP3 or M4A format
//@duration Duration of the audio, in seconds; as defined by the sender
//@title Title of the audio; as defined by the sender
//@performer Performer of the audio; as defined by the sender
//@file_name Original name of the file; as defined by the sender
//@mime_type The MIME type of the file; as defined by the sender
//@album_cover_minithumbnail The minithumbnail of the album cover; may be null
//@album_cover_thumbnail The thumbnail of the album cover in JPEG format; as defined by the sender. The full size thumbnail is expected to be extracted from the downloaded audio file; may be null
//@external_album_covers Album cover variants to use if the downloaded audio file contains no album cover. Provided thumbnail dimensions are approximate
//@audio File containing the audio
audio duration:int32 title:string performer:string file_name:string mime_type:string album_cover_minithumbnail:minithumbnail album_cover_thumbnail:thumbnail external_album_covers:vector<thumbnail> audio:file = Audio;

//@description Describes a document of any type
//@file_name Original name of the file; as defined by the sender
//@mime_type MIME type of the file; as defined by the sender
//@minithumbnail Document minithumbnail; may be null
//@thumbnail Document thumbnail in JPEG or PNG format (PNG will be used only for background patterns); as defined by the sender; may be null
//@document File containing the document
document file_name:string mime_type:string minithumbnail:minithumbnail thumbnail:thumbnail document:file = Document;

//@description Describes a photo
//@has_stickers True, if stickers were added to the photo. The list of corresponding sticker sets can be received using getAttachedStickerSets
//@minithumbnail Photo minithumbnail; may be null
//@sizes Available variants of the photo, in different sizes
photo has_stickers:Bool minithumbnail:minithumbnail sizes:vector<photoSize> = Photo;

//@description Describes a sticker
//@id Unique sticker identifier within the set; 0 if none
//@set_id Identifier of the sticker set to which the sticker belongs; 0 if none
//@width Sticker width; as defined by the sender
//@height Sticker height; as defined by the sender
//@emoji Emoji corresponding to the sticker
//@format Sticker format
//@full_type Sticker's full type
//@outline Sticker's outline represented as a list of closed vector paths; may be empty. The coordinate system origin is in the upper-left corner
//@thumbnail Sticker thumbnail in WEBP or JPEG format; may be null
//@sticker File containing the sticker
sticker id:int64 set_id:int64 width:int32 height:int32 emoji:string format:StickerFormat full_type:StickerFullType outline:vector<closedVectorPath> thumbnail:thumbnail sticker:file = Sticker;

//@description Describes a video file
//@duration Duration of the video, in seconds; as defined by the sender
//@width Video width; as defined by the sender
//@height Video height; as defined by the sender
//@file_name Original name of the file; as defined by the sender
//@mime_type MIME type of the file; as defined by the sender
//@has_stickers True, if stickers were added to the video. The list of corresponding sticker sets can be received using getAttachedStickerSets
//@supports_streaming True, if the video is expected to be streamed
//@minithumbnail Video minithumbnail; may be null
//@thumbnail Video thumbnail in JPEG or MPEG4 format; as defined by the sender; may be null
//@video File containing the video
video duration:int32 width:int32 height:int32 file_name:string mime_type:string has_stickers:Bool supports_streaming:Bool minithumbnail:minithumbnail thumbnail:thumbnail video:file = Video;

//@description Describes a video note. The video must be equal in width and height, cropped to a circle, and stored in MPEG4 format
//@duration Duration of the video, in seconds; as defined by the sender
//@waveform A waveform representation of the video note's audio in 5-bit format; may be empty if unknown
//@length Video width and height; as defined by the sender
//@minithumbnail Video minithumbnail; may be null
//@thumbnail Video thumbnail in JPEG format; as defined by the sender; may be null
//@speech_recognition_result Result of speech recognition in the video note; may be null
//@video File containing the video
videoNote duration:int32 waveform:bytes length:int32 minithumbnail:minithumbnail thumbnail:thumbnail speech_recognition_result:SpeechRecognitionResult video:file = VideoNote;

//@description Describes a voice note
//@duration Duration of the voice note, in seconds; as defined by the sender
//@waveform A waveform representation of the voice note in 5-bit format
//@mime_type MIME type of the file; as defined by the sender. Usually, one of "audio/ogg" for Opus in an OGG container, "audio/mpeg" for an MP3 audio, or "audio/mp4" for an M4A audio
//@speech_recognition_result Result of speech recognition in the voice note; may be null
//@voice File containing the voice note
voiceNote duration:int32 waveform:bytes mime_type:string speech_recognition_result:SpeechRecognitionResult voice:file = VoiceNote;

//@description Describes an animated or custom representation of an emoji
//@sticker Sticker for the emoji; may be null if yet unknown for a custom emoji. If the sticker is a custom emoji, then it can have arbitrary format
//@sticker_width Expected width of the sticker, which can be used if the sticker is null
//@sticker_height Expected height of the sticker, which can be used if the sticker is null
//@fitzpatrick_type Emoji modifier fitzpatrick type; 0-6; 0 if none
//@sound File containing the sound to be played when the sticker is clicked; may be null. The sound is encoded with the Opus codec, and stored inside an OGG container
animatedEmoji sticker:sticker sticker_width:int32 sticker_height:int32 fitzpatrick_type:int32 sound:file = AnimatedEmoji;

//@description Describes a user contact
//@phone_number Phone number of the user
//@first_name First name of the user; 1-255 characters in length
//@last_name Last name of the user
//@vcard Additional data about the user in a form of vCard; 0-2048 bytes in length
//@user_id Identifier of the user, if known; 0 otherwise
contact phone_number:string first_name:string last_name:string vcard:string user_id:int53 = Contact;

//@description Describes a location on planet Earth
//@latitude Latitude of the location in degrees; as defined by the sender
//@longitude Longitude of the location, in degrees; as defined by the sender
//@horizontal_accuracy The estimated horizontal accuracy of the location, in meters; as defined by the sender. 0 if unknown
location latitude:double longitude:double horizontal_accuracy:double = Location;

//@description Describes a venue
//@location Venue location; as defined by the sender
//@title Venue name; as defined by the sender
//@address Venue address; as defined by the sender
//@provider Provider of the venue database; as defined by the sender. Currently, only "foursquare" and "gplaces" (Google Places) need to be supported
//@id Identifier of the venue in the provider database; as defined by the sender
//@type Type of the venue in the provider database; as defined by the sender
venue location:location title:string address:string provider:string id:string type:string = Venue;

//@description Describes a game. Use getInternalLink with internalLinkTypeGame to share the game
//@id Unique game identifier
//@short_name Game short name
//@title Game title
//@text Game text, usually containing scoreboards for a game
//@param_description Game description
//@photo Game photo
//@animation Game animation; may be null
game id:int64 short_name:string title:string text:formattedText description:string photo:photo animation:animation = Game;

//@description Describes a Web App. Use getInternalLink with internalLinkTypeWebApp to share the Web App
//@short_name Web App short name
//@title Web App title
//@param_description Web App description
//@photo Web App photo
//@animation Web App animation; may be null
webApp short_name:string title:string description:string photo:photo animation:animation = WebApp;

//@description Describes a poll
//@id Unique poll identifier
//@question Poll question; 1-300 characters. Only custom emoji entities are allowed
//@options List of poll answer options
//@total_voter_count Total number of voters, participating in the poll
//@recent_voter_ids Identifiers of recent voters, if the poll is non-anonymous
//@is_anonymous True, if the poll is anonymous
//@type Type of the poll
//@open_period Amount of time the poll will be active after creation, in seconds
//@close_date Point in time (Unix timestamp) when the poll will automatically be closed
//@is_closed True, if the poll is closed
poll id:int64 question:formattedText options:vector<pollOption> total_voter_count:int32 recent_voter_ids:vector<MessageSender> is_anonymous:Bool type:PollType open_period:int32 close_date:int32 is_closed:Bool = Poll;


//@description Describes an alternative reencoded quality of a video file
//@width Video width
//@height Video height
//@codec Codec used for video file encoding, for example, "h264", "h265", or "av1"
//@hls_file HLS file describing the video
//@video File containing the video
alternativeVideo width:int32 height:int32 codec:string hls_file:file video:file = AlternativeVideo;


//@description Describes a chat background
//@id Unique background identifier
//@is_default True, if this is one of default backgrounds
//@is_dark True, if the background is dark and is recommended to be used with dark theme
//@name Unique background name
//@document Document with the background; may be null. Null only for filled and chat theme backgrounds
//@type Type of the background
background id:int64 is_default:Bool is_dark:Bool name:string document:document type:BackgroundType = Background;

//@description Contains a list of backgrounds @backgrounds A list of backgrounds
backgrounds backgrounds:vector<background> = Backgrounds;

//@description Describes a background set for a specific chat @background The background @dark_theme_dimming Dimming of the background in dark themes, as a percentage; 0-100. Applied only to Wallpaper and Fill types of background
chatBackground background:background dark_theme_dimming:int32 = ChatBackground;


//@description Describes a user profile photo
//@id Photo identifier; 0 for an empty photo. Can be used to find a photo in a list of user profile photos
//@small A small (160x160) user profile photo. The file can be downloaded only before the photo is changed
//@big A big (640x640) user profile photo. The file can be downloaded only before the photo is changed
//@minithumbnail User profile photo minithumbnail; may be null
//@has_animation True, if the photo has animated variant
//@is_personal True, if the photo is visible only for the current user
profilePhoto id:int64 small:file big:file minithumbnail:minithumbnail has_animation:Bool is_personal:Bool = ProfilePhoto;

//@description Contains basic information about the photo of a chat
//@small A small (160x160) chat photo variant in JPEG format. The file can be downloaded only before the photo is changed
//@big A big (640x640) chat photo variant in JPEG format. The file can be downloaded only before the photo is changed
//@minithumbnail Chat photo minithumbnail; may be null
//@has_animation True, if the photo has animated variant
//@is_personal True, if the photo is visible only for the current user
chatPhotoInfo small:file big:file minithumbnail:minithumbnail has_animation:Bool is_personal:Bool = ChatPhotoInfo;


//@class UserType @description Represents the type of user. The following types are possible: regular users, deleted users and bots

//@description A regular user
userTypeRegular = UserType;

//@description A deleted user or deleted bot. No information on the user besides the user identifier is available. It is not possible to perform any active actions on this type of user
userTypeDeleted = UserType;

//@description A bot (see https://core.telegram.org/bots)
//@can_be_edited True, if the bot is owned by the current user and can be edited using the methods toggleBotUsernameIsActive, reorderBotActiveUsernames, setBotProfilePhoto, setBotName, setBotInfoDescription, and setBotInfoShortDescription
//@can_join_groups True, if the bot can be invited to basic group and supergroup chats
//@can_read_all_group_messages True, if the bot can read all messages in basic group or supergroup chats and not just those addressed to the bot. In private and channel chats a bot can always read all messages
//@has_main_web_app True, if the bot has the main Web App
//@is_inline True, if the bot supports inline queries
//@inline_query_placeholder Placeholder for inline queries (displayed on the application input field)
//@need_location True, if the location of the user is expected to be sent with every inline query to this bot
//@can_connect_to_business True, if the bot supports connection to Telegram Business accounts
//@can_be_added_to_attachment_menu True, if the bot can be added to attachment or side menu
//@active_user_count The number of recently active users of the bot
userTypeBot can_be_edited:Bool can_join_groups:Bool can_read_all_group_messages:Bool has_main_web_app:Bool is_inline:Bool inline_query_placeholder:string need_location:Bool can_connect_to_business:Bool can_be_added_to_attachment_menu:Bool active_user_count:int32 = UserType;

//@description No information on the user besides the user identifier is available, yet this user has not been deleted. This object is extremely rare and must be handled like a deleted user. It is not possible to perform any actions on users of this type
userTypeUnknown = UserType;


//@description Represents a command supported by a bot @command Text of the bot command @param_description Description of the bot command
botCommand command:string description:string = BotCommand;

//@description Contains a list of bot commands @bot_user_id Bot's user identifier @commands List of bot commands
botCommands bot_user_id:int53 commands:vector<botCommand> = BotCommands;

//@description Describes a button to be shown instead of bot commands menu button
//@text Text of the button
//@url URL of a Web App to open when the button is pressed. If the link is of the type internalLinkTypeWebApp, then it must be processed accordingly. Otherwise, the link must be passed to openWebApp
botMenuButton text:string url:string = BotMenuButton;


//@description Represents a location to which a chat is connected @location The location @address Location address; 1-64 characters, as defined by the chat owner
chatLocation location:location address:string = ChatLocation;


//@description Represents a birthdate of a user @day Day of the month; 1-31 @month Month of the year; 1-12 @year Birth year; 0 if unknown
birthdate day:int32 month:int32 year:int32 = Birthdate;

//@description Describes a user that had or will have a birthday soon @user_id User identifier @birthdate Birthdate of the user
closeBirthdayUser user_id:int53 birthdate:birthdate = CloseBirthdayUser;


//@class BusinessAwayMessageSchedule @description Describes conditions for sending of away messages by a Telegram Business account

//@description Send away messages always
businessAwayMessageScheduleAlways = BusinessAwayMessageSchedule;
