use async_trait::async_trait;
use aether_core::actor::actor::Actor;
use aether_core::actor::actor::ActorError;
use aether_core::actor::actor::ErrorReason;
use aether_core::actor::actor::Props;
use aether_core::actor::actor_system::ActorSystem;
use aether_core::actor::context::ContextHandle;
use aether_core::actor::context::{MessagePart, SenderPart, SpawnerPart};
use aether_core::actor::message::Message;
use aether_core::actor::message::MessageHandle;
use aether_core::actor::supervisor::Directive;
use aether_core::actor::supervisor::OneForOneStrategy;
use aether_core::actor::supervisor::SupervisorStrategyHandle;
use aether_core::Message;
use aether_utils::concurrent::AsyncBarrier;
use std::env;
use std::time::Duration;
use tokio::time::sleep;
use tracing_subscriber::EnvFilter;

#[derive(Debug)]
struct Parent;

impl Parent {
    fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Actor for Parent {
    async fn receive(&mut self, mut context_handle: ContextHandle) -> Result<(), ActorError> {
        let message_handle = context_handle.get_message_handle().await;
        let msg = message_handle.to_typed::<Hello>().unwrap();
        let props = Props::from_async_actor_producer(|_| async { Child::new() }).await;
        let child = context_handle.spawn(props).await;
        context_handle.send(child, MessageHandle::new(msg)).await;
        Ok(())
    }
}

#[derive(Debug)]
struct Child;

impl Child {
    fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Actor for Child {
    async fn receive(&mut self, ctx: ContextHandle) -> Result<(), ActorError> {
        let message_handle = ctx.get_message_handle().await;
        let msg = message_handle.to_typed::<Hello>().unwrap();
        tracing::info!("Hello, {}", msg.who);
        msg.async_barrier.wait().await;
        Err(ActorError::ReceiveError(ErrorReason::new(
            "Ouch".to_string(),
            0,
        )))
    }
}

#[derive(Debug, Clone, Message)]
struct Hello {
    who: String,
    async_barrier: AsyncBarrier,
}

impl PartialEq for Hello {
    fn eq(&self, other: &Self) -> bool {
        self.who == other.who
    }
}

impl Eq for Hello {}

impl Hello {
    fn new(who: String, async_barrier: AsyncBarrier) -> Self {
        Self { who, async_barrier }
    }
}

#[tokio::main]
async fn main() {
    let _ = env::set_var("RUST_LOG", "actor_supervision=info");
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let system = ActorSystem::new().await.unwrap();
    let decider = |_| async {
        tracing::error!("occurred error");
        Directive::Stop
    };
    let supervisor = OneForOneStrategy::new(10, Duration::from_millis(1000)).with_decider(decider);
    let mut root_context = system.get_root_context().await;
    let props = Props::from_sync_actor_producer_with_opts(
        |_| Parent::new(),
        [Props::with_supervisor_strategy(
            SupervisorStrategyHandle::new(supervisor),
        )],
    )
    .await;
    let pid = root_context.spawn(props).await;
    let async_barrier = AsyncBarrier::new(2);
    root_context
        .send(
            pid,
            MessageHandle::new(Hello::new("Roger".to_string(), async_barrier.clone())),
        )
        .await;
    async_barrier.wait().await;
    sleep(Duration::from_secs(2)).await;
}
