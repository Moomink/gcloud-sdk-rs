/// A constraint describes a way to restrict resource's configuration. For
/// example, you could enforce a constraint that controls which Google Cloud
/// services can be activated across an organization, or whether a Compute Engine
/// instance can have serial port connections established. Constraints can be
/// configured by the organization policy administrator to fit the needs of the
/// organization by setting a policy that includes constraints at different
/// locations in the organization's resource hierarchy. Policies are inherited
/// down the resource hierarchy from higher levels, but can also be overridden.
/// For details about the inheritance rules please read about
/// [`policies`][google.cloud.OrgPolicy.v2.Policy].
///
/// Constraints have a default behavior determined by the `constraint_default`
/// field, which is the enforcement behavior that is used in the absence of a
/// policy being defined or inherited for the resource in question.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Constraint {
    /// Immutable. The resource name of the constraint. Must be in one of
    /// the following forms:
    ///
    /// * `projects/{project_number}/constraints/{constraint_name}`
    /// * `folders/{folder_id}/constraints/{constraint_name}`
    /// * `organizations/{organization_id}/constraints/{constraint_name}`
    ///
    /// For example, "/projects/123/constraints/compute.disableSerialPortAccess".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The human readable name.
    ///
    /// Mutable.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Detailed description of what this constraint controls as well as how and
    /// where it is enforced.
    ///
    /// Mutable.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// The evaluation behavior of this constraint in the absence of a policy.
    #[prost(enumeration = "constraint::ConstraintDefault", tag = "4")]
    pub constraint_default: i32,
    /// Shows if dry run is supported for this constraint or not.
    #[prost(bool, tag = "7")]
    pub supports_dry_run: bool,
    /// The type of restrictions for this `Constraint`.
    ///
    /// Immutable after creation.
    #[prost(oneof = "constraint::ConstraintType", tags = "5, 6")]
    pub constraint_type: ::core::option::Option<constraint::ConstraintType>,
}
/// Nested message and enum types in `Constraint`.
pub mod constraint {
    /// A constraint that allows or disallows a list of string values, which are
    /// configured by an Organization Policy administrator with a policy.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ListConstraint {
        /// Indicates whether values grouped into categories can be used in
        /// `Policy.allowed_values` and `Policy.denied_values`. For example,
        /// `"in:Python"` would match any value in the 'Python' group.
        #[prost(bool, tag = "1")]
        pub supports_in: bool,
        /// Indicates whether subtrees of the Resource Manager resource hierarchy
        /// can be used in `Policy.allowed_values` and `Policy.denied_values`. For
        /// example, `"under:folders/123"` would match any resource under the
        /// 'folders/123' folder.
        #[prost(bool, tag = "2")]
        pub supports_under: bool,
    }
    /// A constraint that is either enforced or not.
    ///
    /// For example, a constraint `constraints/compute.disableSerialPortAccess`.
    /// If it is enforced on a VM instance, serial port connections will not be
    /// opened to that instance.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BooleanConstraint {}
    /// Specifies the default behavior in the absence of any policy for the
    /// constraint. This must not be `CONSTRAINT_DEFAULT_UNSPECIFIED`.
    ///
    /// Immutable after creation.
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
    pub enum ConstraintDefault {
        /// This is only used for distinguishing unset values and should never be
        /// used.
        Unspecified = 0,
        /// Indicate that all values are allowed for list constraints.
        /// Indicate that enforcement is off for boolean constraints.
        Allow = 1,
        /// Indicate that all values are denied for list constraints.
        /// Indicate that enforcement is on for boolean constraints.
        Deny = 2,
    }
    impl ConstraintDefault {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ConstraintDefault::Unspecified => "CONSTRAINT_DEFAULT_UNSPECIFIED",
                ConstraintDefault::Allow => "ALLOW",
                ConstraintDefault::Deny => "DENY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CONSTRAINT_DEFAULT_UNSPECIFIED" => Some(Self::Unspecified),
                "ALLOW" => Some(Self::Allow),
                "DENY" => Some(Self::Deny),
                _ => None,
            }
        }
    }
    /// The type of restrictions for this `Constraint`.
    ///
    /// Immutable after creation.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConstraintType {
        /// Defines this constraint as being a ListConstraint.
        #[prost(message, tag = "5")]
        ListConstraint(ListConstraint),
        /// Defines this constraint as being a BooleanConstraint.
        #[prost(message, tag = "6")]
        BooleanConstraint(BooleanConstraint),
    }
}
/// A custom constraint defined by customers which can *only* be applied to the
/// given resource types and organization.
///
/// By creating a custom constraint, customers can apply policies of this
/// custom constraint. *Creating a custom constraint itself does NOT apply any
/// policy enforcement*.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomConstraint {
    /// Immutable. Name of the constraint. This is unique within the organization.
    /// Format of the name should be
    ///
    /// * `organizations/{organization_id}/customConstraints/{custom_constraint_id}`
    ///
    /// Example: `organizations/123/customConstraints/custom.createOnlyE2TypeVms`
    ///
    /// The max length is 70 characters and the minimum length is 1. Note that the
    /// prefix `organizations/{organization_id}/customConstraints/` is not counted.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. The resource instance type on which this policy applies. Format
    /// will be of the form : `<canonical service name>/<type>` Example:
    ///
    ///   * `compute.googleapis.com/Instance`.
    #[prost(string, repeated, tag = "2")]
    pub resource_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// All the operations being applied for this constraint.
    #[prost(enumeration = "custom_constraint::MethodType", repeated, tag = "3")]
    pub method_types: ::prost::alloc::vec::Vec<i32>,
    /// Org policy condition/expression. For example:
    /// `resource.instanceName.matches("\[production|test\]_.*_(\d)+")` or,
    /// `resource.management.auto_upgrade == true`
    ///
    /// The max length of the condition is 1000 characters.
    #[prost(string, tag = "4")]
    pub condition: ::prost::alloc::string::String,
    /// Allow or deny type.
    #[prost(enumeration = "custom_constraint::ActionType", tag = "5")]
    pub action_type: i32,
    /// One line display name for the UI.
    /// The max length of the display_name is 200 characters.
    #[prost(string, tag = "6")]
    pub display_name: ::prost::alloc::string::String,
    /// Detailed information about this custom policy constraint.
    /// The max length of the description is 2000 characters.
    #[prost(string, tag = "7")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The last time this custom constraint was updated. This
    /// represents the last time that the `CreateCustomConstraint` or
    /// `UpdateCustomConstraint` RPC was called
    #[prost(message, optional, tag = "8")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `CustomConstraint`.
