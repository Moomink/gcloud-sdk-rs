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
    /// A representation of an [organization
    /// policy](<https://cloud.google.com/resource-manager/docs/organization-policy/overview#organization_policy>).
    /// There can be more than one organization policy with different constraints
    /// set on a given resource.
    #[prost(message, repeated, tag = "6")]
    pub org_policy: ::prost::alloc::vec::Vec<super::super::orgpolicy::v1::Policy>,
    /// The ancestry path of an asset in Google Cloud [resource
    /// hierarchy](<https://cloud.google.com/resource-manager/docs/cloud-platform-resource-hierarchy>),
    /// represented as a list of relative resource names. An ancestry path starts
    /// with the closest ancestor in the hierarchy and ends at root. If the asset
    /// is a project, folder, or organization, the ancestry path starts from the
    /// asset itself.
    ///
    /// Example: `["projects/123456789", "folders/5432", "organizations/1234"]`
    #[prost(string, repeated, tag = "10")]
    pub ancestors: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
    /// The API version. Example: "v1".
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
/// ListAssets request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAssetsRequest {
    /// Required. Name of the organization or project the assets belong to. Format:
    /// "organizations/\[organization-number\]" (such as "organizations/123"),
    /// "projects/\[project-id\]" (such as "projects/my-project-id"), or
    /// "projects/\[project-number\]" (such as "projects/12345").
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Timestamp to take an asset snapshot. This can only be set to a timestamp
    /// between the current time and the current time minus 35 days (inclusive).
    /// If not specified, the current time will be used. Due to delays in resource
    /// data collection and indexing, there is a volatile window during which
    /// running the same query may get different results.
    #[prost(message, optional, tag = "2")]
    pub read_time: ::core::option::Option<::prost_types::Timestamp>,
    /// A list of asset types to take a snapshot for. For example:
    /// "compute.googleapis.com/Disk".
    ///
    /// Regular expression is also supported. For example:
    ///
    /// * "compute.googleapis.com.*" snapshots resources whose asset type starts
    /// with "compute.googleapis.com".
    /// * ".*Instance" snapshots resources whose asset type ends with "Instance".
    /// * ".*Instance.*" snapshots resources whose asset type contains "Instance".
    ///
    /// See \[RE2\](<https://github.com/google/re2/wiki/Syntax>) for all supported
    /// regular expression syntax. If the regular expression does not match any
    /// supported asset type, an INVALID_ARGUMENT error will be returned.
    ///
    /// If specified, only matching assets will be returned, otherwise, it will
    /// snapshot all asset types. See [Introduction to Cloud Asset
    /// Inventory](<https://cloud.google.com/asset-inventory/docs/overview>)
    /// for all supported asset types.
    #[prost(string, repeated, tag = "3")]
    pub asset_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Asset content type. If not specified, no content but the asset name will
    /// be returned.
    #[prost(enumeration = "ContentType", tag = "4")]
    pub content_type: i32,
    /// The maximum number of assets to be returned in a single response. Default
    /// is 100, minimum is 1, and maximum is 1000.
    #[prost(int32, tag = "5")]
    pub page_size: i32,
    /// The `next_page_token` returned from the previous `ListAssetsResponse`, or
    /// unspecified for the first `ListAssetsRequest`. It is a continuation of a
    /// prior `ListAssets` call, and the API should return the next page of assets.
    #[prost(string, tag = "6")]
    pub page_token: ::prost::alloc::string::String,
}
/// ListAssets response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAssetsResponse {
    /// Time the snapshot was taken.
    #[prost(message, optional, tag = "1")]
    pub read_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Assets.
    #[prost(message, repeated, tag = "2")]
    pub assets: ::prost::alloc::vec::Vec<Asset>,
    /// Token to retrieve the next page of results. It expires 72 hours after the
    /// page token for the first page is generated. Set to empty if there are no
    /// remaining results.
    #[prost(string, tag = "3")]
    pub next_page_token: ::prost::alloc::string::String,
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
    /// The organization policy set on an asset.
    OrgPolicy = 4,
    /// The Access Context Manager policy set on an asset.
    AccessPolicy = 5,
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
            ContentType::OrgPolicy => "ORG_POLICY",
            ContentType::AccessPolicy => "ACCESS_POLICY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CONTENT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "RESOURCE" => Some(Self::Resource),
            "IAM_POLICY" => Some(Self::IamPolicy),
            "ORG_POLICY" => Some(Self::OrgPolicy),
            "ACCESS_POLICY" => Some(Self::AccessPolicy),
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
            D: std::convert::TryInto<tonic::transport::Endpoint>,
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
        /// Lists assets with time and resource types and returns paged results in
        /// response.
        pub async fn list_assets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAssetsRequest>,
        ) -> Result<tonic::Response<super::ListAssetsResponse>, tonic::Status> {
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
                "/google.cloud.asset.v1p5beta1.AssetService/ListAssets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
