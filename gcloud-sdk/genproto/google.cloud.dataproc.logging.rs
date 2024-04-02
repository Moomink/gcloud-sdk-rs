/// The short version of cluster configuration for Cloud Logging.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterSize {
    /// The number of primary workers in the cluster.
    #[prost(int32, tag = "1")]
    pub primary_worker_count: i32,
    /// The number of secondary workers in the cluster.
    #[prost(int32, tag = "2")]
    pub secondary_worker_count: i32,
}
/// The main proto that will be converted to JSON format and then written to
/// Logging.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoscalerLog {
    /// The current Autoscaler status.
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<AutoscalerStatus>,
    /// Optional. The autoscaling recommendation including its inputs, outputs,
    /// scaling decision, and detailed explanation.
    #[prost(message, optional, tag = "2")]
    pub recommendation: ::core::option::Option<AutoscalerRecommendation>,
}
/// The Autoscaler's status, including its state and other details.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoscalerStatus {
    /// The high-level Autoscaler state.
    #[prost(enumeration = "AutoscalerState", tag = "1")]
    pub state: i32,
    /// The detailed description of Autoscaler status.
    #[prost(string, tag = "2")]
    pub details: ::prost::alloc::string::String,
    /// The cluster update operation ID.
    #[prost(string, tag = "3")]
    pub update_cluster_operation_id: ::prost::alloc::string::String,
    /// Error message from an Autoscaler exception, if any.
    #[prost(string, tag = "4")]
    pub error: ::prost::alloc::string::String,
}
/// The inputs, outputs, and detailed explanation of the Autoscaling
/// recommendation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoscalerRecommendation {
    /// The autoscaling algorithm inputs.
    #[prost(message, optional, tag = "1")]
    pub inputs: ::core::option::Option<autoscaler_recommendation::Inputs>,
    /// The algorithm outputs for the recommended cluster size.
    #[prost(message, optional, tag = "2")]
    pub outputs: ::core::option::Option<autoscaler_recommendation::Outputs>,
}
/// Nested message and enum types in `AutoscalerRecommendation`.
pub mod autoscaler_recommendation {
    /// The input values for the Autoscaling recommendation algorithm.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Inputs {
        /// The metrics collected by the Dataproc agent running on the cluster.
        /// For example, {"avg-yarn-pending-memory": "1040 MB"}
        #[prost(map = "string, string", tag = "1")]
        pub cluster_metrics: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
        /// The cluster configuration before updating the cluster.
        #[prost(message, optional, tag = "2")]
        pub current_cluster_size: ::core::option::Option<super::ClusterSize>,
        /// The minimum worker counts for each instance group.
        #[prost(message, optional, tag = "3")]
        pub min_worker_counts: ::core::option::Option<super::ClusterSize>,
        /// The maximum worker counts for each instance group.
        #[prost(message, optional, tag = "4")]
        pub max_worker_counts: ::core::option::Option<super::ClusterSize>,
    }
    /// Autoscaler recommendations.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Outputs {
        /// The high-level autoscaling decision, such as SCALE_UP, SCALE_DOWN,
        /// NO_OP.
        #[prost(enumeration = "super::ScalingDecisionType", tag = "1")]
        pub decision: i32,
        /// The recommended cluster size.
        #[prost(message, optional, tag = "2")]
        pub recommended_cluster_size: ::core::option::Option<super::ClusterSize>,
        /// The graceful decommission timeout for downscaling operations.
        #[prost(message, optional, tag = "3")]
        pub graceful_decommission_timeout: ::core::option::Option<
            ::prost_types::Duration,
        >,
        /// Reasons why the Autoscaler didn't add or remove more workers.
        #[prost(enumeration = "super::ConstrainingFactor", repeated, tag = "4")]
        pub constraints_reached: ::prost::alloc::vec::Vec<i32>,
        /// Less significant recommendations that are not included in the
        /// `AutoscalerStatus.details` message.
        #[prost(string, repeated, tag = "5")]
        pub additional_recommendation_details: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
        /// A unique id for this recommendation that should be included when opening
        /// a support ticket.
        #[prost(string, tag = "6")]
        pub recommendation_id: ::prost::alloc::string::String,
        /// The metric source deciding the autoscaling recommendation.
        #[prost(enumeration = "super::MetricType", tag = "7")]
        pub decision_metric: i32,
    }
}
/// The Autoscaler state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AutoscalerState {
    Unspecified = 0,
    /// The Autoscaler is sleeping and waiting for the next update.
    Cooldown = 1,
    /// The Autoscaler is in the process of calculating its recommendation on
    /// whether to scale the cluster, and if so, how to autoscale.
    Recommending = 6,
    /// The Autoscaler is scaling the cluster.
    Scaling = 2,
    /// The Autoscaler has stopped.
    Stopped = 3,
    /// The Autoscaler has failed.
    Failed = 4,
    /// The Autoscaler is initializing.
    Initializing = 5,
}
impl AutoscalerState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AutoscalerState::Unspecified => "AUTOSCALER_STATE_UNSPECIFIED",
            AutoscalerState::Cooldown => "COOLDOWN",
            AutoscalerState::Recommending => "RECOMMENDING",
            AutoscalerState::Scaling => "SCALING",
            AutoscalerState::Stopped => "STOPPED",
            AutoscalerState::Failed => "FAILED",
            AutoscalerState::Initializing => "INITIALIZING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "AUTOSCALER_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "COOLDOWN" => Some(Self::Cooldown),
            "RECOMMENDING" => Some(Self::Recommending),
            "SCALING" => Some(Self::Scaling),
            "STOPPED" => Some(Self::Stopped),
            "FAILED" => Some(Self::Failed),
            "INITIALIZING" => Some(Self::Initializing),
            _ => None,
        }
    }
}
/// The Autoscaling decision type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ScalingDecisionType {
    Unspecified = 0,
    /// Increase the number of primary and/or secondary workers.
    ScaleUp = 1,
    /// Decrease the number of primary and/or secondary workers.
    ScaleDown = 2,
    /// Not changing the number of primary or secondary workers.
    NoScale = 3,
    /// Scale the primary and secondary worker groups in different directions.
    Mixed = 4,
    /// Cancel the ongoing scale down operation.
    Cancel = 5,
    /// Do not cancel the ongoing scale down operation.
    DoNotCancel = 6,
}
impl ScalingDecisionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ScalingDecisionType::Unspecified => "SCALING_DECISION_TYPE_UNSPECIFIED",
            ScalingDecisionType::ScaleUp => "SCALE_UP",
            ScalingDecisionType::ScaleDown => "SCALE_DOWN",
            ScalingDecisionType::NoScale => "NO_SCALE",
            ScalingDecisionType::Mixed => "MIXED",
            ScalingDecisionType::Cancel => "CANCEL",
            ScalingDecisionType::DoNotCancel => "DO_NOT_CANCEL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SCALING_DECISION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "SCALE_UP" => Some(Self::ScaleUp),
            "SCALE_DOWN" => Some(Self::ScaleDown),
            "NO_SCALE" => Some(Self::NoScale),
            "MIXED" => Some(Self::Mixed),
            "CANCEL" => Some(Self::Cancel),
            "DO_NOT_CANCEL" => Some(Self::DoNotCancel),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ConstrainingFactor {
    Unspecified = 0,
    /// The project does not have sufficient regional, global, and or preemptible
    /// quota to allocate a new VM.
    ScalingCappedDueToLackOfQuota = 1,
    /// All worker groups have reached maximum size. This message will not be
    /// issued if one group reached maximum size, but workers were able to be
    /// allocated to another group.
    ReachedMaximumClusterSize = 2,
    /// All worker groups have reached minimum size. This message will not be
    /// issued if workers were able to be removed from another group that had not
    /// reached minimum size.
    ReachedMinimumClusterSize = 3,
    /// The secondary worker group cannot be scaled down by more than 1k nodes in a
    /// single update request.
    SecondaryScaledownSingleRequestLimitReached = 4,
}
impl ConstrainingFactor {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ConstrainingFactor::Unspecified => "CONSTRAINING_FACTOR_UNSPECIFIED",
            ConstrainingFactor::ScalingCappedDueToLackOfQuota => {
                "SCALING_CAPPED_DUE_TO_LACK_OF_QUOTA"
            }
            ConstrainingFactor::ReachedMaximumClusterSize => {
                "REACHED_MAXIMUM_CLUSTER_SIZE"
            }
            ConstrainingFactor::ReachedMinimumClusterSize => {
                "REACHED_MINIMUM_CLUSTER_SIZE"
            }
            ConstrainingFactor::SecondaryScaledownSingleRequestLimitReached => {
                "SECONDARY_SCALEDOWN_SINGLE_REQUEST_LIMIT_REACHED"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CONSTRAINING_FACTOR_UNSPECIFIED" => Some(Self::Unspecified),
            "SCALING_CAPPED_DUE_TO_LACK_OF_QUOTA" => {
                Some(Self::ScalingCappedDueToLackOfQuota)
            }
            "REACHED_MAXIMUM_CLUSTER_SIZE" => Some(Self::ReachedMaximumClusterSize),
            "REACHED_MINIMUM_CLUSTER_SIZE" => Some(Self::ReachedMinimumClusterSize),
            "SECONDARY_SCALEDOWN_SINGLE_REQUEST_LIMIT_REACHED" => {
                Some(Self::SecondaryScaledownSingleRequestLimitReached)
            }
            _ => None,
        }
    }
}
/// The kind of metric input to the Autoscaling algorithm.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MetricType {
    /// Default.
    Unspecified = 0,
    /// The yarn memory metric.
    YarnMemory = 1,
    /// The yarn cores or vCPUs metric.
    YarnCores = 2,
    /// The number of executors in Spark serverless.
    SparkExecutors = 3,
}
impl MetricType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MetricType::Unspecified => "METRIC_TYPE_UNSPECIFIED",
            MetricType::YarnMemory => "YARN_MEMORY",
            MetricType::YarnCores => "YARN_CORES",
            MetricType::SparkExecutors => "SPARK_EXECUTORS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "METRIC_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "YARN_MEMORY" => Some(Self::YarnMemory),
            "YARN_CORES" => Some(Self::YarnCores),
            "SPARK_EXECUTORS" => Some(Self::SparkExecutors),
            _ => None,
        }
    }
}
/// Reconciliation log for session ttl event.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReconciliationLog {
    /// The reconciliation algorithm inputs.
    #[prost(message, optional, tag = "1")]
    pub inputs: ::core::option::Option<reconciliation_log::Inputs>,
    /// The algorithm outputs for the recommended reconciliation operation.
    #[prost(message, optional, tag = "2")]
    pub outputs: ::core::option::Option<reconciliation_log::Outputs>,
}
/// Nested message and enum types in `ReconciliationLog`.
pub mod reconciliation_log {
    /// The input values for the Reconciler recommendation algorithm.
    /// We could add more details in future if required.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Inputs {
        /// Idle duration
        #[prost(message, optional, tag = "1")]
        pub idle_duration: ::core::option::Option<::prost_types::Duration>,
        /// Configured idle TTL
        #[prost(message, optional, tag = "2")]
        pub idle_ttl: ::core::option::Option<::prost_types::Duration>,
        /// Total session lifetime
        #[prost(message, optional, tag = "3")]
        pub session_lifetime: ::core::option::Option<::prost_types::Duration>,
        /// Configured ttl
        #[prost(message, optional, tag = "4")]
        pub ttl: ::core::option::Option<::prost_types::Duration>,
    }
    /// Reconciler recommendations.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Outputs {
        /// The high-level reconciliation decision.
        #[prost(enumeration = "super::ReconciliationDecisionType", tag = "1")]
        pub decision: i32,
        /// Human readable context messages which explain the reconciler decision.
        #[prost(string, tag = "2")]
        pub decision_details: ::prost::alloc::string::String,
    }
}
/// Reconciliation log for cluster heal event.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReconciliationClusterHealLog {
    /// The algorithm outputs for the recommended reconciliation operation.
    #[prost(message, optional, tag = "1")]
    pub outputs: ::core::option::Option<reconciliation_cluster_heal_log::Outputs>,
}
/// Nested message and enum types in `ReconciliationClusterHealLog`.
pub mod reconciliation_cluster_heal_log {
    /// Autohealer decision.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Outputs {
        /// The repair operation id triggered by Autohealer if any.
        #[prost(string, tag = "1")]
        pub repair_operation_id: ::prost::alloc::string::String,
        /// Human readable context messages which explain the autohealer decision.
        #[prost(string, tag = "2")]
        pub decision_details: ::prost::alloc::string::String,
    }
}
/// Decision type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ReconciliationDecisionType {
    /// Unspecified type
    Unspecified = 0,
    /// Terminate session
    ReconciliationTerminateSession = 1,
}
impl ReconciliationDecisionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ReconciliationDecisionType::Unspecified => {
                "RECONCILIATION_DECISION_TYPE_UNSPECIFIED"
            }
            ReconciliationDecisionType::ReconciliationTerminateSession => {
                "RECONCILIATION_TERMINATE_SESSION"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RECONCILIATION_DECISION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "RECONCILIATION_TERMINATE_SESSION" => {
                Some(Self::ReconciliationTerminateSession)
            }
            _ => None,
        }
    }
}
