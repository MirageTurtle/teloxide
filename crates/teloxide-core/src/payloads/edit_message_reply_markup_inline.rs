//! Generated by `codegen_payloads`, do not edit by hand.

use serde::Serialize;

use crate::types::{BusinessConnectionId, InlineKeyboardMarkup, True};

impl_payload! {
    /// Use this method to edit only the reply markup of messages. On success, _True_ is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within **48 hours** from the time they were sent.
    ///
    /// See also: [`EditMessageReplyMarkup`](crate::payloads::EditMessageReplyMarkup)
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub EditMessageReplyMarkupInline (EditMessageReplyMarkupInlineSetters) => True {
        required {
            /// Identifier of the inline message
            pub inline_message_id: String [into],
        }
        optional {
            /// Unique identifier of the business connection on behalf of which the message to be edited was sent
            pub business_connection_id: BusinessConnectionId,
            /// A JSON-serialized object for an [inline keyboard].
            ///
            /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
            pub reply_markup: InlineKeyboardMarkup,
        }
    }
}
