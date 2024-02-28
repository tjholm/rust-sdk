#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiDetailsRequest {
    #[prost(string, tag = "1")]
    pub api_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiDetailsResponse {
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
}
/// ClientMessage sent by the service to the nitric server
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientMessage {
    /// globally unique ID of the request/response pair
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(oneof = "client_message::Content", tags = "2, 3")]
    pub content: ::core::option::Option<client_message::Content>,
}
/// Nested message and enum types in `ClientMessage`.
pub mod client_message {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        /// Register an API route handler
        #[prost(message, tag = "2")]
        RegistrationRequest(super::RegistrationRequest),
        /// Response to an HTTP request
        #[prost(message, tag = "3")]
        HttpResponse(super::HttpResponse),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeaderValue {
    #[prost(string, repeated, tag = "1")]
    pub value: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValue {
    #[prost(string, repeated, tag = "1")]
    pub value: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpRequest {
    /// The request method
    #[prost(string, tag = "1")]
    pub method: ::prost::alloc::string::String,
    /// The path of the request
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
    /// HTTP request headers
    #[prost(map = "string, message", tag = "3")]
    pub headers: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        HeaderValue,
    >,
    /// HTTP Query params
    #[prost(map = "string, message", tag = "4")]
    pub query_params: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        QueryValue,
    >,
    /// HTTP Path parameters
    #[prost(map = "string, string", tag = "5")]
    pub path_params: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// HTTP Request body
    #[prost(bytes = "vec", tag = "6")]
    pub body: ::prost::alloc::vec::Vec<u8>,
}
/// HttpResponseMessage
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpResponse {
    /// The HTTP response status code
    #[prost(int32, tag = "1")]
    pub status: i32,
    /// HTTP response headers
    #[prost(map = "string, message", tag = "2")]
    pub headers: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        HeaderValue,
    >,
    /// HTTP response body
    #[prost(bytes = "vec", tag = "3")]
    pub body: ::prost::alloc::vec::Vec<u8>,
}
/// ServerMessage sent by the nitric server to the service
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerMessage {
    /// globally unique ID of the request/response pair
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(oneof = "server_message::Content", tags = "2, 3")]
    pub content: ::core::option::Option<server_message::Content>,
}
/// Nested message and enum types in `ServerMessage`.
pub mod server_message {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        /// Response to an API serve request
        #[prost(message, tag = "2")]
        RegistrationResponse(super::RegistrationResponse),
        /// HTTP request to be routed to the service (handler)
        #[prost(message, tag = "3")]
        HttpRequest(super::HttpRequest),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegistrationResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiWorkerScopes {
    #[prost(string, repeated, tag = "1")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiWorkerOptions {
    /// Apply security definitions to this operation
    /// This will be mapped to named ApiSecurityDefinitionResource(s)
    #[prost(map = "string, message", tag = "1")]
    pub security: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ApiWorkerScopes,
    >,
    /// explicitly disable security for this endpoint
    /// We need to do this as the default value of a repeated field
    /// is always empty so there is no way of knowing if security is explicitly
    /// disabled
    #[prost(bool, tag = "2")]
    pub security_disabled: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegistrationRequest {
    #[prost(string, tag = "1")]
    pub api: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub methods: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "4")]
    pub options: ::core::option::Option<ApiWorkerOptions>,
}
/// Generated client implementations.
pub mod api_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for API routing and handlers
    #[derive(Debug, Clone)]
    pub struct ApiClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ApiClient<tonic::transport::Channel> {
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
    impl<T> ApiClient<T>
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
        ) -> ApiClient<InterceptedService<T, F>>
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
            ApiClient::new(InterceptedService::new(inner, interceptor))
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
        /// Serve a route on an API
        pub async fn serve(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::ClientMessage>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::ServerMessage>>,
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
                "/nitric.proto.apis.v1.Api/Serve",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("nitric.proto.apis.v1.Api", "Serve"));
            self.inner.streaming(req, path, codec).await
        }
        /// Retrieve details about an API
        pub async fn api_details(
            &mut self,
            request: impl tonic::IntoRequest<super::ApiDetailsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ApiDetailsResponse>,
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
                "/nitric.proto.apis.v1.Api/ApiDetails",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("nitric.proto.apis.v1.Api", "ApiDetails"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod api_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ApiServer.
    #[async_trait]
    pub trait Api: Send + Sync + 'static {
        /// Server streaming response type for the Serve method.
        type ServeStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::ServerMessage, tonic::Status>,
            >
            + Send
            + 'static;
        /// Serve a route on an API
        async fn serve(
            &self,
            request: tonic::Request<tonic::Streaming<super::ClientMessage>>,
        ) -> std::result::Result<tonic::Response<Self::ServeStream>, tonic::Status>;
        /// Retrieve details about an API
        async fn api_details(
            &self,
            request: tonic::Request<super::ApiDetailsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ApiDetailsResponse>,
            tonic::Status,
        >;
    }
    /// Service for API routing and handlers
    #[derive(Debug)]
    pub struct ApiServer<T: Api> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Api> ApiServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ApiServer<T>
    where
        T: Api,
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
                "/nitric.proto.apis.v1.Api/Serve" => {
                    #[allow(non_camel_case_types)]
                    struct ServeSvc<T: Api>(pub Arc<T>);
                    impl<T: Api> tonic::server::StreamingService<super::ClientMessage>
                    for ServeSvc<T> {
                        type Response = super::ServerMessage;
                        type ResponseStream = T::ServeStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::ClientMessage>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Api>::serve(&inner, request).await
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
                        let method = ServeSvc(inner);
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
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/nitric.proto.apis.v1.Api/ApiDetails" => {
                    #[allow(non_camel_case_types)]
                    struct ApiDetailsSvc<T: Api>(pub Arc<T>);
                    impl<T: Api> tonic::server::UnaryService<super::ApiDetailsRequest>
                    for ApiDetailsSvc<T> {
                        type Response = super::ApiDetailsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ApiDetailsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Api>::api_details(&inner, request).await
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
                        let method = ApiDetailsSvc(inner);
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
    impl<T: Api> Clone for ApiServer<T> {
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
    impl<T: Api> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Api> tonic::server::NamedService for ApiServer<T> {
        const NAME: &'static str = "nitric.proto.apis.v1.Api";
    }
}
