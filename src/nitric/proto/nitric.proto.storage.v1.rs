/// ClientMessages are sent from the service to the nitric server
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
        /// Watch for changes on a bucket
        #[prost(message, tag = "2")]
        RegistrationRequest(super::RegistrationRequest),
        /// Response to a blob event (change to a blob)
        #[prost(message, tag = "3")]
        BlobEventResponse(super::BlobEventResponse),
    }
}
/// ServerMessages are sent from the nitric server to the service
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
        /// Watch for changes on a bucket
        #[prost(message, tag = "2")]
        RegistrationResponse(super::RegistrationResponse),
        /// Event for a blob in a bucket
        #[prost(message, tag = "3")]
        BlobEventRequest(super::BlobEventRequest),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlobEventRequest {
    #[prost(string, tag = "1")]
    pub bucket_name: ::prost::alloc::string::String,
    #[prost(oneof = "blob_event_request::Event", tags = "10")]
    pub event: ::core::option::Option<blob_event_request::Event>,
}
/// Nested message and enum types in `BlobEventRequest`.
pub mod blob_event_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag = "10")]
        BlobEvent(super::BlobEvent),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlobEvent {
    /// The key of the blob the event is for
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    /// The type of event that occurred
    #[prost(enumeration = "BlobEventType", tag = "2")]
    pub r#type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlobEventResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegistrationRequest {
    /// Name of the bucket to watch
    #[prost(string, tag = "1")]
    pub bucket_name: ::prost::alloc::string::String,
    /// Event type to listen for
    #[prost(enumeration = "BlobEventType", tag = "2")]
    pub blob_event_type: i32,
    /// A blob key prefix to filter events by
    #[prost(string, tag = "3")]
    pub key_prefix_filter: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegistrationResponse {
    /// The ID of the registration
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// Request to put (create/update) a storage item
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageWriteRequest {
    /// Nitric name of the bucket to store in
    ///   this will be automatically resolved to the provider specific bucket identifier.
    #[prost(string, tag = "1")]
    pub bucket_name: ::prost::alloc::string::String,
    /// Key to store the item under
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
    /// bytes array to store
    #[prost(bytes = "vec", tag = "3")]
    pub body: ::prost::alloc::vec::Vec<u8>,
}
/// Result of putting a storage item
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageWriteResponse {}
/// Request to retrieve a storage item
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageReadRequest {
    /// Nitric name of the bucket to retrieve from
    ///   this will be automatically resolved to the provider specific bucket identifier.
    #[prost(string, tag = "1")]
    pub bucket_name: ::prost::alloc::string::String,
    /// Key of item to retrieve
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
}
/// Returned storage item
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageReadResponse {
    /// The body bytes of the retrieved storage item
    #[prost(bytes = "vec", tag = "1")]
    pub body: ::prost::alloc::vec::Vec<u8>,
}
/// Request to delete a storage item
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageDeleteRequest {
    /// Name of the bucket to delete from
    #[prost(string, tag = "1")]
    pub bucket_name: ::prost::alloc::string::String,
    /// Key of item to delete
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
}
/// Result of deleting a storage item
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageDeleteResponse {}
/// Request to generate a pre-signed URL for a blob to perform a specific operation, such as read or write.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoragePreSignUrlRequest {
    /// Nitric name of the bucket to retrieve from
    ///   this will be automatically resolved to the provider specific bucket identifier.
    #[prost(string, tag = "1")]
    pub bucket_name: ::prost::alloc::string::String,
    /// Key of item to generate the signed URL for.
    /// The URL and the token it contains will only be valid for operations on this resource specifically.
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
    #[prost(enumeration = "storage_pre_sign_url_request::Operation", tag = "3")]
    pub operation: i32,
    /// Expiry defined as as protobuf duration
    #[prost(message, optional, tag = "4")]
    pub expiry: ::core::option::Option<::prost_types::Duration>,
}
/// Nested message and enum types in `StoragePreSignUrlRequest`.
pub mod storage_pre_sign_url_request {
    /// Operation
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Operation {
        Read = 0,
        Write = 1,
    }
    impl Operation {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Operation::Read => "READ",
                Operation::Write => "WRITE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "READ" => Some(Self::Read),
                "WRITE" => Some(Self::Write),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoragePreSignUrlResponse {
    /// The pre-signed url, restricted to the operation, resource and expiry time specified in the request.
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageListBlobsRequest {
    #[prost(string, tag = "1")]
    pub bucket_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub prefix: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Blob {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageListBlobsResponse {
    /// keys of the blobs in the bucket
    #[prost(message, repeated, tag = "1")]
    pub blobs: ::prost::alloc::vec::Vec<Blob>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageExistsRequest {
    #[prost(string, tag = "1")]
    pub bucket_name: ::prost::alloc::string::String,
    /// Key of item to retrieve
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageExistsResponse {
    #[prost(bool, tag = "1")]
    pub exists: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BlobEventType {
    Created = 0,
    Deleted = 1,
}
impl BlobEventType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BlobEventType::Created => "Created",
            BlobEventType::Deleted => "Deleted",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Created" => Some(Self::Created),
            "Deleted" => Some(Self::Deleted),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod storage_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Services for storage and retrieval of blobs in the form of byte arrays, such as text and binary blobs.
    #[derive(Debug, Clone)]
    pub struct StorageClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl StorageClient<tonic::transport::Channel> {
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
    impl<T> StorageClient<T>
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
        ) -> StorageClient<InterceptedService<T, F>>
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
            StorageClient::new(InterceptedService::new(inner, interceptor))
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
        /// Retrieve an item from a bucket
        pub async fn read(
            &mut self,
            request: impl tonic::IntoRequest<super::StorageReadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StorageReadResponse>,
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
                "/nitric.proto.storage.v1.Storage/Read",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("nitric.proto.storage.v1.Storage", "Read"));
            self.inner.unary(req, path, codec).await
        }
        /// Store an item to a bucket
        pub async fn write(
            &mut self,
            request: impl tonic::IntoRequest<super::StorageWriteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StorageWriteResponse>,
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
                "/nitric.proto.storage.v1.Storage/Write",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("nitric.proto.storage.v1.Storage", "Write"));
            self.inner.unary(req, path, codec).await
        }
        /// Delete an item from a bucket
        pub async fn delete(
            &mut self,
            request: impl tonic::IntoRequest<super::StorageDeleteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StorageDeleteResponse>,
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
                "/nitric.proto.storage.v1.Storage/Delete",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("nitric.proto.storage.v1.Storage", "Delete"));
            self.inner.unary(req, path, codec).await
        }
        /// Generate a pre-signed URL for direct operations on an item
        pub async fn pre_sign_url(
            &mut self,
            request: impl tonic::IntoRequest<super::StoragePreSignUrlRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StoragePreSignUrlResponse>,
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
                "/nitric.proto.storage.v1.Storage/PreSignUrl",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("nitric.proto.storage.v1.Storage", "PreSignUrl"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List blobs currently in the bucket
        pub async fn list_blobs(
            &mut self,
            request: impl tonic::IntoRequest<super::StorageListBlobsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StorageListBlobsResponse>,
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
                "/nitric.proto.storage.v1.Storage/ListBlobs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("nitric.proto.storage.v1.Storage", "ListBlobs"));
            self.inner.unary(req, path, codec).await
        }
        /// Determine is an object exists in a bucket
        pub async fn exists(
            &mut self,
            request: impl tonic::IntoRequest<super::StorageExistsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StorageExistsResponse>,
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
                "/nitric.proto.storage.v1.Storage/Exists",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("nitric.proto.storage.v1.Storage", "Exists"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod storage_listener_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct StorageListenerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl StorageListenerClient<tonic::transport::Channel> {
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
    impl<T> StorageListenerClient<T>
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
        ) -> StorageListenerClient<InterceptedService<T, F>>
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
            StorageListenerClient::new(InterceptedService::new(inner, interceptor))
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
        /// Listen for changes on a bucket
        pub async fn listen(
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
                "/nitric.proto.storage.v1.StorageListener/Listen",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("nitric.proto.storage.v1.StorageListener", "Listen"),
                );
            self.inner.streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod storage_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with StorageServer.
    #[async_trait]
    pub trait Storage: Send + Sync + 'static {
        /// Retrieve an item from a bucket
        async fn read(
            &self,
            request: tonic::Request<super::StorageReadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StorageReadResponse>,
            tonic::Status,
        >;
        /// Store an item to a bucket
        async fn write(
            &self,
            request: tonic::Request<super::StorageWriteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StorageWriteResponse>,
            tonic::Status,
        >;
        /// Delete an item from a bucket
        async fn delete(
            &self,
            request: tonic::Request<super::StorageDeleteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StorageDeleteResponse>,
            tonic::Status,
        >;
        /// Generate a pre-signed URL for direct operations on an item
        async fn pre_sign_url(
            &self,
            request: tonic::Request<super::StoragePreSignUrlRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StoragePreSignUrlResponse>,
            tonic::Status,
        >;
        /// List blobs currently in the bucket
        async fn list_blobs(
            &self,
            request: tonic::Request<super::StorageListBlobsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StorageListBlobsResponse>,
            tonic::Status,
        >;
        /// Determine is an object exists in a bucket
        async fn exists(
            &self,
            request: tonic::Request<super::StorageExistsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StorageExistsResponse>,
            tonic::Status,
        >;
    }
    /// Services for storage and retrieval of blobs in the form of byte arrays, such as text and binary blobs.
    #[derive(Debug)]
    pub struct StorageServer<T: Storage> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Storage> StorageServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for StorageServer<T>
    where
        T: Storage,
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
                "/nitric.proto.storage.v1.Storage/Read" => {
                    #[allow(non_camel_case_types)]
                    struct ReadSvc<T: Storage>(pub Arc<T>);
                    impl<
                        T: Storage,
                    > tonic::server::UnaryService<super::StorageReadRequest>
                    for ReadSvc<T> {
                        type Response = super::StorageReadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StorageReadRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Storage>::read(&inner, request).await
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
                        let method = ReadSvc(inner);
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
                "/nitric.proto.storage.v1.Storage/Write" => {
                    #[allow(non_camel_case_types)]
                    struct WriteSvc<T: Storage>(pub Arc<T>);
                    impl<
                        T: Storage,
                    > tonic::server::UnaryService<super::StorageWriteRequest>
                    for WriteSvc<T> {
                        type Response = super::StorageWriteResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StorageWriteRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Storage>::write(&inner, request).await
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
                        let method = WriteSvc(inner);
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
                "/nitric.proto.storage.v1.Storage/Delete" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteSvc<T: Storage>(pub Arc<T>);
                    impl<
                        T: Storage,
                    > tonic::server::UnaryService<super::StorageDeleteRequest>
                    for DeleteSvc<T> {
                        type Response = super::StorageDeleteResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StorageDeleteRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Storage>::delete(&inner, request).await
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
                        let method = DeleteSvc(inner);
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
                "/nitric.proto.storage.v1.Storage/PreSignUrl" => {
                    #[allow(non_camel_case_types)]
                    struct PreSignUrlSvc<T: Storage>(pub Arc<T>);
                    impl<
                        T: Storage,
                    > tonic::server::UnaryService<super::StoragePreSignUrlRequest>
                    for PreSignUrlSvc<T> {
                        type Response = super::StoragePreSignUrlResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StoragePreSignUrlRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Storage>::pre_sign_url(&inner, request).await
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
                        let method = PreSignUrlSvc(inner);
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
                "/nitric.proto.storage.v1.Storage/ListBlobs" => {
                    #[allow(non_camel_case_types)]
                    struct ListBlobsSvc<T: Storage>(pub Arc<T>);
                    impl<
                        T: Storage,
                    > tonic::server::UnaryService<super::StorageListBlobsRequest>
                    for ListBlobsSvc<T> {
                        type Response = super::StorageListBlobsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StorageListBlobsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Storage>::list_blobs(&inner, request).await
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
                        let method = ListBlobsSvc(inner);
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
                "/nitric.proto.storage.v1.Storage/Exists" => {
                    #[allow(non_camel_case_types)]
                    struct ExistsSvc<T: Storage>(pub Arc<T>);
                    impl<
                        T: Storage,
                    > tonic::server::UnaryService<super::StorageExistsRequest>
                    for ExistsSvc<T> {
                        type Response = super::StorageExistsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StorageExistsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Storage>::exists(&inner, request).await
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
                        let method = ExistsSvc(inner);
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
    impl<T: Storage> Clone for StorageServer<T> {
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
    impl<T: Storage> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Storage> tonic::server::NamedService for StorageServer<T> {
        const NAME: &'static str = "nitric.proto.storage.v1.Storage";
    }
}
/// Generated server implementations.
pub mod storage_listener_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with StorageListenerServer.
    #[async_trait]
    pub trait StorageListener: Send + Sync + 'static {
        /// Server streaming response type for the Listen method.
        type ListenStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::ServerMessage, tonic::Status>,
            >
            + Send
            + 'static;
        /// Listen for changes on a bucket
        async fn listen(
            &self,
            request: tonic::Request<tonic::Streaming<super::ClientMessage>>,
        ) -> std::result::Result<tonic::Response<Self::ListenStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct StorageListenerServer<T: StorageListener> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: StorageListener> StorageListenerServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for StorageListenerServer<T>
    where
        T: StorageListener,
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
                "/nitric.proto.storage.v1.StorageListener/Listen" => {
                    #[allow(non_camel_case_types)]
                    struct ListenSvc<T: StorageListener>(pub Arc<T>);
                    impl<
                        T: StorageListener,
                    > tonic::server::StreamingService<super::ClientMessage>
                    for ListenSvc<T> {
                        type Response = super::ServerMessage;
                        type ResponseStream = T::ListenStream;
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
                                <T as StorageListener>::listen(&inner, request).await
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
                        let method = ListenSvc(inner);
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
    impl<T: StorageListener> Clone for StorageListenerServer<T> {
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
    impl<T: StorageListener> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: StorageListener> tonic::server::NamedService for StorageListenerServer<T> {
        const NAME: &'static str = "nitric.proto.storage.v1.StorageListener";
    }
}
