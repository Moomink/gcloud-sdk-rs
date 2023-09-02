/// A running instance of a
/// \[Workflow\](/workflows/docs/reference/rest/v1/projects.locations.workflows).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Execution {
    /// Output only. The resource name of the execution.
    /// Format:
    /// projects/{project}/locations/{location}/workflows/{workflow}/executions/{execution}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Marks the beginning of execution.
    #[prost(message, optional, tag = "2")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Marks the end of execution, successful or not.
    #[prost(message, optional, tag = "3")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Measures the duration of the execution.
    #[prost(message, optional, tag = "12")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    /// Output only. Current state of the execution.
    #[prost(enumeration = "execution::State", tag = "4")]
    pub state: i32,
    /// Input parameters of the execution represented as a JSON string.
    /// The size limit is 32KB.
    ///
    /// *Note*: If you are using the REST API directly to run your workflow, you
    /// must escape any JSON string value of `argument`. Example:
    /// `'{"argument":"{\"firstName\":\"FIRST\",\"lastName\":\"LAST\"}"}'`
    #[prost(string, tag = "5")]
    pub argument: ::prost::alloc::string::String,
    /// Output only. Output of the execution represented as a JSON string. The
    /// value can only be present if the execution's state is `SUCCEEDED`.
    #[prost(string, tag = "6")]
    pub result: ::prost::alloc::string::String,
    /// Output only. The error which caused the execution to finish prematurely.
    /// The value is only present if the execution's state is `FAILED`
    /// or `CANCELLED`.
    #[prost(message, optional, tag = "7")]
    pub error: ::core::option::Option<execution::Error>,
    /// Output only. Revision of the workflow this execution is using.
    #[prost(string, tag = "8")]
    pub workflow_revision_id: ::prost::alloc::string::String,
    /// The call logging level associated to this execution.
    #[prost(enumeration = "execution::CallLogLevel", tag = "9")]
    pub call_log_level: i32,
    /// Output only. Status tracks the current steps and progress data of this
    /// execution.
    #[prost(message, optional, tag = "10")]
    pub status: ::core::option::Option<execution::Status>,
    /// Labels associated with this execution.
    /// Labels can contain at most 64 entries. Keys and values can be no longer
    /// than 63 characters and can only contain lowercase letters, numeric
    /// characters, underscores, and dashes. Label keys must start with a letter.
    /// International characters are allowed.
    /// By default, labels are inherited from the workflow but are overridden by
    /// any labels associated with the execution.
    #[prost(map = "string, string", tag = "11")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. Error regarding the state of the Execution resource. For
    /// example, this field will have error details if the execution data is
    /// unavailable due to revoked KMS key permissions.
    #[prost(message, optional, tag = "13")]
    pub state_error: ::core::option::Option<execution::StateError>,
}
/// Nested message and enum types in `Execution`.
pub mod execution {
    /// A single stack element (frame) where an error occurred.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StackTraceElement {
        /// The step the error occurred at.
        #[prost(string, tag = "1")]
        pub step: ::prost::alloc::string::String,
        /// The routine where the error occurred.
        #[prost(string, tag = "2")]
        pub routine: ::prost::alloc::string::String,
        /// The source position information of the stack trace element.
        #[prost(message, optional, tag = "3")]
        pub position: ::core::option::Option<stack_trace_element::Position>,
    }
    /// Nested message and enum types in `StackTraceElement`.
    pub mod stack_trace_element {
        /// Position contains source position information about the stack trace
        /// element such as line number, column number and length of the code block
        /// in bytes.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Position {
            /// The source code line number the current instruction was generated from.
            #[prost(int64, tag = "1")]
            pub line: i64,
            /// The source code column position (of the line) the current instruction
            /// was generated from.
            #[prost(int64, tag = "2")]
            pub column: i64,
            /// The number of bytes of source code making up this stack trace element.
            #[prost(int64, tag = "3")]
            pub length: i64,
        }
    }
    /// A collection of stack elements (frames) where an error occurred.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StackTrace {
        /// An array of stack elements.
        #[prost(message, repeated, tag = "1")]
        pub elements: ::prost::alloc::vec::Vec<StackTraceElement>,
    }
    /// Error describes why the execution was abnormally terminated.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Error {
        /// Error message and data returned represented as a JSON string.
        #[prost(string, tag = "1")]
        pub payload: ::prost::alloc::string::String,
        /// Human-readable stack trace string.
        #[prost(string, tag = "2")]
        pub context: ::prost::alloc::string::String,
        /// Stack trace with detailed information of where error was generated.
        #[prost(message, optional, tag = "3")]
        pub stack_trace: ::core::option::Option<StackTrace>,
    }
    /// Represents the current status of this execution.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Status {
        /// A list of currently executing or last executed step names for the
        /// workflow execution currently running. If the workflow has succeeded or
        /// failed, this is the last attempted or executed step. Presently, if the
        /// current step is inside a subworkflow, the list only includes that step.
        /// In the future, the list will contain items for each step in the call
        /// stack, starting with the outermost step in the `main` subworkflow, and
        /// ending with the most deeply nested step.
        #[prost(message, repeated, tag = "1")]
        pub current_steps: ::prost::alloc::vec::Vec<status::Step>,
    }
    /// Nested message and enum types in `Status`.
    pub mod status {
        /// Represents a step of the workflow this execution is running.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Step {
            /// Name of a routine within the workflow.
            #[prost(string, tag = "1")]
            pub routine: ::prost::alloc::string::String,
            /// Name of a step within the routine.
            #[prost(string, tag = "2")]
            pub step: ::prost::alloc::string::String,
        }
    }
    /// Describes an error related to the current state of the Execution resource.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StateError {
        /// Provides specifics about the error.
        #[prost(string, tag = "1")]
        pub details: ::prost::alloc::string::String,
        /// The type of this state error.
        #[prost(enumeration = "state_error::Type", tag = "2")]
        pub r#type: i32,
    }
    /// Nested message and enum types in `StateError`.
    pub mod state_error {
        /// Describes the possible types of a state error.
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
            /// No type specified.
            Unspecified = 0,
            /// Caused by an issue with KMS.
            KmsError = 1,
        }
        impl Type {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Type::Unspecified => "TYPE_UNSPECIFIED",
                    Type::KmsError => "KMS_ERROR",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                    "KMS_ERROR" => Some(Self::KmsError),
                    _ => None,
                }
            }
        }
    }
    /// Describes the current state of the execution. More states might be added
    /// in the future.
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
        /// Invalid state.
        Unspecified = 0,
        /// The execution is in progress.
        Active = 1,
        /// The execution finished successfully.
        Succeeded = 2,
        /// The execution failed with an error.
        Failed = 3,
        /// The execution was stopped intentionally.
        Cancelled = 4,
        /// Execution data is unavailable. See the `state_error` field.
        Unavailable = 5,
        /// Request has been placed in the backlog for processing at a later time.
        Queued = 6,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Active => "ACTIVE",
                State::Succeeded => "SUCCEEDED",
                State::Failed => "FAILED",
                State::Cancelled => "CANCELLED",
                State::Unavailable => "UNAVAILABLE",
                State::Queued => "QUEUED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ACTIVE" => Some(Self::Active),
                "SUCCEEDED" => Some(Self::Succeeded),
                "FAILED" => Some(Self::Failed),
                "CANCELLED" => Some(Self::Cancelled),
                "UNAVAILABLE" => Some(Self::Unavailable),
                "QUEUED" => Some(Self::Queued),
                _ => None,
            }
        }
    }
    /// Describes the level of platform logging to apply to calls and call
    /// responses during workflow executions.
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
    pub enum CallLogLevel {
        /// No call logging level specified.
        Unspecified = 0,
        /// Log all call steps within workflows, all call returns, and all exceptions
        /// raised.
        LogAllCalls = 1,
        /// Log only exceptions that are raised from call steps within workflows.
        LogErrorsOnly = 2,
        /// Explicitly log nothing.
        LogNone = 3,
    }
    impl CallLogLevel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CallLogLevel::Unspecified => "CALL_LOG_LEVEL_UNSPECIFIED",
                CallLogLevel::LogAllCalls => "LOG_ALL_CALLS",
                CallLogLevel::LogErrorsOnly => "LOG_ERRORS_ONLY",
                CallLogLevel::LogNone => "LOG_NONE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CALL_LOG_LEVEL_UNSPECIFIED" => Some(Self::Unspecified),
                "LOG_ALL_CALLS" => Some(Self::LogAllCalls),
                "LOG_ERRORS_ONLY" => Some(Self::LogErrorsOnly),
                "LOG_NONE" => Some(Self::LogNone),
                _ => None,
            }
        }
    }
}
/// Request for the
/// \[ListExecutions][\]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListExecutionsRequest {
    /// Required. Name of the workflow for which the executions should be listed.
    /// Format: projects/{project}/locations/{location}/workflows/{workflow}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of executions to return per call.
    /// Max supported value depends on the selected Execution view: it's 1000 for
    /// BASIC and 100 for FULL. The default value used if the field is not
    /// specified is 100, regardless of the selected view. Values greater than
    /// the max value will be coerced down to it.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListExecutions` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListExecutions` must
    /// match the call that provided the page token.
    ///
    /// Note that pagination is applied to dynamic data. The list of executions
    /// returned can change between page requests.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. A view defining which fields should be filled in the returned
    /// executions. The API will default to the BASIC view.
    #[prost(enumeration = "ExecutionView", tag = "4")]
    pub view: i32,
    /// Optional. Filters applied to the \[Executions.ListExecutions\] results.
    /// The following fields are supported for filtering:
    /// executionID, state, startTime, endTime, duration, workflowRevisionID,
    /// stepName, and label.
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. The ordering applied to the \[Executions.ListExecutions\] results.
    /// By default the ordering is based on descending start time.
    /// The following fields are supported for order by:
    /// executionID, startTime, endTime, duration, state, and workflowRevisionID.
    #[prost(string, tag = "6")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response for the
