/// Update request to the `RaftApp`.
/// This type of request is serialized in the log and processed sequentially.
/// `request_id` is unique identifier of the request to avoid executing duplicating requests.
/// Client may send a write requests twice but they are executed only once as long as
/// they have the same `request_id`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteRequest {
    #[prost(uint32, tag = "1")]
    pub lane_id: u32,
    #[prost(bytes = "bytes", tag = "2")]
    pub message: ::prost::bytes::Bytes,
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Read-only request to the `RaftApp`.
/// This type of request is processed in optimized path.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadRequest {
    #[prost(uint32, tag = "1")]
    pub lane_id: u32,
    #[prost(bytes = "bytes", tag = "2")]
    pub message: ::prost::bytes::Bytes,
}
/// Response from the `RaftApp`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(bytes = "bytes", tag = "1")]
    pub message: ::prost::bytes::Bytes,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Clock {
    #[prost(uint64, tag = "1")]
    pub term: u64,
    #[prost(uint64, tag = "2")]
    pub index: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KernRequest {
    #[prost(uint32, tag = "1")]
    pub lane_id: u32,
    #[prost(bytes = "bytes", tag = "2")]
    pub message: ::prost::bytes::Bytes,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicationStreamHeader {
    #[prost(uint32, tag = "1")]
    pub lane_id: u32,
    #[prost(string, tag = "2")]
    pub sender_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub prev_clock: ::core::option::Option<Clock>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicationStreamEntry {
    #[prost(message, optional, tag = "1")]
    pub clock: ::core::option::Option<Clock>,
    #[prost(bytes = "bytes", tag = "2")]
    pub command: ::prost::bytes::Bytes,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicationStreamChunk {
    #[prost(oneof = "replication_stream_chunk::Elem", tags = "1, 2")]
    pub elem: ::core::option::Option<replication_stream_chunk::Elem>,
}
/// Nested message and enum types in `ReplicationStreamChunk`.
pub mod replication_stream_chunk {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Elem {
        #[prost(message, tag = "1")]
        Header(super::ReplicationStreamHeader),
        #[prost(message, tag = "2")]
        Entry(super::ReplicationStreamEntry),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicationStreamResponse {
    #[prost(uint64, tag = "1")]
    pub n_inserted: u64,
    #[prost(uint64, tag = "2")]
    pub log_last_index: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSnapshotRequest {
    #[prost(uint32, tag = "1")]
    pub lane_id: u32,
    #[prost(uint64, tag = "2")]
    pub index: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotChunk {
    #[prost(bytes = "bytes", tag = "1")]
    pub data: ::prost::bytes::Bytes,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoteRequest {
    #[prost(uint32, tag = "1")]
    pub lane_id: u32,
    #[prost(uint64, tag = "2")]
    pub vote_term: u64,
    #[prost(string, tag = "3")]
    pub candidate_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub candidate_clock: ::core::option::Option<Clock>,
    #[prost(bool, tag = "5")]
    pub force_vote: bool,
    #[prost(bool, tag = "6")]
    pub pre_vote: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoteResponse {
    #[prost(bool, tag = "1")]
    pub vote_granted: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaderCommitState {
    #[prost(uint64, tag = "1")]
    pub leader_term: u64,
    #[prost(uint64, tag = "2")]
    pub leader_commit_index: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Heartbeat {
    #[prost(string, tag = "1")]
    pub leader_id: ::prost::alloc::string::String,
    #[prost(map = "uint32, message", tag = "2")]
    pub leader_commit_states: ::std::collections::HashMap<u32, LeaderCommitState>,
}
/// Request to add a Raft process with `server_id` to a lane.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddServerRequest {
    #[prost(uint32, tag = "1")]
    pub lane_id: u32,
    #[prost(string, tag = "2")]
    pub server_id: ::prost::alloc::string::String,
}
/// Request to remove a Raft process with `server_id` from a lane.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveServerRequest {
    #[prost(uint32, tag = "1")]
    pub lane_id: u32,
    #[prost(string, tag = "2")]
    pub server_id: ::prost::alloc::string::String,
}
/// On receiving this request, a server starts a new election
/// to become a leader disregarding the election timeout.
/// You can use this request to rebalance the leaders in the cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeoutNow {
    #[prost(uint32, tag = "1")]
    pub lane_id: u32,
}
/// Generated client implementations.
pub mod raft_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct RaftClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RaftClient<tonic::transport::Channel> {
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
    impl<T> RaftClient<T>
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
        ) -> RaftClient<InterceptedService<T, F>>
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
            RaftClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn write(
            &mut self,
            request: impl tonic::IntoRequest<super::WriteRequest>,
        ) -> std::result::Result<tonic::Response<super::Response>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/lolraft.Raft/Write");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("lolraft.Raft", "Write"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn read(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadRequest>,
        ) -> std::result::Result<tonic::Response<super::Response>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/lolraft.Raft/Read");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("lolraft.Raft", "Read"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn process_kern_request(
            &mut self,
            request: impl tonic::IntoRequest<super::KernRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/lolraft.Raft/ProcessKernRequest",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("lolraft.Raft", "ProcessKernRequest"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_vote(
            &mut self,
            request: impl tonic::IntoRequest<super::VoteRequest>,
        ) -> std::result::Result<tonic::Response<super::VoteResponse>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/lolraft.Raft/RequestVote");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("lolraft.Raft", "RequestVote"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_server(
            &mut self,
            request: impl tonic::IntoRequest<super::AddServerRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/lolraft.Raft/AddServer");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("lolraft.Raft", "AddServer"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_server(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveServerRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/lolraft.Raft/RemoveServer",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("lolraft.Raft", "RemoveServer"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn send_replication_stream(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::ReplicationStreamChunk,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ReplicationStreamResponse>,
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
                "/lolraft.Raft/SendReplicationStream",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("lolraft.Raft", "SendReplicationStream"));
            self.inner.client_streaming(req, path, codec).await
        }
        pub async fn get_snapshot(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSnapshotRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::SnapshotChunk>>,
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
            let path = http::uri::PathAndQuery::from_static("/lolraft.Raft/GetSnapshot");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("lolraft.Raft", "GetSnapshot"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn send_heartbeat(
            &mut self,
            request: impl tonic::IntoRequest<super::Heartbeat>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/lolraft.Raft/SendHeartbeat",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("lolraft.Raft", "SendHeartbeat"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn send_timeout_now(
            &mut self,
            request: impl tonic::IntoRequest<super::TimeoutNow>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/lolraft.Raft/SendTimeoutNow",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("lolraft.Raft", "SendTimeoutNow"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod raft_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with RaftServer.
    #[async_trait]
    pub trait Raft: Send + Sync + 'static {
        async fn write(
            &self,
            request: tonic::Request<super::WriteRequest>,
        ) -> std::result::Result<tonic::Response<super::Response>, tonic::Status>;
        async fn read(
            &self,
            request: tonic::Request<super::ReadRequest>,
        ) -> std::result::Result<tonic::Response<super::Response>, tonic::Status>;
        async fn process_kern_request(
            &self,
            request: tonic::Request<super::KernRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        async fn request_vote(
            &self,
            request: tonic::Request<super::VoteRequest>,
        ) -> std::result::Result<tonic::Response<super::VoteResponse>, tonic::Status>;
        async fn add_server(
            &self,
            request: tonic::Request<super::AddServerRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        async fn remove_server(
            &self,
            request: tonic::Request<super::RemoveServerRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        async fn send_replication_stream(
            &self,
            request: tonic::Request<tonic::Streaming<super::ReplicationStreamChunk>>,
        ) -> std::result::Result<
            tonic::Response<super::ReplicationStreamResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the GetSnapshot method.
        type GetSnapshotStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::SnapshotChunk, tonic::Status>,
            >
            + Send
            + 'static;
        async fn get_snapshot(
            &self,
            request: tonic::Request<super::GetSnapshotRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::GetSnapshotStream>,
            tonic::Status,
        >;
        async fn send_heartbeat(
            &self,
            request: tonic::Request<super::Heartbeat>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        async fn send_timeout_now(
            &self,
            request: tonic::Request<super::TimeoutNow>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct RaftServer<T: Raft> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Raft> RaftServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for RaftServer<T>
    where
        T: Raft,
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
                "/lolraft.Raft/Write" => {
                    #[allow(non_camel_case_types)]
                    struct WriteSvc<T: Raft>(pub Arc<T>);
                    impl<T: Raft> tonic::server::UnaryService<super::WriteRequest>
                    for WriteSvc<T> {
                        type Response = super::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::WriteRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Raft>::write(&inner, request).await
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
                "/lolraft.Raft/Read" => {
                    #[allow(non_camel_case_types)]
                    struct ReadSvc<T: Raft>(pub Arc<T>);
                    impl<T: Raft> tonic::server::UnaryService<super::ReadRequest>
                    for ReadSvc<T> {
                        type Response = super::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReadRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Raft>::read(&inner, request).await
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
                "/lolraft.Raft/ProcessKernRequest" => {
                    #[allow(non_camel_case_types)]
                    struct ProcessKernRequestSvc<T: Raft>(pub Arc<T>);
                    impl<T: Raft> tonic::server::UnaryService<super::KernRequest>
                    for ProcessKernRequestSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::KernRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Raft>::process_kern_request(&inner, request).await
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
                        let method = ProcessKernRequestSvc(inner);
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
                "/lolraft.Raft/RequestVote" => {
                    #[allow(non_camel_case_types)]
                    struct RequestVoteSvc<T: Raft>(pub Arc<T>);
                    impl<T: Raft> tonic::server::UnaryService<super::VoteRequest>
                    for RequestVoteSvc<T> {
                        type Response = super::VoteResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VoteRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Raft>::request_vote(&inner, request).await
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
                        let method = RequestVoteSvc(inner);
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
                "/lolraft.Raft/AddServer" => {
                    #[allow(non_camel_case_types)]
                    struct AddServerSvc<T: Raft>(pub Arc<T>);
                    impl<T: Raft> tonic::server::UnaryService<super::AddServerRequest>
                    for AddServerSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddServerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Raft>::add_server(&inner, request).await
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
                        let method = AddServerSvc(inner);
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
                "/lolraft.Raft/RemoveServer" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveServerSvc<T: Raft>(pub Arc<T>);
                    impl<T: Raft> tonic::server::UnaryService<super::RemoveServerRequest>
                    for RemoveServerSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveServerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Raft>::remove_server(&inner, request).await
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
                        let method = RemoveServerSvc(inner);
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
                "/lolraft.Raft/SendReplicationStream" => {
                    #[allow(non_camel_case_types)]
                    struct SendReplicationStreamSvc<T: Raft>(pub Arc<T>);
                    impl<
                        T: Raft,
                    > tonic::server::ClientStreamingService<
                        super::ReplicationStreamChunk,
                    > for SendReplicationStreamSvc<T> {
                        type Response = super::ReplicationStreamResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::ReplicationStreamChunk>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Raft>::send_replication_stream(&inner, request).await
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
                        let method = SendReplicationStreamSvc(inner);
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
                        let res = grpc.client_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/lolraft.Raft/GetSnapshot" => {
                    #[allow(non_camel_case_types)]
                    struct GetSnapshotSvc<T: Raft>(pub Arc<T>);
                    impl<
                        T: Raft,
                    > tonic::server::ServerStreamingService<super::GetSnapshotRequest>
                    for GetSnapshotSvc<T> {
                        type Response = super::SnapshotChunk;
                        type ResponseStream = T::GetSnapshotStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetSnapshotRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Raft>::get_snapshot(&inner, request).await
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
                        let method = GetSnapshotSvc(inner);
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
                "/lolraft.Raft/SendHeartbeat" => {
                    #[allow(non_camel_case_types)]
                    struct SendHeartbeatSvc<T: Raft>(pub Arc<T>);
                    impl<T: Raft> tonic::server::UnaryService<super::Heartbeat>
                    for SendHeartbeatSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Heartbeat>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Raft>::send_heartbeat(&inner, request).await
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
                        let method = SendHeartbeatSvc(inner);
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
                "/lolraft.Raft/SendTimeoutNow" => {
                    #[allow(non_camel_case_types)]
                    struct SendTimeoutNowSvc<T: Raft>(pub Arc<T>);
                    impl<T: Raft> tonic::server::UnaryService<super::TimeoutNow>
                    for SendTimeoutNowSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TimeoutNow>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Raft>::send_timeout_now(&inner, request).await
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
                        let method = SendTimeoutNowSvc(inner);
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
    impl<T: Raft> Clone for RaftServer<T> {
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
    impl<T: Raft> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Raft> tonic::server::NamedService for RaftServer<T> {
        const NAME: &'static str = "lolraft.Raft";
    }
}
