use crate::actor::message::Message;
use aether_message_derive::Message;

#[derive(Debug, Clone, PartialEq, Eq, Message)]
pub enum MailboxMessage {
    SuspendMailbox,
    ResumeMailbox,
}
