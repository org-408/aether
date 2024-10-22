mod actor {
    pub use aether_core::generated::actor::*;
}
pub mod remote {
    #![allow(clippy::enum_variant_names)]
    include!("../generated/remote.rs");
}
pub mod remote_impl;

pub mod cluster {
    #![allow(clippy::enum_variant_names)]
    include!("../generated/cluster.rs");
}
pub mod cluster_impl;
