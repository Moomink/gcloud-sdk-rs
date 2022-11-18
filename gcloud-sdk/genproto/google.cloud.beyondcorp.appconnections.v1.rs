/// Request message for BeyondCorp.ListAppConnections.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAppConnectionsRequest {
    /// Required. The resource name of the AppConnection location using the form:
    /// `projects/{project_id}/locations/{location_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of items to return.
    /// If not specified, a default value of 50 will be used by the service.
    /// Regardless of the page_size value, the response may include a partial list
    /// and a caller should only rely on response's
    /// \[next_page_token][BeyondCorp.ListAppConnectionsResponse.next_page_token\] to
    /// determine if there are more instances left to be queried.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The next_page_token value returned from a previous
    /// ListAppConnectionsRequest, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. A filter specifying constraints of a list operation.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Specifies the ordering of results. See
    /// [Sorting
    /// order](<https://cloud.google.com/apis/design/design_patterns#sorting_order>)
    /// for more information.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for BeyondCorp.ListAppConnections.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAppConnectionsResponse {
    /// A list of BeyondCorp AppConnections in the project.
    #[prost(message, repeated, tag = "1")]
    pub app_connections: ::prost::alloc::vec::Vec<AppConnection>,
    /// A token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// A list of locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for BeyondCorp.GetAppConnection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAppConnectionRequest {
    /// Required. BeyondCorp AppConnection name using the form:
    /// `projects/{project_id}/locations/{location_id}/appConnections/{app_connection_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for BeyondCorp.CreateAppConnection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAppConnectionRequest {
    /// Required. The resource project name of the AppConnection location using the
    /// form: `projects/{project_id}/locations/{location_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. User-settable AppConnection resource ID.
    ///   * Must start with a letter.
    ///   * Must contain between 4-63 characters from `/\[a-z][0-9\]-/`.
    ///   * Must end with a number or a letter.
    #[prost(string, tag = "2")]
    pub app_connection_id: ::prost::alloc::string::String,
    /// Required. A BeyondCorp AppConnection resource.
    #[prost(message, optional, tag = "3")]
    pub app_connection: ::core::option::Option<AppConnection>,
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
    /// Optional. If set, validates request by executing a dry-run which would not
    /// alter the resource in any way.
    #[prost(bool, tag = "5")]
    pub validate_only: bool,
}
/// Request message for BeyondCorp.UpdateAppConnection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAppConnectionRequest {
    /// Required. Mask of fields to update. At least one path must be supplied in
    /// this field. The elements of the repeated paths field may only include these
    /// fields from \[BeyondCorp.AppConnection\]:
    /// * `labels`
    /// * `display_name`
    /// * `application_endpoint`
    /// * `connectors`
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. AppConnection message with updated fields. Only supported fields
    /// specified in update_mask are updated.
    #[prost(message, optional, tag = "2")]
    pub app_connection: ::core::option::Option<AppConnection>,
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
    /// Optional. If set, validates request by executing a dry-run which would not
    /// alter the resource in any way.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
    /// Optional. If set as true, will create the resource if it is not found.
    #[prost(bool, tag = "5")]
    pub allow_missing: bool,
}
/// Request message for BeyondCorp.DeleteAppConnection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAppConnectionRequest {
    /// Required. BeyondCorp Connector name using the form:
    /// `projects/{project_id}/locations/{location_id}/appConnections/{app_connection_id}`
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
    /// Optional. If set, validates request by executing a dry-run which would not
    /// alter the resource in any way.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
}
/// Request message for BeyondCorp.ResolveAppConnections.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveAppConnectionsRequest {
    /// Required. The resource name of the AppConnection location using the form:
    /// `projects/{project_id}/locations/{location_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. BeyondCorp Connector name of the connector associated with those
    /// AppConnections using the form:
    /// `projects/{project_id}/locations/{location_id}/appConnectors/{app_connector_id}`
    #[prost(string, tag = "2")]
    pub app_connector_id: ::prost::alloc::string::String,
    /// Optional. The maximum number of items to return.
    /// If not specified, a default value of 50 will be used by the service.
    /// Regardless of the page_size value, the response may include a partial list
    /// and a caller should only rely on response's
    /// \[next_page_token][BeyondCorp.ResolveAppConnectionsResponse.next_page_token\]
    /// to determine if there are more instances left to be queried.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Optional. The next_page_token value returned from a previous
    /// ResolveAppConnectionsResponse, if any.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for BeyondCorp.ResolveAppConnections.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveAppConnectionsResponse {
    /// A list of BeyondCorp AppConnections with details in the project.
    #[prost(message, repeated, tag = "1")]
    pub app_connection_details: ::prost::alloc::vec::Vec<
        resolve_app_connections_response::AppConnectionDetails,
    >,
    /// A token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// A list of locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `ResolveAppConnectionsResponse`.
