/// Represents a collection of external workload identities. You can define IAM
/// policies to grant these identities access to Google Cloud resources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkloadIdentityPool {
    /// Output only. The resource name of the pool.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// A display name for the pool. Cannot exceed 32 characters.
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    /// A description of the pool. Cannot exceed 256 characters.
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The state of the pool.
    #[prost(enumeration="workload_identity_pool::State", tag="4")]
    pub state: i32,
    /// Whether the pool is disabled. You cannot use a disabled pool to exchange
    /// tokens, or use existing tokens to access resources. If
    /// the pool is re-enabled, existing tokens grant access again.
    #[prost(bool, tag="5")]
    pub disabled: bool,
}
/// Nested message and enum types in `WorkloadIdentityPool`.
pub mod workload_identity_pool {
    /// The current state of the pool.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// State unspecified.
        Unspecified = 0,
        /// The pool is active, and may be used in Google Cloud policies.
        Active = 1,
        /// The pool is soft-deleted. Soft-deleted pools are permanently deleted
        /// after approximately 30 days. You can restore a soft-deleted pool using
        /// \[UndeleteWorkloadIdentityPool][google.iam.v1beta.WorkloadIdentityPools.UndeleteWorkloadIdentityPool\].
        ///
        /// You cannot reuse the ID of a soft-deleted pool until it is permanently
        /// deleted.
        ///
        /// While a pool is deleted, you cannot use it to exchange tokens, or use
        /// existing tokens to access resources. If the pool is undeleted, existing
        /// tokens grant access again.
        Deleted = 2,
    }
}
/// A configuration for an external identity provider.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkloadIdentityPoolProvider {
    /// Output only. The resource name of the provider.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// A display name for the provider. Cannot exceed 32 characters.
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    /// A description for the provider. Cannot exceed 256 characters.
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The state of the provider.
    #[prost(enumeration="workload_identity_pool_provider::State", tag="4")]
    pub state: i32,
    /// Whether the provider is disabled. You cannot use a disabled provider to
    /// exchange tokens. However, existing tokens still grant access.
    #[prost(bool, tag="5")]
    pub disabled: bool,
    /// Maps attributes from authentication credentials issued by an external
    /// identity provider to Google Cloud attributes, such as `subject` and
    /// `segment`.
    ///
    /// Each key must be a string specifying the Google Cloud IAM attribute to
    /// map to.
    ///
    /// The following keys are supported:
    ///
    /// * `google.subject`: The principal IAM is authenticating. You can reference
    ///                     this value in IAM bindings. This is also the
    ///                     subject that appears in Cloud Logging logs.
    ///                     Cannot exceed 127 characters.
    ///
    /// * `google.groups`: Groups the external identity belongs to. You can grant
    ///                    groups access to resources using an IAM `principalSet`
    ///                    binding; access applies to all members of the group.
    ///
    /// You can also provide custom attributes by specifying
    /// `attribute.{custom_attribute}`, where `{custom_attribute}` is the name of
    /// the custom attribute to be mapped. You can define a maximum of 50 custom
    /// attributes. The maximum length of a mapped attribute key is
    /// 100 characters, and the key may only contain the characters \[a-z0-9_\].
    ///
    /// You can reference these attributes in IAM policies to define fine-grained
    /// access for a workload to Google Cloud resources. For example:
    ///
    /// * `google.subject`:
    /// `principal://iam.googleapis.com/projects/{project}/locations/{location}/workloadIdentityPools/{pool}/subject/{value}`
    ///
    /// * `google.groups`:
    /// `principalSet://iam.googleapis.com/projects/{project}/locations/{location}/workloadIdentityPools/{pool}/group/{value}`
    ///
    /// * `attribute.{custom_attribute}`:
    /// `principalSet://iam.googleapis.com/projects/{project}/locations/{location}/workloadIdentityPools/{pool}/attribute.{custom_attribute}/{value}`
    ///
    /// Each value must be a [Common Expression Language]
    /// (<https://opensource.google/projects/cel>) function that maps an
    /// identity provider credential to the normalized attribute specified by the
    /// corresponding map key.
    ///
    /// You can use the `assertion` keyword in the expression to access a JSON
    /// representation of the authentication credential issued by the provider.
    ///
    /// The maximum length of an attribute mapping expression is 2048 characters.
    /// When evaluated, the total size of all mapped attributes must not exceed
    /// 8KB.
    ///
    /// For AWS providers, the following rules apply:
    ///
    /// - If no attribute mapping is defined, the following default mapping
    ///   applies:
    ///
    ///   ```
    ///   {
    ///     "google.subject":"assertion.arn",
    ///     "attribute.aws_role":
    ///         "assertion.arn.contains('assumed-role')"
    ///         " ? assertion.arn.extract('{account_arn}assumed-role/')"
    ///         "   + 'assumed-role/'"
    ///         "   + assertion.arn.extract('assumed-role/{role_name}/')"
    ///         " : assertion.arn",
    ///   }
    ///   ```
    ///
    /// - If any custom attribute mappings are defined, they must include a mapping
    ///   to the `google.subject` attribute.
    ///
    ///
    /// For OIDC providers, the following rules apply:
    ///
    /// - Custom attribute mappings must be defined, and must include a mapping to
    ///   the `google.subject` attribute. For example, the following maps the
    ///   `sub` claim of the incoming credential to the `subject` attribute on
    ///   a Google token.
    ///
    ///   ```
    ///   {"google.subject": "assertion.sub"}
    ///   ```
    #[prost(map="string, string", tag="6")]
    pub attribute_mapping: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// [A Common Expression Language](<https://opensource.google/projects/cel>)
    /// expression, in plain text, to restrict what otherwise valid authentication
    /// credentials issued by the provider should not be accepted.
    ///
    /// The expression must output a boolean representing whether to allow the
    /// federation.
    ///
    /// The following keywords may be referenced in the expressions:
    ///
    /// * `assertion`: JSON representing the authentication credential issued by
    ///                the provider.
    /// * `google`: The Google attributes mapped from the assertion in the
    ///             `attribute_mappings`.
    /// * `attribute`: The custom attributes mapped from the assertion in the
    ///                `attribute_mappings`.
    ///
    /// The maximum length of the attribute condition expression is 4096
    /// characters. If unspecified, all valid authentication credential are
    /// accepted.
    ///
    /// The following example shows how to only allow credentials with a mapped
    /// `google.groups` value of `admins`:
    ///
    /// ```
    /// "'admins' in google.groups"
    /// ```
    #[prost(string, tag="7")]
    pub attribute_condition: ::prost::alloc::string::String,
    /// Identity provider configuration types.
    #[prost(oneof="workload_identity_pool_provider::ProviderConfig", tags="8, 9")]
    pub provider_config: ::core::option::Option<workload_identity_pool_provider::ProviderConfig>,
}
/// Nested message and enum types in `WorkloadIdentityPoolProvider`.
pub mod workload_identity_pool_provider {
    /// Represents an Amazon Web Services identity provider.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Aws {
        /// Required. The AWS account ID.
        #[prost(string, tag="1")]
        pub account_id: ::prost::alloc::string::String,
    }
    /// Represents an OpenId Connect 1.0 identity provider.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Oidc {
        /// Required. The OIDC issuer URL.
        #[prost(string, tag="1")]
        pub issuer_uri: ::prost::alloc::string::String,
        /// Acceptable values for the `aud` field (audience) in the OIDC token. Token
        /// exchange requests are rejected if the token audience does not match one
        /// of the configured values. Each audience may be at most 256 characters. A
        /// maximum of 10 audiences may be configured.
        ///
        /// If this list is empty, the OIDC token audience must be equal to
        /// the full canonical resource name of the WorkloadIdentityPoolProvider,
        /// with or without the HTTPS prefix. For example:
        ///
        /// ```
        /// //iam.googleapis.com/projects/<project-number>/locations/<location>/workloadIdentityPools/<pool-id>/providers/<provider-id>
        /// <https://iam.googleapis.com/projects/<project-number>/locations/<location>/workloadIdentityPools/<pool-id>/providers/<provider-id>>
        /// ```
        #[prost(string, repeated, tag="2")]
        pub allowed_audiences: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// The current state of the provider.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// State unspecified.
        Unspecified = 0,
        /// The provider is active, and may be used to validate authentication
        /// credentials.
        Active = 1,
        /// The provider is soft-deleted. Soft-deleted providers are permanently
        /// deleted after approximately 30 days. You can restore a soft-deleted
        /// provider using
        /// \[UndeleteWorkloadIdentityPoolProvider][google.iam.v1beta.WorkloadIdentityPools.UndeleteWorkloadIdentityPoolProvider\].
        ///
        /// You cannot reuse the ID of a soft-deleted provider until it is
        /// permanently deleted.
        Deleted = 2,
    }
    /// Identity provider configuration types.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ProviderConfig {
        /// An Amazon Web Services identity provider.
        #[prost(message, tag="8")]
        Aws(Aws),
        /// An OpenId Connect 1.0 identity provider.
        #[prost(message, tag="9")]
        Oidc(Oidc),
    }
}
/// Request message for ListWorkloadIdentityPools.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkloadIdentityPoolsRequest {
    /// Required. The parent resource to list pools for.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of pools to return.
    /// If unspecified, at most 50 pools are returned.
    /// The maximum value is 1000; values above are 1000 truncated to 1000.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListWorkloadIdentityPools`
    /// call. Provide this to retrieve the subsequent page.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// Whether to return soft-deleted pools.
    #[prost(bool, tag="4")]
    pub show_deleted: bool,
}
/// Response message for ListWorkloadIdentityPools.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkloadIdentityPoolsResponse {
    /// A list of pools.
    #[prost(message, repeated, tag="1")]
    pub workload_identity_pools: ::prost::alloc::vec::Vec<WorkloadIdentityPool>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for GetWorkloadIdentityPool.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWorkloadIdentityPoolRequest {
    /// Required. The name of the pool to retrieve.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for CreateWorkloadIdentityPool.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWorkloadIdentityPoolRequest {
    /// Required. The parent resource to create the pool in. The only supported
    /// location is `global`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The pool to create.
    #[prost(message, optional, tag="2")]
    pub workload_identity_pool: ::core::option::Option<WorkloadIdentityPool>,
    /// Required. The ID to use for the pool, which becomes the
    /// final component of the resource name. This value should be 4-32 characters,
    /// and may contain the characters \[a-z0-9-\]. The prefix `gcp-` is
    /// reserved for use by Google, and may not be specified.
    #[prost(string, tag="3")]
    pub workload_identity_pool_id: ::prost::alloc::string::String,
}
/// Request message for UpdateWorkloadIdentityPool.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateWorkloadIdentityPoolRequest {
    /// Required. The pool to update. The `name` field is used to identify the pool.
    #[prost(message, optional, tag="1")]
    pub workload_identity_pool: ::core::option::Option<WorkloadIdentityPool>,
    /// Required. The list of fields update.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for DeleteWorkloadIdentityPool.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteWorkloadIdentityPoolRequest {
    /// Required. The name of the pool to delete.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for UndeleteWorkloadIdentityPool.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndeleteWorkloadIdentityPoolRequest {
    /// Required. The name of the pool to undelete.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListWorkloadIdentityPoolProviders.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkloadIdentityPoolProvidersRequest {
    /// Required. The pool to list providers for.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of providers to return.
    /// If unspecified, at most 50 providers are returned.
    /// The maximum value is 100; values above 100 are truncated to 100.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// A page token, received from a previous
    /// `ListWorkloadIdentityPoolProviders` call. Provide this to retrieve the
    /// subsequent page.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// Whether to return soft-deleted providers.
    #[prost(bool, tag="4")]
    pub show_deleted: bool,
}
/// Response message for ListWorkloadIdentityPoolProviders.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkloadIdentityPoolProvidersResponse {
    /// A list of providers.
    #[prost(message, repeated, tag="1")]
    pub workload_identity_pool_providers: ::prost::alloc::vec::Vec<WorkloadIdentityPoolProvider>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for GetWorkloadIdentityPoolProvider.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWorkloadIdentityPoolProviderRequest {
    /// Required. The name of the provider to retrieve.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for CreateWorkloadIdentityPoolProvider.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWorkloadIdentityPoolProviderRequest {
    /// Required. The pool to create this provider in.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The provider to create.
    #[prost(message, optional, tag="2")]
    pub workload_identity_pool_provider: ::core::option::Option<WorkloadIdentityPoolProvider>,
    /// Required. The ID for the provider, which becomes the
    /// final component of the resource name. This value must be 4-32 characters,
    /// and may contain the characters \[a-z0-9-\]. The prefix `gcp-` is
    /// reserved for use by Google, and may not be specified.
    #[prost(string, tag="3")]
    pub workload_identity_pool_provider_id: ::prost::alloc::string::String,
}
/// Request message for UpdateWorkloadIdentityPoolProvider.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateWorkloadIdentityPoolProviderRequest {
    /// Required. The provider to update.
    #[prost(message, optional, tag="1")]
    pub workload_identity_pool_provider: ::core::option::Option<WorkloadIdentityPoolProvider>,
    /// Required. The list of fields to update.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for DeleteWorkloadIdentityPoolProvider.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteWorkloadIdentityPoolProviderRequest {
    /// Required. The name of the provider to delete.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for UndeleteWorkloadIdentityPoolProvider.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndeleteWorkloadIdentityPoolProviderRequest {
    /// Required. The name of the provider to undelete.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Metadata for long-running WorkloadIdentityPool operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkloadIdentityPoolOperationMetadata {
}
/// Metadata for long-running WorkloadIdentityPoolProvider operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkloadIdentityPoolProviderOperationMetadata {
}
/// Generated client implementations.
pub mod workload_identity_pools_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Manages WorkloadIdentityPools.
    #[derive(Debug, Clone)]
    pub struct WorkloadIdentityPoolsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl WorkloadIdentityPoolsClient<tonic::transport::Channel> {
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
    impl<T> WorkloadIdentityPoolsClient<T>
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
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> WorkloadIdentityPoolsClient<InterceptedService<T, F>>
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
            WorkloadIdentityPoolsClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        /// Lists all non-deleted
        /// [WorkloadIdentityPool][google.iam.v1beta.WorkloadIdentityPool]s in a
        /// project. If `show_deleted` is set to `true`, then deleted pools are also
        /// listed.
        pub async fn list_workload_identity_pools(
            &mut self,
            request: impl tonic::IntoRequest<super::ListWorkloadIdentityPoolsRequest>,
        ) -> Result<
            tonic::Response<super::ListWorkloadIdentityPoolsResponse>,
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
                "/google.iam.v1beta.WorkloadIdentityPools/ListWorkloadIdentityPools",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets an individual
        /// [WorkloadIdentityPool][google.iam.v1beta.WorkloadIdentityPool].
        pub async fn get_workload_identity_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::GetWorkloadIdentityPoolRequest>,
        ) -> Result<tonic::Response<super::WorkloadIdentityPool>, tonic::Status> {
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
                "/google.iam.v1beta.WorkloadIdentityPools/GetWorkloadIdentityPool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new
        /// [WorkloadIdentityPool][google.iam.v1beta.WorkloadIdentityPool].
        ///
        /// You cannot reuse the name of a deleted pool until 30 days after deletion.
        pub async fn create_workload_identity_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateWorkloadIdentityPoolRequest>,
        ) -> Result<
            tonic::Response<super::super::super::longrunning::Operation>,
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
                "/google.iam.v1beta.WorkloadIdentityPools/CreateWorkloadIdentityPool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates an existing
        /// [WorkloadIdentityPool][google.iam.v1beta.WorkloadIdentityPool].
        pub async fn update_workload_identity_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateWorkloadIdentityPoolRequest>,
        ) -> Result<
            tonic::Response<super::super::super::longrunning::Operation>,
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
                "/google.iam.v1beta.WorkloadIdentityPools/UpdateWorkloadIdentityPool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a
        /// [WorkloadIdentityPool][google.iam.v1beta.WorkloadIdentityPool].
        ///
        /// You cannot use a deleted pool to exchange external
        /// credentials for Google Cloud credentials. However, deletion does
        /// not revoke credentials that have already been issued.
        /// Credentials issued for a deleted pool do not grant access to resources.
        /// If the pool is undeleted, and the credentials are not expired, they
        /// grant access again.
        /// You can undelete a pool for 30 days. After 30 days, deletion is
        /// permanent.
        /// You cannot update deleted pools. However, you can view and list them.
        pub async fn delete_workload_identity_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteWorkloadIdentityPoolRequest>,
        ) -> Result<
            tonic::Response<super::super::super::longrunning::Operation>,
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
                "/google.iam.v1beta.WorkloadIdentityPools/DeleteWorkloadIdentityPool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Undeletes a [WorkloadIdentityPool][google.iam.v1beta.WorkloadIdentityPool],
        /// as long as it was deleted fewer than 30 days ago.
        pub async fn undelete_workload_identity_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::UndeleteWorkloadIdentityPoolRequest>,
        ) -> Result<
            tonic::Response<super::super::super::longrunning::Operation>,
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
                "/google.iam.v1beta.WorkloadIdentityPools/UndeleteWorkloadIdentityPool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists all non-deleted
        /// [WorkloadIdentityPoolProvider][google.iam.v1beta.WorkloadIdentityPoolProvider]s
        /// in a [WorkloadIdentityPool][google.iam.v1beta.WorkloadIdentityPool].
        /// If `show_deleted` is set to `true`, then deleted providers are also listed.
        pub async fn list_workload_identity_pool_providers(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ListWorkloadIdentityPoolProvidersRequest,
            >,
        ) -> Result<
            tonic::Response<super::ListWorkloadIdentityPoolProvidersResponse>,
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
                "/google.iam.v1beta.WorkloadIdentityPools/ListWorkloadIdentityPoolProviders",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets an individual
        /// [WorkloadIdentityPoolProvider][google.iam.v1beta.WorkloadIdentityPoolProvider].
        pub async fn get_workload_identity_pool_provider(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetWorkloadIdentityPoolProviderRequest,
            >,
        ) -> Result<
            tonic::Response<super::WorkloadIdentityPoolProvider>,
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
                "/google.iam.v1beta.WorkloadIdentityPools/GetWorkloadIdentityPoolProvider",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new
        /// [WorkloadIdentityPoolProvider][google.iam.v1beta.WorkloadIdentityProvider]
        /// in a [WorkloadIdentityPool][google.iam.v1beta.WorkloadIdentityPool].
        ///
        /// You cannot reuse the name of a deleted provider until 30 days after
        /// deletion.
        pub async fn create_workload_identity_pool_provider(
            &mut self,
            request: impl tonic::IntoRequest<
                super::CreateWorkloadIdentityPoolProviderRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::longrunning::Operation>,
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
                "/google.iam.v1beta.WorkloadIdentityPools/CreateWorkloadIdentityPoolProvider",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates an existing
        /// [WorkloadIdentityPoolProvider][google.iam.v1beta.WorkloadIdentityProvider].
        pub async fn update_workload_identity_pool_provider(
            &mut self,
            request: impl tonic::IntoRequest<
                super::UpdateWorkloadIdentityPoolProviderRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::longrunning::Operation>,
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
                "/google.iam.v1beta.WorkloadIdentityPools/UpdateWorkloadIdentityPoolProvider",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a
        /// [WorkloadIdentityPoolProvider][google.iam.v1beta.WorkloadIdentityProvider].
        /// Deleting a provider does not revoke credentials that have already been
        /// issued; they continue to grant access.
        /// You can undelete a provider for 30 days. After 30 days, deletion is
        /// permanent.
        /// You cannot update deleted providers. However, you can view and list them.
        pub async fn delete_workload_identity_pool_provider(
            &mut self,
            request: impl tonic::IntoRequest<
                super::DeleteWorkloadIdentityPoolProviderRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::longrunning::Operation>,
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
                "/google.iam.v1beta.WorkloadIdentityPools/DeleteWorkloadIdentityPoolProvider",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Undeletes a
        /// [WorkloadIdentityPoolProvider][google.iam.v1beta.WorkloadIdentityProvider],
        /// as long as it was deleted fewer than 30 days ago.
        pub async fn undelete_workload_identity_pool_provider(
            &mut self,
            request: impl tonic::IntoRequest<
                super::UndeleteWorkloadIdentityPoolProviderRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::longrunning::Operation>,
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
                "/google.iam.v1beta.WorkloadIdentityPools/UndeleteWorkloadIdentityPoolProvider",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
