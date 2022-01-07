//! # furink-proto
//! Rust definitions for the fur.ink gRPC backend services.

pub mod database {
    tonic::include_proto!("furink.database");
}
pub mod session {
    tonic::include_proto!("furink.session");
}
