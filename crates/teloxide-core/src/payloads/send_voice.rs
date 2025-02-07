//! Generated by `codegen_payloads`, do not edit by hand.

use serde::Serialize;

use crate::types::{
    BusinessConnectionId, InputFile, Message, MessageEntity, ParseMode, Recipient, ReplyMarkup,
    ReplyParameters, ThreadId,
};

impl_payload! {
    @[multipart = voice]
    /// Use this method to send audio files, if you want Telegram clients to display the file as a playable voice message. For this to work, your audio must be in an .OGG file encoded with OPUS (other formats may be sent as [`Audio`] or [`Document`]). On success, the sent [`Message`] is returned. Bots can currently send voice messages of up to 50 MB in size, this limit may be changed in the future.
    ///
    /// [`Document`]: crate::types::Document
    /// [`Audio`]: crate::types::Audio
    /// [`Message`]: crate::types::Message
    #[derive(Debug, Clone, Serialize)]
    pub SendVoice (SendVoiceSetters) => Message {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
            pub chat_id: Recipient [into],
            /// Audio file to send. Pass a file_id as String to send an audio file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get an audio file from the Internet, or upload a new one using multipart/form-data. [More info on Sending Files »]
            ///
            /// [More info on Sending Files »]: crate::types::InputFile
            pub voice: InputFile,
        }
        optional {
            /// Unique identifier of the business connection on behalf of which the message will be sent
            pub business_connection_id: BusinessConnectionId,
            /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
            pub message_thread_id: ThreadId,
            /// Voice message caption, 0-1024 characters after entities parsing
            pub caption: String [into],
            /// Mode for parsing entities in the voice message caption. See [formatting options] for more details.
            ///
            /// [formatting options]: https://core.telegram.org/bots/api#formatting-options
            pub parse_mode: ParseMode,
            /// List of special entities that appear in the photo caption, which can be specified instead of _parse\_mode_
            pub caption_entities: Vec<MessageEntity> [collect],
            /// Duration of the voice message in seconds
            pub duration: u32,
            /// Sends the message [silently]. Users will receive a notification with no sound.
            ///
            /// [silently]: https://telegram.org/blog/channels-2-0#silent-messages
            pub disable_notification: bool,
            /// Protects the contents of sent message from forwarding and saving
            pub protect_content: bool,
            /// Description of the message to reply to
            pub reply_parameters: ReplyParameters,
            /// Additional interface options. A JSON-serialized object for an [inline keyboard], [custom reply keyboard], instructions to remove a reply keyboard or to force a reply from the user. Not supported for messages sent on behalf of a business account.
            ///
            /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
            /// [custom reply keyboard]: https://core.telegram.org/bots#keyboards
            pub reply_markup: ReplyMarkup [into],
        }
    }
}
