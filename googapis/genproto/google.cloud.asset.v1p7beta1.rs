// The Cloud Asset API.

/// An asset in Google Cloud. An asset can be any resource in the Google Cloud
/// [resource
/// hierarchy](https://cloud.google.com/resource-manager/docs/cloud-platform-resource-hierarchy),
/// a resource outside the Google Cloud resource hierarchy (such as Google
/// Kubernetes Engine clusters and objects), or a policy (e.g. Cloud IAM policy).
/// See [Supported asset
/// types](https://cloud.google.com/asset-inventory/docs/supported-asset-types)
/// for more information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Asset {
    /// The last update timestamp of an asset. update_time is updated when
    /// create/update/delete operation is performed.
    #[prost(message, optional, tag = "11")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The full name of the asset. Example:
    /// `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`
    ///
    /// See [Resource
    /// names](https://cloud.google.com/apis/design/resource_names#full_resource_name)
    /// for more information.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The type of the asset. Example: `compute.googleapis.com/Disk`
    ///
    /// See [Supported asset
    /// types](https://cloud.google.com/asset-inventory/docs/supported-asset-types)
    /// for more information.
    #[prost(string, tag = "2")]
    pub asset_type: ::prost::alloc::string::String,
    /// A representation of the resource.
    #[prost(message, optional, tag = "3")]
    pub resource: ::core::option::Option<Resource>,
    /// A representation of the Cloud IAM policy set on a Google Cloud resource.
    /// There can be a maximum of one Cloud IAM policy set on any given resource.
    /// In addition, Cloud IAM policies inherit their granted access scope from any
    /// policies set on parent resources in the resource hierarchy. Therefore, the
    /// effectively policy is the union of both the policy set on this resource
    /// and each policy set on all of the resource's ancestry resource levels in
    /// the hierarchy. See
    /// [this topic](https://cloud.google.com/iam/docs/policies#inheritance) for
    /// more information.
    #[prost(message, optional, tag = "4")]
    pub iam_policy: ::core::option::Option<super::super::super::iam::v1::Policy>,
    /// A representation of an [organization
    /// policy](https://cloud.google.com/resource-manager/docs/organization-policy/overview#organization_policy).
    /// There can be more than one organization policy with different constraints
    /// set on a given resource.
    #[prost(message, repeated, tag = "6")]
    pub org_policy: ::prost::alloc::vec::Vec<super::super::orgpolicy::v1::Policy>,
    /// The related assets of the asset of one relationship type.
    /// One asset only represents one type of relationship.
    #[prost(message, optional, tag = "13")]
    pub related_assets: ::core::option::Option<RelatedAssets>,
    /// The ancestry path of an asset in Google Cloud [resource
    /// hierarchy](https://cloud.google.com/resource-manager/docs/cloud-platform-resource-hierarchy),
    /// represented as a list of relative resource names. An ancestry path starts
    /// with the closest ancestor in the hierarchy and ends at root. If the asset
    /// is a project, folder, or organization, the ancestry path starts from the
    /// asset itself.
    ///
    /// Example: `["projects/123456789", "folders/5432", "organizations/1234"]`
    #[prost(string, repeated, tag = "10")]
    pub ancestors: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A representation of an [access
    /// policy](https://cloud.google.com/access-context-manager/docs/overview#access-policies).
    #[prost(oneof = "asset::AccessContextPolicy", tags = "7, 8, 9")]
    pub access_context_policy: ::core::option::Option<asset::AccessContextPolicy>,
}
/// Nested message and enum types in `Asset`.
pub mod asset {
    /// A representation of an [access
    /// policy](https://cloud.google.com/access-context-manager/docs/overview#access-policies).
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AccessContextPolicy {
        /// Please also refer to the [access policy user
        /// guide](https://cloud.google.com/access-context-manager/docs/overview#access-policies).
        #[prost(message, tag = "7")]
        AccessPolicy(super::super::super::super::identity::accesscontextmanager::v1::AccessPolicy),
        /// Please also refer to the [access level user
        /// guide](https://cloud.google.com/access-context-manager/docs/overview#access-levels).
        #[prost(message, tag = "8")]
        AccessLevel(super::super::super::super::identity::accesscontextmanager::v1::AccessLevel),
        /// Please also refer to the [service perimeter user
        /// guide](https://cloud.google.com/vpc-service-controls/docs/overview).
        #[prost(message, tag = "9")]
        ServicePerimeter(
            super::super::super::super::identity::accesscontextmanager::v1::ServicePerimeter,
        ),
    }
}
/// A representation of a Google Cloud resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resource {
    /// The API version. Example: `v1`
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
    /// The URL of the discovery document containing the resource's JSON schema.
    /// Example:
    /// `https://www.googleapis.com/discovery/v1/apis/compute/v1/rest`
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
    /// `https://cloudresourcemanager.googleapis.com/v1/projects/my-project-123`
    ///
    /// This value is unspecified for resources without a REST API.
    #[prost(string, tag = "4")]
    pub resource_url: ::prost::alloc::string::String,
    /// The full name of the immediate parent of this resource. See
    /// [Resource
    /// Names](https://cloud.google.com/apis/design/resource_names#full_resource_name)
    /// for more information.
    ///
    /// For Google Cloud assets, this value is the parent resource defined in the
    /// [Cloud IAM policy
    /// hierarchy](https://cloud.google.com/iam/docs/overview#policy_hierarchy).
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
    /// The location of the resource in Google Cloud, such as its zone and region.
    /// For more information, see https://cloud.google.com/about/locations/.
    #[prost(string, tag = "8")]
    pub location: ::prost::alloc::string::String,
}
/// The detailed related assets with the `relationship_type`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelatedAssets {
    /// The detailed relation attributes.
    #[prost(message, optional, tag = "1")]
    pub relationship_attributes: ::core::option::Option<RelationshipAttributes>,
    /// The peer resources of the relationship.
    #[prost(message, repeated, tag = "2")]
    pub assets: ::prost::alloc::vec::Vec<RelatedAsset>,
}
/// The relationship attributes which include  `type`, `source_resource_type`,
/// `target_resource_type` and `action`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelationshipAttributes {
    /// The unique identifier of the relationship type. Example:
    /// `INSTANCE_TO_INSTANCEGROUP`
    #[prost(string, tag = "4")]
    pub r#type: ::prost::alloc::string::String,
    /// The source asset type. Example: `compute.googleapis.com/Instance`
    #[prost(string, tag = "1")]
    pub source_resource_type: ::prost::alloc::string::String,
    /// The target asset type. Example: `compute.googleapis.com/Disk`
    #[prost(string, tag = "2")]
    pub target_resource_type: ::prost::alloc::string::String,
    /// The detail of the relationship, e.g. `contains`, `attaches`
    #[prost(string, tag = "3")]
    pub action: ::prost::alloc::string::String,
}
/// An asset identify in Google Cloud which contains its name, type and
/// ancestors. An asset can be any resource in the Google Cloud [resource
/// hierarchy](https://cloud.google.com/resource-manager/docs/cloud-platform-resource-hierarchy),
/// a resource outside the Google Cloud resource hierarchy (such as Google
/// Kubernetes Engine clusters and objects), or a policy (e.g. Cloud IAM policy).
/// See [Supported asset
/// types](https://cloud.google.com/asset-inventory/docs/supported-asset-types)
/// for more information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelatedAsset {
    /// The full name of the asset. Example:
    /// `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`
    ///
    /// See [Resource
    /// names](https://cloud.google.com/apis/design/resource_names#full_resource_name)
    /// for more information.
    #[prost(string, tag = "1")]
    pub asset: ::prost::alloc::string::String,
    /// The type of the asset. Example: `compute.googleapis.com/Disk`
    ///
    /// See [Supported asset
    /// types](https://cloud.google.com/asset-inventory/docs/supported-asset-types)
    /// for more information.
    #[prost(string, tag = "2")]
    pub asset_type: ::prost::alloc::string::String,
    /// The ancestors of an asset in Google Cloud [resource
    /// hierarchy](https://cloud.google.com/resource-manager/docs/cloud-platform-resource-hierarchy),
    /// represented as a list of relative resource names. An ancestry path starts
    /// with the closest ancestor in the hierarchy and ends at root.
    ///
    /// Example: `["projects/123456789", "folders/5432", "organizations/1234"]`
    #[prost(string, repeated, tag = "3")]
    pub ancestors: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Export asset request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportAssetsRequest {
    /// Required. The relative name of the root asset. This can only be an
    /// organization number (such as "organizations/123"), a project ID (such as
    /// "projects/my-project-id"), or a project number (such as "projects/12345"),
    /// or a folder number (such as "folders/123").
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
    /// Regular expressions are also supported. For example:
    ///
    /// * "compute.googleapis.com.*" snapshots resources whose asset type starts
    /// with "compute.googleapis.com".
    /// * ".*Instance" snapshots resources whose asset type ends with "Instance".
    /// * ".*Instance.*" snapshots resources whose asset type contains "Instance".
    ///
    /// See [RE2](https://github.com/google/re2/wiki/Syntax) for all supported
    /// regular expression syntax. If the regular expression does not match any
    /// supported asset type, an INVALID_ARGUMENT error will be returned.
    ///
    /// If specified, only matching assets will be returned, otherwise, it will
    /// snapshot all asset types. See [Introduction to Cloud Asset
    /// Inventory](https://cloud.google.com/asset-inventory/docs/overview)
    /// for all supported asset types.
    #[prost(string, repeated, tag = "3")]
    pub asset_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Asset content type. If not specified, no content but the asset name will be
    /// returned.
    #[prost(enumeration = "ContentType", tag = "4")]
    pub content_type: i32,
    /// Required. Output configuration indicating where the results will be output
    /// to.
    #[prost(message, optional, tag = "5")]
    pub output_config: ::core::option::Option<OutputConfig>,
    /// A list of relationship types to export, for example:
    /// `INSTANCE_TO_INSTANCEGROUP`. This field should only be specified if
    /// content_type=RELATIONSHIP. If specified, it will snapshot [asset_types]'
    /// specified relationships, or give errors if any relationship_types'
    /// supported types are not in [asset_types]. If not specified, it will
    /// snapshot all [asset_types]' supported relationships. An unspecified
    /// [asset_types] field means all supported asset_types. See [Introduction to
    /// Cloud Asset
    /// Inventory](https://cloud.google.com/asset-inventory/docs/overview) for all
    /// supported asset types and relationship types.
    #[prost(string, repeated, tag = "6")]
    pub relationship_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The export asset response. This message is returned by the
/// [google.longrunning.Operations.GetOperation][google.longrunning.Operations.GetOperation]
/// method in the returned
/// [google.longrunning.Operation.response][google.longrunning.Operation.response]
/// field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportAssetsResponse {
    /// Time the snapshot was taken.
    #[prost(message, optional, tag = "1")]
    pub read_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output configuration indicating where the results were output to.
    #[prost(message, optional, tag = "2")]
    pub output_config: ::core::option::Option<OutputConfig>,
    /// Output result indicating where the assets were exported to. For example, a
    /// set of actual Google Cloud Storage object uris where the assets are
    /// exported to. The uris can be different from what [output_config] has
    /// specified, as the service will split the output object into multiple ones
    /// once it exceeds a single Google Cloud Storage object limit.
    #[prost(message, optional, tag = "3")]
    pub output_result: ::core::option::Option<OutputResult>,
}
/// Output configuration for export assets destination.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutputConfig {
    /// Asset export destination.
    #[prost(oneof = "output_config::Destination", tags = "1, 2")]
    pub destination: ::core::option::Option<output_config::Destination>,
}
/// Nested message and enum types in `OutputConfig`.
pub mod output_config {
    /// Asset export destination.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// Destination on Cloud Storage.
        #[prost(message, tag = "1")]
        GcsDestination(super::GcsDestination),
        /// Destination on BigQuery. The output table stores the fields in asset
        /// proto as columns in BigQuery.
        #[prost(message, tag = "2")]
        BigqueryDestination(super::BigQueryDestination),
    }
}
/// Output result of export assets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutputResult {
    /// Asset export result.
    #[prost(oneof = "output_result::Result", tags = "1")]
    pub result: ::core::option::Option<output_result::Result>,
}
/// Nested message and enum types in `OutputResult`.
pub mod output_result {
    /// Asset export result.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Result {
        /// Export result on Cloud Storage.
        #[prost(message, tag = "1")]
        GcsResult(super::GcsOutputResult),
    }
}
/// A Cloud Storage output result.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsOutputResult {
    /// List of uris of the Cloud Storage objects. Example:
    /// "gs://bucket_name/object_name".
    #[prost(string, repeated, tag = "1")]
    pub uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A Cloud Storage location.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsDestination {
    /// Required.
    #[prost(oneof = "gcs_destination::ObjectUri", tags = "1, 2")]
    pub object_uri: ::core::option::Option<gcs_destination::ObjectUri>,
}
/// Nested message and enum types in `GcsDestination`.
pub mod gcs_destination {
    /// Required.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ObjectUri {
        /// The uri of the Cloud Storage object. It's the same uri that is used by
        /// gsutil. Example: "gs://bucket_name/object_name". See [Viewing and
        /// Editing Object
        /// Metadata](https://cloud.google.com/storage/docs/viewing-editing-metadata)
        /// for more information.
        #[prost(string, tag = "1")]
        Uri(::prost::alloc::string::String),
        /// The uri prefix of all generated Cloud Storage objects. Example:
        /// "gs://bucket_name/object_name_prefix". Each object uri is in format:
        /// "gs://bucket_name/object_name_prefix/{ASSET_TYPE}/{SHARD_NUMBER} and only
        /// contains assets for that type. <shard number> starts from 0. Example:
        /// "gs://bucket_name/object_name_prefix/compute.googleapis.com/Disk/0" is
        /// the first shard of output objects containing all
        /// compute.googleapis.com/Disk assets. An INVALID_ARGUMENT error will be
        /// returned if file with the same name "gs://bucket_name/object_name_prefix"
        /// already exists.
        #[prost(string, tag = "2")]
        UriPrefix(::prost::alloc::string::String),
    }
}
/// A BigQuery destination for exporting assets to.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigQueryDestination {
    /// Required. The BigQuery dataset in format
    /// "projects/projectId/datasets/datasetId", to which the snapshot result
    /// should be exported. If this dataset does not exist, the export call returns
    /// an INVALID_ARGUMENT error.
    #[prost(string, tag = "1")]
    pub dataset: ::prost::alloc::string::String,
    /// Required. The BigQuery table to which the snapshot result should be
    /// written. If this table does not exist, a new table with the given name
    /// will be created.
    #[prost(string, tag = "2")]
    pub table: ::prost::alloc::string::String,
    /// If the destination table already exists and this flag is `TRUE`, the
    /// table will be overwritten by the contents of assets snapshot. If the flag
    /// is `FALSE` or unset and the destination table already exists, the export
    /// call returns an INVALID_ARGUMEMT error.
    #[prost(bool, tag = "3")]
    pub force: bool,
    /// [partition_spec] determines whether to export to partitioned table(s) and
    /// how to partition the data.
    ///
    /// If [partition_spec] is unset or [partition_spec.partition_key] is unset or
    /// `PARTITION_KEY_UNSPECIFIED`, the snapshot results will be exported to
    /// non-partitioned table(s). [force] will decide whether to overwrite existing
    /// table(s).
    ///
    /// If [partition_spec] is specified. First, the snapshot results will be
    /// written to partitioned table(s) with two additional timestamp columns,
    /// readTime and requestTime, one of which will be the partition key. Secondly,
    /// in the case when any destination table already exists, it will first try to
    /// update existing table's schema as necessary by appending additional
    /// columns. Then, if [force] is `TRUE`, the corresponding partition will be
    /// overwritten by the snapshot results (data in different partitions will
    /// remain intact); if [force] is unset or `FALSE`, it will append the data. An
    /// error will be returned if the schema update or data appension fails.
    #[prost(message, optional, tag = "4")]
    pub partition_spec: ::core::option::Option<PartitionSpec>,
    /// If this flag is `TRUE`, the snapshot results will be written to one or
    /// multiple tables, each of which contains results of one asset type. The
    /// [force] and [partition_spec] fields will apply to each of them.
    ///
    /// Field [table] will be concatenated with "_" and the asset type names (see
    /// https://cloud.google.com/asset-inventory/docs/supported-asset-types for
    /// supported asset types) to construct per-asset-type table names, in which
    /// all non-alphanumeric characters like "." and "/" will be substituted by
    /// "_". Example: if field [table] is "mytable" and snapshot results
    /// contain "storage.googleapis.com/Bucket" assets, the corresponding table
    /// name will be "mytable_storage_googleapis_com_Bucket". If any of these
    /// tables does not exist, a new table with the concatenated name will be
    /// created.
    ///
    /// When [content_type] in the ExportAssetsRequest is `RESOURCE`, the schema of
    /// each table will include RECORD-type columns mapped to the nested fields in
    /// the Asset.resource.data field of that asset type (up to the 15 nested level
    /// BigQuery supports
    /// (https://cloud.google.com/bigquery/docs/nested-repeated#limitations)). The
    /// fields in >15 nested levels will be stored in JSON format string as a child
    /// column of its parent RECORD column.
    ///
    /// If error occurs when exporting to any table, the whole export call will
    /// return an error but the export results that already succeed will persist.
    /// Example: if exporting to table_type_A succeeds when exporting to
    /// table_type_B fails during one export call, the results in table_type_A will
    /// persist and there will not be partial results persisting in a table.
    #[prost(bool, tag = "5")]
    pub separate_tables_per_asset_type: bool,
}
/// Specifications of BigQuery partitioned table as export destination.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartitionSpec {
    /// The partition key for BigQuery partitioned table.
    #[prost(enumeration = "partition_spec::PartitionKey", tag = "1")]
    pub partition_key: i32,
}
/// Nested message and enum types in `PartitionSpec`.
pub mod partition_spec {
    /// This enum is used to determine the partition key column when exporting
    /// assets to BigQuery partitioned table(s). Note that, if the partition key is
    /// a timestamp column, the actual partition is based on its date value
    /// (expressed in UTC. see details in
    /// https://cloud.google.com/bigquery/docs/partitioned-tables#date_timestamp_partitioned_tables).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PartitionKey {
        /// Unspecified partition key. If used, it means using non-partitioned table.
        Unspecified = 0,
        /// The time when the snapshot is taken. If specified as partition key, the
        /// result table(s) is partitoned by the additional timestamp column,
        /// readTime. If [read_time] in ExportAssetsRequest is specified, the
        /// readTime column's value will be the same as it. Otherwise, its value will
        /// be the current time that is used to take the snapshot.
        ReadTime = 1,
        /// The time when the request is received and started to be processed. If
        /// specified as partition key, the result table(s) is partitoned by the
        /// requestTime column, an additional timestamp column representing when the
        /// request was received.
        RequestTime = 2,
    }
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
    /// The Cloud Organization Policy set on an asset.
    OrgPolicy = 4,
    /// The Cloud Access context manager Policy set on an asset.
    AccessPolicy = 5,
    /// The related resources.
    Relationship = 7,
}
#[doc = r" Generated client implementations."]
pub mod asset_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Asset service definition."]
    #[derive(Debug, Clone)]
    pub struct AssetServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AssetServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> AssetServiceClient<InterceptedService<T, F>>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            AssetServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Exports assets with time and resource types to a given Cloud Storage"]
        #[doc = " location/BigQuery table. For Cloud Storage location destinations, the"]
        #[doc = " output format is newline-delimited JSON. Each line represents a"]
        #[doc = " [google.cloud.asset.v1p7beta1.Asset][google.cloud.asset.v1p7beta1.Asset] in"]
        #[doc = " the JSON format; for BigQuery table destinations, the output table stores"]
        #[doc = " the fields in asset proto as columns. This API implements the"]
        #[doc = " [google.longrunning.Operation][google.longrunning.Operation] API , which"]
        #[doc = " allows you to keep track of the export. We recommend intervals of at least"]
        #[doc = " 2 seconds with exponential retry to poll the export operation result. For"]
        #[doc = " regular-size resource parent, the export operation usually finishes within"]
        #[doc = " 5 minutes."]
        pub async fn export_assets(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportAssetsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.asset.v1p7beta1.AssetService/ExportAssets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
