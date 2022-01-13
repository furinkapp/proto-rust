//! # furink-proto
//! Rust definitions for the fur.ink gRPC backend services.

pub mod users {
    tonic::include_proto!("furink.users");
}

pub mod posts {
    tonic::include_proto!("furink.posts");
}
pub mod session {
    tonic::include_proto!("furink.session");
}
