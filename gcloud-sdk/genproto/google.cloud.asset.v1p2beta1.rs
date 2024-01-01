/// An asset in Google Cloud and its temporal metadata, including the time window
/// when it was observed and its status during that window.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TemporalAsset {
    /// The time window when the asset data and state was observed.
    #[prost(message, optional, tag = "1")]
    pub window: ::core::option::Option<TimeWindow>,
    /// Whether the asset has been deleted or not.
    #[prost(bool, tag = "2")]
    pub deleted: bool,
    /// An asset in Google Cloud.
    #[prost(message, optional, tag = "3")]
    pub asset: ::core::option::Option<Asset>,
}
/// A time window specified by its `start_time` and `end_time`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeWindow {
    /// Start time of the time window (exclusive).
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// End time of the time window (inclusive). If not specified, the current
    /// timestamp is used instead.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// An asset in Google Cloud. An asset can be any resource in the Google Cloud
/// [resource
/// hierarchy](<https://cloud.google.com/resource-manager/docs/cloud-platform-resource-hierarchy>),
/// a resource outside the Google Cloud resource hierarchy (such as Google
/// Kubernetes Engine clusters and objects), or a policy (e.g. IAM policy).
/// See [Supported asset
/// types](<https://cloud.google.com/asset-inventory/docs/supported-asset-types>)
/// for more information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Asset {
    /// The full name of the asset. Example:
    /// `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`
    ///
    /// See [Resource
    /// names](<https://cloud.google.com/apis/design/resource_names#full_resource_name>)
    /// for more information.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The type of the asset. Example: `compute.googleapis.com/Disk`
    ///
    /// See [Supported asset
    /// types](<https://cloud.google.com/asset-inventory/docs/supported-asset-types>)
    /// for more information.
    #[prost(string, tag = "2")]
    pub asset_type: ::prost::alloc::string::String,
    /// A representation of the resource.
    #[prost(message, optional, tag = "3")]
    pub resource: ::core::option::Option<Resource>,
    /// A representation of the IAM policy set on a Google Cloud resource.
    /// There can be a maximum of one IAM policy set on any given resource.
    /// In addition, IAM policies inherit their granted access scope from any
    /// policies set on parent resources in the resource hierarchy. Therefore, the
    /// effectively policy is the union of both the policy set on this resource
    /// and each policy set on all of the resource's ancestry resource levels in
    /// the hierarchy. See
    /// [this topic](<https://cloud.google.com/iam/help/allow-policies/inheritance>)
    /// for more information.
    #[prost(message, optional, tag = "4")]
    pub iam_policy: ::core::option::Option<super::super::super::iam::v1::Policy>,
    /// The ancestry path of an asset in Google Cloud [resource
    /// hierarchy](<https://cloud.google.com/resource-manager/docs/cloud-platform-resource-hierarchy>),
    /// represented as a list of relative resource names. An ancestry path starts
    /// with the closest ancestor in the hierarchy and ends at root. If the asset
    /// is a project, folder, or organization, the ancestry path starts from the
    /// asset itself.
    ///
    /// Example: `\["projects/123456789", "folders/5432", "organizations/1234"\]`
    #[prost(string, repeated, tag = "6")]
    pub ancestors: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A representation of an [organization
    /// policy](<https://cloud.google.com/resource-manager/docs/organization-policy/overview#organization_policy>).
    /// There can be more than one organization policy with different constraints
    /// set on a given resource.
    #[prost(message, repeated, tag = "10")]
    pub org_policy: ::prost::alloc::vec::Vec<super::super::orgpolicy::v1::Policy>,
    /// A representation of an [access
    /// policy](<https://cloud.google.com/access-context-manager/docs/overview#access-policies>).
    #[prost(oneof = "asset::AccessContextPolicy", tags = "7, 8, 9")]
    pub access_context_policy: ::core::option::Option<asset::AccessContextPolicy>,
}
/// Nested message and enum types in `Asset`.
pub mod asset {
    /// A representation of an [access
    /// policy](<https://cloud.google.com/access-context-manager/docs/overview#access-policies>).
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AccessContextPolicy {
        /// Please also refer to the [access policy user
        /// guide](<https://cloud.google.com/access-context-manager/docs/overview#access-policies>).
        #[prost(message, tag = "7")]
        AccessPolicy(
            super::super::super::super::identity::accesscontextmanager::v1::AccessPolicy,
        ),
        /// Please also refer to the [access level user
        /// guide](<https://cloud.google.com/access-context-manager/docs/overview#access-levels>).
        #[prost(message, tag = "8")]
        AccessLevel(
            super::super::super::super::identity::accesscontextmanager::v1::AccessLevel,
        ),
        /// Please also refer to the [service perimeter user
        /// guide](<https://cloud.google.com/vpc-service-controls/docs/overview>).
        #[prost(message, tag = "9")]
        ServicePerimeter(
            super::super::super::super::identity::accesscontextmanager::v1::ServicePerimeter,
        ),
    }
}
/// A representation of a Google Cloud resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resource {
    /// The API version. Example: `v1`
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
    /// The URL of the discovery document containing the resource's JSON schema.
    /// Example:
    /// `<https://www.googleapis.com/discovery/v1/apis/compute/v1/rest`>
    ///
    /// This value is unspecified for resources that do not have an API based on a
    /// discovery document, such as Cloud Bigtable.
    #[prost(string, tag = "2")]
    pub discovery_document_uri: ::prost::alloc::string::String,
    /// The JSON schema name listed in the discovery document. Example:
    /// `Project`
    ///
    /// This value is unspecified for resources that do not have an API based on a
    /// discovery document, such as Cloud Bigtable.
    #[prost(string, tag = "3")]
    pub discovery_name: ::prost::alloc::string::String,
    /// The REST URL for accessing the resource. An HTTP `GET` request using this
    /// URL returns the resource itself. Example:
    /// `<https://cloudresourcemanager.googleapis.com/v1/projects/my-project-123`>
    ///
    /// This value is unspecified for resources without a REST API.
    #[prost(string, tag = "4")]
    pub resource_url: ::prost::alloc::string::String,
    /// The full name of the immediate parent of this resource. See
    /// [Resource
    /// Names](<https://cloud.google.com/apis/design/resource_names#full_resource_name>)
    /// for more information.
    ///
    /// For Google Cloud assets, this value is the parent resource defined in the
    /// [IAM policy
    /// hierarchy](<https://cloud.google.com/iam/docs/overview#policy_hierarchy>).
    /// Example:
    /// `//cloudresourcemanager.googleapis.com/projects/my_project_123`
    ///
    /// For third-party assets, this field may be set differently.
    #[prost(string, tag = "5")]
    pub parent: ::prost::alloc::string::String,
    /// The content of the resource, in which some sensitive fields are removed
    /// and may not be present.
    #[prost(message, optional, tag = "6")]
    pub data: ::core::option::Option<::prost_types::Struct>,
}
/// The export asset response. This message is returned by the
/// [google.longrunning.Operations.GetOperation][google.longrunning.Operations.GetOperation]
/// method in the returned
/// [google.longrunning.Operation.response][google.longrunning.Operation.response]
/// field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportAssetsResponse {
    /// Time the snapshot was taken.
    #[prost(message, optional, tag = "1")]
    pub read_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output configuration indicating where the results were output to.
    #[prost(message, optional, tag = "2")]
    pub output_config: ::core::option::Option<OutputConfig>,
}
/// Batch get assets history response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetAssetsHistoryResponse {
    /// A list of assets with valid time windows.
    #[prost(message, repeated, tag = "1")]
    pub assets: ::prost::alloc::vec::Vec<TemporalAsset>,
}
/// Create asset feed request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFeedRequest {
    /// Required. The name of the project/folder/organization where this feed
    /// should be created in. It can only be an organization number (such as
    /// "organizations/123"), a folder number (such as "folders/123"), a project ID
    /// (such as "projects/my-project-id"), or a project number (such as
    /// "projects/12345").
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. This is the client-assigned asset feed identifier and it needs to
    /// be unique under a specific parent project/folder/organization.
    #[prost(string, tag = "2")]
    pub feed_id: ::prost::alloc::string::String,
    /// Required. The feed details. The field `name` must be empty and it will be
    /// generated in the format of: projects/project_number/feeds/feed_id
    /// folders/folder_number/feeds/feed_id
    /// organizations/organization_number/feeds/feed_id
    #[prost(message, optional, tag = "3")]
    pub feed: ::core::option::Option<Feed>,
}
/// Get asset feed request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFeedRequest {
    /// Required. The name of the Feed and it must be in the format of:
    /// projects/project_number/feeds/feed_id
    /// folders/folder_number/feeds/feed_id
    /// organizations/organization_number/feeds/feed_id
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// List asset feeds request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFeedsRequest {
    /// Required. The parent project/folder/organization whose feeds are to be
    /// listed. It can only be using project/folder/organization number (such as
    /// "folders/12345")", or a project ID (such as "projects/my-project-id").
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFeedsResponse {
    /// A list of feeds.
    #[prost(message, repeated, tag = "1")]
    pub feeds: ::prost::alloc::vec::Vec<Feed>,
}
/// Update asset feed request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFeedRequest {
    /// Required. The new values of feed details. It must match an existing feed
    /// and the field `name` must be in the format of:
    /// projects/project_number/feeds/feed_id or
    /// folders/folder_number/feeds/feed_id or
    /// organizations/organization_number/feeds/feed_id.
    #[prost(message, optional, tag = "1")]
    pub feed: ::core::option::Option<Feed>,
    /// Required. Only updates the `feed` fields indicated by this mask.
    /// The field mask must not be empty, and it must not contain fields that
    /// are immutable or only set by the server.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFeedRequest {
    /// Required. The name of the feed and it must be in the format of:
    /// projects/project_number/feeds/feed_id
    /// folders/folder_number/feeds/feed_id
    /// organizations/organization_number/feeds/feed_id
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Output configuration for export assets destination.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutputConfig {
    /// Asset export destination.
    #[prost(oneof = "output_config::Destination", tags = "1")]
    pub destination: ::core::option::Option<output_config::Destination>,
}
/// Nested message and enum types in `OutputConfig`.
pub mod output_config {
    /// Asset export destination.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// Destination on Cloud Storage.
        #[prost(message, tag = "1")]
        GcsDestination(super::GcsDestination),
    }
}
/// A Cloud Storage location.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsDestination {
    /// Required.
    #[prost(oneof = "gcs_destination::ObjectUri", tags = "1")]
    pub object_uri: ::core::option::Option<gcs_destination::ObjectUri>,
}
/// Nested message and enum types in `GcsDestination`.
pub mod gcs_destination {
    /// Required.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ObjectUri {
        /// The URI of the Cloud Storage object. It's the same URI that is used by
        /// gsutil. For example: "gs://bucket_name/object_name". See [Viewing and
        /// Editing Object
        /// Metadata](<https://cloud.google.com/storage/docs/viewing-editing-metadata>)
        /// for more information.
        #[prost(string, tag = "1")]
        Uri(::prost::alloc::string::String),
    }
}
/// A Pub/Sub destination.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubsubDestination {
    /// The name of the Pub/Sub topic to publish to.
    /// For example: `projects/PROJECT_ID/topics/TOPIC_ID`.
    #[prost(string, tag = "1")]
    pub topic: ::prost::alloc::string::String,
}
/// Output configuration for asset feed destination.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedOutputConfig {
    /// Asset feed destination.
    #[prost(oneof = "feed_output_config::Destination", tags = "1")]
    pub destination: ::core::option::Option<feed_output_config::Destination>,
}
/// Nested message and enum types in `FeedOutputConfig`.
pub mod feed_output_config {
    /// Asset feed destination.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// Destination on Pub/Sub.
        #[prost(message, tag = "1")]
        PubsubDestination(super::PubsubDestination),
    }
}
/// An asset feed used to export asset updates to a destinations.
/// An asset feed filter controls what updates are exported.
/// The asset feed must be created within a project, organization, or
/// folder. Supported destinations are:
/// Cloud Pub/Sub topics.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Feed {
    /// Required. The format will be
    /// projects/{project_number}/feeds/{client-assigned_feed_identifier} or
    /// folders/{folder_number}/feeds/{client-assigned_feed_identifier} or
    /// organizations/{organization_number}/feeds/{client-assigned_feed_identifier}
    ///
    /// The client-assigned feed identifier must be unique within the parent
    /// project/folder/organization.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A list of the full names of the assets to receive updates. You must specify
    /// either or both of asset_names and asset_types. Only asset updates matching
    /// specified asset_names or asset_types are exported to the feed. For
    /// example:
    /// `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`.
    /// See [Resource
    /// Names](<https://cloud.google.com/apis/design/resource_names#full_resource_name>)
    /// for more info.
    #[prost(string, repeated, tag = "2")]
    pub asset_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A list of types of the assets to receive updates. You must specify either
    /// or both of asset_names and asset_types. Only asset updates matching
    /// specified asset_names or asset_types are exported to the feed.
    /// For example:
    /// "compute.googleapis.com/Disk" See [Introduction to Cloud Asset
    /// Inventory](<https://cloud.google.com/resource-manager/docs/cloud-asset-inventory/overview>)
    /// for all supported asset types.
    #[prost(string, repeated, tag = "3")]
    pub asset_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Asset content type. If not specified, no content but the asset name and
    /// type will be returned.
    #[prost(enumeration = "ContentType", tag = "4")]
    pub content_type: i32,
    /// Required. Feed output configuration defining where the asset updates are
    /// published to.
    #[prost(message, optional, tag = "5")]
    pub feed_output_config: ::core::option::Option<FeedOutputConfig>,
}
/// Asset content type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ContentType {
    /// Unspecified content type.
    Unspecified = 0,
    /// Resource metadata.
    Resource = 1,
    /// The actual IAM policy set on a resource.
    IamPolicy = 2,
}
impl ContentType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ContentType::Unspecified => "CONTENT_TYPE_UNSPECIFIED",
            ContentType::Resource => "RESOURCE",
            ContentType::IamPolicy => "IAM_POLICY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CONTENT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "RESOURCE" => Some(Self::Resource),
            "IAM_POLICY" => Some(Self::IamPolicy),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod asset_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Asset service definition.
    #[derive(Debug, Clone)]
    pub struct AssetServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AssetServiceClient<tonic::transport::Channel> {
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
    impl<T> AssetServiceClient<T>
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
        ) -> AssetServiceClient<InterceptedService<T, F>>
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
            AssetServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a feed in a parent project/folder/organization to listen to its
        /// asset updates.
        pub async fn create_feed(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateFeedRequest>,
        ) -> std::result::Result<tonic::Response<super::Feed>, tonic::Status> {
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
                "/google.cloud.asset.v1p2beta1.AssetService/CreateFeed",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.asset.v1p2beta1.AssetService",
                        "CreateFeed",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details about an asset feed.
        pub async fn get_feed(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFeedRequest>,
        ) -> std::result::Result<tonic::Response<super::Feed>, tonic::Status> {
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
                "/google.cloud.asset.v1p2beta1.AssetService/GetFeed",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.asset.v1p2beta1.AssetService",
                        "GetFeed",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all asset feeds in a parent project/folder/organization.
        pub async fn list_feeds(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFeedsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListFeedsResponse>,
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
                "/google.cloud.asset.v1p2beta1.AssetService/ListFeeds",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.asset.v1p2beta1.AssetService",
                        "ListFeeds",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates an asset feed configuration.
        pub async fn update_feed(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateFeedRequest>,
        ) -> std::result::Result<tonic::Response<super::Feed>, tonic::Status> {
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
                "/google.cloud.asset.v1p2beta1.AssetService/UpdateFeed",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.asset.v1p2beta1.AssetService",
                        "UpdateFeed",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes an asset feed.
        pub async fn delete_feed(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteFeedRequest>,
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
                "/google.cloud.asset.v1p2beta1.AssetService/DeleteFeed",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.asset.v1p2beta1.AssetService",
                        "DeleteFeed",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
