mod actor {
    pub use aether_core::generated::actor::*;
}

pub mod cluster {
    include!("../generated/cluster.rs");
}
pub mod cluster_impl;
