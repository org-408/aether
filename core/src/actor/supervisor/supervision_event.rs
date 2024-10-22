use crate::actor::actor::ErrorReason;
use crate::actor::actor::ExtendedPid;
use crate::actor::actor_system::ActorSystem;
use crate::actor::message::Message;
use crate::actor::supervisor::directive::Directive;
use crate::event_stream::Subscription;
use aether_message_derive::Message;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Message)]
pub struct SupervisorEvent {
    pub child: ExtendedPid,
    pub reason: ErrorReason,
    pub directive: Directive,
}

pub async fn subscribe_supervision(actor_system: &ActorSystem) -> Subscription {
    actor_system
        .get_event_stream()
        .await
        .subscribe(move |evt| {
            let evt = evt
                .as_any()
                .downcast_ref::<SupervisorEvent>()
                .cloned()
                .map(Arc::new);
            async move {
                if let Some(supervisor_event) = evt {
                    tracing::debug!("[SUPERVISION]: {:?}", supervisor_event.reason.backtrace());
                }
            }
        })
        .await
}