pub mod custom_constraint {
    /// The operation for which this constraint will be applied. To apply this
    /// constraint only when creating new VMs, the `method_types` should be
    /// `CREATE` only. To apply this constraint when creating or deleting
    /// VMs, the `method_types` should be `CREATE` and `DELETE`.
    ///
    /// `UPDATE` only custom constraints are not supported. Use `CREATE` or
    /// `CREATE, UPDATE`.
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
    pub enum MethodType {
        /// Unspecified. Results in an error.
        Unspecified = 0,
        /// Constraint applied when creating the resource.
        Create = 1,
        /// Constraint applied when updating the resource.
        Update = 2,
        /// Constraint applied when deleting the resource.
        /// Not supported yet.
        Delete = 3,
    }
    impl MethodType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MethodType::Unspecified => "METHOD_TYPE_UNSPECIFIED",
                MethodType::Create => "CREATE",
                MethodType::Update => "UPDATE",
                MethodType::Delete => "DELETE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "METHOD_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATE" => Some(Self::Create),
                "UPDATE" => Some(Self::Update),
                "DELETE" => Some(Self::Delete),
                _ => None,
            }
        }
    }
    /// Allow or deny type.
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
    pub enum ActionType {
        /// Unspecified. Results in an error.
        Unspecified = 0,
        /// Allowed action type.
        Allow = 1,
        /// Deny action type.
        Deny = 2,
    }
    impl ActionType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ActionType::Unspecified => "ACTION_TYPE_UNSPECIFIED",
                ActionType::Allow => "ALLOW",
                ActionType::Deny => "DENY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ACTION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "ALLOW" => Some(Self::Allow),
                "DENY" => Some(Self::Deny),
                _ => None,
            }
        }
    }
}
/// Defines an organization policy which is used to specify constraints
/// for configurations of Google Cloud resources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Policy {
    /// Immutable. The resource name of the policy. Must be one of the following
    /// forms, where `constraint_name` is the name of the constraint which this
    /// policy configures:
    ///
    /// * `projects/{project_number}/policies/{constraint_name}`
    /// * `folders/{folder_id}/policies/{constraint_name}`
    /// * `organizations/{organization_id}/policies/{constraint_name}`
    ///
    /// For example, `projects/123/policies/compute.disableSerialPortAccess`.
    ///
    /// Note: `projects/{project_id}/policies/{constraint_name}` is also an
    /// acceptable name for API requests, but responses will return the name using
    /// the equivalent project number.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Basic information about the Organization Policy.
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<PolicySpec>,
    /// Deprecated.
    #[deprecated]
    #[prost(message, optional, tag = "3")]
    pub alternate: ::core::option::Option<AlternatePolicySpec>,
    /// Dry-run policy.
    /// Audit-only policy, can be used to monitor how the policy would have
    /// impacted the existing and future resources if it's enforced.
    #[prost(message, optional, tag = "4")]
    pub dry_run_spec: ::core::option::Option<PolicySpec>,
    /// Optional. An opaque tag indicating the current state of the policy, used
    /// for concurrency control. This 'etag' is computed by the server based on the
    /// value of other fields, and may be sent on update and delete requests to
    /// ensure the client has an up-to-date value before proceeding.
    #[prost(string, tag = "5")]
    pub etag: ::prost::alloc::string::String,
}
/// Similar to PolicySpec but with an extra 'launch' field for launch reference.
/// The PolicySpec here is specific for dry-run/darklaunch.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlternatePolicySpec {
    /// Reference to the launch that will be used while audit logging and to
    /// control the launch.
    /// Should be set only in the alternate policy.
    #[prost(string, tag = "1")]
    pub launch: ::prost::alloc::string::String,
    /// Specify constraint for configurations of Google Cloud resources.
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<PolicySpec>,
}
/// Defines a Google Cloud policy specification which is used to specify
/// constraints for configurations of Google Cloud resources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicySpec {
    /// An opaque tag indicating the current version of the policySpec, used for
    /// concurrency control.
    ///
    /// This field is ignored if used in a `CreatePolicy` request.
    ///
    /// When the policy is returned from either a `GetPolicy` or a
    /// `ListPolicies` request, this `etag` indicates the version of the
    /// current policySpec to use when executing a read-modify-write loop.
    ///
    /// When the policy is returned from a `GetEffectivePolicy` request, the
    /// `etag` will be unset.
    #[prost(string, tag = "1")]
    pub etag: ::prost::alloc::string::String,
    /// Output only. The time stamp this was previously updated. This
    /// represents the last time a call to `CreatePolicy` or `UpdatePolicy` was
    /// made for that policy.
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// In policies for boolean constraints, the following requirements apply:
    ///
    ///    - There must be one and only one policy rule where condition is unset.
    ///    - Boolean policy rules with conditions must set `enforced` to the
    ///      opposite of the policy rule without a condition.
    ///    - During policy evaluation, policy rules with conditions that are
    ///      true for a target resource take precedence.
    #[prost(message, repeated, tag = "3")]
    pub rules: ::prost::alloc::vec::Vec<policy_spec::PolicyRule>,
    /// Determines the inheritance behavior for this policy.
    ///
    /// If `inherit_from_parent` is true, policy rules set higher up in the
    /// hierarchy (up to the closest root) are inherited and present in the
    /// effective policy. If it is false, then no rules are inherited, and this
    /// policy becomes the new root for evaluation.
    /// This field can be set only for policies which configure list constraints.
    #[prost(bool, tag = "4")]
    pub inherit_from_parent: bool,
    /// Ignores policies set above this resource and restores the
    /// `constraint_default` enforcement behavior of the specific constraint at
    /// this resource.
    /// This field can be set in policies for either list or boolean
    /// constraints. If set, `rules` must be empty and `inherit_from_parent`
    /// must be set to false.
    #[prost(bool, tag = "5")]
    pub reset: bool,
}
/// Nested message and enum types in `PolicySpec`.
pub mod policy_spec {
    /// A rule used to express this policy.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PolicyRule {
        /// A condition which determines whether this rule is used
        /// in the evaluation of the policy. When set, the `expression` field in
        /// the `Expr' must include from 1 to 10 subexpressions, joined by the "||"
        /// or "&&" operators. Each subexpression must be of the form
        /// "resource.matchTag('<ORG_ID>/tag_key_short_name,
        /// 'tag_value_short_name')". or "resource.matchTagId('tagKeys/key_id',
        /// 'tagValues/value_id')". where key_name and value_name are the resource
        /// names for Label Keys and Values. These names are available from the Tag
        /// Manager Service. An example expression is:
        /// "resource.matchTag('123456789/environment,
        /// 'prod')". or "resource.matchTagId('tagKeys/123',
        /// 'tagValues/456')".
        #[prost(message, optional, tag = "5")]
        pub condition: ::core::option::Option<super::super::super::super::r#type::Expr>,
        #[prost(oneof = "policy_rule::Kind", tags = "1, 2, 3, 4")]
        pub kind: ::core::option::Option<policy_rule::Kind>,
    }
    /// Nested message and enum types in `PolicyRule`.
    pub mod policy_rule {
        /// A message that holds specific allowed and denied values.
        /// This message can define specific values and subtrees of the Resource
        /// Manager resource hierarchy (`Organizations`, `Folders`, `Projects`) that
        /// are allowed or denied. This is achieved by using the `under:` and
        /// optional `is:` prefixes.
        /// The `under:` prefix is used to denote resource subtree values.
        /// The `is:` prefix is used to denote specific values, and is required only
        /// if the value contains a ":". Values prefixed with "is:" are treated the
        /// same as values with no prefix.
        /// Ancestry subtrees must be in one of the following formats:
        ///
        /// - `projects/<project-id>` (for example, `projects/tokyo-rain-123`)
        /// - `folders/<folder-id>` (for example, `folders/1234`)
        /// - `organizations/<organization-id>` (for example, `organizations/1234`)
        ///
        /// The `supports_under` field of the associated `Constraint`  defines
        /// whether ancestry prefixes can be used.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct StringValues {
            /// List of values allowed at this resource.
            #[prost(string, repeated, tag = "1")]
            pub allowed_values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            /// List of values denied at this resource.
            #[prost(string, repeated, tag = "2")]
            pub denied_values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Kind {
            /// List of values to be used for this policy rule. This field can be set
            /// only in policies for list constraints.
            #[prost(message, tag = "1")]
            Values(StringValues),
            /// Setting this to true means that all values are allowed. This field can
            /// be set only in policies for list constraints.
            #[prost(bool, tag = "2")]
            AllowAll(bool),
            /// Setting this to true means that all values are denied. This field can
            /// be set only in policies for list constraints.
            #[prost(bool, tag = "3")]
            DenyAll(bool),
            /// If `true`, then the policy is enforced. If `false`, then any
            /// configuration is acceptable.
            /// This field can be set only in policies for boolean constraints.
            #[prost(bool, tag = "4")]
            Enforce(bool),
        }
    }
}
/// The request sent to the \[ListConstraints\]
/// \[google.cloud.orgpolicy.v2.OrgPolicy.ListConstraints\] method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConstraintsRequest {
    /// Required. The Google Cloud resource that parents the constraint. Must be in
    /// one of the following forms:
    ///
    /// * `projects/{project_number}`
    /// * `projects/{project_id}`
    /// * `folders/{folder_id}`
    /// * `organizations/{organization_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Size of the pages to be returned. This is currently unsupported and will
    /// be ignored. The server may at any point start using this field to limit
    /// page size.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Page token used to retrieve the next page. This is currently unsupported
    /// and will be ignored. The server may at any point start using this field.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response returned from the \[ListConstraints\]
