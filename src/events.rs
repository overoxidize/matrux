use std::collections::HashMap;
use std::time::Duration;
use regex::Regex;
use html_escape::decode_html_entities;
use lazy_static::lazy_static;

lazy_static! {
  static ref HTML_REGEX: Regex = Regex::new(r#"<[^<]+?>"#).unwrap();
}

#[derive(Debug, Clone, Default)]
/// The `Event` struct represents a single event in the system.
///
/// It contains various fields that describe the event, including the state key, sender, event type, timestamp, and more.
///
/// # Fields
///
/// * `state_key`: The state key associated with the event.
/// * `sender`: The sender of the event.
/// * `etype`: The type of the event.
/// * `time_stamp`: The timestamp of the event.
/// * `id`: The ID of the event.
/// * `room_id`: The ID of the room where the event occurred.
/// * `redacts`: The ID of the event that this event is redacting.
/// * `unsigned`: A map of unsigned fields.
/// * `content`: A map of content fields.
/// * `prev_content`: A map of previous content fields.
/// * `is_init`: A boolean indicating whether the event is initialized.
///
/// # Implementations
///
/// This struct is implemented for the `Clone` trait, allowing it to be cloned.
///
/// # Examples
///
/// Here is an example of creating an `Event` instance:
///
/// ```rust
/// use std::collections::HashMap;
///
/// let event = Event {
///     state_key: "state_key".to_string(),
///     sender: "sender".to_string(),
///     etype: "etype".to_string(),
///     time_stamp: 1234567890,
///     id: "id".to_string(),
///     room_id: "room_id".to_string(),
///     redacts: "redacts".to_string(),
///     unsigned: HashMap::new(),
///     content: HashMap::new(),
///     prev_content: HashMap::new(),
///     is_init: true,
/// };
/// ```
pub struct Event<T: Clone> {
    pub state_key: String,
    pub sender: String,
    pub etype: String,
    pub time_stamp: i64,
    pub id: String,
    pub room_id: String,
    pub redacts: String,
    pub unsigned: HashMap<String, T>,
    pub content: HashMap<String, T>,
    pub prev_content: HashMap<String, T>,
    pub is_init: bool,
}

impl<T: Clone + 'static> Event<T>
where
    String: for<'a> From<&'a T>,
{
    pub fn new(
        state_key: String,
        sender: String,
        etype: String,
        time_stamp: i64,
        id: String,
        room_id: String,
        redacts: String,
        unsigned: HashMap<String, T>,
        content: HashMap<String, T>,
        prev_content: HashMap<String, T>,
        is_init: bool,
    ) -> Self {
        Self {
            state_key,
            sender,
            etype,
            time_stamp,
            id,
            room_id,
            redacts,
            unsigned,
            content,
            prev_content,
            is_init
        }
    }

    fn body(&self) -> Option<&T> {
        self.content.get("body")
    }


    fn message_type(&self) -> Option<&T> {
      self.content.get("msgtype")
    }
}

#[derive(Debug, Clone)]
pub struct TextMessage {
  pub message_type: String,
  pub body: String,
  pub formatted_body: String,
  pub format: String
}

#[derive(Debug, Clone)]
pub struct ThumbnailInfo {
  pub height: u64,
  pub width: u64,
  pub mime_type: String,
  pub size: u64
}
#[derive(Debug, Clone)]
pub struct ImageInfo {
  pub height: u64,
  pub width: u64,
  pub mime_type: String,
  pub size: u64,
  pub thumbnail_info: ThumbnailInfo,
  pub thumbnail_url: String
}

#[derive(Debug, Clone)]
pub struct VideoInfo {
  pub height: u64,
  pub width: u64,
  pub mime_type: String,
  pub size: u64,
  pub thumbnail_info: ThumbnailInfo,
  pub thumbnail_url: String,
  pub duration: Duration,
}

#[derive(Debug, Clone)]
pub struct VideoMessage {
  pub message_type: String,
  pub body: String,
  pub url: String,
  pub info: VideoInfo
}

#[derive(Debug, Clone)]
pub struct ImageMessage {
  pub message_type: String,
  pub body: String,
  pub url: String,
  pub info: ImageInfo
}

#[derive(Debug, Clone)]
pub struct HTMLMessage {
  pub message_type: String,
  pub body: String,
  pub formatted_body: String,
  pub format: String
}

#[derive(Debug, Clone)]
pub struct FileInfo {
  mime_type: String,
  size: u64,
}

#[derive(Debug, Clone)]
pub struct FileMessage {
  message_type: String,
  body: String,
  url: String,
  file_name: String,
  info: FileInfo,
  thumbnail_info: ThumbnailInfo,
  thumbnail_url: String,
}

#[derive(Debug, Clone)]
pub struct LocationMessage {
  message_type: String,
  body: String,
  url: String,
  file_name: String,
  geo_uri: String,
  thumbnail_info: ImageInfo,
  thumbnail_url: String,
}


#[derive(Debug, Clone)]
pub struct AudioInfo {
    mime_type: String,
    size: u64,
    duration: Duration, // Duration in milliseconds
}


#[derive(Debug, Clone)]
pub struct AudioMessage {
    message_type: String, // Must be `m.audio`
    body: String,
    url: String,
    info: AudioInfo,
}

fn get_html_message(message_type: String, html_text: String) -> HTMLMessage {

  HTMLMessage {
    body: decode_html_entities(&HTML_REGEX.replace_all(&html_text, "")).to_string(),
    message_type,
    format: String::from("org.matrix.custom.html"),
    formatted_body: html_text
  }
}