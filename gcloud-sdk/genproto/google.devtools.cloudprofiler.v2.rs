/// CreateProfileRequest describes a profile resource online creation request.
/// The deployment field must be populated. The profile_type specifies the list
/// of profile types supported by the agent. The creation call will hang until a
/// profile of one of these types needs to be collected.
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateProfileRequest {
    /// Parent project to create the profile in.
    #[prost(string, tag = "4")]
    pub parent: ::prost::alloc::string::String,
    /// Deployment details.
    #[prost(message, optional, tag = "1")]
    pub deployment: ::core::option::Option<Deployment>,
    /// One or more profile types that the agent is capable of providing.
    #[prost(enumeration = "ProfileType", repeated, tag = "2")]
    pub profile_type: ::prost::alloc::vec::Vec<i32>,
}
/// CreateOfflineProfileRequest describes a profile resource offline creation
/// request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOfflineProfileRequest {
    /// Parent project to create the profile in.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Contents of the profile to create.
    #[prost(message, optional, tag = "2")]
    pub profile: ::core::option::Option<Profile>,
}
/// UpdateProfileRequest contains the profile to update.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateProfileRequest {
    /// Profile to update.
    #[prost(message, optional, tag = "1")]
    pub profile: ::core::option::Option<Profile>,
    /// Field mask used to specify the fields to be overwritten. Currently only
    /// profile_bytes and labels fields are supported by UpdateProfile, so only
    /// those fields can be specified in the mask. When no mask is provided, all
    /// fields are overwritten.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Profile resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Profile {
    /// Output only. Opaque, server-assigned, unique ID for this profile.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Type of profile.
    /// For offline mode, this must be specified when creating the profile. For
    /// online mode it is assigned and returned by the server.
    #[prost(enumeration = "ProfileType", tag = "2")]
    pub profile_type: i32,
    /// Deployment this profile corresponds to.
    #[prost(message, optional, tag = "3")]
    pub deployment: ::core::option::Option<Deployment>,
    /// Duration of the profiling session.
    /// Input (for the offline mode) or output (for the online mode).
    /// The field represents requested profiling duration. It may slightly differ
    /// from the effective profiling duration, which is recorded in the profile
    /// data, in case the profiling can't be stopped immediately (e.g. in case
    /// stopping the profiling is handled asynchronously).
    #[prost(message, optional, tag = "4")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    /// Input only. Profile bytes, as a gzip compressed serialized proto, the
    /// format is <https://github.com/google/pprof/blob/master/proto/profile.proto.>
    #[prost(bytes = "vec", tag = "5")]
    pub profile_bytes: ::prost::alloc::vec::Vec<u8>,
    /// Input only. Labels associated to this specific profile. These labels will
    /// get merged with the deployment labels for the final data set. See
    /// documentation on deployment labels for validation rules and limits.
    #[prost(map = "string, string", tag = "6")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. Start time for the profile.
    /// This output is only present in response from the ListProfiles method.
    #[prost(message, optional, tag = "7")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Deployment contains the deployment identification information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Deployment {
    /// Project ID is the ID of a cloud project.
    /// Validation regex: `^[a-z][-a-z0-9:.]{4,61}\[a-z0-9\]$`.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Target is the service name used to group related deployments:
    /// * Service name for App Engine Flex / Standard.
    /// * Cluster and container name for GKE.
    /// * User-specified string for direct Compute Engine profiling (e.g. Java).
    /// * Job name for Dataflow.
    /// Validation regex: `^[a-z0-9](\[-a-z0-9_.\]{0,253}\[a-z0-9\])?$`.
    #[prost(string, tag = "2")]
    pub target: ::prost::alloc::string::String,
    /// Labels identify the deployment within the user universe and same target.
    /// Validation regex for label names: `^[a-z0-9](\[a-z0-9-\]{0,61}\[a-z0-9\])?$`.
    /// Value for an individual label must be <= 512 bytes, the total
    /// size of all label names and values must be <= 1024 bytes.
    ///
    /// Label named "language" can be used to record the programming language of
    /// the profiled deployment. The standard choices for the value include "java",
    /// "go", "python", "ruby", "nodejs", "php", "dotnet".
    ///
    /// For deployments running on Google Cloud Platform, "zone" or "region" label
    /// should be present describing the deployment location. An example of a zone
    /// is "us-central1-a", an example of a region is "us-central1" or
    /// "us-central".
    #[prost(map = "string, string", tag = "3")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// ListProfilesRequest contains request parameters for listing profiles for
/// deployments in projects which the user has permissions to view.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProfilesRequest {
    /// Required. The parent, which owns this collection of profiles.
    /// Format: projects/{user_project_id}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    /// Default page_size is 1000.
    /// Max limit is 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The token to continue pagination and get profiles from a particular page.
    /// When paginating, all other parameters provided to `ListProfiles` must match
    /// the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// ListProfileResponse contains the list of collected profiles for deployments
