/// Request message for
/// \[KeyDashboardService.ListCryptoKeys][google.cloud.kms.inventory.v1.KeyDashboardService.ListCryptoKeys\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCryptoKeysRequest {
    /// Required. The Google Cloud project for which to retrieve key metadata, in
    /// the format `projects/*`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of keys to return. The service may return
    /// fewer than this value. If unspecified, at most 1000 keys will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Pass this into a subsequent request in order to receive the next
    /// page of results.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for
/// \[KeyDashboardService.ListCryptoKeys][google.cloud.kms.inventory.v1.KeyDashboardService.ListCryptoKeys\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCryptoKeysResponse {
    /// The list of \[CryptoKeys][google.cloud.kms.v1.CryptoKey\].
    #[prost(message, repeated, tag = "1")]
    pub crypto_keys: ::prost::alloc::vec::Vec<super::super::v1::CryptoKey>,
    /// The page token returned from the previous response if the next page is
    /// desired.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod key_dashboard_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Provides a cross-region view of all Cloud KMS keys in a given Cloud project.
    #[derive(Debug, Clone)]
    pub struct KeyDashboardServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl KeyDashboardServiceClient<tonic::transport::Channel> {
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
    impl<T> KeyDashboardServiceClient<T>
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
        ) -> KeyDashboardServiceClient<InterceptedService<T, F>>
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
            KeyDashboardServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Returns cryptographic keys managed by Cloud KMS in a given Cloud project.
        /// Note that this data is sourced from snapshots, meaning it may not
        /// completely reflect the actual state of key metadata at call time.
        pub async fn list_crypto_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCryptoKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCryptoKeysResponse>,
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
                "/google.cloud.kms.inventory.v1.KeyDashboardService/ListCryptoKeys",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.kms.inventory.v1.KeyDashboardService",
                        "ListCryptoKeys",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Request message for
/// \[KeyTrackingService.GetProtectedResourcesSummary][google.cloud.kms.inventory.v1.KeyTrackingService.GetProtectedResourcesSummary\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProtectedResourcesSummaryRequest {
    /// Required. The resource name of the
    /// \[CryptoKey][google.cloud.kms.v1.CryptoKey\].
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Aggregate information about the resources protected by a Cloud KMS key in the
/// same Cloud organization as the key.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtectedResourcesSummary {
    /// The full name of the ProtectedResourcesSummary resource.
    /// Example:
    /// projects/test-project/locations/us/keyRings/test-keyring/cryptoKeys/test-key/protectedResourcesSummary
    #[prost(string, tag = "5")]
    pub name: ::prost::alloc::string::String,
    /// The total number of protected resources in the same Cloud organization as
    /// the key.
    #[prost(int64, tag = "1")]
    pub resource_count: i64,
    /// The number of distinct Cloud projects in the same Cloud organization as the
    /// key that have resources protected by the key.
    #[prost(int32, tag = "2")]
    pub project_count: i32,
    /// The number of resources protected by the key grouped by resource type.
    #[prost(map = "string, int64", tag = "3")]
    pub resource_types: ::std::collections::HashMap<::prost::alloc::string::String, i64>,
    /// The number of resources protected by the key grouped by Cloud product.
    #[prost(map = "string, int64", tag = "6")]
    pub cloud_products: ::std::collections::HashMap<::prost::alloc::string::String, i64>,
    /// The number of resources protected by the key grouped by region.
    #[prost(map = "string, int64", tag = "4")]
    pub locations: ::std::collections::HashMap<::prost::alloc::string::String, i64>,
}
/// Request message for
/// \[KeyTrackingService.SearchProtectedResources][google.cloud.kms.inventory.v1.KeyTrackingService.SearchProtectedResources\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchProtectedResourcesRequest {
    /// Required. Resource name of the organization.
    /// Example: organizations/123
    #[prost(string, tag = "2")]
    pub scope: ::prost::alloc::string::String,
    /// Required. The resource name of the
    /// \[CryptoKey][google.cloud.kms.v1.CryptoKey\].
    #[prost(string, tag = "1")]
    pub crypto_key: ::prost::alloc::string::String,
    /// The maximum number of resources to return. The service may return fewer
    /// than this value.
    /// If unspecified, at most 500 resources will be returned.
    /// The maximum value is 500; values above 500 will be coerced to 500.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// A page token, received from a previous
    /// \[KeyTrackingService.SearchProtectedResources][google.cloud.kms.inventory.v1.KeyTrackingService.SearchProtectedResources\]
    /// call. Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// \[KeyTrackingService.SearchProtectedResources][google.cloud.kms.inventory.v1.KeyTrackingService.SearchProtectedResources\]
    /// must match the call that provided the page token.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for
