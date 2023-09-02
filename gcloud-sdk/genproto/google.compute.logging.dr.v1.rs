/// An event signifying a Compute Engine resource is impacted by the disaster
/// recovery.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisasterRecoveryEvent {
    /// The severity level.
    #[prost(enumeration = "disaster_recovery_event::Severity", optional, tag = "1")]
    pub severity: ::core::option::Option<i32>,
    /// Details about the impact on the Compute Engine resource, e.g. "the resource
    /// is deleted during the disaster recovery".
    #[prost(string, optional, tag = "2")]
    pub details: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `DisasterRecoveryEvent`.
pub mod disaster_recovery_event {
    /// The severity of the disaster recovery event.
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
    pub enum Severity {
        /// Unspecified.
        Unspecified = 0,
        /// The Compute Engine resource is broken. A person must take an action.
        ActionRequired = 1,
        /// The Compute Engine resource is functioning. A change was applied to the
        /// resource during disaster recovery. Please take action to review
        /// the change to avoid unexpected problems.
        ActionSuggested = 2,
        /// Normal maintenance opeartions during disaster recovery, such as start up,
        /// shut down.
        Notice = 3,
    }
    impl Severity {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Severity::Unspecified => "SEVERITY_UNSPECIFIED",
                Severity::ActionRequired => "ACTION_REQUIRED",
                Severity::ActionSuggested => "ACTION_SUGGESTED",
                Severity::Notice => "NOTICE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SEVERITY_UNSPECIFIED" => Some(Self::Unspecified),
                "ACTION_REQUIRED" => Some(Self::ActionRequired),
                "ACTION_SUGGESTED" => Some(Self::ActionSuggested),
                "NOTICE" => Some(Self::Notice),
                _ => None,
            }
        }
    }
}
