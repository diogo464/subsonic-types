use serde::{Deserialize, Serialize};

use crate::common::Milliseconds;

/// Returns the current visible (non-expired) chat messages.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#getChatMessages>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetChatMessages {
    /// Only return messages newer than this time.
    pub since: Option<Milliseconds>,
}

/// Adds a message to the chat log.
///
/// For more information, see <http://www.subsonic.org/pages/api.jsp#addChatMessage>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddChatMessage {
    /// The chat message.
    pub message: String,
}