/// \[KeyTrackingService.SearchProtectedResources][google.cloud.kms.inventory.v1.KeyTrackingService.SearchProtectedResources\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchProtectedResourcesResponse {
    /// Protected resources for this page.
    #[prost(message, repeated, tag = "1")]
    pub protected_resources: ::prost::alloc::vec::Vec<ProtectedResource>,
    /// A token that can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Metadata about a resource protected by a Cloud KMS key.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtectedResource {
    /// The full resource name of the resource.
    /// Example:
    /// `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Format: `projects/{PROJECT_NUMBER}`.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    /// The ID of the project that owns the resource.
    #[prost(string, tag = "9")]
    pub project_id: ::prost::alloc::string::String,
    /// The Cloud product that owns the resource.
    /// Example: `compute`
    #[prost(string, tag = "8")]
    pub cloud_product: ::prost::alloc::string::String,
    /// Example: `compute.googleapis.com/Disk`
    #[prost(string, tag = "3")]
    pub resource_type: ::prost::alloc::string::String,
    /// Location can be `global`, regional like `us-east1`, or zonal like
    /// `us-west1-b`.
    #[prost(string, tag = "4")]
    pub location: ::prost::alloc::string::String,
    /// A key-value pair of the resource's labels (v1) to their values.
    #[prost(map = "string, string", tag = "5")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The name of the Cloud KMS
    /// \[CryptoKeyVersion\](<https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyRings.cryptoKeys.cryptoKeyVersions?hl=en>)
    /// used to protect this resource via CMEK. This field is empty if the
    /// Google Cloud product owning the resource does not provide key version data
    /// to Asset Inventory. If there are multiple key versions protecting the
    /// resource, then this is same value as the first element of
    /// crypto_key_versions.
    #[prost(string, tag = "6")]
    pub crypto_key_version: ::prost::alloc::string::String,
    /// The names of the Cloud KMS
    /// \[CryptoKeyVersion\](<https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyRings.cryptoKeys.cryptoKeyVersions?hl=en>)
    /// used to protect this resource via CMEK. This field is empty if the
    /// Google Cloud product owning the resource does not provide key versions data
    /// to Asset Inventory. The first element of this field is stored in
    /// crypto_key_version.
    #[prost(string, repeated, tag = "10")]
    pub crypto_key_versions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. The time at which this resource was created. The granularity
    /// is in seconds. Timestamp.nanos will always be 0.
    #[prost(message, optional, tag = "7")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Generated client implementations.
pub mod key_tracking_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Returns information about the resources in an org that are protected by a
    /// given Cloud KMS key via CMEK.
    #[derive(Debug, Clone)]
    pub struct KeyTrackingServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl KeyTrackingServiceClient<tonic::transport::Channel> {
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
    impl<T> KeyTrackingServiceClient<T>
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
        ) -> KeyTrackingServiceClient<InterceptedService<T, F>>
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
            KeyTrackingServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Returns aggregate information about the resources protected by the given
        /// Cloud KMS [CryptoKey][google.cloud.kms.v1.CryptoKey]. Only resources within
        /// the same Cloud organization as the key will be returned. The project that
        /// holds the key must be part of an organization in order for this call to
        /// succeed.
        pub async fn get_protected_resources_summary(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProtectedResourcesSummaryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ProtectedResourcesSummary>,
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
                "/google.cloud.kms.inventory.v1.KeyTrackingService/GetProtectedResourcesSummary",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.kms.inventory.v1.KeyTrackingService",
                        "GetProtectedResourcesSummary",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns metadata about the resources protected by the given Cloud KMS
        /// [CryptoKey][google.cloud.kms.v1.CryptoKey] in the given Cloud organization.
        pub async fn search_protected_resources(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchProtectedResourcesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SearchProtectedResourcesResponse>,
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
                "/google.cloud.kms.inventory.v1.KeyTrackingService/SearchProtectedResources",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.kms.inventory.v1.KeyTrackingService",
                        "SearchProtectedResources",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
