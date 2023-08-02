/// Information about the principal, resource, and permission to check.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessTuple {
    /// Required. The principal whose access you want to check, in the form of
    /// the email address that represents that principal. For example,
    /// `alice@example.com` or
    /// `my-service-account@my-project.iam.gserviceaccount.com`.
    ///
    /// The principal must be a Google Account or a service account. Other types of
    /// principals are not supported.
    #[prost(string, tag = "1")]
    pub principal: ::prost::alloc::string::String,
    /// Required. The full resource name that identifies the resource. For example,
    /// `//compute.googleapis.com/projects/my-project/zones/us-central1-a/instances/my-instance`.
    ///
    /// For examples of full resource names for Google Cloud services, see
    /// <https://cloud.google.com/iam/help/troubleshooter/full-resource-names.>
    #[prost(string, tag = "2")]
    pub full_resource_name: ::prost::alloc::string::String,
    /// Required. The IAM permission to check for the specified principal and
    /// resource.
    ///
    /// For a complete list of IAM permissions, see
    /// <https://cloud.google.com/iam/help/permissions/reference.>
    ///
    /// For a complete list of predefined IAM roles and the permissions in each
    /// role, see <https://cloud.google.com/iam/help/roles/reference.>
    #[prost(string, tag = "3")]
    pub permission: ::prost::alloc::string::String,
}
/// Details about how a specific IAM \[Policy][google.iam.v1.Policy\] contributed
/// to the access check.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExplainedPolicy {
    /// Indicates whether _this policy_ provides the specified permission to the
    /// specified principal for the specified resource.
    ///
    /// This field does _not_ indicate whether the principal actually has the
    /// permission for the resource. There might be another policy that overrides
    /// this policy. To determine whether the principal actually has the
    /// permission, use the `access` field in the
    /// \[TroubleshootIamPolicyResponse][IamChecker.TroubleshootIamPolicyResponse\].
    #[prost(enumeration = "AccessState", tag = "1")]
    pub access: i32,
    /// The full resource name that identifies the resource. For example,
    /// `//compute.googleapis.com/projects/my-project/zones/us-central1-a/instances/my-instance`.
    ///
    /// If the user who created the
    /// \[Replay][google.cloud.policysimulator.v1.Replay\] does not have
    /// access to the policy, this field is omitted.
    ///
    /// For examples of full resource names for Google Cloud services, see
    /// <https://cloud.google.com/iam/help/troubleshooter/full-resource-names.>
    #[prost(string, tag = "2")]
    pub full_resource_name: ::prost::alloc::string::String,
    /// The IAM policy attached to the resource.
    ///
    /// If the user who created the
    /// \[Replay][google.cloud.policysimulator.v1.Replay\] does not have
    /// access to the policy, this field is empty.
    #[prost(message, optional, tag = "3")]
    pub policy: ::core::option::Option<super::super::super::iam::v1::Policy>,
    /// Details about how each binding in the policy affects the principal's
    /// ability, or inability, to use the permission for the resource.
    ///
    /// If the user who created the
    /// \[Replay][google.cloud.policysimulator.v1.Replay\] does not have
    /// access to the policy, this field is omitted.
    #[prost(message, repeated, tag = "4")]
    pub binding_explanations: ::prost::alloc::vec::Vec<BindingExplanation>,
    /// The relevance of this policy to the overall determination in the
    /// \[TroubleshootIamPolicyResponse][IamChecker.TroubleshootIamPolicyResponse\].
    ///
    /// If the user who created the
    /// \[Replay][google.cloud.policysimulator.v1.Replay\] does not have
    /// access to the policy, this field is omitted.
    #[prost(enumeration = "HeuristicRelevance", tag = "5")]
    pub relevance: i32,
}
/// Details about how a binding in a policy affects a principal's ability to use
/// a permission.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BindingExplanation {
    /// Required. Indicates whether _this binding_ provides the specified
    /// permission to the specified principal for the specified resource.
    ///
    /// This field does _not_ indicate whether the principal actually has the
    /// permission for the resource. There might be another binding that overrides
    /// this binding. To determine whether the principal actually has the
    /// permission, use the `access` field in the
    /// \[TroubleshootIamPolicyResponse][IamChecker.TroubleshootIamPolicyResponse\].
    #[prost(enumeration = "AccessState", tag = "1")]
    pub access: i32,
    /// The role that this binding grants. For example,
    /// `roles/compute.serviceAgent`.
    ///
    /// For a complete list of predefined IAM roles, as well as the permissions in
    /// each role, see <https://cloud.google.com/iam/help/roles/reference.>
    #[prost(string, tag = "2")]
    pub role: ::prost::alloc::string::String,
    /// Indicates whether the role granted by this binding contains the specified
    /// permission.
    #[prost(enumeration = "binding_explanation::RolePermission", tag = "3")]
    pub role_permission: i32,
    /// The relevance of the permission's existence, or nonexistence, in the role
    /// to the overall determination for the entire policy.
    #[prost(enumeration = "HeuristicRelevance", tag = "4")]
    pub role_permission_relevance: i32,
    /// Indicates whether each principal in the binding includes the principal
    /// specified in the request, either directly or indirectly. Each key
    /// identifies a principal in the binding, and each value indicates whether the
    /// principal in the binding includes the principal in the request.
    ///
    /// For example, suppose that a binding includes the following principals:
    ///
    /// * `user:alice@example.com`
    /// * `group:product-eng@example.com`
    ///
    /// The principal in the replayed access tuple is `user:bob@example.com`. This
    /// user is a principal of the group `group:product-eng@example.com`.
    ///
    /// For the first principal in the binding, the key is
    /// `user:alice@example.com`, and the `membership` field in the value is set to
    /// `MEMBERSHIP_NOT_INCLUDED`.
    ///
    /// For the second principal in the binding, the key is
    /// `group:product-eng@example.com`, and the `membership` field in the value is
    /// set to `MEMBERSHIP_INCLUDED`.
    #[prost(map = "string, message", tag = "5")]
    pub memberships: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        binding_explanation::AnnotatedMembership,
    >,
    /// The relevance of this binding to the overall determination for the entire
    /// policy.
    #[prost(enumeration = "HeuristicRelevance", tag = "6")]
    pub relevance: i32,
    /// A condition expression that prevents this binding from granting access
    /// unless the expression evaluates to `true`.
    ///
    /// To learn about IAM Conditions, see
    /// <https://cloud.google.com/iam/docs/conditions-overview.>
    #[prost(message, optional, tag = "7")]
    pub condition: ::core::option::Option<super::super::super::r#type::Expr>,
}
/// Nested message and enum types in `BindingExplanation`.
pub mod binding_explanation {
    /// Details about whether the binding includes the principal.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AnnotatedMembership {
        /// Indicates whether the binding includes the principal.
        #[prost(enumeration = "Membership", tag = "1")]
        pub membership: i32,
        /// The relevance of the principal's status to the overall determination for
        /// the binding.
        #[prost(enumeration = "super::HeuristicRelevance", tag = "2")]
        pub relevance: i32,
    }
    /// Whether a role includes a specific permission.
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
    pub enum RolePermission {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// The permission is included in the role.
        Included = 1,
        /// The permission is not included in the role.
        NotIncluded = 2,
        /// The user who created the
        /// \[Replay][google.cloud.policysimulator.v1.Replay\] is not
        /// allowed to access the binding.
        UnknownInfoDenied = 3,
    }
    impl RolePermission {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RolePermission::Unspecified => "ROLE_PERMISSION_UNSPECIFIED",
                RolePermission::Included => "ROLE_PERMISSION_INCLUDED",
                RolePermission::NotIncluded => "ROLE_PERMISSION_NOT_INCLUDED",
                RolePermission::UnknownInfoDenied => {
                    "ROLE_PERMISSION_UNKNOWN_INFO_DENIED"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ROLE_PERMISSION_UNSPECIFIED" => Some(Self::Unspecified),
                "ROLE_PERMISSION_INCLUDED" => Some(Self::Included),
                "ROLE_PERMISSION_NOT_INCLUDED" => Some(Self::NotIncluded),
                "ROLE_PERMISSION_UNKNOWN_INFO_DENIED" => Some(Self::UnknownInfoDenied),
                _ => None,
            }
        }
    }
    /// Whether the binding includes the principal.
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
    pub enum Membership {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// The binding includes the principal. The principal can be included
        /// directly or indirectly. For example:
        ///
        /// * A principal is included directly if that principal is listed in the
        ///    binding.
        /// * A principal is included indirectly if that principal is in a Google
        ///    group or Google Workspace domain that is listed in the binding.
        Included = 1,
        /// The binding does not include the principal.
        NotIncluded = 2,
        /// The user who created the
        /// \[Replay][google.cloud.policysimulator.v1.Replay\] is not
        /// allowed to access the binding.
        UnknownInfoDenied = 3,
        /// The principal is an unsupported type. Only Google Accounts and service
        /// accounts are supported.
        UnknownUnsupported = 4,
    }
    impl Membership {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Membership::Unspecified => "MEMBERSHIP_UNSPECIFIED",
                Membership::Included => "MEMBERSHIP_INCLUDED",
                Membership::NotIncluded => "MEMBERSHIP_NOT_INCLUDED",
                Membership::UnknownInfoDenied => "MEMBERSHIP_UNKNOWN_INFO_DENIED",
                Membership::UnknownUnsupported => "MEMBERSHIP_UNKNOWN_UNSUPPORTED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MEMBERSHIP_UNSPECIFIED" => Some(Self::Unspecified),
                "MEMBERSHIP_INCLUDED" => Some(Self::Included),
                "MEMBERSHIP_NOT_INCLUDED" => Some(Self::NotIncluded),
                "MEMBERSHIP_UNKNOWN_INFO_DENIED" => Some(Self::UnknownInfoDenied),
                "MEMBERSHIP_UNKNOWN_UNSUPPORTED" => Some(Self::UnknownUnsupported),
                _ => None,
            }
        }
    }
}
/// Whether a principal has a permission for a resource.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AccessState {
    /// Default value. This value is unused.
    Unspecified = 0,
    /// The principal has the permission.
    Granted = 1,
    /// The principal does not have the permission.
    NotGranted = 2,
    /// The principal has the permission only if a condition expression evaluates
    /// to `true`.
    UnknownConditional = 3,
    /// The user who created the
    /// \[Replay][google.cloud.policysimulator.v1.Replay\] does not have
    /// access to all of the policies that Policy Simulator needs to evaluate.
    UnknownInfoDenied = 4,
}
impl AccessState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AccessState::Unspecified => "ACCESS_STATE_UNSPECIFIED",
            AccessState::Granted => "GRANTED",
            AccessState::NotGranted => "NOT_GRANTED",
            AccessState::UnknownConditional => "UNKNOWN_CONDITIONAL",
            AccessState::UnknownInfoDenied => "UNKNOWN_INFO_DENIED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ACCESS_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "GRANTED" => Some(Self::Granted),
            "NOT_GRANTED" => Some(Self::NotGranted),
            "UNKNOWN_CONDITIONAL" => Some(Self::UnknownConditional),
            "UNKNOWN_INFO_DENIED" => Some(Self::UnknownInfoDenied),
            _ => None,
        }
    }
}
/// The extent to which a single data point, such as the existence of a binding
/// or whether a binding includes a specific principal, contributes to an overall
/// determination.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HeuristicRelevance {
    /// Default value. This value is unused.
    Unspecified = 0,
    /// The data point has a limited effect on the result. Changing the data point
    /// is unlikely to affect the overall determination.
    Normal = 1,
    /// The data point has a strong effect on the result. Changing the data point
    /// is likely to affect the overall determination.
    High = 2,
}
impl HeuristicRelevance {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            HeuristicRelevance::Unspecified => "HEURISTIC_RELEVANCE_UNSPECIFIED",
            HeuristicRelevance::Normal => "NORMAL",
            HeuristicRelevance::High => "HIGH",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "HEURISTIC_RELEVANCE_UNSPECIFIED" => Some(Self::Unspecified),
            "NORMAL" => Some(Self::Normal),
            "HIGH" => Some(Self::High),
            _ => None,
        }
    }
}
/// A resource describing a `Replay`, or simulation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Replay {
    /// Output only. The resource name of the `Replay`, which has the following
    /// format:
    ///
    /// `{projects|folders|organizations}/{resource-id}/locations/global/replays/{replay-id}`,
    /// where `{resource-id}` is the ID of the project, folder, or organization
    /// that owns the Replay.
    ///
    /// Example:
    /// `projects/my-example-project/locations/global/replays/506a5f7f-38ce-4d7d-8e03-479ce1833c36`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The current state of the `Replay`.
    #[prost(enumeration = "replay::State", tag = "2")]
    pub state: i32,
    /// Required. The configuration used for the `Replay`.
    #[prost(message, optional, tag = "3")]
    pub config: ::core::option::Option<ReplayConfig>,
    /// Output only. Summary statistics about the replayed log entries.
    #[prost(message, optional, tag = "5")]
    pub results_summary: ::core::option::Option<replay::ResultsSummary>,
}
/// Nested message and enum types in `Replay`.
pub mod replay {
    /// Summary statistics about the replayed log entries.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResultsSummary {
        /// The total number of log entries replayed.
        #[prost(int32, tag = "1")]
        pub log_count: i32,
        /// The number of replayed log entries with no difference between
        /// baseline and simulated policies.
        #[prost(int32, tag = "2")]
        pub unchanged_count: i32,
        /// The number of replayed log entries with a difference between baseline and
        /// simulated policies.
        #[prost(int32, tag = "3")]
        pub difference_count: i32,
        /// The number of log entries that could not be replayed.
        #[prost(int32, tag = "4")]
        pub error_count: i32,
        /// The date of the oldest log entry replayed.
        #[prost(message, optional, tag = "5")]
        pub oldest_date: ::core::option::Option<
            super::super::super::super::r#type::Date,
        >,
        /// The date of the newest log entry replayed.
        #[prost(message, optional, tag = "6")]
        pub newest_date: ::core::option::Option<
            super::super::super::super::r#type::Date,
        >,
    }
    /// The current state of the \[Replay][google.cloud.policysimulator.v1.Replay\].
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
        /// The `Replay` has not started yet.
        Pending = 1,
        /// The `Replay` is currently running.
        Running = 2,
        /// The `Replay` has successfully completed.
        Succeeded = 3,
        /// The `Replay` has finished with an error.
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
                State::Pending => "PENDING",
                State::Running => "RUNNING",
                State::Succeeded => "SUCCEEDED",
                State::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "PENDING" => Some(Self::Pending),
                "RUNNING" => Some(Self::Running),
                "SUCCEEDED" => Some(Self::Succeeded),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
}
/// The result of replaying a single access tuple against a simulated state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplayResult {
    /// The resource name of the `ReplayResult`, in the following format:
    ///
    /// `{projects|folders|organizations}/{resource-id}/locations/global/replays/{replay-id}/results/{replay-result-id}`,
    /// where `{resource-id}` is the ID of the project, folder, or organization
    /// that owns the \[Replay][google.cloud.policysimulator.v1.Replay\].
    ///
    /// Example:
    /// `projects/my-example-project/locations/global/replays/506a5f7f-38ce-4d7d-8e03-479ce1833c36/results/1234`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The \[Replay][google.cloud.policysimulator.v1.Replay\] that the access tuple
    /// was included in.
    #[prost(string, tag = "2")]
    pub parent: ::prost::alloc::string::String,
    /// The access tuple that was replayed. This field includes information about
    /// the principal, resource, and permission that were involved in the access
    /// attempt.
    #[prost(message, optional, tag = "3")]
    pub access_tuple: ::core::option::Option<AccessTuple>,
    /// The latest date this access tuple was seen in the logs.
    #[prost(message, optional, tag = "4")]
    pub last_seen_date: ::core::option::Option<super::super::super::r#type::Date>,
    /// The result of replaying the access tuple.
    #[prost(oneof = "replay_result::Result", tags = "5, 6")]
    pub result: ::core::option::Option<replay_result::Result>,
}
/// Nested message and enum types in `ReplayResult`.
pub mod replay_result {
    /// The result of replaying the access tuple.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Result {
        /// The difference between the principal's access under the current
        /// (baseline) policies and the principal's access under the proposed
        /// (simulated) policies.
        ///
        /// This field is only included for access tuples that were successfully
        /// replayed and had different results under the current policies and the
        /// proposed policies.
        #[prost(message, tag = "5")]
        Diff(super::ReplayDiff),
        /// The error that caused the access tuple replay to fail.
        ///
        /// This field is only included for access tuples that were not replayed
        /// successfully.
        #[prost(message, tag = "6")]
        Error(super::super::super::super::rpc::Status),
    }
}
/// Request message for
/// \[Simulator.CreateReplay][google.cloud.policysimulator.v1.Simulator.CreateReplay\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateReplayRequest {
    /// Required. The parent resource where this
    /// \[Replay][google.cloud.policysimulator.v1.Replay\] will be created. This
    /// resource must be a project, folder, or organization with a location.
    ///
    /// Example: `projects/my-example-project/locations/global`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The \[Replay][google.cloud.policysimulator.v1.Replay\] to create.
    /// Set `Replay.ReplayConfig` to configure the replay.
    #[prost(message, optional, tag = "2")]
    pub replay: ::core::option::Option<Replay>,
}
/// Metadata about a Replay operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplayOperationMetadata {
    /// Time when the request was received.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request message for
