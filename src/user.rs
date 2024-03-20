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

#[allow(non_snake_case)]
#[derive(Deserialize, Clone, Eq)]
pub(crate) struct User {
    #[serde(default)]
    pub(crate) bio: String,
    #[serde(default)]
    pub(crate) bioLinks: Vec<String>,
    #[serde(default)]
    pub(crate) currentAvatarThumbnailImageUrl: String,
    pub(crate) displayName: String,
    pub(crate) id: String,
    pub(crate) isFriend: bool,
    pub(crate) location: String,
    pub(crate) travelingToLocation: Option<String>,
    pub(crate) status: Status,
    #[serde(default)]
    pub(crate) statusDescription: String,
    pub(crate) tags: Vec<String>,
    #[serde(default)]
    pub(crate) userIcon: String,
    #[serde(default)]
    pub(crate) profilePicOverride: String,
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
