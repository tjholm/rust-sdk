#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyResource {
    #[prost(message, repeated, tag = "1")]
    pub principals: ::prost::alloc::vec::Vec<ResourceIdentifier>,
    #[prost(enumeration = "Action", repeated, tag = "2")]
    pub actions: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, repeated, tag = "3")]
    pub resources: ::prost::alloc::vec::Vec<ResourceIdentifier>,
}
/// Unique identifier for a resource within a nitric application.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceIdentifier {
    #[prost(enumeration = "ResourceType", tag = "1")]
    pub r#type: i32,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceDeclareRequest {
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<ResourceIdentifier>,
    #[prost(
        oneof = "resource_declare_request::Config",
        tags = "10, 11, 12, 13, 14, 15, 16, 17"
    )]
    pub config: ::core::option::Option<resource_declare_request::Config>,
}
/// Nested message and enum types in `ResourceDeclareRequest`.
pub mod resource_declare_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Config {
        #[prost(message, tag = "10")]
        Policy(super::PolicyResource),
        #[prost(message, tag = "11")]
        Bucket(super::BucketResource),
        #[prost(message, tag = "12")]
        Topic(super::TopicResource),
        #[prost(message, tag = "13")]
        KeyValueStore(super::KeyValueStoreResource),
        #[prost(message, tag = "14")]
        Secret(super::SecretResource),
        #[prost(message, tag = "15")]
        Api(super::ApiResource),
        #[prost(message, tag = "16")]
        ApiSecurityDefinition(super::ApiSecurityDefinitionResource),
        #[prost(message, tag = "17")]
        Queue(super::QueueResource),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BucketResource {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicResource {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueueResource {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyValueStoreResource {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecretResource {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiOpenIdConnectionDefinition {
    #[prost(string, tag = "1")]
    pub issuer: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub audiences: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiSecurityDefinitionResource {
    #[prost(string, tag = "1")]
    pub api_name: ::prost::alloc::string::String,
    #[prost(oneof = "api_security_definition_resource::Definition", tags = "2")]
    pub definition: ::core::option::Option<api_security_definition_resource::Definition>,
}
/// Nested message and enum types in `ApiSecurityDefinitionResource`.
pub mod api_security_definition_resource {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Definition {
        #[prost(message, tag = "2")]
        Oidc(super::ApiOpenIdConnectionDefinition),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiScopes {
    #[prost(string, repeated, tag = "1")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiResource {
    /// root level security map for this api
    /// references ApiSecurityDefinitionResource(s)
    #[prost(map = "string, message", tag = "1")]
    pub security: ::std::collections::HashMap<::prost::alloc::string::String, ApiScopes>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceDeclareResponse {}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ResourceType {
    Api = 0,
    Service = 1,
    Bucket = 2,
    Topic = 3,
    Schedule = 4,
    Subscription = 5,
    KeyValueStore = 6,
    Policy = 7,
    Secret = 8,
    BucketListener = 9,
    Websocket = 10,
    Http = 11,
    ApiSecurityDefinition = 12,
    Queue = 13,
}
impl ResourceType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ResourceType::Api => "Api",
            ResourceType::Service => "Service",
            ResourceType::Bucket => "Bucket",
            ResourceType::Topic => "Topic",
            ResourceType::Schedule => "Schedule",
            ResourceType::Subscription => "Subscription",
            ResourceType::KeyValueStore => "KeyValueStore",
            ResourceType::Policy => "Policy",
            ResourceType::Secret => "Secret",
            ResourceType::BucketListener => "BucketListener",
            ResourceType::Websocket => "Websocket",
            ResourceType::Http => "Http",
            ResourceType::ApiSecurityDefinition => "ApiSecurityDefinition",
            ResourceType::Queue => "Queue",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Api" => Some(Self::Api),
            "Service" => Some(Self::Service),
            "Bucket" => Some(Self::Bucket),
            "Topic" => Some(Self::Topic),
            "Schedule" => Some(Self::Schedule),
            "Subscription" => Some(Self::Subscription),
            "KeyValueStore" => Some(Self::KeyValueStore),
            "Policy" => Some(Self::Policy),
            "Secret" => Some(Self::Secret),
            "BucketListener" => Some(Self::BucketListener),
            "Websocket" => Some(Self::Websocket),
            "Http" => Some(Self::Http),
            "ApiSecurityDefinition" => Some(Self::ApiSecurityDefinition),
            "Queue" => Some(Self::Queue),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Action {
    /// Bucket Permissions: 0XX
    BucketFileList = 0,
    BucketFileGet = 1,
    BucketFilePut = 2,
    BucketFileDelete = 3,
    /// Topic Permissions: 2XX
    TopicPublish = 200,
    /// KeyValue Store Permissions: 3XX
    KeyValueStoreRead = 300,
    KeyValueStoreWrite = 301,
    KeyValueStoreDelete = 302,
    /// Secret Permissions: 4XX
    SecretPut = 400,
    SecretAccess = 401,
    /// Websocket Permissions: 5XX
    WebsocketManage = 500,
    /// Queue Permissions: 6XX
    QueueEnqueue = 600,
    QueueDequeue = 601,
}
impl Action {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Action::BucketFileList => "BucketFileList",
            Action::BucketFileGet => "BucketFileGet",
            Action::BucketFilePut => "BucketFilePut",
            Action::BucketFileDelete => "BucketFileDelete",
            Action::TopicPublish => "TopicPublish",
            Action::KeyValueStoreRead => "KeyValueStoreRead",
            Action::KeyValueStoreWrite => "KeyValueStoreWrite",
            Action::KeyValueStoreDelete => "KeyValueStoreDelete",
            Action::SecretPut => "SecretPut",
            Action::SecretAccess => "SecretAccess",
            Action::WebsocketManage => "WebsocketManage",
            Action::QueueEnqueue => "QueueEnqueue",
            Action::QueueDequeue => "QueueDequeue",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BucketFileList" => Some(Self::BucketFileList),
            "BucketFileGet" => Some(Self::BucketFileGet),
            "BucketFilePut" => Some(Self::BucketFilePut),
            "BucketFileDelete" => Some(Self::BucketFileDelete),
            "TopicPublish" => Some(Self::TopicPublish),
            "KeyValueStoreRead" => Some(Self::KeyValueStoreRead),
            "KeyValueStoreWrite" => Some(Self::KeyValueStoreWrite),
            "KeyValueStoreDelete" => Some(Self::KeyValueStoreDelete),
            "SecretPut" => Some(Self::SecretPut),
            "SecretAccess" => Some(Self::SecretAccess),
            "WebsocketManage" => Some(Self::WebsocketManage),
            "QueueEnqueue" => Some(Self::QueueEnqueue),
            "QueueDequeue" => Some(Self::QueueDequeue),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod resources_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Nitric Resource Service
    /// The service definition exists to allow a nitric application to declare peripheral dependencies
    #[derive(Debug, Clone)]
    pub struct ResourcesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ResourcesClient<tonic::transport::Channel> {
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
    impl<T> ResourcesClient<T>
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
        ) -> ResourcesClient<InterceptedService<T, F>>
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
            ResourcesClient::new(InterceptedService::new(inner, interceptor))
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
        /// Declare a resource for the nitric application
        /// At Deploy time this will create resources as part of the nitric stacks dependency graph
        /// At runtime
        pub async fn declare(
            &mut self,
            request: impl tonic::IntoRequest<super::ResourceDeclareRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResourceDeclareResponse>,
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
                "/nitric.proto.resources.v1.Resources/Declare",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("nitric.proto.resources.v1.Resources", "Declare"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod resources_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ResourcesServer.
    #[async_trait]
    pub trait Resources: Send + Sync + 'static {
        /// Declare a resource for the nitric application
        /// At Deploy time this will create resources as part of the nitric stacks dependency graph
        /// At runtime
        async fn declare(
            &self,
            request: tonic::Request<super::ResourceDeclareRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResourceDeclareResponse>,
            tonic::Status,
        >;
    }
    /// Nitric Resource Service
    /// The service definition exists to allow a nitric application to declare peripheral dependencies
    #[derive(Debug)]
    pub struct ResourcesServer<T: Resources> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Resources> ResourcesServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ResourcesServer<T>
    where
        T: Resources,
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
                "/nitric.proto.resources.v1.Resources/Declare" => {
                    #[allow(non_camel_case_types)]
                    struct DeclareSvc<T: Resources>(pub Arc<T>);
                    impl<
                        T: Resources,
                    > tonic::server::UnaryService<super::ResourceDeclareRequest>
                    for DeclareSvc<T> {
                        type Response = super::ResourceDeclareResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ResourceDeclareRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Resources>::declare(&inner, request).await
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
                        let method = DeclareSvc(inner);
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
                        let res = grpc.unary(method, req).await;
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
    impl<T: Resources> Clone for ResourcesServer<T> {
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
    impl<T: Resources> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Resources> tonic::server::NamedService for ResourcesServer<T> {
        const NAME: &'static str = "nitric.proto.resources.v1.Resources";
    }
}