/// \[google.cloud.orgpolicy.v2.OrgPolicy.ListConstraints\] method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConstraintsResponse {
    /// The collection of constraints that are available on the targeted resource.
    #[prost(message, repeated, tag = "1")]
    pub constraints: ::prost::alloc::vec::Vec<Constraint>,
    /// Page token used to retrieve the next page. This is currently not used.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request sent to the \[ListPolicies\]
/// \[google.cloud.orgpolicy.v2.OrgPolicy.ListPolicies\] method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPoliciesRequest {
    /// Required. The target Google Cloud resource that parents the set of
    /// constraints and policies that will be returned from this call. Must be in
    /// one of the following forms:
    ///
    /// * `projects/{project_number}`
    /// * `projects/{project_id}`
    /// * `folders/{folder_id}`
    /// * `organizations/{organization_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Size of the pages to be returned. This is currently unsupported and will
    /// be ignored. The server may at any point start using this field to limit
    /// page size.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Page token used to retrieve the next page. This is currently unsupported
    /// and will be ignored. The server may at any point start using this field.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response returned from the \[ListPolicies\]
/// \[google.cloud.orgpolicy.v2.OrgPolicy.ListPolicies\] method. It will be empty
/// if no policies are set on the resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPoliciesResponse {
    /// All policies that exist on the resource. It will be empty if no
    /// policies are set.
    #[prost(message, repeated, tag = "1")]
    pub policies: ::prost::alloc::vec::Vec<Policy>,
    /// Page token used to retrieve the next page. This is currently not used, but
    /// the server may at any point start supplying a valid token.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request sent to the \[GetPolicy\]
