use aether_core::actor::actor::Props;
use aether_core::actor::actor_system::ActorSystem;
use aether_core::actor::context::{BasePart, MessagePart, SenderPart, SpawnerPart};
use aether_core::actor::message::Message;
use aether_core::actor::message::MessageHandle;
use aether_core::actor::message::ResponseHandle;
use aether_core::Message;
use std::env;
use std::time::Duration;
use tracing_subscriber::EnvFilter;

#[derive(Debug, Clone, PartialEq, Eq, Message)]
struct Hello {
    who: String,
}

#[tokio::main]
async fn main() {
    let _ = env::set_var("RUST_LOG", "actor_request_response=info");
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let system = ActorSystem::new().await.unwrap();
    let mut root_context = system.get_root_context().await;
    let props = Props::from_async_actor_receiver(|ctx| async move {
        if let Some(msg) = ctx.get_message_handle().await.to_typed::<Hello>() {
            ctx.respond(ResponseHandle::new(format!("Hello, {}!", msg.who)))
                .await;
        }
        Ok(())
    })
    .await;
    let pid = root_context.spawn(props).await;
    let msg = MessageHandle::new(Hello {
        who: "world".to_string(),
    });
    let future = root_context
        .request_future(pid, msg, Duration::from_secs(1))
        .await;
    let result = future.result().await;
    tracing::info!("{:?}", result.unwrap());
}
