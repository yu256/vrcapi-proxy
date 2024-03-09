use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Notification {
    id: String,
    senderUserId: String,
    senderUsername: String,
    r#type: String,
    message: String,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // details: Option<Details>,
    // seen: bool,
    created_at: String,
}

// #[allow(clippy::enum_variant_names)]
// #[derive(Serialize, Deserialize, Debug)]
// enum Details {
//     NotificationDetailInvite,
//     NotificationDetailInviteResponse,
//     NotificationDetailRequestInvite,
//     NotificationDetailRequestInviteResponse,
//     NotificationDetailVoteToKick,
// }
