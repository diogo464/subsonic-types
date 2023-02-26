use serde::{Deserialize, Serialize};
use subsonic_macro::{FromQuery, SubsonicRequest, ToQuery};

use crate::common::Milliseconds;

/// Returns the current visible (non-expired) chat messages.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getChatMessages>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.2.0", path = "getChatMessages")]
pub struct GetChatMessages {
    /// Only return messages newer than this time.
    pub since: Option<Milliseconds>,
}

/// Adds a message to the chat log.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#addChatMessage>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToQuery, FromQuery, SubsonicRequest)]
#[serde(rename_all = "camelCase")]
#[subsonic(since = "1.2.0", path = "addChatMessage")]
pub struct AddChatMessage {
    /// The chat message.
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::super::tests::test_request_encode;
    use super::*;

    #[test]
    fn test_get_chat_messages() {
        let request = super::GetChatMessages { since: None };
        let query = test_request_encode(&request);
        assert_eq!("", query);

        // Test with since
        let request = super::GetChatMessages {
            since: Some(Milliseconds::new(1000)),
        };
        let query = test_request_encode(&request);
        assert_eq!("since=1000", query);
    }

    #[test]
    fn test_add_chat_message() {
        let request = super::AddChatMessage {
            message: "test".to_string(),
        };
        let query = test_request_encode(&request);
        assert_eq!("message=test", query);
    }
}
