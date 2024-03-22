use crate::unsanitizer::Unsanitizer as _;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Ord, PartialEq, PartialOrd, Eq, Clone, Copy)]
pub(crate) enum Status {
    #[serde(rename = "join me")]
    JoinMe,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "ask me")]
    AskMe,
    #[serde(rename = "busy")]
    Busy,
    #[serde(rename = "offline")]
    Offline,
}

impl Default for Status {
    fn default() -> Self {
        Self::Offline
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Eq)]
pub struct User {
    pub id: String,
    pub location: Option<String>,
    pub travelingToLocation: Option<String>,
    pub displayName: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "str::is_empty")]
    pub userIcon: String,
    #[serde(default)]
    pub bio: String,
    #[serde(default)]
    pub bioLinks: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "str::is_empty")]
    pub profilePicOverride: String,
    #[serde(default)]
    pub statusDescription: String,
    #[serde(default)]
    pub currentAvatarImageUrl: String,
    #[serde(default)]
    pub currentAvatarThumbnailImageUrl: String,
    pub tags: Vec<String>,
    pub developerType: String,
    pub last_login: String,
    pub last_platform: String,
    pub status: Status,
    pub isFriend: bool,
    pub friendKey: String,
}

impl User {
    pub(crate) fn unsanitize(&mut self) {
        self.bio = self.bio.unsanitize();
        self.statusDescription = self.statusDescription.unsanitize();
    }
}

impl Ord for User {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.status.cmp(&other.status)
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.status == other.status
    }
}

impl PartialOrd for User {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.status.cmp(&other.status))
    }
}
