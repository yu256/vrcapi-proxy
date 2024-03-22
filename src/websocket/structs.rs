use crate::user::{Status, User};
use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub(super) struct StreamBody {
    pub(super) r#type: String,
    pub(super) content: String, // json
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct FriendLocation {
    pub userId: String,
    pub location: Option<String>,
    pub travelingToLocation: Option<String>,
    pub worldId: Option<String>,
    pub canRequestInvite: Option<bool>,
    pub user: LocationEventUser,
    pub world: Option<World>,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct LocationEventUser {
    pub id: String,
    pub displayName: String,
    #[serde(default)]
    pub userIcon: String,
    #[serde(default)]
    pub bio: String,
    #[serde(default)]
    pub bioLinks: Vec<String>,
    #[serde(default)]
    pub profilePicOverride: String,
    #[serde(default)]
    pub statusDescription: String,
    #[serde(default)]
    pub currentAvatarImageUrl: String,
    #[serde(default)]
    pub currentAvatarThumbnailImageUrl: String,
    pub currentAvatarTags: Vec<String>,
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
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct World {
    pub id: String,
    pub name: String,
    pub description: String,
    pub authorId: String,
    pub authorName: String,
    pub releaseStatus: String,
    pub featured: bool,
    pub capacity: i32,
    pub recommendedCapacity: i32,
    pub imageUrl: String,
    pub thumbnailImageUrl: String,
    pub namespace: String,
    pub version: i32,
    pub organization: String,
    pub previewYoutubeId: Option<String>,
    pub udonProducts: Vec<String>,
    pub favorites: i32,
    pub visits: i32,
    pub popularity: i32,
    pub heat: i32,
    pub publicationDate: String,
    pub labsPublicationDate: String,
    pub instances: Vec<String>,
    pub publicOccupants: i32,
    pub privateOccupants: i32,
    pub occupants: i32,
    pub tags: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl FriendLocation {
    pub fn normalize(self) -> (User, Option<World>) {
        (
            User {
                bio: self.user.bio,
                bioLinks: self.user.bioLinks,
                currentAvatarThumbnailImageUrl: self.user.currentAvatarThumbnailImageUrl,
                displayName: self.user.displayName,
                id: self.user.id,
                isFriend: self.user.isFriend,
                location: self.location,
                travelingToLocation: self.travelingToLocation,
                status: self.user.status,
                statusDescription: self.user.statusDescription,
                tags: self.user.tags,
                userIcon: self.user.userIcon,
                profilePicOverride: self.user.profilePicOverride,
                currentAvatarImageUrl: self.user.currentAvatarImageUrl,
                developerType: self.user.developerType,
                last_login: self.user.last_login,
                last_platform: self.user.last_platform,
                friendKey: self.user.friendKey,
            },
            self.world,
        )
    }
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub(super) struct FriendActive {
    pub(super) user: User,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub(super) struct UserIdContent {
    pub(super) userId: String,
}
