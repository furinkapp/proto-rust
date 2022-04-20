//! # furink-proto
//! Rust definitions for the fur.ink gRPC backend services.


//! Protocol version string, compiled from git revision and semver.
pub static VERSION: &'static str = env!("VERGEN_GIT_SEMVER");

pub mod discovery {
    //! DIscoery service definitions.
    tonic::include_proto!("furink.discovery");
}

pub mod users {
    //! Users service definitions.
    tonic::include_proto!("furink.users");
}

pub mod posts {
    //! Posts service definitions.
    tonic::include_proto!("furink.posts");
}
pub mod session {
    //! Session service definitions.
    tonic::include_proto!("furink.session");
}

pub mod version {
    //! Version service definitions.
    tonic::include_proto!("furink.version");
}
