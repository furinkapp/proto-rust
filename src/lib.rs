//! # furink-proto
//! Rust definitions for the fur.ink gRPC backend services.

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
