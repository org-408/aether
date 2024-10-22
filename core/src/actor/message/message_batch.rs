use crate::actor::message::{Message, MessageHandle};
use aether_message_derive::Message;

#[derive(Debug, Clone, PartialEq, Eq, Message)]
pub struct MessageBatch(Vec<MessageHandle>);

impl MessageBatch {
    pub fn new(messages: impl IntoIterator<Item = MessageHandle>) -> Self {
        Self(messages.into_iter().collect::<Vec<_>>())
    }

    pub fn get_messages(&self) -> &Vec<MessageHandle> {
        &self.0
    }
}
