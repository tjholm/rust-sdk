#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeploymentUpRequest {
    /// The spec to deploy
    #[prost(message, optional, tag = "1")]
    pub spec: ::core::option::Option<Spec>,
    /// A map of attributes related to the deploy request
    /// this allows for adding project identifiers etc.
    #[prost(message, optional, tag = "2")]
    pub attributes: ::core::option::Option<::prost_types::Struct>,
    /// A hint to the provider of the kind of output that the client can accept
    /// This will allow provider developers to provider richer output back to clients.
    #[prost(bool, tag = "3")]
    pub interactive: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeploymentUpEvent {
    #[prost(oneof = "deployment_up_event::Content", tags = "1, 2, 3")]
    pub content: ::core::option::Option<deployment_up_event::Content>,
}
/// Nested message and enum types in `DeploymentUpEvent`.
pub mod deployment_up_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        #[prost(string, tag = "1")]
        Message(::prost::alloc::string::String),
        #[prost(message, tag = "2")]
        Update(super::ResourceUpdate),
        #[prost(message, tag = "3")]
        Result(super::UpResult),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceUpdate {
    /// The resource being updated, if this is nil the update applies to the stack
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::super::resources::v1::ResourceIdentifier>,
    /// The type of update being applied
    #[prost(enumeration = "ResourceDeploymentAction", tag = "3")]
    pub action: i32,
    /// The current status of the action being applied
    #[prost(enumeration = "ResourceDeploymentStatus", tag = "4")]
    pub status: i32,
    /// (optional) A globally unique identifier (scoped to the id above), used when Nitric Resources map 1:many in a cloud provider.
    /// e.g. the container image repository for a service deployment.
    /// This can also be set when id is nil above and it will imply a non-nitric resource that is necessary to deploy for a stack to operate
    /// e.g. an Azure StorageAccount
    #[prost(string, tag = "5")]
    pub sub_resource: ::prost::alloc::string::String,
    /// Additional information about the update
    #[prost(string, tag = "6")]
    pub message: ::prost::alloc::string::String,
}
/// Terminal message indicating deployment success
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpResult {
    /// Indicate the success status
    #[prost(bool, tag = "1")]
    pub success: bool,
    #[prost(oneof = "up_result::Content", tags = "2")]
    pub content: ::core::option::Option<up_result::Content>,
}
/// Nested message and enum types in `UpResult`.
pub mod up_result {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        /// Simple text output as result
        #[prost(string, tag = "2")]
        Text(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeploymentDownRequest {
    /// A map of attributes related to the deploy request
    /// this allows for adding project identifiers etc.
    #[prost(message, optional, tag = "1")]
    pub attributes: ::core::option::Option<::prost_types::Struct>,
    /// A hint to the provider of the kind of output that the client can accept
    /// This will allow provider developers to provider richer output back to clients.
    #[prost(bool, tag = "2")]
    pub interactive: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeploymentDownEvent {
    #[prost(oneof = "deployment_down_event::Content", tags = "1, 2, 3")]
    pub content: ::core::option::Option<deployment_down_event::Content>,
}
/// Nested message and enum types in `DeploymentDownEvent`.
pub mod deployment_down_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        #[prost(string, tag = "1")]
        Message(::prost::alloc::string::String),
        #[prost(message, tag = "2")]
        Result(super::DownResult),
        #[prost(message, tag = "3")]
        Update(super::ResourceUpdate),
    }
}
/// Terminal message indicating deployment success
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownResult {}
/// An image source to be used for service deployment
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageSource {
    /// URI of the docker image
    /// To support remote images this may also need to provide auth information
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
}
/// A unit of compute (i.e. function/container)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Service {
    /// Expected worker count for this service
    #[prost(int32, tag = "10")]
    pub workers: i32,
    /// Configurable timeout for request handling
    #[deprecated]
    #[prost(int32, tag = "11")]
    pub timeout: i32,
    /// Configurable memory size for this instance
    #[deprecated]
    #[prost(int32, tag = "12")]
    pub memory: i32,
    /// A simple type property
    /// describes the requested type of service that this should be
    /// for this project, a provider can implement how this request is satisfied
    /// in any way
    #[prost(string, tag = "13")]
    pub r#type: ::prost::alloc::string::String,
    /// Environment variables for this service
    #[prost(map = "string, string", tag = "14")]
    pub env: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Source of the service
    #[prost(oneof = "service::Source", tags = "1")]
    pub source: ::core::option::Option<service::Source>,
}
/// Nested message and enum types in `Service`.
pub mod service {
    /// Source of the service
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// Container image as a service
        ///
        /// Alternative sources could include
        /// - zipped code sources
        /// - git/scm repository URIs
        #[prost(message, tag = "1")]
        Image(super::ImageSource),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bucket {
    #[prost(message, repeated, tag = "1")]
    pub listeners: ::prost::alloc::vec::Vec<BucketListener>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BucketListener {
    #[prost(message, optional, tag = "1")]
    pub config: ::core::option::Option<super::super::storage::v1::RegistrationRequest>,
    #[prost(oneof = "bucket_listener::Target", tags = "2")]
    pub target: ::core::option::Option<bucket_listener::Target>,
}
/// Nested message and enum types in `BucketListener`.
pub mod bucket_listener {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Target {
        /// The name of an service to target
        #[prost(string, tag = "2")]
        Service(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Topic {
    #[prost(message, repeated, tag = "1")]
    pub subscriptions: ::prost::alloc::vec::Vec<SubscriptionTarget>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Queue {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyValueStore {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Secret {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscriptionTarget {
    #[prost(oneof = "subscription_target::Target", tags = "1")]
    pub target: ::core::option::Option<subscription_target::Target>,
}
/// Nested message and enum types in `SubscriptionTarget`.
pub mod subscription_target {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Target {
        /// The name of an service to target
        ///
        /// Additional targets could potentially include
        /// - HTTP/API Endpoints
        /// - Queues
        #[prost(string, tag = "1")]
        Service(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicSubscription {
    #[prost(message, optional, tag = "1")]
    pub target: ::core::option::Option<SubscriptionTarget>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpTarget {
    #[prost(oneof = "http_target::Target", tags = "1")]
    pub target: ::core::option::Option<http_target::Target>,
}
/// Nested message and enum types in `HttpTarget`.
pub mod http_target {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Target {
        /// The name of an service to target
        #[prost(string, tag = "1")]
        Service(::prost::alloc::string::String),
    }
}
/// An http proxy resource
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Http {
    #[prost(message, optional, tag = "1")]
    pub target: ::core::option::Option<HttpTarget>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Api {
    #[prost(oneof = "api::Document", tags = "1")]
    pub document: ::core::option::Option<api::Document>,
}
/// Nested message and enum types in `Api`.
pub mod api {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Document {
        /// An OpenAPI document for deployment
        /// This document will contain extensions that hint of services that should be targeted as part of the deployment
        #[prost(string, tag = "1")]
        Openapi(::prost::alloc::string::String),
    }
}
/// Declare a new websocket
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Websocket {
    /// Target for handling new client connections
    #[prost(message, optional, tag = "1")]
    pub connect_target: ::core::option::Option<WebsocketTarget>,
    /// Target for handling client disconnections
    #[prost(message, optional, tag = "2")]
    pub disconnect_target: ::core::option::Option<WebsocketTarget>,
    /// Target for handling all other message types
    #[prost(message, optional, tag = "3")]
    pub message_target: ::core::option::Option<WebsocketTarget>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebsocketTarget {
    #[prost(oneof = "websocket_target::Target", tags = "1")]
    pub target: ::core::option::Option<websocket_target::Target>,
}
/// Nested message and enum types in `WebsocketTarget`.
pub mod websocket_target {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Target {
        /// The name of an service to target
        ///
        /// Additional targets could potentially include
        /// - HTTP/API Endpoints
        #[prost(string, tag = "1")]
        Service(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScheduleTarget {
    #[prost(oneof = "schedule_target::Target", tags = "1")]
    pub target: ::core::option::Option<schedule_target::Target>,
}
/// Nested message and enum types in `ScheduleTarget`.
pub mod schedule_target {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Target {
        /// The name of an service to target
        ///
        /// Additional targets could potentially include
        /// - HTTP/API Endpoints
        #[prost(string, tag = "1")]
        Service(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Schedule {
    #[prost(message, optional, tag = "1")]
    pub target: ::core::option::Option<ScheduleTarget>,
    #[prost(oneof = "schedule::Cadence", tags = "10, 11")]
    pub cadence: ::core::option::Option<schedule::Cadence>,
}
/// Nested message and enum types in `Schedule`.
pub mod schedule {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Cadence {
        #[prost(message, tag = "10")]
        Every(super::ScheduleEvery),
        #[prost(message, tag = "11")]
        Cron(super::ScheduleCron),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScheduleEvery {
    /// rate string e.g. '5 minutes'. Value frequencies are 'minutes', 'hours', 'days'.
    #[prost(string, tag = "1")]
    pub rate: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScheduleCron {
    /// standard unix cron expression
    #[prost(string, tag = "1")]
    pub expression: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resource {
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::super::resources::v1::ResourceIdentifier>,
    #[prost(
        oneof = "resource::Config",
        tags = "10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20"
    )]
    pub config: ::core::option::Option<resource::Config>,
}
/// Nested message and enum types in `Resource`.
pub mod resource {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Config {
        #[prost(message, tag = "10")]
        Service(super::Service),
        #[prost(message, tag = "11")]
        Bucket(super::Bucket),
        #[prost(message, tag = "12")]
        Topic(super::Topic),
        #[prost(message, tag = "13")]
        Api(super::Api),
        #[prost(message, tag = "14")]
        Policy(super::Policy),
        #[prost(message, tag = "15")]
        Schedule(super::Schedule),
        #[prost(message, tag = "16")]
        KeyValueStore(super::KeyValueStore),
        #[prost(message, tag = "17")]
        Secret(super::Secret),
        #[prost(message, tag = "18")]
        Websocket(super::Websocket),
        #[prost(message, tag = "19")]
        Http(super::Http),
        #[prost(message, tag = "20")]
        Queue(super::Queue),
    }
}
/// This is already defined in the resource contracts,
/// unfortunately there are parts we don't want to duplicate, such as API config
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Policy {
    #[prost(message, repeated, tag = "1")]
    pub principals: ::prost::alloc::vec::Vec<Resource>,
    #[prost(enumeration = "super::super::resources::v1::Action", repeated, tag = "2")]
    pub actions: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, repeated, tag = "3")]
    pub resources: ::prost::alloc::vec::Vec<Resource>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Spec {
    /// list of resources to deploy
    #[prost(message, repeated, tag = "1")]
    pub resources: ::prost::alloc::vec::Vec<Resource>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ResourceDeploymentAction {
    /// A new resource is being created
    Create = 0,
    /// An existing resource is being updated
    Update = 1,
    /// An existing resource is being replaced
    Replace = 2,
    /// No-op on the resource (it already exists and requires no changes)
    Same = 3,
    /// An existing resource is being deleted
    Delete = 4,
}
impl ResourceDeploymentAction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ResourceDeploymentAction::Create => "CREATE",
            ResourceDeploymentAction::Update => "UPDATE",
            ResourceDeploymentAction::Replace => "REPLACE",
            ResourceDeploymentAction::Same => "SAME",
            ResourceDeploymentAction::Delete => "DELETE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CREATE" => Some(Self::Create),
            "UPDATE" => Some(Self::Update),
            "REPLACE" => Some(Self::Replace),
            "SAME" => Some(Self::Same),
            "DELETE" => Some(Self::Delete),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ResourceDeploymentStatus {
    /// The action hasn't started, usually due to a dependency
    Pending = 0,
    /// The action in currently in-flight, e.g. waiting for cloud provder to provision a resource
    InProgress = 1,
    /// The action has been applied successfully
    Success = 2,
    /// The action has failed to be (completely) applied
    Failed = 3,
}
impl ResourceDeploymentStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ResourceDeploymentStatus::Pending => "PENDING",
            ResourceDeploymentStatus::InProgress => "IN_PROGRESS",
            ResourceDeploymentStatus::Success => "SUCCESS",
            ResourceDeploymentStatus::Failed => "FAILED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PENDING" => Some(Self::Pending),
            "IN_PROGRESS" => Some(Self::InProgress),
            "SUCCESS" => Some(Self::Success),
            "FAILED" => Some(Self::Failed),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod deployment_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// The Nitric Deloyment Service contract
    #[derive(Debug, Clone)]
    pub struct DeploymentClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DeploymentClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> DeploymentClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> DeploymentClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            DeploymentClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Begins a new deployment
        /// Server will stream updates back to the connected client
        /// on the status of the deployment
        pub async fn up(
            &mut self,
            request: impl tonic::IntoRequest<super::DeploymentUpRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::DeploymentUpEvent>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/nitric.proto.deployments.v1.Deployment/Up",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("nitric.proto.deployments.v1.Deployment", "Up"));
            self.inner.server_streaming(req, path, codec).await
        }
        /// Tears down an existing deployment
        /// Server will stream updates back to the connected client
        /// on the status of the teardown
        pub async fn down(
            &mut self,
            request: impl tonic::IntoRequest<super::DeploymentDownRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::DeploymentDownEvent>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/nitric.proto.deployments.v1.Deployment/Down",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("nitric.proto.deployments.v1.Deployment", "Down"),
                );
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod deployment_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with DeploymentServer.
    #[async_trait]
    pub trait Deployment: Send + Sync + 'static {
        /// Server streaming response type for the Up method.
        type UpStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::DeploymentUpEvent, tonic::Status>,
            >
            + Send
            + 'static;
        /// Begins a new deployment
        /// Server will stream updates back to the connected client
        /// on the status of the deployment
        async fn up(
            &self,
            request: tonic::Request<super::DeploymentUpRequest>,
        ) -> std::result::Result<tonic::Response<Self::UpStream>, tonic::Status>;
        /// Server streaming response type for the Down method.
        type DownStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::DeploymentDownEvent, tonic::Status>,
            >
            + Send
            + 'static;
        /// Tears down an existing deployment
        /// Server will stream updates back to the connected client
        /// on the status of the teardown
        async fn down(
            &self,
            request: tonic::Request<super::DeploymentDownRequest>,
        ) -> std::result::Result<tonic::Response<Self::DownStream>, tonic::Status>;
    }
    /// The Nitric Deloyment Service contract
    #[derive(Debug)]
    pub struct DeploymentServer<T: Deployment> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Deployment> DeploymentServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for DeploymentServer<T>
    where
        T: Deployment,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/nitric.proto.deployments.v1.Deployment/Up" => {
                    #[allow(non_camel_case_types)]
                    struct UpSvc<T: Deployment>(pub Arc<T>);
                    impl<
                        T: Deployment,
                    > tonic::server::ServerStreamingService<super::DeploymentUpRequest>
                    for UpSvc<T> {
                        type Response = super::DeploymentUpEvent;
                        type ResponseStream = T::UpStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeploymentUpRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Deployment>::up(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/nitric.proto.deployments.v1.Deployment/Down" => {
                    #[allow(non_camel_case_types)]
                    struct DownSvc<T: Deployment>(pub Arc<T>);
                    impl<
                        T: Deployment,
                    > tonic::server::ServerStreamingService<super::DeploymentDownRequest>
                    for DownSvc<T> {
                        type Response = super::DeploymentDownEvent;
                        type ResponseStream = T::DownStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeploymentDownRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Deployment>::down(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DownSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Deployment> Clone for DeploymentServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Deployment> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Deployment> tonic::server::NamedService for DeploymentServer<T> {
        const NAME: &'static str = "nitric.proto.deployments.v1.Deployment";
    }
}