/// \[google.cloud.orgpolicy.v2.OrgPolicy.GetPolicy\] method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPolicyRequest {
    /// Required. Resource name of the policy. See
    /// [Policy][google.cloud.orgpolicy.v2.Policy] for naming requirements.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request sent to the \[GetEffectivePolicy\]
/// \[google.cloud.orgpolicy.v2.OrgPolicy.GetEffectivePolicy\] method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEffectivePolicyRequest {
    /// Required. The effective policy to compute. See
    /// [Policy][google.cloud.orgpolicy.v2.Policy] for naming requirements.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request sent to the \[CreatePolicyRequest\]
/// \[google.cloud.orgpolicy.v2.OrgPolicy.CreatePolicy\] method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePolicyRequest {
    /// Required. The Google Cloud resource that will parent the new policy. Must
    /// be in one of the following forms:
    ///
    /// * `projects/{project_number}`
    /// * `projects/{project_id}`
    /// * `folders/{folder_id}`
    /// * `organizations/{organization_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Policy to create.
    #[prost(message, optional, tag = "3")]
    pub policy: ::core::option::Option<Policy>,
}
/// The request sent to the \[UpdatePolicyRequest\]
/// \[google.cloud.orgpolicy.v2.OrgPolicy.UpdatePolicy\] method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePolicyRequest {
    /// Required. Policy to update.
    #[prost(message, optional, tag = "1")]
    pub policy: ::core::option::Option<Policy>,
    /// Field mask used to specify the fields to be overwritten in the policy
    /// by the set. The fields specified in the update_mask are relative to the
    /// policy, not the full request.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The request sent to the \[DeletePolicy\]
