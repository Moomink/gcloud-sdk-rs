/// Cloud Logging structured payload for events generated from Data Pipelines API
/// requests.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestLogEntry {
    /// Type of the Data Pipelines API request.
    #[prost(enumeration = "request_log_entry::RequestType", tag = "1")]
    pub request_type: i32,
    /// The resulting status of the Data Pipelines API request.
    #[prost(message, optional, tag = "2")]
    pub status: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// Cause of the error status.
    #[prost(enumeration = "request_log_entry::ErrorCause", tag = "3")]
    pub error_cause: i32,
}
/// Nested message and enum types in `RequestLogEntry`.
pub mod request_log_entry {
    /// Type of a Data Pipelines API request.
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
    pub enum RequestType {
        /// Default value. This value is not used.
        Unspecified = 0,
        /// A Data Pipelines Create Pipeline request.
        CreatePipeline = 1,
        /// A Data Pipelines Update Pipeline request.
        UpdatePipeline = 2,
        /// A Data Pipelines Delete Pipeline request.
        DeletePipeline = 3,
        /// A Data Pipelines List Pipelines request.
        ListPipelines = 4,
        /// A Data Pipelines Get Pipeline request.
        GetPipeline = 5,
        /// A Data Pipelines Stop Pipeline request.
        StopPipeline = 6,
        /// A Data Pipelines Run Pipeline request.
        RunPipeline = 7,
        /// A Data Pipelines List Jobs request.
        ListJobs = 8,
    }
    impl RequestType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RequestType::Unspecified => "REQUEST_TYPE_UNSPECIFIED",
                RequestType::CreatePipeline => "CREATE_PIPELINE",
                RequestType::UpdatePipeline => "UPDATE_PIPELINE",
                RequestType::DeletePipeline => "DELETE_PIPELINE",
                RequestType::ListPipelines => "LIST_PIPELINES",
                RequestType::GetPipeline => "GET_PIPELINE",
                RequestType::StopPipeline => "STOP_PIPELINE",
                RequestType::RunPipeline => "RUN_PIPELINE",
                RequestType::ListJobs => "LIST_JOBS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "REQUEST_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATE_PIPELINE" => Some(Self::CreatePipeline),
                "UPDATE_PIPELINE" => Some(Self::UpdatePipeline),
                "DELETE_PIPELINE" => Some(Self::DeletePipeline),
                "LIST_PIPELINES" => Some(Self::ListPipelines),
                "GET_PIPELINE" => Some(Self::GetPipeline),
                "STOP_PIPELINE" => Some(Self::StopPipeline),
                "RUN_PIPELINE" => Some(Self::RunPipeline),
                "LIST_JOBS" => Some(Self::ListJobs),
                _ => None,
            }
        }
    }
    /// Cause code for a Data Pipelines API request error.
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
    pub enum ErrorCause {
        /// Default value. This value is not used.
        Unspecified = 0,
        /// The request is invalid.
        InvalidRequest = 1,
        /// Failed to fetch project number for the provided project id.
        ProjectNumberNotFound = 2,
        /// The given pipeline already exists.
        PipelineIdAlreadyExists = 3,
        /// Failed to allocate a token for the per project pipeline count quota.
        PipelineQuotaAllocationFailed = 4,
        /// The given pipeline is not found.
        PipelineNotFound = 5,
        /// The pipeline's workload is invalid.
        InvalidPipelineWorkload = 6,
        /// The user cannot act as the Dataflow worker service account.
        DataflowWorkerServiceAccountPermissionDenied = 7,
        /// The user cannot act as the Cloud Scheduler service account.
        CloudSchedulerServiceAccountPermissionDenied = 8,
        /// Issues related to the per service per project service account.
        InternalDataPipelinesServiceAccountIssue = 9,
        /// Invalid argument in Cloud Scheduler service call.
        CloudSchedulerInvalidArgument = 10,
        /// Exceeds Cloud Scheduler service quota limit.
        CloudSchedulerResourceExhausted = 11,
        /// Cloud Scheduler job not found.
        CloudSchedulerJobNotFound = 12,
        /// Other Cloud Scheduler related issues.
        OtherCloudSchedulerIssue = 13,
        /// Dataflow job with the same name already exists.
        DataflowJobAlreadyExists = 14,
        /// Invalid argument in Dataflow service call.
        DataflowInvalidArgument = 15,
        /// Exceeds Dataflow service quota limit.
        DataflowResourceExhausted = 16,
        /// Dataflow job not found.
        DataflowJobNotFound = 17,
        /// Other Dataflow related issues.
        OtherDataflowIssue = 18,
        /// Database related issues.
        DatabaseError = 19,
        /// Request with the wrong pipeline type. For example, RunPipeline cannot be
        /// used with a streaming pipeline.
        WrongPipelineType = 20,
        /// Issues related to other Google internal services/systems.
        InternalError = 21,
        /// Cannot find the given pipeline or project.
        PipelineOrProjectNotFound = 22,
    }
    impl ErrorCause {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ErrorCause::Unspecified => "ERROR_CAUSE_UNSPECIFIED",
                ErrorCause::InvalidRequest => "INVALID_REQUEST",
                ErrorCause::ProjectNumberNotFound => "PROJECT_NUMBER_NOT_FOUND",
                ErrorCause::PipelineIdAlreadyExists => "PIPELINE_ID_ALREADY_EXISTS",
                ErrorCause::PipelineQuotaAllocationFailed => {
                    "PIPELINE_QUOTA_ALLOCATION_FAILED"
                }
                ErrorCause::PipelineNotFound => "PIPELINE_NOT_FOUND",
                ErrorCause::InvalidPipelineWorkload => "INVALID_PIPELINE_WORKLOAD",
                ErrorCause::DataflowWorkerServiceAccountPermissionDenied => {
                    "DATAFLOW_WORKER_SERVICE_ACCOUNT_PERMISSION_DENIED"
                }
                ErrorCause::CloudSchedulerServiceAccountPermissionDenied => {
                    "CLOUD_SCHEDULER_SERVICE_ACCOUNT_PERMISSION_DENIED"
                }
                ErrorCause::InternalDataPipelinesServiceAccountIssue => {
                    "INTERNAL_DATA_PIPELINES_SERVICE_ACCOUNT_ISSUE"
                }
                ErrorCause::CloudSchedulerInvalidArgument => {
                    "CLOUD_SCHEDULER_INVALID_ARGUMENT"
                }
                ErrorCause::CloudSchedulerResourceExhausted => {
                    "CLOUD_SCHEDULER_RESOURCE_EXHAUSTED"
                }
                ErrorCause::CloudSchedulerJobNotFound => "CLOUD_SCHEDULER_JOB_NOT_FOUND",
                ErrorCause::OtherCloudSchedulerIssue => "OTHER_CLOUD_SCHEDULER_ISSUE",
                ErrorCause::DataflowJobAlreadyExists => "DATAFLOW_JOB_ALREADY_EXISTS",
                ErrorCause::DataflowInvalidArgument => "DATAFLOW_INVALID_ARGUMENT",
                ErrorCause::DataflowResourceExhausted => "DATAFLOW_RESOURCE_EXHAUSTED",
                ErrorCause::DataflowJobNotFound => "DATAFLOW_JOB_NOT_FOUND",
                ErrorCause::OtherDataflowIssue => "OTHER_DATAFLOW_ISSUE",
                ErrorCause::DatabaseError => "DATABASE_ERROR",
                ErrorCause::WrongPipelineType => "WRONG_PIPELINE_TYPE",
                ErrorCause::InternalError => "INTERNAL_ERROR",
                ErrorCause::PipelineOrProjectNotFound => "PIPELINE_OR_PROJECT_NOT_FOUND",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ERROR_CAUSE_UNSPECIFIED" => Some(Self::Unspecified),
                "INVALID_REQUEST" => Some(Self::InvalidRequest),
                "PROJECT_NUMBER_NOT_FOUND" => Some(Self::ProjectNumberNotFound),
                "PIPELINE_ID_ALREADY_EXISTS" => Some(Self::PipelineIdAlreadyExists),
                "PIPELINE_QUOTA_ALLOCATION_FAILED" => {
                    Some(Self::PipelineQuotaAllocationFailed)
                }
                "PIPELINE_NOT_FOUND" => Some(Self::PipelineNotFound),
                "INVALID_PIPELINE_WORKLOAD" => Some(Self::InvalidPipelineWorkload),
                "DATAFLOW_WORKER_SERVICE_ACCOUNT_PERMISSION_DENIED" => {
                    Some(Self::DataflowWorkerServiceAccountPermissionDenied)
                }
                "CLOUD_SCHEDULER_SERVICE_ACCOUNT_PERMISSION_DENIED" => {
                    Some(Self::CloudSchedulerServiceAccountPermissionDenied)
                }
                "INTERNAL_DATA_PIPELINES_SERVICE_ACCOUNT_ISSUE" => {
                    Some(Self::InternalDataPipelinesServiceAccountIssue)
                }
                "CLOUD_SCHEDULER_INVALID_ARGUMENT" => {
                    Some(Self::CloudSchedulerInvalidArgument)
                }
                "CLOUD_SCHEDULER_RESOURCE_EXHAUSTED" => {
                    Some(Self::CloudSchedulerResourceExhausted)
                }
                "CLOUD_SCHEDULER_JOB_NOT_FOUND" => Some(Self::CloudSchedulerJobNotFound),
                "OTHER_CLOUD_SCHEDULER_ISSUE" => Some(Self::OtherCloudSchedulerIssue),
                "DATAFLOW_JOB_ALREADY_EXISTS" => Some(Self::DataflowJobAlreadyExists),
                "DATAFLOW_INVALID_ARGUMENT" => Some(Self::DataflowInvalidArgument),
                "DATAFLOW_RESOURCE_EXHAUSTED" => Some(Self::DataflowResourceExhausted),
                "DATAFLOW_JOB_NOT_FOUND" => Some(Self::DataflowJobNotFound),
                "OTHER_DATAFLOW_ISSUE" => Some(Self::OtherDataflowIssue),
                "DATABASE_ERROR" => Some(Self::DatabaseError),
                "WRONG_PIPELINE_TYPE" => Some(Self::WrongPipelineType),
                "INTERNAL_ERROR" => Some(Self::InternalError),
                "PIPELINE_OR_PROJECT_NOT_FOUND" => Some(Self::PipelineOrProjectNotFound),
                _ => None,
            }
        }
    }
}
