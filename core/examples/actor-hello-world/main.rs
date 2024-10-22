use async_trait::async_trait;
use aether_core::actor::actor::Actor;
use aether_core::actor::actor::ActorError;
use aether_core::actor::actor::Props;
use aether_core::actor::actor_system::ActorSystem;
use aether_core::actor::context::ContextHandle;
use aether_core::actor::context::{MessagePart, SenderPart, SpawnerPart};
use aether_core::actor::message::Message;
use aether_core::actor::message::MessageHandle;
use aether_core::Message;
use std::env;
use std::time::Duration;
use tokio::time::sleep;
use tracing_subscriber::EnvFilter;

#[derive(Debug, Clone, PartialEq, Eq, Message)]
struct Hello {
    who: String,
}

#[derive(Debug)]
struct HelloActor;

#[async_trait]
impl Actor for HelloActor {
    async fn receive(&mut self, ctx: ContextHandle) -> Result<(), ActorError> {
        let message_handle = ctx.get_message_handle().await;
        let hello = message_handle.to_typed::<Hello>().unwrap();
        tracing::info!("Hello, {}!", hello.who);
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let _ = env::set_var("RUST_LOG", "actor_hello_world=info");
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let system = ActorSystem::new().await.unwrap();
    let mut root_context = system.get_root_context().await;
    let actor_producer = |_| async { HelloActor };
    let pid = root_context
        .spawn(Props::from_async_actor_producer(actor_producer).await)
        .await;
    root_context
        .send(
            pid,
            MessageHandle::new(Hello {
                who: "world".to_string(),
            }),
        )
        .await;
    sleep(Duration::from_secs(1)).await;
}
