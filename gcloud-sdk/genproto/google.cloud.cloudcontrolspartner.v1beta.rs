/// Details about the Access request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessApprovalRequest {
    /// Identifier. Format:
    /// organizations/{organization}/locations/{location}/customers/{customer}/workloads/{workload}/accessApprovalRequests/{access_approval_request}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The time at which approval was requested.
    #[prost(message, optional, tag = "2")]
    pub request_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The justification for which approval is being requested.
    #[prost(message, optional, tag = "3")]
    pub requested_reason: ::core::option::Option<AccessReason>,
    /// The requested expiration for the approval. If the request is approved,
    /// access will be granted from the time of approval until the expiration time.
    #[prost(message, optional, tag = "4")]
    pub requested_expiration_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request for getting the access requests associated with a workload.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAccessApprovalRequestsRequest {
    /// Required. Parent resource
    /// Format:
    /// organizations/{organization}/locations/{location}/customers/{customer}/workloads/{workload}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of access requests to return. The service may
    /// return fewer than this value. If unspecified, at most 500 access requests
    /// will be returned.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous
    /// `ListAccessApprovalRequests` call. Provide this to retrieve the subsequent
    /// page.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filtering results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Hint for how to order the results.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for list access requests.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAccessApprovalRequestsResponse {
    /// List of access approval requests
    #[prost(message, repeated, tag = "1")]
    pub access_approval_requests: ::prost::alloc::vec::Vec<AccessApprovalRequest>,
    /// A token that can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Reason for the access.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessReason {
    /// Type of access justification.
    #[prost(enumeration = "access_reason::Type", tag = "1")]
    pub r#type: i32,
    /// More detail about certain reason types. See comments for each type above.
    #[prost(string, tag = "2")]
    pub detail: ::prost::alloc::string::String,
}
/// Nested message and enum types in `AccessReason`.
pub mod access_reason {
    /// Type of access justification.
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
        /// Default value for proto, shouldn't be used.
        Unspecified = 0,
        /// Customer made a request or raised an issue that required the principal to
        /// access customer data. `detail` is of the form ("#####" is the issue ID):
        ///
        /// - "Feedback Report: #####"
        /// - "Case Number: #####"
        /// - "Case ID: #####"
        /// - "E-PIN Reference: #####"
        /// - "Google-#####"
        /// - "T-#####"
        CustomerInitiatedSupport = 1,
        /// The principal accessed customer data in order to diagnose or resolve a
        /// suspected issue in services. Often this access is used to confirm that
        /// customers are not affected by a suspected service issue or to remediate a
        /// reversible system issue.
        GoogleInitiatedService = 2,
        /// Google initiated service for security, fraud, abuse, or compliance
        /// purposes.
        GoogleInitiatedReview = 3,
        /// The principal was compelled to access customer data in order to respond
        /// to a legal third party data request or process, including legal processes
        /// from customers themselves.
        ThirdPartyDataRequest = 4,
        /// The principal accessed customer data in order to diagnose or resolve a
        /// suspected issue in services or a known outage.
        GoogleResponseToProductionAlert = 5,
        /// Similar to 'GOOGLE_INITIATED_SERVICE' or 'GOOGLE_INITIATED_REVIEW', but
        /// with universe agnostic naming. The principal accessed customer data in
        /// order to diagnose or resolve a suspected issue in services or a known
        /// outage, or for security, fraud, abuse, or compliance review purposes.
        CloudInitiatedAccess = 6,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::CustomerInitiatedSupport => "CUSTOMER_INITIATED_SUPPORT",
                Type::GoogleInitiatedService => "GOOGLE_INITIATED_SERVICE",
                Type::GoogleInitiatedReview => "GOOGLE_INITIATED_REVIEW",
                Type::ThirdPartyDataRequest => "THIRD_PARTY_DATA_REQUEST",
                Type::GoogleResponseToProductionAlert => {
                    "GOOGLE_RESPONSE_TO_PRODUCTION_ALERT"
                }
                Type::CloudInitiatedAccess => "CLOUD_INITIATED_ACCESS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "CUSTOMER_INITIATED_SUPPORT" => Some(Self::CustomerInitiatedSupport),
                "GOOGLE_INITIATED_SERVICE" => Some(Self::GoogleInitiatedService),
                "GOOGLE_INITIATED_REVIEW" => Some(Self::GoogleInitiatedReview),
                "THIRD_PARTY_DATA_REQUEST" => Some(Self::ThirdPartyDataRequest),
                "GOOGLE_RESPONSE_TO_PRODUCTION_ALERT" => {
                    Some(Self::GoogleResponseToProductionAlert)
                }
                "CLOUD_INITIATED_ACCESS" => Some(Self::CloudInitiatedAccess),
                _ => None,
            }
        }
    }
}
/// Enum for possible completion states.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CompletionState {
    /// Unspecified completion state.
    Unspecified = 0,
    /// Task started (has start date) but not yet completed.
    Pending = 1,
    /// Succeeded state.
    Succeeded = 2,
    /// Failed state.
    Failed = 3,
    /// Not applicable state.
    NotApplicable = 4,
}
impl CompletionState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CompletionState::Unspecified => "COMPLETION_STATE_UNSPECIFIED",
            CompletionState::Pending => "PENDING",
            CompletionState::Succeeded => "SUCCEEDED",
            CompletionState::Failed => "FAILED",
            CompletionState::NotApplicable => "NOT_APPLICABLE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "COMPLETION_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "PENDING" => Some(Self::Pending),
            "SUCCEEDED" => Some(Self::Succeeded),
            "FAILED" => Some(Self::Failed),
            "NOT_APPLICABLE" => Some(Self::NotApplicable),
            _ => None,
        }
    }
}
/// Contains metadata around the [Workload
/// resource](<https://cloud.google.com/assured-workloads/docs/reference/rest/Shared.Types/Workload>)
/// in the Assured Workloads API.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Workload {
    /// Identifier. Format:
    /// organizations/{organization}/locations/{location}/customers/{customer}/workloads/{workload}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Folder id this workload is associated with
    #[prost(int64, tag = "2")]
    pub folder_id: i64,
    /// Output only. Time the resource was created.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The name of container folder of the assured workload
    #[prost(string, tag = "4")]
    pub folder: ::prost::alloc::string::String,
    /// Container for workload onboarding steps.
    #[prost(message, optional, tag = "5")]
    pub workload_onboarding_state: ::core::option::Option<WorkloadOnboardingState>,
    /// Indicates whether a workload is fully onboarded.
    #[prost(bool, tag = "6")]
    pub is_onboarded: bool,
    /// The project id of the key management project for the workload
    #[prost(string, tag = "7")]
    pub key_management_project_id: ::prost::alloc::string::String,
    /// The Google Cloud location of the workload
    #[prost(string, tag = "8")]
    pub location: ::prost::alloc::string::String,
    /// Partner associated with this workload.
    #[prost(enumeration = "workload::Partner", tag = "9")]
    pub partner: i32,
}
/// Nested message and enum types in `Workload`.
pub mod workload {
    /// Supported Assured Workloads Partners.
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
    pub enum Partner {
        /// Unknown Partner.
        Unspecified = 0,
        /// Enum representing S3NS (Thales) partner.
        LocalControlsByS3ns = 1,
        /// Enum representing T_SYSTEM (TSI) partner.
        SovereignControlsByTSystems = 2,
        /// Enum representing SIA_MINSAIT (Indra) partner.
        SovereignControlsBySiaMinsait = 3,
        /// Enum representing PSN (TIM) partner.
        SovereignControlsByPsn = 4,
        /// Enum representing CNTXT (Kingdom of Saudi Arabia) partner.
        SovereignControlsByCntxt = 6,
        /// Enum representing CNXT (Kingdom of Saudi Arabia) partner offering without
        /// EKM provisioning.
        SovereignControlsByCntxtNoEkm = 7,
    }
    impl Partner {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Partner::Unspecified => "PARTNER_UNSPECIFIED",
                Partner::LocalControlsByS3ns => "PARTNER_LOCAL_CONTROLS_BY_S3NS",
                Partner::SovereignControlsByTSystems => {
                    "PARTNER_SOVEREIGN_CONTROLS_BY_T_SYSTEMS"
                }
                Partner::SovereignControlsBySiaMinsait => {
                    "PARTNER_SOVEREIGN_CONTROLS_BY_SIA_MINSAIT"
                }
                Partner::SovereignControlsByPsn => "PARTNER_SOVEREIGN_CONTROLS_BY_PSN",
                Partner::SovereignControlsByCntxt => {
                    "PARTNER_SOVEREIGN_CONTROLS_BY_CNTXT"
                }
                Partner::SovereignControlsByCntxtNoEkm => {
                    "PARTNER_SOVEREIGN_CONTROLS_BY_CNTXT_NO_EKM"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PARTNER_UNSPECIFIED" => Some(Self::Unspecified),
                "PARTNER_LOCAL_CONTROLS_BY_S3NS" => Some(Self::LocalControlsByS3ns),
                "PARTNER_SOVEREIGN_CONTROLS_BY_T_SYSTEMS" => {
                    Some(Self::SovereignControlsByTSystems)
                }
                "PARTNER_SOVEREIGN_CONTROLS_BY_SIA_MINSAIT" => {
                    Some(Self::SovereignControlsBySiaMinsait)
                }
                "PARTNER_SOVEREIGN_CONTROLS_BY_PSN" => Some(Self::SovereignControlsByPsn),
                "PARTNER_SOVEREIGN_CONTROLS_BY_CNTXT" => {
                    Some(Self::SovereignControlsByCntxt)
                }
                "PARTNER_SOVEREIGN_CONTROLS_BY_CNTXT_NO_EKM" => {
                    Some(Self::SovereignControlsByCntxtNoEkm)
                }
                _ => None,
            }
        }
    }
}
/// Request to list customer workloads.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkloadsRequest {
    /// Required. Parent resource
    /// Format:
    /// organizations/{organization}/locations/{location}/customers/{customer}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of workloads to return. The service may return fewer
    /// than this value. If unspecified, at most 500 workloads will be returned.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListWorkloads` call.
    /// Provide this to retrieve the subsequent page.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filtering results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Hint for how to order the results.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for list customer workloads requests.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkloadsResponse {
    /// List of customer workloads
    #[prost(message, repeated, tag = "1")]
    pub workloads: ::prost::alloc::vec::Vec<Workload>,
    /// A token that can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for getting a customer workload.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWorkloadRequest {
    /// Required. Format:
    /// organizations/{organization}/locations/{location}/customers/{customer}/workloads/{workload}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Container for workload onboarding steps.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkloadOnboardingState {
    /// List of workload onboarding steps.
    #[prost(message, repeated, tag = "1")]
    pub onboarding_steps: ::prost::alloc::vec::Vec<WorkloadOnboardingStep>,
}
/// Container for workload onboarding information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkloadOnboardingStep {
    /// The onboarding step.
    #[prost(enumeration = "workload_onboarding_step::Step", tag = "1")]
    pub step: i32,
    /// The starting time of the onboarding step.
    #[prost(message, optional, tag = "2")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The completion time of the onboarding step.
    #[prost(message, optional, tag = "3")]
    pub completion_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The completion state of the onboarding step.
    #[prost(enumeration = "CompletionState", tag = "4")]
    pub completion_state: i32,
}
/// Nested message and enum types in `WorkloadOnboardingStep`.
pub mod workload_onboarding_step {
    /// Enum for possible onboarding steps.
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
    pub enum Step {
        /// Unspecified step.
        Unspecified = 0,
        /// EKM Provisioned step.
        EkmProvisioned = 1,
        /// Signed Access Approval step.
        SignedAccessApprovalConfigured = 2,
    }
    impl Step {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Step::Unspecified => "STEP_UNSPECIFIED",
                Step::EkmProvisioned => "EKM_PROVISIONED",
                Step::SignedAccessApprovalConfigured => {
                    "SIGNED_ACCESS_APPROVAL_CONFIGURED"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STEP_UNSPECIFIED" => Some(Self::Unspecified),
                "EKM_PROVISIONED" => Some(Self::EkmProvisioned),
                "SIGNED_ACCESS_APPROVAL_CONFIGURED" => {
                    Some(Self::SignedAccessApprovalConfigured)
                }
                _ => None,
            }
        }
    }
}
/// Contains metadata around a Cloud Controls Partner Customer
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Customer {
    /// Identifier. Format:
    /// organizations/{organization}/locations/{location}/customers/{customer}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The customer organization's display name. E.g. "google.com".
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Container for customer onboarding steps
    #[prost(message, optional, tag = "3")]
    pub customer_onboarding_state: ::core::option::Option<CustomerOnboardingState>,
    /// Indicates whether a customer is fully onboarded
    #[prost(bool, tag = "4")]
    pub is_onboarded: bool,
}
/// Request to list customers
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCustomersRequest {
    /// Required. Parent resource
    /// Format: organizations/{organization}/locations/{location}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of Customers to return. The service may return fewer
    /// than this value. If unspecified, at most 500 Customers will be returned.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListCustomers` call.
    /// Provide this to retrieve the subsequent page.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filtering results
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Hint for how to order the results
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for list customer Customers requests
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCustomersResponse {
    /// List of customers
    #[prost(message, repeated, tag = "1")]
    pub customers: ::prost::alloc::vec::Vec<Customer>,
    /// A token that can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for getting a customer
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomerRequest {
    /// Required. Format:
    /// organizations/{organization}/locations/{location}/customers/{customer}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Container for customer onboarding steps
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerOnboardingState {
    /// List of customer onboarding steps
    #[prost(message, repeated, tag = "1")]
    pub onboarding_steps: ::prost::alloc::vec::Vec<CustomerOnboardingStep>,
}
/// Container for customer onboarding information
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerOnboardingStep {
    /// The onboarding step
    #[prost(enumeration = "customer_onboarding_step::Step", tag = "1")]
    pub step: i32,
    /// The starting time of the onboarding step
    #[prost(message, optional, tag = "2")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The completion time of the onboarding step
    #[prost(message, optional, tag = "3")]
    pub completion_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Current state of the step
    #[prost(enumeration = "CompletionState", tag = "4")]
    pub completion_state: i32,
}
/// Nested message and enum types in `CustomerOnboardingStep`.
pub mod customer_onboarding_step {
    /// Enum for possible onboarding steps
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
    pub enum Step {
        /// Unspecified step
        Unspecified = 0,
        /// KAJ Enrollment
        KajEnrollment = 1,
        /// Customer Environment
        CustomerEnvironment = 2,
    }
    impl Step {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Step::Unspecified => "STEP_UNSPECIFIED",
                Step::KajEnrollment => "KAJ_ENROLLMENT",
                Step::CustomerEnvironment => "CUSTOMER_ENVIRONMENT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STEP_UNSPECIFIED" => Some(Self::Unspecified),
                "KAJ_ENROLLMENT" => Some(Self::KajEnrollment),
                "CUSTOMER_ENVIRONMENT" => Some(Self::CustomerEnvironment),
                _ => None,
            }
        }
    }
}
/// The EKM connections associated with a workload
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EkmConnections {
    /// Identifier. Format:
    /// organizations/{organization}/locations/{location}/customers/{customer}/workloads/{workload}/ekmConnections
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The EKM connections associated with the workload
    #[prost(message, repeated, tag = "2")]
    pub ekm_connections: ::prost::alloc::vec::Vec<EkmConnection>,
}
/// Request for getting the EKM connections associated with a workload
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEkmConnectionsRequest {
    /// Required. Format:
    /// organizations/{organization}/locations/{location}/customers/{customer}/workloads/{workload}/ekmConnections
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Details about the EKM connection
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EkmConnection {
    /// Resource name of the EKM connection in the format:
    /// projects/{project}/locations/{location}/ekmConnections/{ekm_connection}
    #[prost(string, tag = "1")]
    pub connection_name: ::prost::alloc::string::String,
    /// Output only. The connection state
    #[prost(enumeration = "ekm_connection::ConnectionState", tag = "2")]
    pub connection_state: i32,
    /// The connection error that occurred if any
    #[prost(message, optional, tag = "3")]
    pub connection_error: ::core::option::Option<ekm_connection::ConnectionError>,
}
/// Nested message and enum types in `EkmConnection`.
pub mod ekm_connection {
    /// Information around the error that occurred if the connection state is
    /// anything other than available or unspecified
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConnectionError {
        /// The error domain for the error
        #[prost(string, tag = "1")]
        pub error_domain: ::prost::alloc::string::String,
        /// The error message for the error
        #[prost(string, tag = "2")]
        pub error_message: ::prost::alloc::string::String,
    }
    /// The EKM connection state.
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
    pub enum ConnectionState {
        /// Unspecified EKM connection state
        Unspecified = 0,
        /// Available EKM connection state
        Available = 1,
        /// Not available EKM connection state
        NotAvailable = 2,
        /// Error EKM connection state
        Error = 3,
        /// Permission denied EKM connection state
        PermissionDenied = 4,
    }
    impl ConnectionState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ConnectionState::Unspecified => "CONNECTION_STATE_UNSPECIFIED",
                ConnectionState::Available => "AVAILABLE",
                ConnectionState::NotAvailable => "NOT_AVAILABLE",
                ConnectionState::Error => "ERROR",
                ConnectionState::PermissionDenied => "PERMISSION_DENIED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CONNECTION_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "AVAILABLE" => Some(Self::Available),
                "NOT_AVAILABLE" => Some(Self::NotAvailable),
                "ERROR" => Some(Self::Error),
                "PERMISSION_DENIED" => Some(Self::PermissionDenied),
                _ => None,
            }
        }
    }
}
/// The permissions granted to the partner for a workload
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartnerPermissions {
    /// Identifier. Format:
    /// organizations/{organization}/locations/{location}/customers/{customer}/workloads/{workload}/partnerPermissions
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The partner permissions granted for the workload
    #[prost(enumeration = "partner_permissions::Permission", repeated, tag = "2")]
    pub partner_permissions: ::prost::alloc::vec::Vec<i32>,
}
/// Nested message and enum types in `PartnerPermissions`.
pub mod partner_permissions {
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
    pub enum Permission {
        /// Unspecified partner permission
        Unspecified = 0,
        /// Permission for Access Transparency and emergency logs
        AccessTransparencyAndEmergencyAccessLogs = 1,
        /// Permission for Assured Workloads monitoring violations
        AssuredWorkloadsMonitoring = 2,
        /// Permission for Access Approval requests
        AccessApprovalRequests = 3,
        /// Permission for External Key Manager connection status
        AssuredWorkloadsEkmConnectionStatus = 4,
    }
    impl Permission {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Permission::Unspecified => "PERMISSION_UNSPECIFIED",
                Permission::AccessTransparencyAndEmergencyAccessLogs => {
                    "ACCESS_TRANSPARENCY_AND_EMERGENCY_ACCESS_LOGS"
                }
                Permission::AssuredWorkloadsMonitoring => "ASSURED_WORKLOADS_MONITORING",
                Permission::AccessApprovalRequests => "ACCESS_APPROVAL_REQUESTS",
                Permission::AssuredWorkloadsEkmConnectionStatus => {
                    "ASSURED_WORKLOADS_EKM_CONNECTION_STATUS"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PERMISSION_UNSPECIFIED" => Some(Self::Unspecified),
                "ACCESS_TRANSPARENCY_AND_EMERGENCY_ACCESS_LOGS" => {
                    Some(Self::AccessTransparencyAndEmergencyAccessLogs)
                }
                "ASSURED_WORKLOADS_MONITORING" => Some(Self::AssuredWorkloadsMonitoring),
                "ACCESS_APPROVAL_REQUESTS" => Some(Self::AccessApprovalRequests),
                "ASSURED_WORKLOADS_EKM_CONNECTION_STATUS" => {
                    Some(Self::AssuredWorkloadsEkmConnectionStatus)
                }
                _ => None,
            }
        }
    }
}
/// Request for getting the partner permissions granted for a workload
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPartnerPermissionsRequest {
    /// Required. Name of the resource to get in the format:
    /// organizations/{organization}/locations/{location}/customers/{customer}/workloads/{workload}/partnerPermissions
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message describing Partner resource
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Partner {
    /// Identifier. The resource name of the partner.
    /// Format: organizations/{organization}/locations/{location}/partner
    /// Example: "organizations/123456/locations/us-central1/partner"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// List of SKUs the partner is offering
    #[prost(message, repeated, tag = "3")]
    pub skus: ::prost::alloc::vec::Vec<Sku>,
    /// List of Google Cloud supported EKM partners supported by the partner
    #[prost(message, repeated, tag = "4")]
    pub ekm_solutions: ::prost::alloc::vec::Vec<EkmMetadata>,
    /// List of Google Cloud regions that the partner sells services to customers.
    /// Valid Google Cloud regions found here:
    /// <https://cloud.google.com/compute/docs/regions-zones>
    #[prost(string, repeated, tag = "5")]
    pub operated_cloud_regions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Google Cloud project ID in the partner's Google Cloud organization for
    /// receiving enhanced Logs for Partners.
    #[prost(string, tag = "7")]
    pub partner_project_id: ::prost::alloc::string::String,
    /// Output only. Time the resource was created
    #[prost(message, optional, tag = "9")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last time the resource was updated
    #[prost(message, optional, tag = "10")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Message for getting a Partner
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPartnerRequest {
    /// Required. Format: organizations/{organization}/locations/{location}/partner
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Represents the SKU a partner owns inside Google Cloud to sell to customers.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sku {
    /// Argentum product SKU, that is associated with the partner offerings to
    /// customers used by Syntro for billing purposes. SKUs can represent resold
    /// Google products or support services.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Display name of the product identified by the SKU. A partner may want to
    /// show partner branded names for their offerings such as local sovereign
    /// cloud solutions.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
}
/// Holds information needed by Mudbray to use partner EKMs for workloads.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EkmMetadata {
    /// The Cloud EKM partner.
    #[prost(enumeration = "ekm_metadata::EkmSolution", tag = "1")]
    pub ekm_solution: i32,
    /// Endpoint for sending requests to the EKM for key provisioning during
    /// Assured Workload creation.
    #[prost(string, tag = "2")]
    pub ekm_endpoint_uri: ::prost::alloc::string::String,
}
/// Nested message and enum types in `EkmMetadata`.
pub mod ekm_metadata {
    /// Represents Google Cloud supported external key management partners
    /// [Google Cloud EKM partners
    /// docs](<https://cloud.google.com/kms/docs/ekm#supported_partners>).
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
    pub enum EkmSolution {
        /// Unspecified EKM solution
        Unspecified = 0,
        /// EKM Partner Fortanix
        Fortanix = 1,
        /// EKM Partner FutureX
        Futurex = 2,
        /// EKM Partner Thales
        Thales = 3,
        /// EKM Partner Virtu
        Virtru = 4,
    }
    impl EkmSolution {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EkmSolution::Unspecified => "EKM_SOLUTION_UNSPECIFIED",
                EkmSolution::Fortanix => "FORTANIX",
                EkmSolution::Futurex => "FUTUREX",
                EkmSolution::Thales => "THALES",
                EkmSolution::Virtru => "VIRTRU",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "EKM_SOLUTION_UNSPECIFIED" => Some(Self::Unspecified),
                "FORTANIX" => Some(Self::Fortanix),
                "FUTUREX" => Some(Self::Futurex),
                "THALES" => Some(Self::Thales),
                "VIRTRU" => Some(Self::Virtru),
                _ => None,
            }
        }
    }
}
/// Represents the metadata of the long-running operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
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
    /// of the operation. Operations that have been cancelled successfully
    /// have [Operation.error][] value with a
    /// [google.rpc.Status.code][google.rpc.Status.code] of 1, corresponding to
    /// `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod cloud_controls_partner_core_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service describing handlers for resources
    #[derive(Debug, Clone)]
    pub struct CloudControlsPartnerCoreClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CloudControlsPartnerCoreClient<tonic::transport::Channel> {
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
    impl<T> CloudControlsPartnerCoreClient<T>
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
        ) -> CloudControlsPartnerCoreClient<InterceptedService<T, F>>
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
            CloudControlsPartnerCoreClient::new(
                InterceptedService::new(inner, interceptor),
            )
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
        /// Gets details of a single workload
        pub async fn get_workload(
            &mut self,
            request: impl tonic::IntoRequest<super::GetWorkloadRequest>,
        ) -> std::result::Result<tonic::Response<super::Workload>, tonic::Status> {
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
                "/google.cloud.cloudcontrolspartner.v1beta.CloudControlsPartnerCore/GetWorkload",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.cloudcontrolspartner.v1beta.CloudControlsPartnerCore",
                        "GetWorkload",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists customer workloads for a given customer org id
        pub async fn list_workloads(
            &mut self,
            request: impl tonic::IntoRequest<super::ListWorkloadsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListWorkloadsResponse>,
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
                "/google.cloud.cloudcontrolspartner.v1beta.CloudControlsPartnerCore/ListWorkloads",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.cloudcontrolspartner.v1beta.CloudControlsPartnerCore",
                        "ListWorkloads",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single customer
        pub async fn get_customer(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCustomerRequest>,
        ) -> std::result::Result<tonic::Response<super::Customer>, tonic::Status> {
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
                "/google.cloud.cloudcontrolspartner.v1beta.CloudControlsPartnerCore/GetCustomer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.cloudcontrolspartner.v1beta.CloudControlsPartnerCore",
                        "GetCustomer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists customers of a partner identified by its Google Cloud organization ID
        pub async fn list_customers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCustomersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCustomersResponse>,
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
                "/google.cloud.cloudcontrolspartner.v1beta.CloudControlsPartnerCore/ListCustomers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.cloudcontrolspartner.v1beta.CloudControlsPartnerCore",
                        "ListCustomers",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the EKM connections associated with a workload
        pub async fn get_ekm_connections(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEkmConnectionsRequest>,
        ) -> std::result::Result<tonic::Response<super::EkmConnections>, tonic::Status> {
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
                "/google.cloud.cloudcontrolspartner.v1beta.CloudControlsPartnerCore/GetEkmConnections",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.cloudcontrolspartner.v1beta.CloudControlsPartnerCore",
                        "GetEkmConnections",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the partner permissions granted for a workload
        pub async fn get_partner_permissions(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPartnerPermissionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PartnerPermissions>,
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
                "/google.cloud.cloudcontrolspartner.v1beta.CloudControlsPartnerCore/GetPartnerPermissions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.cloudcontrolspartner.v1beta.CloudControlsPartnerCore",
                        "GetPartnerPermissions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists access requests associated with a workload
        pub async fn list_access_approval_requests(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAccessApprovalRequestsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAccessApprovalRequestsResponse>,
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
                "/google.cloud.cloudcontrolspartner.v1beta.CloudControlsPartnerCore/ListAccessApprovalRequests",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.cloudcontrolspartner.v1beta.CloudControlsPartnerCore",
                        "ListAccessApprovalRequests",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get details of a Partner.
        pub async fn get_partner(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPartnerRequest>,
        ) -> std::result::Result<tonic::Response<super::Partner>, tonic::Status> {
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
                "/google.cloud.cloudcontrolspartner.v1beta.CloudControlsPartnerCore/GetPartner",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.cloudcontrolspartner.v1beta.CloudControlsPartnerCore",
                        "GetPartner",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Details of resource Violation
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Violation {
    /// Identifier. Format:
    /// organizations/{organization}/locations/{location}/customers/{customer}/workloads/{workload}/violations/{violation}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Description for the Violation.
    /// e.g. OrgPolicy gcp.resourceLocations has non compliant value.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Time of the event which triggered the Violation.
    #[prost(message, optional, tag = "3")]
    pub begin_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last time when the Violation record was updated.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time of the event which fixed the Violation.
    /// If the violation is ACTIVE this will be empty.
    #[prost(message, optional, tag = "5")]
    pub resolve_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Category under which this violation is mapped.
    /// e.g. Location, Service Usage, Access, Encryption, etc.
    #[prost(string, tag = "6")]
    pub category: ::prost::alloc::string::String,
    /// Output only. State of the violation
    #[prost(enumeration = "violation::State", tag = "7")]
    pub state: i32,
    /// Output only. Immutable. Name of the OrgPolicy which was modified with
    /// non-compliant change and resulted this violation. Format:
    ///   projects/{project_number}/policies/{constraint_name}
    ///   folders/{folder_id}/policies/{constraint_name}
    ///   organizations/{organization_id}/policies/{constraint_name}
    #[prost(string, tag = "8")]
    pub non_compliant_org_policy: ::prost::alloc::string::String,
    /// The folder_id of the violation
    #[prost(int64, tag = "9")]
    pub folder_id: i64,
    /// Output only. Compliance violation remediation
    #[prost(message, optional, tag = "13")]
    pub remediation: ::core::option::Option<violation::Remediation>,
}
/// Nested message and enum types in `Violation`.
pub mod violation {
    /// Represents remediation guidance to resolve compliance violation for
    /// AssuredWorkload
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Remediation {
        /// Required. Remediation instructions to resolve violations
        #[prost(message, optional, tag = "1")]
        pub instructions: ::core::option::Option<remediation::Instructions>,
        /// Values that can resolve the violation
        /// For example: for list org policy violations, this will either be the list
        /// of allowed or denied values
        #[prost(string, repeated, tag = "2")]
        pub compliant_values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Output only. Remediation type based on the type of org policy values
        /// violated
        #[prost(enumeration = "remediation::RemediationType", tag = "3")]
        pub remediation_type: i32,
    }
    /// Nested message and enum types in `Remediation`.
    pub mod remediation {
        /// Instructions to remediate violation
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Instructions {
            /// Remediation instructions to resolve violation via gcloud cli
            #[prost(message, optional, tag = "1")]
            pub gcloud_instructions: ::core::option::Option<instructions::Gcloud>,
            /// Remediation instructions to resolve violation via cloud console
            #[prost(message, optional, tag = "2")]
            pub console_instructions: ::core::option::Option<instructions::Console>,
        }
        /// Nested message and enum types in `Instructions`.
        pub mod instructions {
            /// Remediation instructions to resolve violation via gcloud cli
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Gcloud {
                /// Gcloud command to resolve violation
                #[prost(string, repeated, tag = "1")]
                pub gcloud_commands: ::prost::alloc::vec::Vec<
                    ::prost::alloc::string::String,
                >,
                /// Steps to resolve violation via gcloud cli
                #[prost(string, repeated, tag = "2")]
                pub steps: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
                /// Additional urls for more information about steps
                #[prost(string, repeated, tag = "3")]
                pub additional_links: ::prost::alloc::vec::Vec<
                    ::prost::alloc::string::String,
                >,
            }
            /// Remediation instructions to resolve violation via cloud console
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Console {
                /// Link to console page where violations can be resolved
                #[prost(string, repeated, tag = "1")]
                pub console_uris: ::prost::alloc::vec::Vec<
                    ::prost::alloc::string::String,
                >,
                /// Steps to resolve violation via cloud console
                #[prost(string, repeated, tag = "2")]
                pub steps: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
                /// Additional urls for more information about steps
                #[prost(string, repeated, tag = "3")]
                pub additional_links: ::prost::alloc::vec::Vec<
                    ::prost::alloc::string::String,
                >,
            }
        }
        /// Classifying remediation into various types based on the kind of
        /// violation. For example, violations caused due to changes in boolean org
        /// policy requires different remediation instructions compared to violation
        /// caused due to changes in allowed values of list org policy.
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
        pub enum RemediationType {
            /// Unspecified remediation type
            Unspecified = 0,
            /// Remediation type for boolean org policy
            RemediationBooleanOrgPolicyViolation = 1,
            /// Remediation type for list org policy which have allowed values in the
            /// monitoring rule
            RemediationListAllowedValuesOrgPolicyViolation = 2,
            /// Remediation type for list org policy which have denied values in the
            /// monitoring rule
            RemediationListDeniedValuesOrgPolicyViolation = 3,
            /// Remediation type for gcp.restrictCmekCryptoKeyProjects
            RemediationRestrictCmekCryptoKeyProjectsOrgPolicyViolation = 4,
            /// Remediation type for resource violation.
            RemediationResourceViolation = 5,
        }
        impl RemediationType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    RemediationType::Unspecified => "REMEDIATION_TYPE_UNSPECIFIED",
                    RemediationType::RemediationBooleanOrgPolicyViolation => {
                        "REMEDIATION_BOOLEAN_ORG_POLICY_VIOLATION"
                    }
                    RemediationType::RemediationListAllowedValuesOrgPolicyViolation => {
                        "REMEDIATION_LIST_ALLOWED_VALUES_ORG_POLICY_VIOLATION"
                    }
                    RemediationType::RemediationListDeniedValuesOrgPolicyViolation => {
                        "REMEDIATION_LIST_DENIED_VALUES_ORG_POLICY_VIOLATION"
                    }
                    RemediationType::RemediationRestrictCmekCryptoKeyProjectsOrgPolicyViolation => {
                        "REMEDIATION_RESTRICT_CMEK_CRYPTO_KEY_PROJECTS_ORG_POLICY_VIOLATION"
                    }
                    RemediationType::RemediationResourceViolation => {
                        "REMEDIATION_RESOURCE_VIOLATION"
                    }
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "REMEDIATION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                    "REMEDIATION_BOOLEAN_ORG_POLICY_VIOLATION" => {
                        Some(Self::RemediationBooleanOrgPolicyViolation)
                    }
                    "REMEDIATION_LIST_ALLOWED_VALUES_ORG_POLICY_VIOLATION" => {
                        Some(Self::RemediationListAllowedValuesOrgPolicyViolation)
                    }
                    "REMEDIATION_LIST_DENIED_VALUES_ORG_POLICY_VIOLATION" => {
                        Some(Self::RemediationListDeniedValuesOrgPolicyViolation)
                    }
                    "REMEDIATION_RESTRICT_CMEK_CRYPTO_KEY_PROJECTS_ORG_POLICY_VIOLATION" => {
                        Some(
                            Self::RemediationRestrictCmekCryptoKeyProjectsOrgPolicyViolation,
                        )
                    }
                    "REMEDIATION_RESOURCE_VIOLATION" => {
                        Some(Self::RemediationResourceViolation)
                    }
                    _ => None,
                }
            }
        }
    }
    /// Violation State Values
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
        /// Unspecified state.
        Unspecified = 0,
        /// Violation is resolved.
        Resolved = 1,
        /// Violation is Unresolved
        Unresolved = 2,
        /// Violation is Exception
        Exception = 3,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Resolved => "RESOLVED",
                State::Unresolved => "UNRESOLVED",
                State::Exception => "EXCEPTION",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "RESOLVED" => Some(Self::Resolved),
                "UNRESOLVED" => Some(Self::Unresolved),
                "EXCEPTION" => Some(Self::Exception),
                _ => None,
            }
        }
    }
}
/// Message for requesting list of Violations
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListViolationsRequest {
    /// Required. Parent resource
    /// Format
    /// organizations/{organization}/locations/{location}/customers/{customer}/workloads/{workload}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of customers row to return. The service may
    /// return fewer than this value. If unspecified, at most 10 customers will be
    /// returned.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListViolations` call.
    /// Provide this to retrieve the subsequent page.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filtering results
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Hint for how to order the results
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
    /// Optional. Specifies the interval for retrieving violations.
    /// if unspecified, all violations will be returned.
    #[prost(message, optional, tag = "6")]
    pub interval: ::core::option::Option<super::super::super::r#type::Interval>,
}
/// Response message for list customer violation requests
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListViolationsResponse {
    /// List of violation
    #[prost(message, repeated, tag = "1")]
    pub violations: ::prost::alloc::vec::Vec<Violation>,
    /// A token that can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Workloads that could not be reached due to permission errors or any other
    /// error. Ref: <https://google.aip.dev/217>
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for getting a Violation
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetViolationRequest {
    /// Required. Format:
    /// organizations/{organization}/locations/{location}/customers/{customer}/workloads/{workload}/violations/{violation}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod cloud_controls_partner_monitoring_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service describing handlers for resources
    #[derive(Debug, Clone)]
    pub struct CloudControlsPartnerMonitoringClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CloudControlsPartnerMonitoringClient<tonic::transport::Channel> {
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
    impl<T> CloudControlsPartnerMonitoringClient<T>
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
        ) -> CloudControlsPartnerMonitoringClient<InterceptedService<T, F>>
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
            CloudControlsPartnerMonitoringClient::new(
                InterceptedService::new(inner, interceptor),
            )
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
        /// Lists Violations for a workload
        /// Callers may also choose to read across multiple Customers or for a single
        /// customer as per
        /// [AIP-159](https://google.aip.dev/159) by using '-' (the hyphen or dash
        /// character) as a wildcard character instead of {customer} & {workload}.
        /// Format:
        /// `organizations/{organization}/locations/{location}/customers/{customer}/workloads/{workload}`
        pub async fn list_violations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListViolationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListViolationsResponse>,
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
                "/google.cloud.cloudcontrolspartner.v1beta.CloudControlsPartnerMonitoring/ListViolations",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.cloudcontrolspartner.v1beta.CloudControlsPartnerMonitoring",
                        "ListViolations",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single Violation.
        pub async fn get_violation(
            &mut self,
            request: impl tonic::IntoRequest<super::GetViolationRequest>,
        ) -> std::result::Result<tonic::Response<super::Violation>, tonic::Status> {
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
                "/google.cloud.cloudcontrolspartner.v1beta.CloudControlsPartnerMonitoring/GetViolation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.cloudcontrolspartner.v1beta.CloudControlsPartnerMonitoring",
                        "GetViolation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}