/// \[ListExecutions][google.cloud.workflows.executions.v1.Executions.ListExecutions\]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListExecutionsResponse {
    /// The executions which match the request.
    #[prost(message, repeated, tag = "1")]
    pub executions: ::prost::alloc::vec::Vec<Execution>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request for the
/// \[CreateExecution][google.cloud.workflows.executions.v1.Executions.CreateExecution\]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateExecutionRequest {
    /// Required. Name of the workflow for which an execution should be created.
    /// Format: projects/{project}/locations/{location}/workflows/{workflow}
    /// The latest revision of the workflow will be used.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Execution to be created.
    #[prost(message, optional, tag = "2")]
    pub execution: ::core::option::Option<Execution>,
}
/// Request for the
/// \[GetExecution][google.cloud.workflows.executions.v1.Executions.GetExecution\]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetExecutionRequest {
    /// Required. Name of the execution to be retrieved.
    /// Format:
    /// projects/{project}/locations/{location}/workflows/{workflow}/executions/{execution}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A view defining which fields should be filled in the returned
    /// execution. The API will default to the FULL view.
    #[prost(enumeration = "ExecutionView", tag = "2")]
    pub view: i32,
}
/// Request for the
/// \[CancelExecution][google.cloud.workflows.executions.v1.Executions.CancelExecution\]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelExecutionRequest {
    /// Required. Name of the execution to be cancelled.
    /// Format:
    /// projects/{project}/locations/{location}/workflows/{workflow}/executions/{execution}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Defines possible views for execution resource.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExecutionView {
    /// The default / unset value.
    Unspecified = 0,
    /// Includes only basic metadata about the execution.
    /// The following fields are returned: name, start_time, end_time, duration,
    /// state, and workflow_revision_id.
    Basic = 1,
    /// Includes all data.
    Full = 2,
}
impl ExecutionView {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ExecutionView::Unspecified => "EXECUTION_VIEW_UNSPECIFIED",
            ExecutionView::Basic => "BASIC",
            ExecutionView::Full => "FULL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EXECUTION_VIEW_UNSPECIFIED" => Some(Self::Unspecified),
            "BASIC" => Some(Self::Basic),
            "FULL" => Some(Self::Full),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod executions_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Executions is used to start and manage running instances of
    /// [Workflows][google.cloud.workflows.v1.Workflow] called executions.
    #[derive(Debug, Clone)]
    pub struct ExecutionsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ExecutionsClient<tonic::transport::Channel> {
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
    impl<T> ExecutionsClient<T>
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
        ) -> ExecutionsClient<InterceptedService<T, F>>
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
            ExecutionsClient::new(InterceptedService::new(inner, interceptor))
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
        /// Returns a list of executions which belong to the workflow with
        /// the given name. The method returns executions of all workflow
        /// revisions. Returned executions are ordered by their start time (newest
        /// first).
        pub async fn list_executions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListExecutionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListExecutionsResponse>,
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
                "/google.cloud.workflows.executions.v1.Executions/ListExecutions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workflows.executions.v1.Executions",
                        "ListExecutions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new execution using the latest revision of the given workflow.
        pub async fn create_execution(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateExecutionRequest>,
        ) -> std::result::Result<tonic::Response<super::Execution>, tonic::Status> {
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
                "/google.cloud.workflows.executions.v1.Executions/CreateExecution",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workflows.executions.v1.Executions",
                        "CreateExecution",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns an execution of the given name.
        pub async fn get_execution(
            &mut self,
            request: impl tonic::IntoRequest<super::GetExecutionRequest>,
        ) -> std::result::Result<tonic::Response<super::Execution>, tonic::Status> {
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
                "/google.cloud.workflows.executions.v1.Executions/GetExecution",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workflows.executions.v1.Executions",
                        "GetExecution",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Cancels an execution of the given name.
        pub async fn cancel_execution(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelExecutionRequest>,
        ) -> std::result::Result<tonic::Response<super::Execution>, tonic::Status> {
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
                "/google.cloud.workflows.executions.v1.Executions/CancelExecution",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workflows.executions.v1.Executions",
                        "CancelExecution",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
