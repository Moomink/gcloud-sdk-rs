/// Structured payload for logs generated from Dataform workflow invocation
/// completions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkflowInvocationCompletionLogEntry {
    /// Required. Identifier of the workflow invocation.
    #[prost(string, tag = "1")]
    pub workflow_invocation_id: ::prost::alloc::string::String,
    /// Optional. Identifier of the workflow config.
    #[prost(string, tag = "2")]
    pub workflow_config_id: ::prost::alloc::string::String,
    /// Optional. Identifier of the release config.
    #[prost(string, tag = "3")]
    pub release_config_id: ::prost::alloc::string::String,
    /// Required. The workflow invocation's final termination state.
    #[prost(
        enumeration = "workflow_invocation_completion_log_entry::TerminalState",
        tag = "4"
    )]
    pub terminal_state: i32,
}
/// Nested message and enum types in `WorkflowInvocationCompletionLogEntry`.
pub mod workflow_invocation_completion_log_entry {
    /// Represents the final termination state of a workflow invocation.
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
    pub enum TerminalState {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// The workflow invocation succeeded.
        Succeeded = 1,
        /// The workflow invocation was cancelled.
        Cancelled = 2,
        /// The workflow invocation failed.
        Failed = 3,
    }
    impl TerminalState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TerminalState::Unspecified => "TERMINAL_STATE_UNSPECIFIED",
                TerminalState::Succeeded => "SUCCEEDED",
                TerminalState::Cancelled => "CANCELLED",
                TerminalState::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TERMINAL_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "SUCCEEDED" => Some(Self::Succeeded),
                "CANCELLED" => Some(Self::Cancelled),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
}
