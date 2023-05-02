/// The request for
/// \[ConnectionService.CreateConnection][google.cloud.bigquery.connection.v1beta1.ConnectionService.CreateConnection\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateConnectionRequest {
    /// Required. Parent resource name.
    /// Must be in the format `projects/{project_id}/locations/{location_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Connection id that should be assigned to the created connection.
    #[prost(string, tag = "2")]
    pub connection_id: ::prost::alloc::string::String,
    /// Required. Connection to create.
    #[prost(message, optional, tag = "3")]
    pub connection: ::core::option::Option<Connection>,
}
/// The request for
/// \[ConnectionService.GetConnection][google.cloud.bigquery.connection.v1beta1.ConnectionService.GetConnection\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConnectionRequest {
    /// Required. Name of the requested connection, for example:
    /// `projects/{project_id}/locations/{location_id}/connections/{connection_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request for
/// \[ConnectionService.ListConnections][google.cloud.bigquery.connection.v1beta1.ConnectionService.ListConnections\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConnectionsRequest {
    /// Required. Parent resource name.
    /// Must be in the form: `projects/{project_id}/locations/{location_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Maximum number of results per page.
    #[prost(message, optional, tag = "2")]
    pub max_results: ::core::option::Option<u32>,
    /// Page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response for
/// \[ConnectionService.ListConnections][google.cloud.bigquery.connection.v1beta1.ConnectionService.ListConnections\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConnectionsResponse {
    /// Next page token.
    #[prost(string, tag = "1")]
    pub next_page_token: ::prost::alloc::string::String,
    /// List of connections.
    #[prost(message, repeated, tag = "2")]
    pub connections: ::prost::alloc::vec::Vec<Connection>,
}
/// The request for
/// \[ConnectionService.UpdateConnection][google.cloud.bigquery.connection.v1beta1.ConnectionService.UpdateConnection\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateConnectionRequest {
    /// Required. Name of the connection to update, for example:
    /// `projects/{project_id}/locations/{location_id}/connections/{connection_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Connection containing the updated fields.
    #[prost(message, optional, tag = "2")]
    pub connection: ::core::option::Option<Connection>,
    /// Required. Update mask for the connection fields to be updated.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The request for
/// \[ConnectionService.UpdateConnectionCredential][google.cloud.bigquery.connection.v1beta1.ConnectionService.UpdateConnectionCredential\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateConnectionCredentialRequest {
    /// Required. Name of the connection, for example:
    /// `projects/{project_id}/locations/{location_id}/connections/{connection_id}/credential`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Credential to use with the connection.
    #[prost(message, optional, tag = "2")]
    pub credential: ::core::option::Option<ConnectionCredential>,
}
/// The request for \[ConnectionService.DeleteConnectionRequest][\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteConnectionRequest {
    /// Required. Name of the deleted connection, for example:
    /// `projects/{project_id}/locations/{location_id}/connections/{connection_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Configuration parameters to establish connection with an external data