/// \[Simulator.GetReplay][google.cloud.policysimulator.v1.Simulator.GetReplay\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReplayRequest {
    /// Required. The name of the \[Replay][google.cloud.policysimulator.v1.Replay\]
    /// to retrieve, in the following format:
    ///
    /// `{projects|folders|organizations}/{resource-id}/locations/global/replays/{replay-id}`,
    /// where `{resource-id}` is the ID of the project, folder, or organization
    /// that owns the `Replay`.
    ///
    /// Example:
    /// `projects/my-example-project/locations/global/replays/506a5f7f-38ce-4d7d-8e03-479ce1833c36`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// \[Simulator.ListReplayResults][google.cloud.policysimulator.v1.Simulator.ListReplayResults\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReplayResultsRequest {
    /// Required. The \[Replay][google.cloud.policysimulator.v1.Replay\] whose
    /// results are listed, in the following format:
    ///
    /// `{projects|folders|organizations}/{resource-id}/locations/global/replays/{replay-id}`
    ///
    /// Example:
    /// `projects/my-project/locations/global/replays/506a5f7f-38ce-4d7d-8e03-479ce1833c36`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of
    /// \[ReplayResult][google.cloud.policysimulator.v1.ReplayResult\] objects to
    /// return. Defaults to 5000.
    ///
    /// The maximum value is 5000; values above 5000 are rounded down to 5000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous
    /// \[Simulator.ListReplayResults][google.cloud.policysimulator.v1.Simulator.ListReplayResults\]
    /// call. Provide this token to retrieve the next page of results.
    ///
    /// When paginating, all other parameters provided to
    /// \[Simulator.ListReplayResults[\] must match the call that provided the page
    /// token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for
