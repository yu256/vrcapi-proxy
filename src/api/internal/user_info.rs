use hyper::Method;
use serde::Deserialize;

use crate::{fetcher::{request, ResponseExt}, user::Status};

pub(crate) async fn fetch_user_info(token: &str) -> anyhow::Result<UserProfile> {
    request(
        Method::GET,
        "https://api.vrchat.cloud/api/1/auth/user",
        token,
    )
    .await?
    .json()
    .await
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct UserProfile {
    pub id: String,
    pub displayName: String,
    pub userIcon: String,
    pub bio: String,
    pub bioLinks: Vec<String>,
    pub profilePicOverride: String,
    pub statusDescription: String,
    pub username: String,
    pub pastDisplayNames: Vec<String>,
    pub hasEmail: bool,
    pub hasPendingEmail: bool,
    pub obfuscatedEmail: String,
    pub obfuscatedPendingEmail: String,
    pub emailVerified: bool,
    pub hasBirthday: bool,
    pub hideContentFilterSettings: bool,
    pub unsubscribe: bool,
    pub statusHistory: Vec<String>,
    pub statusFirstTime: bool,
    pub friends: Vec<String>,
    pub friendGroupNames: Vec<String>,
    pub queuedInstance: Option<String>,
    pub userLanguage: String,
    pub userLanguageCode: String,
    pub currentAvatarImageUrl: String,
    pub currentAvatarThumbnailImageUrl: String,
    pub currentAvatarTags: Vec<String>,
    pub currentAvatar: String,
    pub currentAvatarAssetUrl: String,
    pub fallbackAvatar: String,
    pub accountDeletionDate: Option<String>,
    pub accountDeletionLog: Option<String>,
    pub acceptedTOSVersion: u32,
    pub acceptedPrivacyVersion: u32,
    pub steamId: String,
    pub steamDetails: SteamDetails,
    pub googleId: String,
    pub googleDetails: GoogleDetails,
    pub oculusId: String,
    pub picoId: String,
    pub viveId: String,
    pub hasLoggedInFromClient: bool,
    pub homeLocation: String,
    pub twoFactorAuthEnabled: bool,
    pub twoFactorAuthEnabledDate: Option<String>,
    pub updated_at: String,
    pub state: String,
    pub tags: Vec<String>,
    pub developerType: String,
    pub last_login: String,
    pub last_platform: String,
    pub allowAvatarCopying: bool,
    pub status: Status,
    pub date_joined: String,
    pub isFriend: bool,
    pub friendKey: String,
    pub last_activity: String,
    pub onlineFriends: Vec<String>,
    pub activeFriends: Vec<String>,
    pub presence: Presence,
    pub offlineFriends: Vec<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct SteamDetails {}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct GoogleDetails {}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct Presence {
    pub platform: String,
    pub instance: String,
    pub profilePicOverride: String,
    pub currentAvatarTags: String,
    pub avatarThumbnail: String,
    pub status: String,
    pub instanceType: String,
    pub travelingToWorld: String,
    pub travelingToInstance: String,
    pub groups: Vec<String>,
    pub world: String,
    pub displayName: String,
    pub id: String,
    pub debugflag: String,
    pub isRejoining: String,
    pub userIcon: String,
}