/// source, except the credential attributes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Connection {
    /// The resource name of the connection in the form of:
    /// `projects/{project_id}/locations/{location_id}/connections/{connection_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// User provided display name for the connection.
    #[prost(string, tag = "2")]
    pub friendly_name: ::prost::alloc::string::String,
    /// User provided description.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The creation timestamp of the connection.
    #[prost(int64, tag = "5")]
    pub creation_time: i64,
    /// Output only. The last update timestamp of the connection.
    #[prost(int64, tag = "6")]
    pub last_modified_time: i64,
    /// Output only. True, if credential is configured for this connection.
    #[prost(bool, tag = "7")]
    pub has_credential: bool,
    /// Properties specific to the underlying data source.
    #[prost(oneof = "connection::Properties", tags = "4")]
    pub properties: ::core::option::Option<connection::Properties>,
}
/// Nested message and enum types in `Connection`.
pub mod connection {
    /// Properties specific to the underlying data source.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Properties {
        /// Cloud SQL properties.
        #[prost(message, tag = "4")]
        CloudSql(super::CloudSqlProperties),
    }
}
/// Credential to use with a connection.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionCredential {
    /// Credential specific to the underlying data source.
    #[prost(oneof = "connection_credential::Credential", tags = "1")]
    pub credential: ::core::option::Option<connection_credential::Credential>,
}
/// Nested message and enum types in `ConnectionCredential`.
pub mod connection_credential {
    /// Credential specific to the underlying data source.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Credential {
        /// Credential for Cloud SQL database.
        #[prost(message, tag = "1")]
        CloudSql(super::CloudSqlCredential),
    }
}
/// Connection properties specific to the Cloud SQL.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudSqlProperties {
    /// Cloud SQL instance ID in the form `project:location:instance`.
    #[prost(string, tag = "1")]
    pub instance_id: ::prost::alloc::string::String,
    /// Database name.
    #[prost(string, tag = "2")]
    pub database: ::prost::alloc::string::String,
    /// Type of the Cloud SQL database.
    #[prost(enumeration = "cloud_sql_properties::DatabaseType", tag = "3")]
    pub r#type: i32,
    /// Input only. Cloud SQL credential.
    #[prost(message, optional, tag = "4")]
    pub credential: ::core::option::Option<CloudSqlCredential>,
    /// Output only. The account ID of the service used for the purpose of this
    /// connection.
    ///
    /// When the connection is used in the context of an operation in
    /// BigQuery, this service account will serve as the identity being used for
    /// connecting to the CloudSQL instance specified in this connection.
    #[prost(string, tag = "5")]
    pub service_account_id: ::prost::alloc::string::String,
}
/// Nested message and enum types in `CloudSqlProperties`.
pub mod cloud_sql_properties {
    /// Supported Cloud SQL database types.
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
    pub enum DatabaseType {
        /// Unspecified database type.
        Unspecified = 0,
        /// Cloud SQL for PostgreSQL.
        Postgres = 1,
        /// Cloud SQL for MySQL.
        Mysql = 2,
    }
    impl DatabaseType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DatabaseType::Unspecified => "DATABASE_TYPE_UNSPECIFIED",
                DatabaseType::Postgres => "POSTGRES",
                DatabaseType::Mysql => "MYSQL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DATABASE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "POSTGRES" => Some(Self::Postgres),
                "MYSQL" => Some(Self::Mysql),
                _ => None,
            }
        }
    }
}
/// Credential info for the Cloud SQL.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudSqlCredential {
    /// The username for the credential.
    #[prost(string, tag = "1")]
    pub username: ::prost::alloc::string::String,
    /// The password for the credential.
    #[prost(string, tag = "2")]
    pub password: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod connection_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Manages external data source connections and credentials.
    #[derive(Debug, Clone)]
    pub struct ConnectionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ConnectionServiceClient<tonic::transport::Channel> {
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
    impl<T> ConnectionServiceClient<T>
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
        ) -> ConnectionServiceClient<InterceptedService<T, F>>
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
            ConnectionServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a new connection.
        pub async fn create_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateConnectionRequest>,
        ) -> std::result::Result<tonic::Response<super::Connection>, tonic::Status> {
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
                "/google.cloud.bigquery.connection.v1beta1.ConnectionService/CreateConnection",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.connection.v1beta1.ConnectionService",
                        "CreateConnection",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns specified connection.
        pub async fn get_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConnectionRequest>,
        ) -> std::result::Result<tonic::Response<super::Connection>, tonic::Status> {
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
                "/google.cloud.bigquery.connection.v1beta1.ConnectionService/GetConnection",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.connection.v1beta1.ConnectionService",
                        "GetConnection",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns a list of connections in the given project.
        pub async fn list_connections(
            &mut self,
            request: impl tonic::IntoRequest<super::ListConnectionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListConnectionsResponse>,
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
                "/google.cloud.bigquery.connection.v1beta1.ConnectionService/ListConnections",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.connection.v1beta1.ConnectionService",
                        "ListConnections",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the specified connection. For security reasons, also resets
        /// credential if connection properties are in the update field mask.
        pub async fn update_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateConnectionRequest>,
        ) -> std::result::Result<tonic::Response<super::Connection>, tonic::Status> {
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
                "/google.cloud.bigquery.connection.v1beta1.ConnectionService/UpdateConnection",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.connection.v1beta1.ConnectionService",
                        "UpdateConnection",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Sets the credential for the specified connection.
        pub async fn update_connection_credential(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateConnectionCredentialRequest>,
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
                "/google.cloud.bigquery.connection.v1beta1.ConnectionService/UpdateConnectionCredential",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.connection.v1beta1.ConnectionService",
                        "UpdateConnectionCredential",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes connection and associated credential.
        pub async fn delete_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteConnectionRequest>,
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
                "/google.cloud.bigquery.connection.v1beta1.ConnectionService/DeleteConnection",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.connection.v1beta1.ConnectionService",
                        "DeleteConnection",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the access control policy for a resource.
        /// Returns an empty policy if the resource exists and does not have a policy
        /// set.
        pub async fn get_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::iam::v1::GetIamPolicyRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::iam::v1::Policy>,
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
                "/google.cloud.bigquery.connection.v1beta1.ConnectionService/GetIamPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.connection.v1beta1.ConnectionService",
                        "GetIamPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Sets the access control policy on the specified resource. Replaces any
        /// existing policy.
        ///
        /// Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
        pub async fn set_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::iam::v1::SetIamPolicyRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::iam::v1::Policy>,
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
                "/google.cloud.bigquery.connection.v1beta1.ConnectionService/SetIamPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.connection.v1beta1.ConnectionService",
                        "SetIamPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns permissions that a caller has on the specified resource.
        /// If the resource does not exist, this will return an empty set of
        /// permissions, not a `NOT_FOUND` error.
        ///
        /// Note: This operation is designed to be used for building permission-aware
        /// UIs and command-line tools, not for authorization checking. This operation
        /// may "fail open" without warning.
        pub async fn test_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::super::super::iam::v1::TestIamPermissionsResponse,
            >,
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
                "/google.cloud.bigquery.connection.v1beta1.ConnectionService/TestIamPermissions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.connection.v1beta1.ConnectionService",
                        "TestIamPermissions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