/// \[google.cloud.orgpolicy.v2.OrgPolicy.DeletePolicy\] method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePolicyRequest {
    /// Required. Name of the policy to delete.
    /// See the policy entry for naming rules.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The current etag of policy. If an etag is provided and does not
    /// match the current etag of the policy, deletion will be blocked and an
    /// ABORTED error will be returned.
    #[prost(string, tag = "2")]
    pub etag: ::prost::alloc::string::String,
}
/// The request sent to the \[CreateCustomConstraintRequest\]
/// \[google.cloud.orgpolicy.v2.OrgPolicy.CreateCustomConstraint\] method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCustomConstraintRequest {
    /// Required. Must be in the following form:
    ///
    /// * `organizations/{organization_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Custom constraint to create.
    #[prost(message, optional, tag = "2")]
    pub custom_constraint: ::core::option::Option<CustomConstraint>,
}
/// The request sent to the \[GetCustomConstraint\]
/// \[google.cloud.orgpolicy.v2.OrgPolicy.GetCustomConstraint\] method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomConstraintRequest {
    /// Required. Resource name of the custom constraint. See the custom constraint
    /// entry for naming requirements.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request sent to the \[ListCustomConstraints\]
/// \[google.cloud.orgpolicy.v2.OrgPolicy.ListCustomConstraints\] method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCustomConstraintsRequest {
    /// Required. The target Google Cloud resource that parents the set of custom
    /// constraints that will be returned from this call. Must be in one of the
    /// following forms:
    ///
    /// * `organizations/{organization_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Size of the pages to be returned. This is currently unsupported and will
    /// be ignored. The server may at any point start using this field to limit
    /// page size.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Page token used to retrieve the next page. This is currently unsupported
    /// and will be ignored. The server may at any point start using this field.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response returned from the \[ListCustomConstraints\]