pub mod resolve_app_connections_response {
    /// Details of the AppConnection.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AppConnectionDetails {
        /// A BeyondCorp AppConnection in the project.
        #[prost(message, optional, tag = "1")]
        pub app_connection: ::core::option::Option<super::AppConnection>,
        /// If type=GCP_REGIONAL_MIG, contains most recent VM instances, like
        /// `<https://www.googleapis.com/compute/v1/projects/{project_id}/zones/{zone_id}/instances/{instance_id}`.>
        #[prost(string, repeated, tag = "2")]
        pub recent_mig_vms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
/// A BeyondCorp AppConnection resource represents a BeyondCorp protected
/// AppConnection to a remote application. It creates all the necessary GCP
/// components needed for creating a BeyondCorp protected AppConnection. Multiple
/// connectors can be authorised for a single AppConnection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppConnection {
    /// Required. Unique resource name of the AppConnection.
    /// The name is ignored when creating a AppConnection.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Timestamp when the resource was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Timestamp when the resource was last modified.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Resource labels to represent user provided metadata.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Optional. An arbitrary user-provided name for the AppConnection. Cannot
    /// exceed 64 characters.
    #[prost(string, tag = "5")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. A unique identifier for the instance generated by the
    /// system.
    #[prost(string, tag = "6")]
    pub uid: ::prost::alloc::string::String,
    /// Required. The type of network connectivity used by the AppConnection.
    #[prost(enumeration = "app_connection::Type", tag = "7")]
    pub r#type: i32,
    /// Required. Address of the remote application endpoint for the BeyondCorp
    /// AppConnection.
    #[prost(message, optional, tag = "8")]
    pub application_endpoint: ::core::option::Option<
        app_connection::ApplicationEndpoint,
    >,
    /// Optional. List of \[google.cloud.beyondcorp.v1main.Connector.name\] that are
    /// authorised to be associated with this AppConnection.
    #[prost(string, repeated, tag = "9")]
    pub connectors: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. The current state of the AppConnection.
    #[prost(enumeration = "app_connection::State", tag = "10")]
    pub state: i32,
    /// Optional. Gateway used by the AppConnection.
    #[prost(message, optional, tag = "11")]
    pub gateway: ::core::option::Option<app_connection::Gateway>,
}
/// Nested message and enum types in `AppConnection`.
pub mod app_connection {
    /// ApplicationEndpoint represents a remote application endpoint.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ApplicationEndpoint {
        /// Required. Hostname or IP address of the remote application endpoint.
        #[prost(string, tag = "1")]
        pub host: ::prost::alloc::string::String,
        /// Required. Port of the remote application endpoint.
        #[prost(int32, tag = "2")]
        pub port: i32,
    }
    /// Gateway represents a user facing component that serves as an entrance to
    /// enable connectivity.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Gateway {
        /// Required. The type of hosting used by the gateway.
        #[prost(enumeration = "gateway::Type", tag = "2")]
        pub r#type: i32,
        /// Output only. Server-defined URI for this resource.
        #[prost(string, tag = "3")]
        pub uri: ::prost::alloc::string::String,
        /// Output only. Ingress port reserved on the gateways for this
        /// AppConnection, if not specified or zero, the default port is 19443.
        #[prost(int32, tag = "4")]
        pub ingress_port: i32,
        /// Required. AppGateway name in following format:
        /// `projects/{project_id}/locations/{location_id}/appgateways/{gateway_id}`
        #[prost(string, tag = "5")]
        pub app_gateway: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `Gateway`.
    pub mod gateway {
        /// Enum listing possible gateway hosting options.
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
        pub enum Type {
            /// Default value. This value is unused.
            Unspecified = 0,
            /// Gateway hosted in a GCP regional managed instance group.
            GcpRegionalMig = 1,
        }
        impl Type {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Type::Unspecified => "TYPE_UNSPECIFIED",
                    Type::GcpRegionalMig => "GCP_REGIONAL_MIG",
                }
            }
        }
    }
    /// Enum containing list of all possible network connectivity options
    /// supported by BeyondCorp AppConnection.
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
    pub enum Type {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// TCP Proxy based BeyondCorp AppConnection. API will default to this if
        /// unset.
        TcpProxy = 1,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::TcpProxy => "TCP_PROXY",
            }
        }
    }
    /// Represents the different states of a AppConnection.
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
        /// Default value. This value is unused.
        Unspecified = 0,
        /// AppConnection is being created.
        Creating = 1,
        /// AppConnection has been created.
        Created = 2,
        /// AppConnection's configuration is being updated.
        Updating = 3,
        /// AppConnection is being deleted.
        Deleting = 4,
        /// AppConnection is down and may be restored in the future.
        /// This happens when CCFE sends ProjectState = OFF.
        Down = 5,
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
                State::Created => "CREATED",
                State::Updating => "UPDATING",
                State::Deleting => "DELETING",
                State::Down => "DOWN",
            }
        }
    }
}
/// Represents the metadata of the long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppConnectionOperationMetadata {
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
    /// of the operation. Operations that have successfully been cancelled
    /// have \[Operation.error][\] value with a
    /// \[google.rpc.Status.code][google.rpc.Status.code\] of 1, corresponding to
    /// `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod app_connections_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// API Overview:
    ///
    /// The `beyondcorp.googleapis.com` service implements the Google Cloud
    /// BeyondCorp API.
    ///
    /// Data Model:
    ///
    /// The AppConnectionsService exposes the following resources:
    ///
    /// * AppConnections, named as follows:
    ///   `projects/{project_id}/locations/{location_id}/appConnections/{app_connection_id}`.
    ///
    /// The AppConnectionsService service provides methods to manage
    /// (create/read/update/delete) BeyondCorp AppConnections.
    #[derive(Debug, Clone)]
    pub struct AppConnectionsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AppConnectionsServiceClient<tonic::transport::Channel> {
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
    impl<T> AppConnectionsServiceClient<T>
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
        ) -> AppConnectionsServiceClient<InterceptedService<T, F>>
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
            AppConnectionsServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists AppConnections in a given project and location.
        pub async fn list_app_connections(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAppConnectionsRequest>,
        ) -> Result<tonic::Response<super::ListAppConnectionsResponse>, tonic::Status> {
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
                "/google.cloud.beyondcorp.appconnections.v1.AppConnectionsService/ListAppConnections",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single AppConnection.
        pub async fn get_app_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAppConnectionRequest>,
        ) -> Result<tonic::Response<super::AppConnection>, tonic::Status> {
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
                "/google.cloud.beyondcorp.appconnections.v1.AppConnectionsService/GetAppConnection",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new AppConnection in a given project and location.
        pub async fn create_app_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAppConnectionRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.beyondcorp.appconnections.v1.AppConnectionsService/CreateAppConnection",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the parameters of a single AppConnection.
        pub async fn update_app_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAppConnectionRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.beyondcorp.appconnections.v1.AppConnectionsService/UpdateAppConnection",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single AppConnection.
        pub async fn delete_app_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAppConnectionRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.beyondcorp.appconnections.v1.AppConnectionsService/DeleteAppConnection",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Resolves AppConnections details for a given AppConnector.
        /// An internal method called by a connector to find AppConnections to connect
        /// to.
        pub async fn resolve_app_connections(
            &mut self,
            request: impl tonic::IntoRequest<super::ResolveAppConnectionsRequest>,
        ) -> Result<
            tonic::Response<super::ResolveAppConnectionsResponse>,
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
                "/google.cloud.beyondcorp.appconnections.v1.AppConnectionsService/ResolveAppConnections",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
