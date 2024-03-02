/// A Parallelstore instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instance {
    /// Identifier. The resource name of the instance, in the format
    /// `projects/{project}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The description of the instance. 2048 characters or less.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The instance state.
    #[prost(enumeration = "instance::State", tag = "3")]
    pub state: i32,
    /// Output only. The time when the instance was created.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the instance was updated.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Cloud Labels are a flexible and lightweight mechanism for
    /// organizing cloud resources into groups that reflect a customer's
    /// organizational needs and deployment strategies. Cloud Labels can be used to
    /// filter collections of resources. They can be used to control how resource
    /// metrics are aggregated. And they can be used as arguments to policy
    /// management rules (e.g. route, firewall, load balancing, etc.).
    ///
    ///   * Label keys must be between 1 and 63 characters long and must conform to
    ///     the following regular expression: `[a-z][a-z0-9_-]{0,62}`.
    ///   * Label values must be between 0 and 63 characters long and must conform
    ///     to the regular expression `\[a-z0-9_-\]{0,63}`.
    ///   * No more than 64 labels can be associated with a given resource.
    ///
    /// See <https://goo.gl/xmQnxf> for more information on and examples of labels.
    ///
    /// If you plan to use labels in your own code, please note that additional
    /// characters may be allowed in the future. Therefore, you are advised to use
    /// an internal label representation, such as JSON, which doesn't rely upon
    /// specific characters being disallowed.  For example, representing labels
    /// as the string:  name + "_" + value  would prove problematic if we were to
    /// allow "_" in a future release.
    #[prost(map = "string, string", tag = "6")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Required. Immutable. Storage capacity of Parallelstore instance in
    /// Gibibytes (GiB).
    #[prost(int64, tag = "8")]
    pub capacity_gib: i64,
    /// Output only. The version of DAOS software running in the instance
    #[prost(string, tag = "9")]
    pub daos_version: ::prost::alloc::string::String,
    /// Output only. List of access_points.
    /// Contains a list of IPv4 addresses used for client side configuration.
    #[prost(string, repeated, tag = "10")]
    pub access_points: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. Immutable. The name of the Google Compute Engine
    /// [VPC network](<https://cloud.google.com/vpc/docs/vpc>) to which the
    /// instance is connected.
    #[prost(string, tag = "11")]
    pub network: ::prost::alloc::string::String,
    /// Optional. Immutable. Contains the id of allocated IP address range
    /// associated with the private service access connection for example,
    /// "test-default" associated with IP range 10.0.0.0/29. If no range id is
    /// provided all ranges will be considered.
    #[prost(string, tag = "12")]
    pub reserved_ip_range: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Instance`.
pub mod instance {
    /// Represents the different states of a Parallelstore instance.
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
    pub enum State {
        /// Not set.
        Unspecified = 0,
        /// The instance is being created.
        Creating = 1,
        /// The instance is available for use.
        Active = 2,
        /// The instance is being deleted.
        Deleting = 3,
        /// The instance is not usable.
        Failed = 4,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Creating => "CREATING",
                State::Active => "ACTIVE",
                State::Deleting => "DELETING",
                State::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "ACTIVE" => Some(Self::Active),
                "DELETING" => Some(Self::Deleting),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
}
/// Message for requesting list of Instances
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesRequest {
    /// Required. The project and location for which to retrieve instance
    /// information, in the format `projects/{project_id}/locations/{location}`.
    /// For Parallelstore locations map to Google Cloud zones, for example
    /// **us-central1-a**.
    /// To retrieve instance information for all locations, use "-" for the
    /// `{location}` value.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server may return fewer items than
    /// requested. If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filtering results
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Hint for how to order the results
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Message for response to listing Instances
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesResponse {
    /// The list of Parallelstore Instances
    #[prost(message, repeated, tag = "1")]
    pub instances: ::prost::alloc::vec::Vec<Instance>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request to get an instance's details.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstanceRequest {
    /// Required. The instance resource name, in the format
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for
/// [CreateInstance][google.cloud.parallelstore.v1beta.Parallelstore.CreateInstance]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInstanceRequest {
    /// Required. The instance's project and location, in the format
    /// `projects/{project}/locations/{location}`.
    /// Locations map to Google Cloud zones, for example **us-west1-b**.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The logical name of the Parallelstore instance in the user
    /// project with the following restrictions:
    ///
    /// * Must contain only lowercase letters, numbers, and hyphens.
    /// * Must start with a letter.
    /// * Must be between 1-63 characters.
    /// * Must end with a number or a letter.
    /// * Must be unique within the customer project / location
    #[prost(string, tag = "2")]
    pub instance_id: ::prost::alloc::string::String,
    /// Required. The instance to create.
    #[prost(message, optional, tag = "3")]
    pub instance: ::core::option::Option<Instance>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for updating a Instance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInstanceRequest {
    /// Required. Mask of fields to update .Field mask is used to specify the
    /// fields to be overwritten in the Instance resource by the update. At least
    /// one path must be supplied in this field. The fields specified in the
    /// update_mask are relative to the resource, not the full request.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The instance to update
    #[prost(message, optional, tag = "2")]
    pub instance: ::core::option::Option<Instance>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for deleting a Instance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteInstanceRequest {
    /// Required. Name of the resource
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Represents the metadata of the long-running operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// Output only. The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Server-defined resource path for the target of the operation.
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// Output only. Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: ::prost::alloc::string::String,
    /// Output only. Human-readable status of the operation, if any.
    #[prost(string, tag = "5")]
    pub status_message: ::prost::alloc::string::String,
    /// Output only. Identifies whether the user has requested cancellation
    /// of the operation. Operations that have been cancelled successfully
    /// have [Operation.error][] value with a
    /// [google.rpc.Status.code][google.rpc.Status.code] of 1, corresponding to
    /// `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod parallelstore_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service describing handlers for resources
    /// Configures and manages parallelstore resources.
    ///
    /// Parallelstore service.
    ///
    /// The `parallelstore.googleapis.com` service implements the parallelstore API
    /// and defines the following resource model for managing instances:
    /// * The service works with a collection of cloud projects, named: `/projects/*`
    /// * Each project has a collection of available locations, named: `/locations/*`
    /// * Each location has a collection of instances named `/instances/*`.
    /// * Parallelstore instances are resources of the form:
    ///   `/projects/{project_id}/locations/{location_id}/instances/{instance_id}`
    ///
    /// Note that location_id must be a Google Cloud `zone`; for example:
    /// * `projects/12345/locations/us-central1-c/instances/my-parallelstore-share`
    #[derive(Debug, Clone)]
    pub struct ParallelstoreClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ParallelstoreClient<tonic::transport::Channel> {
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
    impl<T> ParallelstoreClient<T>
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
        ) -> ParallelstoreClient<InterceptedService<T, F>>
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
            ParallelstoreClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists Instances in a given project and location.
        pub async fn list_instances(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInstancesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListInstancesResponse>,
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
                "/google.cloud.parallelstore.v1beta.Parallelstore/ListInstances",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.parallelstore.v1beta.Parallelstore",
                        "ListInstances",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single Instance.
        pub async fn get_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInstanceRequest>,
        ) -> std::result::Result<tonic::Response<super::Instance>, tonic::Status> {
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
                "/google.cloud.parallelstore.v1beta.Parallelstore/GetInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.parallelstore.v1beta.Parallelstore",
                        "GetInstance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a Parallelstore instance in a given project and location.
        pub async fn create_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateInstanceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.parallelstore.v1beta.Parallelstore/CreateInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.parallelstore.v1beta.Parallelstore",
                        "CreateInstance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the parameters of a single Instance.
        pub async fn update_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateInstanceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.parallelstore.v1beta.Parallelstore/UpdateInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.parallelstore.v1beta.Parallelstore",
                        "UpdateInstance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single Instance.
        pub async fn delete_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteInstanceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.parallelstore.v1beta.Parallelstore/DeleteInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.parallelstore.v1beta.Parallelstore",
                        "DeleteInstance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
