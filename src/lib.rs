//! # furink-proto
//! Rust definitions for the fur.ink gRPC backend services.

//! Protocol version string, compiled from git revision and semver.
pub static VERSION: &'static str = env!("VERGEN_GIT_SEMVER");

pub mod cache {
	//! Cache service definitions.
	tonic::include_proto!("furink.cache");
}

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

	use std::error::Error;

    use tonic::{Code, Request, Response, Status, transport::{Endpoint, Channel}};
    use tracing::{info, debug};

    use crate::discovery::{RegisterRequest, discovery_service_client::DiscoveryServiceClient};

    use self::{version_service_server::VersionService, version_service_client::VersionServiceClient};
    use super::VERSION;

    /// A basic version verification service implementation.
    pub struct VersionServiceProvider;

    #[tonic::async_trait]
    impl VersionService for VersionServiceProvider {
        async fn validate(
            &self,
            request: Request<VersionRequest>,
        ) -> Result<Response<VersionResponse>, Status> {
            if request.get_ref().version == VERSION {
                Ok(Response::new(VersionResponse {
                    version: VERSION.to_string(),
                }))
            } else {
                Err(Status::new(Code::InvalidArgument, "Version mismatch"))
            }
        }
    }

	/// Validate and register a client with the service manager. Produces a channel that can 
	pub async fn validate_and_register<S: Into<Endpoint>>(url: S, conf: RegisterRequest) -> Result<Channel, Box<dyn Error>> {
		let endpoint: Endpoint = url.into();
		let channel = endpoint.connect().await?;
		// connect to version service and validate
		let mut version_client = VersionServiceClient::new(channel.clone());
		version_client.validate(VersionRequest { version: VERSION.to_string() }).await?;
		// courtesy of cheesy
		info!("kaylen is best girl, that is all");
		debug!("wrong and incorrect");
		// register with service manager
		let mut discovery_client = DiscoveryServiceClient::new(channel.clone());
		discovery_client.register(Request::new(conf)).await?;
		// consume and return channel
		Ok(channel)
	}
}