/// \[google.cloud.orgpolicy.v2.OrgPolicy.ListCustomConstraints\] method. It will
/// be empty if no custom constraints are set on the organization resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCustomConstraintsResponse {
    /// All custom constraints that exist on the organization resource. It will be
    /// empty if no custom constraints are set.
    #[prost(message, repeated, tag = "1")]
    pub custom_constraints: ::prost::alloc::vec::Vec<CustomConstraint>,
    /// Page token used to retrieve the next page. This is currently not used, but
    /// the server may at any point start supplying a valid token.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request sent to the \[UpdateCustomConstraintRequest\]
/// \[google.cloud.orgpolicy.v2.OrgPolicy.UpdateCustomConstraint\] method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCustomConstraintRequest {
    /// Required. `CustomConstraint` to update.
    #[prost(message, optional, tag = "1")]
    pub custom_constraint: ::core::option::Option<CustomConstraint>,
}
/// The request sent to the \[DeleteCustomConstraint\]
/// \[google.cloud.orgpolicy.v2.OrgPolicy.DeleteCustomConstraint\] method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCustomConstraintRequest {
    /// Required. Name of the custom constraint to delete.
    /// See the custom constraint entry for naming rules.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod org_policy_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// An interface for managing organization policies.
    ///
    /// The Organization Policy Service provides a simple mechanism for
    /// organizations to restrict the allowed configurations across their entire
    /// resource hierarchy.
    ///
    /// You can use a policy to configure restrictions on resources. For
    /// example, you can enforce a policy that restricts which Google
    /// Cloud APIs can be activated in a certain part of your resource
    /// hierarchy, or prevents serial port access to VM instances in a
    /// particular folder.
    ///
    /// Policies are inherited down through the resource hierarchy. A policy
    /// applied to a parent resource automatically applies to all its child resources
    /// unless overridden with a policy lower in the hierarchy.
    ///
    /// A constraint defines an aspect of a resource's configuration that can be
    /// controlled by an organization's policy administrator. Policies are a
    /// collection of constraints that defines their allowable configuration on a
    /// particular resource and its child resources.
    #[derive(Debug, Clone)]
    pub struct OrgPolicyClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OrgPolicyClient<tonic::transport::Channel> {
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
    impl<T> OrgPolicyClient<T>
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
        ) -> OrgPolicyClient<InterceptedService<T, F>>
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
            OrgPolicyClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists constraints that could be applied on the specified resource.
        pub async fn list_constraints(
            &mut self,
            request: impl tonic::IntoRequest<super::ListConstraintsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListConstraintsResponse>,
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
                "/google.cloud.orgpolicy.v2.OrgPolicy/ListConstraints",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.orgpolicy.v2.OrgPolicy",
                        "ListConstraints",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves all of the policies that exist on a particular resource.
        pub async fn list_policies(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPoliciesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPoliciesResponse>,
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
                "/google.cloud.orgpolicy.v2.OrgPolicy/ListPolicies",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.orgpolicy.v2.OrgPolicy",
                        "ListPolicies",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a policy on a resource.
        ///
        /// If no policy is set on the resource, `NOT_FOUND` is returned. The
        /// `etag` value can be used with `UpdatePolicy()` to update a
        /// policy during read-modify-write.
        pub async fn get_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPolicyRequest>,
        ) -> std::result::Result<tonic::Response<super::Policy>, tonic::Status> {
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
                "/google.cloud.orgpolicy.v2.OrgPolicy/GetPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.orgpolicy.v2.OrgPolicy", "GetPolicy"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the effective policy on a resource. This is the result of merging
        /// policies in the resource hierarchy and evaluating conditions. The
        /// returned policy will not have an `etag` or `condition` set because it is
        /// an evaluated policy across multiple resources.
        /// Subtrees of Resource Manager resource hierarchy with 'under:' prefix will
        /// not be expanded.
        pub async fn get_effective_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEffectivePolicyRequest>,
        ) -> std::result::Result<tonic::Response<super::Policy>, tonic::Status> {
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
                "/google.cloud.orgpolicy.v2.OrgPolicy/GetEffectivePolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.orgpolicy.v2.OrgPolicy",
                        "GetEffectivePolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a policy.
        ///
        /// Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the
        /// constraint does not exist.
        /// Returns a `google.rpc.Status` with `google.rpc.Code.ALREADY_EXISTS` if the
        /// policy already exists on the given Google Cloud resource.
        pub async fn create_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePolicyRequest>,
        ) -> std::result::Result<tonic::Response<super::Policy>, tonic::Status> {
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
                "/google.cloud.orgpolicy.v2.OrgPolicy/CreatePolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.orgpolicy.v2.OrgPolicy",
                        "CreatePolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a policy.
        ///
        /// Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the
        /// constraint or the policy do not exist.
        /// Returns a `google.rpc.Status` with `google.rpc.Code.ABORTED` if the etag
        /// supplied in the request does not match the persisted etag of the policy
        ///
        /// Note: the supplied policy will perform a full overwrite of all
        /// fields.
        pub async fn update_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePolicyRequest>,
        ) -> std::result::Result<tonic::Response<super::Policy>, tonic::Status> {
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
                "/google.cloud.orgpolicy.v2.OrgPolicy/UpdatePolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.orgpolicy.v2.OrgPolicy",
                        "UpdatePolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a policy.
        ///
        /// Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the
        /// constraint or organization policy does not exist.
        pub async fn delete_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePolicyRequest>,
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
                "/google.cloud.orgpolicy.v2.OrgPolicy/DeletePolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.orgpolicy.v2.OrgPolicy",
                        "DeletePolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a custom constraint.
        ///
        /// Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the
        /// organization does not exist.
        /// Returns a `google.rpc.Status` with `google.rpc.Code.ALREADY_EXISTS` if the
        /// constraint already exists on the given organization.
        pub async fn create_custom_constraint(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCustomConstraintRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CustomConstraint>,
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
                "/google.cloud.orgpolicy.v2.OrgPolicy/CreateCustomConstraint",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.orgpolicy.v2.OrgPolicy",
                        "CreateCustomConstraint",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a custom constraint.
        ///
        /// Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the
        /// constraint does not exist.
        ///
        /// Note: the supplied policy will perform a full overwrite of all
        /// fields.
        pub async fn update_custom_constraint(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCustomConstraintRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CustomConstraint>,
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
                "/google.cloud.orgpolicy.v2.OrgPolicy/UpdateCustomConstraint",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.orgpolicy.v2.OrgPolicy",
                        "UpdateCustomConstraint",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a custom constraint.
        ///
        /// Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the
        /// custom constraint does not exist.
        pub async fn get_custom_constraint(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCustomConstraintRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CustomConstraint>,
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
                "/google.cloud.orgpolicy.v2.OrgPolicy/GetCustomConstraint",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.orgpolicy.v2.OrgPolicy",
                        "GetCustomConstraint",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves all of the custom constraints that exist on a particular
        /// organization resource.
        pub async fn list_custom_constraints(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCustomConstraintsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCustomConstraintsResponse>,
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
                "/google.cloud.orgpolicy.v2.OrgPolicy/ListCustomConstraints",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.orgpolicy.v2.OrgPolicy",
                        "ListCustomConstraints",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a custom constraint.
        ///
        /// Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the
        /// constraint does not exist.
        pub async fn delete_custom_constraint(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCustomConstraintRequest>,
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
                "/google.cloud.orgpolicy.v2.OrgPolicy/DeleteCustomConstraint",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.orgpolicy.v2.OrgPolicy",
                        "DeleteCustomConstraint",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