/// in projects which the user has permissions to view.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProfilesResponse {
    /// List of profiles fetched.
    #[prost(message, repeated, tag = "1")]
    pub profiles: ::prost::alloc::vec::Vec<Profile>,
    /// Token to receive the next page of results.
    /// This field maybe empty if there are no more profiles to fetch.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Number of profiles that were skipped in the current page since they were
    /// not able to be fetched successfully. This should typically be zero. A
    /// non-zero value may indicate a transient failure, in which case if the
    /// number is too high for your use case, the call may be retried.
    #[prost(int32, tag = "3")]
    pub skipped_profiles: i32,
}
/// ProfileType is type of profiling data.
/// NOTE: the enumeration member names are used (in lowercase) as unique string
/// identifiers of profile types, so they must not be renamed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProfileType {
    /// Unspecified profile type.
    Unspecified = 0,
    /// Thread CPU time sampling.
    Cpu = 1,
    /// Wallclock time sampling. More expensive as stops all threads.
    Wall = 2,
    /// In-use heap profile. Represents a snapshot of the allocations that are
    /// live at the time of the profiling.
    Heap = 3,
    /// Single-shot collection of all thread stacks.
    Threads = 4,
    /// Synchronization contention profile.
    Contention = 5,
    /// Peak heap profile.
    PeakHeap = 6,
    /// Heap allocation profile. It represents the aggregation of all allocations
    /// made over the duration of the profile. All allocations are included,
    /// including those that might have been freed by the end of the profiling
    /// interval. The profile is in particular useful for garbage collecting
    /// languages to understand which parts of the code create most of the garbage
    /// collection pressure to see if those can be optimized.
    HeapAlloc = 7,
}
impl ProfileType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProfileType::Unspecified => "PROFILE_TYPE_UNSPECIFIED",
            ProfileType::Cpu => "CPU",
            ProfileType::Wall => "WALL",
            ProfileType::Heap => "HEAP",
            ProfileType::Threads => "THREADS",
            ProfileType::Contention => "CONTENTION",
            ProfileType::PeakHeap => "PEAK_HEAP",
            ProfileType::HeapAlloc => "HEAP_ALLOC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PROFILE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "CPU" => Some(Self::Cpu),
            "WALL" => Some(Self::Wall),
            "HEAP" => Some(Self::Heap),
            "THREADS" => Some(Self::Threads),
            "CONTENTION" => Some(Self::Contention),
            "PEAK_HEAP" => Some(Self::PeakHeap),
            "HEAP_ALLOC" => Some(Self::HeapAlloc),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod profiler_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Manage the collection of continuous profiling data provided by profiling
    /// agents running in the cloud or by an offline provider of profiling data.
    ///
    /// __The APIs listed in this service are intended for use within our profiler
    /// agents only.__
    #[derive(Debug, Clone)]
    pub struct ProfilerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ProfilerServiceClient<tonic::transport::Channel> {
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
    impl<T> ProfilerServiceClient<T>
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
        ) -> ProfilerServiceClient<InterceptedService<T, F>>
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
            ProfilerServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// CreateProfile creates a new profile resource in the online mode.
        ///
        /// _Direct use of this API is discouraged, please use a [supported
        /// profiler
        /// agent](https://cloud.google.com/profiler/docs/about-profiler#profiling_agent)
        /// instead for profile collection._
        ///
        /// The server ensures that the new profiles are created at a constant rate per
        /// deployment, so the creation request may hang for some time until the next
        /// profile session is available.
        ///
        /// The request may fail with ABORTED error if the creation is not available
        /// within ~1m, the response will indicate the duration of the backoff the
        /// client should take before attempting creating a profile again. The backoff
        /// duration is returned in google.rpc.RetryInfo extension on the response
        /// status. To a gRPC client, the extension will be return as a
        /// binary-serialized proto in the trailing metadata item named
        /// "google.rpc.retryinfo-bin".
        ///
        pub async fn create_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateProfileRequest>,
        ) -> std::result::Result<tonic::Response<super::Profile>, tonic::Status> {
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
                "/google.devtools.cloudprofiler.v2.ProfilerService/CreateProfile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.cloudprofiler.v2.ProfilerService",
                        "CreateProfile",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// CreateOfflineProfile creates a new profile resource in the offline
        /// mode. The client provides the profile to create along with the profile
        /// bytes, the server records it.
        ///
        /// _Direct use of this API is discouraged, please use a [supported
        /// profiler
        /// agent](https://cloud.google.com/profiler/docs/about-profiler#profiling_agent)
        /// instead for profile collection._
        pub async fn create_offline_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateOfflineProfileRequest>,
        ) -> std::result::Result<tonic::Response<super::Profile>, tonic::Status> {
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
                "/google.devtools.cloudprofiler.v2.ProfilerService/CreateOfflineProfile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.cloudprofiler.v2.ProfilerService",
                        "CreateOfflineProfile",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// UpdateProfile updates the profile bytes and labels on the profile resource
        /// created in the online mode. Updating the bytes for profiles created in the
        /// offline mode is currently not supported: the profile content must be
        /// provided at the time of the profile creation.
        ///
        /// _Direct use of this API is discouraged, please use a [supported
        /// profiler
        /// agent](https://cloud.google.com/profiler/docs/about-profiler#profiling_agent)
        /// instead for profile collection._
        pub async fn update_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateProfileRequest>,
        ) -> std::result::Result<tonic::Response<super::Profile>, tonic::Status> {
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
                "/google.devtools.cloudprofiler.v2.ProfilerService/UpdateProfile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.cloudprofiler.v2.ProfilerService",
                        "UpdateProfile",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod export_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service allows existing Cloud Profiler customers to export their profile data
    /// out of Google Cloud.
    #[derive(Debug, Clone)]
    pub struct ExportServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ExportServiceClient<tonic::transport::Channel> {
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
    impl<T> ExportServiceClient<T>
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
        ) -> ExportServiceClient<InterceptedService<T, F>>
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
            ExportServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists profiles which have been collected so far and for which the caller
        /// has permission to view.
        pub async fn list_profiles(
            &mut self,
            request: impl tonic::IntoRequest<super::ListProfilesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListProfilesResponse>,
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
                "/google.devtools.cloudprofiler.v2.ExportService/ListProfiles",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.cloudprofiler.v2.ExportService",
                        "ListProfiles",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