/// \[Simulator.ListReplayResults][google.cloud.policysimulator.v1.Simulator.ListReplayResults\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReplayResultsResponse {
    /// The results of running a \[Replay][google.cloud.policysimulator.v1.Replay\].
    #[prost(message, repeated, tag = "1")]
    pub replay_results: ::prost::alloc::vec::Vec<ReplayResult>,
    /// A token that you can use to retrieve the next page of
    /// \[ReplayResult][google.cloud.policysimulator.v1.ReplayResult\] objects. If
    /// this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The configuration used for a
/// \[Replay][google.cloud.policysimulator.v1.Replay\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplayConfig {
    /// A mapping of the resources that you want to simulate policies for and the
    /// policies that you want to simulate.
    ///
    /// Keys are the full resource names for the resources. For example,
    /// `//cloudresourcemanager.googleapis.com/projects/my-project`.
    /// For examples of full resource names for Google Cloud services, see
    /// <https://cloud.google.com/iam/help/troubleshooter/full-resource-names.>
    ///
    /// Values are \[Policy][google.iam.v1.Policy\] objects representing the policies
    /// that you want to simulate.
    ///
    /// Replays automatically take into account any IAM policies inherited through
    /// the resource hierarchy, and any policies set on descendant resources. You
    /// do not need to include these policies in the policy overlay.
    #[prost(map = "string, message", tag = "1")]
    pub policy_overlay: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::super::super::iam::v1::Policy,
    >,
    /// The logs to use as input for the
    /// \[Replay][google.cloud.policysimulator.v1.Replay\].
    #[prost(enumeration = "replay_config::LogSource", tag = "2")]
    pub log_source: i32,
}
/// Nested message and enum types in `ReplayConfig`.
pub mod replay_config {
    /// The source of the logs to use for a
    /// \[Replay][google.cloud.policysimulator.v1.Replay\].
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
    pub enum LogSource {
        /// An unspecified log source.
        /// If the log source is unspecified, the
        /// \[Replay][google.cloud.policysimulator.v1.Replay\] defaults to using
        /// `RECENT_ACCESSES`.
        Unspecified = 0,
        /// All access logs from the last 90 days. These logs may not include logs
        /// from the most recent 7 days.
        RecentAccesses = 1,
    }
    impl LogSource {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LogSource::Unspecified => "LOG_SOURCE_UNSPECIFIED",
                LogSource::RecentAccesses => "RECENT_ACCESSES",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LOG_SOURCE_UNSPECIFIED" => Some(Self::Unspecified),
                "RECENT_ACCESSES" => Some(Self::RecentAccesses),
                _ => None,
            }
        }
    }
}
/// The difference between the results of evaluating an access tuple under
/// the current (baseline) policies and under the proposed (simulated) policies.
/// This difference explains how a principal's access could change if the
/// proposed policies were applied.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplayDiff {
    /// A summary and comparison of the principal's access under the current
    /// (baseline) policies and the proposed (simulated) policies for a single
    /// access tuple.
    ///
    /// The evaluation of the principal's access is reported in the
    /// \[AccessState][google.cloud.policysimulator.v1.AccessState\] field.
    #[prost(message, optional, tag = "2")]
    pub access_diff: ::core::option::Option<AccessStateDiff>,
}
/// A summary and comparison of the principal's access under the current
/// (baseline) policies and the proposed (simulated) policies for a single
/// access tuple.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessStateDiff {
    /// The results of evaluating the access tuple under the current (baseline)
    /// policies.
    ///
    /// If the \[AccessState][google.cloud.policysimulator.v1.AccessState\] couldn't
    /// be fully evaluated, this field explains why.
    #[prost(message, optional, tag = "1")]
    pub baseline: ::core::option::Option<ExplainedAccess>,
    /// The results of evaluating the access tuple under the proposed (simulated)
    /// policies.
    ///
    /// If the AccessState couldn't be fully evaluated, this field explains why.
    #[prost(message, optional, tag = "2")]
    pub simulated: ::core::option::Option<ExplainedAccess>,
    /// How the principal's access, specified in the AccessState field, changed
    /// between the current (baseline) policies and proposed (simulated) policies.
    #[prost(enumeration = "access_state_diff::AccessChangeType", tag = "3")]
    pub access_change: i32,
}
/// Nested message and enum types in `AccessStateDiff`.
pub mod access_state_diff {
    /// How the principal's access, specified in the AccessState field, changed
    /// between the current (baseline) policies and proposed (simulated) policies.
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
    pub enum AccessChangeType {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// The principal's access did not change.
        /// This includes the case where both baseline and simulated are UNKNOWN,
        /// but the unknown information is equivalent.
        NoChange = 1,
        /// The principal's access under both the current policies and the proposed
        /// policies is `UNKNOWN`, but the unknown information differs between them.
        UnknownChange = 2,
        /// The principal had access under the current policies (`GRANTED`), but will
        /// no longer have access after the proposed changes (`NOT_GRANTED`).
        AccessRevoked = 3,
        /// The principal did not have access under the current policies
        /// (`NOT_GRANTED`), but will have access after the proposed changes
        /// (`GRANTED`).
        AccessGained = 4,
        /// This result can occur for the following reasons:
        ///
        /// * The principal had access under the current policies (`GRANTED`), but
        ///    their access after the proposed changes is `UNKNOWN`.
        ///
        /// * The principal's access under the current policies is `UNKNOWN`, but
        /// they
        ///    will not have access after the proposed changes (`NOT_GRANTED`).
        AccessMaybeRevoked = 5,
        /// This result can occur for the following reasons:
        ///
        /// * The principal did not have access under the current policies
        ///    (`NOT_GRANTED`), but their access after the proposed changes is
        ///    `UNKNOWN`.
        ///
        /// * The principal's access under the current policies is `UNKNOWN`, but
        /// they will have access after the proposed changes (`GRANTED`).
        AccessMaybeGained = 6,
    }
    impl AccessChangeType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AccessChangeType::Unspecified => "ACCESS_CHANGE_TYPE_UNSPECIFIED",
                AccessChangeType::NoChange => "NO_CHANGE",
                AccessChangeType::UnknownChange => "UNKNOWN_CHANGE",
                AccessChangeType::AccessRevoked => "ACCESS_REVOKED",
                AccessChangeType::AccessGained => "ACCESS_GAINED",
                AccessChangeType::AccessMaybeRevoked => "ACCESS_MAYBE_REVOKED",
                AccessChangeType::AccessMaybeGained => "ACCESS_MAYBE_GAINED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ACCESS_CHANGE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "NO_CHANGE" => Some(Self::NoChange),
                "UNKNOWN_CHANGE" => Some(Self::UnknownChange),
                "ACCESS_REVOKED" => Some(Self::AccessRevoked),
                "ACCESS_GAINED" => Some(Self::AccessGained),
                "ACCESS_MAYBE_REVOKED" => Some(Self::AccessMaybeRevoked),
                "ACCESS_MAYBE_GAINED" => Some(Self::AccessMaybeGained),
                _ => None,
            }
        }
    }
}
/// Details about how a set of policies, listed in
/// \[ExplainedPolicy][google.cloud.policysimulator.v1.ExplainedPolicy\], resulted
/// in a certain \[AccessState][google.cloud.policysimulator.v1.AccessState\] when
/// replaying an access tuple.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExplainedAccess {
    /// Whether the principal in the access tuple has permission to access the
    /// resource in the access tuple under the given policies.
    #[prost(enumeration = "AccessState", tag = "1")]
    pub access_state: i32,
    /// If the \[AccessState][google.cloud.policysimulator.v1.AccessState\] is
    /// `UNKNOWN`, this field contains the policies that led to that result.
    ///
    /// If the `AccessState` is `GRANTED` or `NOT_GRANTED`, this field is
    /// omitted.
    #[prost(message, repeated, tag = "2")]
    pub policies: ::prost::alloc::vec::Vec<ExplainedPolicy>,
    /// If the \[AccessState][google.cloud.policysimulator.v1.AccessState\] is
    /// `UNKNOWN`, this field contains a list of errors explaining why the result
    /// is `UNKNOWN`.
    ///
    /// If the `AccessState` is `GRANTED` or `NOT_GRANTED`, this field is
    /// omitted.
    #[prost(message, repeated, tag = "3")]
    pub errors: ::prost::alloc::vec::Vec<super::super::super::rpc::Status>,
}
/// Generated client implementations.
pub mod simulator_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Policy Simulator API service.
    ///
    /// Policy Simulator is a collection of endpoints for creating, running, and
    /// viewing a [Replay][google.cloud.policysimulator.v1.Replay]. A
    /// [Replay][google.cloud.policysimulator.v1.Replay] is a type of simulation that
    /// lets you see how your principals' access to resources might change if you
    /// changed your IAM policy.
    ///
    /// During a [Replay][google.cloud.policysimulator.v1.Replay], Policy Simulator
    /// re-evaluates, or replays, past access attempts under both the current policy
    /// and  your proposed policy, and compares those results to determine how your
    /// principals' access might change under the proposed policy.
    #[derive(Debug, Clone)]
    pub struct SimulatorClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SimulatorClient<tonic::transport::Channel> {
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
    impl<T> SimulatorClient<T>
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
        ) -> SimulatorClient<InterceptedService<T, F>>
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
            SimulatorClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets the specified [Replay][google.cloud.policysimulator.v1.Replay]. Each
        /// `Replay` is available for at least 7 days.
        pub async fn get_replay(
            &mut self,
            request: impl tonic::IntoRequest<super::GetReplayRequest>,
        ) -> std::result::Result<tonic::Response<super::Replay>, tonic::Status> {
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
                "/google.cloud.policysimulator.v1.Simulator/GetReplay",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.policysimulator.v1.Simulator",
                        "GetReplay",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates and starts a [Replay][google.cloud.policysimulator.v1.Replay] using
        /// the given [ReplayConfig][google.cloud.policysimulator.v1.ReplayConfig].
        pub async fn create_replay(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateReplayRequest>,
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
                "/google.cloud.policysimulator.v1.Simulator/CreateReplay",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.policysimulator.v1.Simulator",
                        "CreateReplay",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists the results of running a
        /// [Replay][google.cloud.policysimulator.v1.Replay].
        pub async fn list_replay_results(
            &mut self,
            request: impl tonic::IntoRequest<super::ListReplayResultsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListReplayResultsResponse>,
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
                "/google.cloud.policysimulator.v1.Simulator/ListReplayResults",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.policysimulator.v1.Simulator",
                        "ListReplayResults",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
