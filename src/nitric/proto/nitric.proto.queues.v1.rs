#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueueEnqueueRequest {
    /// The Nitric name for the queue
    /// this will automatically be resolved to the provider specific queue identifier.
    #[prost(string, tag = "1")]
    pub queue_name: ::prost::alloc::string::String,
    /// Array of messages to push to the queue
    #[prost(message, repeated, tag = "2")]
    pub messages: ::prost::alloc::vec::Vec<QueueMessage>,
}
/// Response for sending messages to a queue
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueueEnqueueResponse {
    /// A list of messages that failed to be queued
    #[prost(message, repeated, tag = "1")]
    pub failed_messages: ::prost::alloc::vec::Vec<FailedEnqueueMessage>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueueDequeueRequest {
    /// The nitric name for the queue
    /// this will automatically be resolved to the provider specific queue identifier.
    #[prost(string, tag = "1")]
    pub queue_name: ::prost::alloc::string::String,
    /// The max number of messages to pop off the queue, may be capped by provider specific limitations
    #[prost(int32, tag = "2")]
    pub depth: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueueDequeueResponse {
    /// Array of messages popped off the queue
    #[prost(message, repeated, tag = "1")]
    pub messages: ::prost::alloc::vec::Vec<DequeuedMessage>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueueCompleteRequest {
    /// The nitric name for the queue
    ///   this will automatically be resolved to the provider specific queue identifier.
    #[prost(string, tag = "1")]
    pub queue_name: ::prost::alloc::string::String,
    /// Lease id of the message to be completed
    #[prost(string, tag = "2")]
    pub lease_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueueCompleteResponse {}
/// An message to be sent to a queue.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueueMessage {
    /// The queue message contents
    #[prost(oneof = "queue_message::Content", tags = "1")]
    pub content: ::core::option::Option<queue_message::Content>,
}
/// Nested message and enum types in `QueueMessage`.
pub mod queue_message {
    /// The queue message contents
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        #[prost(message, tag = "1")]
        StructPayload(::prost_types::Struct),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DequeuedMessage {
    #[prost(string, tag = "1")]
    pub lease_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub message: ::core::option::Option<QueueMessage>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FailedEnqueueMessage {
    /// The message that failed to be pushed
    #[prost(message, optional, tag = "1")]
    pub message: ::core::option::Option<QueueMessage>,
    /// A description of the failure
    #[prost(string, tag = "2")]
    pub details: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod queues_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// The Nitric Queue Service contract
    #[derive(Debug, Clone)]
    pub struct QueuesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl QueuesClient<tonic::transport::Channel> {
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
    impl<T> QueuesClient<T>
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
        ) -> QueuesClient<InterceptedService<T, F>>
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
            QueuesClient::new(InterceptedService::new(inner, interceptor))
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
        /// Send message(s) to a queue
        pub async fn enqueue(
            &mut self,
            request: impl tonic::IntoRequest<super::QueueEnqueueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueueEnqueueResponse>,
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
                "/nitric.proto.queues.v1.Queues/Enqueue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("nitric.proto.queues.v1.Queues", "Enqueue"));
            self.inner.unary(req, path, codec).await
        }
        /// Receive message(s) from a queue
        pub async fn dequeue(
            &mut self,
            request: impl tonic::IntoRequest<super::QueueDequeueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueueDequeueResponse>,
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
                "/nitric.proto.queues.v1.Queues/Dequeue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("nitric.proto.queues.v1.Queues", "Dequeue"));
            self.inner.unary(req, path, codec).await
        }
        /// Complete an message previously popped from a queue
        pub async fn complete(
            &mut self,
            request: impl tonic::IntoRequest<super::QueueCompleteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueueCompleteResponse>,
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
                "/nitric.proto.queues.v1.Queues/Complete",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("nitric.proto.queues.v1.Queues", "Complete"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod queues_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with QueuesServer.
    #[async_trait]
    pub trait Queues: Send + Sync + 'static {
        /// Send message(s) to a queue
        async fn enqueue(
            &self,
            request: tonic::Request<super::QueueEnqueueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueueEnqueueResponse>,
            tonic::Status,
        >;
        /// Receive message(s) from a queue
        async fn dequeue(
            &self,
            request: tonic::Request<super::QueueDequeueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueueDequeueResponse>,
            tonic::Status,
        >;
        /// Complete an message previously popped from a queue
        async fn complete(
            &self,
            request: tonic::Request<super::QueueCompleteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueueCompleteResponse>,
            tonic::Status,
        >;
    }
    /// The Nitric Queue Service contract
    #[derive(Debug)]
    pub struct QueuesServer<T: Queues> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Queues> QueuesServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for QueuesServer<T>
    where
        T: Queues,
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
                "/nitric.proto.queues.v1.Queues/Enqueue" => {
                    #[allow(non_camel_case_types)]
                    struct EnqueueSvc<T: Queues>(pub Arc<T>);
                    impl<
                        T: Queues,
                    > tonic::server::UnaryService<super::QueueEnqueueRequest>
                    for EnqueueSvc<T> {
                        type Response = super::QueueEnqueueResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueueEnqueueRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Queues>::enqueue(&inner, request).await
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
                        let method = EnqueueSvc(inner);
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
                "/nitric.proto.queues.v1.Queues/Dequeue" => {
                    #[allow(non_camel_case_types)]
                    struct DequeueSvc<T: Queues>(pub Arc<T>);
                    impl<
                        T: Queues,
                    > tonic::server::UnaryService<super::QueueDequeueRequest>
                    for DequeueSvc<T> {
                        type Response = super::QueueDequeueResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueueDequeueRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Queues>::dequeue(&inner, request).await
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
                        let method = DequeueSvc(inner);
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
                "/nitric.proto.queues.v1.Queues/Complete" => {
                    #[allow(non_camel_case_types)]
                    struct CompleteSvc<T: Queues>(pub Arc<T>);
                    impl<
                        T: Queues,
                    > tonic::server::UnaryService<super::QueueCompleteRequest>
                    for CompleteSvc<T> {
                        type Response = super::QueueCompleteResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueueCompleteRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Queues>::complete(&inner, request).await
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
                        let method = CompleteSvc(inner);
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
    impl<T: Queues> Clone for QueuesServer<T> {
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
    impl<T: Queues> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Queues> tonic::server::NamedService for QueuesServer<T> {
        const NAME: &'static str = "nitric.proto.queues.v1.Queues";
    }
}
