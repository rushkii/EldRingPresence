use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub activity_type: Option<ActivityTypeConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_display_type: Option<StatusDisplayTypeConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<ButtonConfig>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ButtonConfig {
    pub label: String,
    pub url: String,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum ActivityTypeConfig {
    Playing = 0,
    Listening = 2,
    Watching = 3,
    Competing = 5,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum StatusDisplayTypeConfig {
    Name = 0,
    State = 1,
    Details = 2,
}
