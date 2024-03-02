/// Consumer provided attributes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Attributes {
    /// Optional. User-defined criticality information.
    #[prost(message, optional, tag = "1")]
    pub criticality: ::core::option::Option<Criticality>,
    /// Optional. User-defined environment information.
    #[prost(message, optional, tag = "2")]
    pub environment: ::core::option::Option<Environment>,
    /// Optional. Developer team that owns development and coding.
    #[prost(message, repeated, tag = "3")]
    pub developer_owners: ::prost::alloc::vec::Vec<ContactInfo>,
    /// Optional. Operator team that ensures runtime and operations.
    #[prost(message, repeated, tag = "4")]
    pub operator_owners: ::prost::alloc::vec::Vec<ContactInfo>,
    /// Optional. Business team that ensures user needs are met and value is
    /// delivered
    #[prost(message, repeated, tag = "5")]
    pub business_owners: ::prost::alloc::vec::Vec<ContactInfo>,
}
/// Criticality of the Application, Service, or Workload
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Criticality {
    /// Required. Criticality Type.
    #[prost(enumeration = "criticality::Type", tag = "3")]
    pub r#type: i32,
}
/// Nested message and enum types in `Criticality`.
pub mod criticality {
    /// Criticality Type.
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
        /// Unspecified type.
        Unspecified = 0,
        /// Mission critical service, application or workload.
        MissionCritical = 1,
        /// High impact.
        High = 2,
        /// Medium impact.
        Medium = 3,
        /// Low impact.
        Low = 4,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::MissionCritical => "MISSION_CRITICAL",
                Type::High => "HIGH",
                Type::Medium => "MEDIUM",
                Type::Low => "LOW",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "MISSION_CRITICAL" => Some(Self::MissionCritical),
                "HIGH" => Some(Self::High),
                "MEDIUM" => Some(Self::Medium),
                "LOW" => Some(Self::Low),
                _ => None,
            }
        }
    }
}
/// Environment of the Application, Service, or Workload
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Environment {
    /// Required. Environment Type.
    #[prost(enumeration = "environment::Type", tag = "2")]
    pub r#type: i32,
}
/// Nested message and enum types in `Environment`.
pub mod environment {
    /// Environment Type.
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
        /// Unspecified type.
        Unspecified = 0,
        /// Production environment.
        Production = 1,
        /// Staging environment.
        Staging = 2,
        /// Test environment.
        Test = 3,
        /// Development environment.
        Development = 4,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::Production => "PRODUCTION",
                Type::Staging => "STAGING",
                Type::Test => "TEST",
                Type::Development => "DEVELOPMENT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "PRODUCTION" => Some(Self::Production),
                "STAGING" => Some(Self::Staging),
                "TEST" => Some(Self::Test),
                "DEVELOPMENT" => Some(Self::Development),
                _ => None,
            }
        }
    }
}
/// Contact information of stakeholders.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContactInfo {
    /// Optional. Contact's name.
    /// Can have a maximum length of 63 characters.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// Required. Email address of the contacts.
    #[prost(string, tag = "2")]
    pub email: ::prost::alloc::string::String,
}
/// Application defines the governance boundary for App Hub Entities that
/// perform a logical end-to-end business function.
/// App Hub supports application level IAM permission to align with governance
/// requirements.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Application {
    /// Identifier. The resource name of an Application. Format:
    /// "projects/{host-project-id}/locations/{location}/applications/{application-id}"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. User-defined name for the Application.
    /// Can have a maximum length of 63 characters.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. User-defined description of an Application.
    /// Can have a maximum length of 2048 characters.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Optional. Consumer provided attributes.
    #[prost(message, optional, tag = "4")]
    pub attributes: ::core::option::Option<Attributes>,
    /// Output only. Create time.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Update time.
    #[prost(message, optional, tag = "6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. Immutable. Defines what data can be included into this
    /// Application. Limits which Services and Workloads can be registered.
    #[prost(message, optional, tag = "9")]
    pub scope: ::core::option::Option<Scope>,
    /// Output only. A universally unique identifier (in UUID4 format) for the
    /// `Application`.
    #[prost(string, tag = "10")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. Application state.
    #[prost(enumeration = "application::State", tag = "11")]
    pub state: i32,
}
/// Nested message and enum types in `Application`.
pub mod application {
    /// Application state.
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
        /// The Application is being created.
        Creating = 1,
        /// The Application is ready to register Services and Workloads.
        Active = 2,
        /// The Application is being deleted.
        Deleting = 3,
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
                State::Active => "ACTIVE",
                State::Deleting => "DELETING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "ACTIVE" => Some(Self::Active),
                "DELETING" => Some(Self::Deleting),
                _ => None,
            }
        }
    }
}
/// Scope of an application.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Scope {
    /// Required. Scope Type.
    #[prost(enumeration = "scope::Type", tag = "1")]
    pub r#type: i32,
}
/// Nested message and enum types in `Scope`.
pub mod scope {
    /// Scope Type.
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
        /// Unspecified type.
        Unspecified = 0,
        /// Regional type.
        Regional = 1,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::Regional => "REGIONAL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "REGIONAL" => Some(Self::Regional),
                _ => None,
            }
        }
    }
}
/// Service is an App Hub data model that contains a discovered service, which
/// represents a network/api interface that exposes some functionality to clients
/// for consumption over the network.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Service {
    /// Identifier. The resource name of a Service. Format:
    /// "projects/{host-project-id}/locations/{location}/applications/{application-id}/services/{service-id}"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. User-defined name for the Service.
    /// Can have a maximum length of 63 characters.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. User-defined description of a Service.
    /// Can have a maximum length of 2048 characters.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Reference to an underlying networking resource that can
    /// comprise a Service. These are immutable.
    #[prost(message, optional, tag = "4")]
    pub service_reference: ::core::option::Option<ServiceReference>,
    /// Output only. Properties of an underlying compute resource that can comprise
    /// a Service. These are immutable.
    #[prost(message, optional, tag = "5")]
    pub service_properties: ::core::option::Option<ServiceProperties>,
    /// Optional. Consumer provided attributes.
    #[prost(message, optional, tag = "6")]
    pub attributes: ::core::option::Option<Attributes>,
    /// Required. Immutable. The resource name of the original discovered service.
    #[prost(string, tag = "7")]
    pub discovered_service: ::prost::alloc::string::String,
    /// Output only. Create time.
    #[prost(message, optional, tag = "8")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Update time.
    #[prost(message, optional, tag = "9")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. A universally unique identifier (UUID) for the `Service` in
    /// the UUID4 format.
    #[prost(string, tag = "10")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. Service state.
    #[prost(enumeration = "service::State", tag = "11")]
    pub state: i32,
}
/// Nested message and enum types in `Service`.
pub mod service {
    /// Service state.
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
        /// The service is being created.
        Creating = 1,
        /// The service is ready.
        Active = 2,
        /// The service is being deleted.
        Deleting = 3,
        /// The underlying networking resources have been deleted.
        Detached = 4,
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
                State::Active => "ACTIVE",
                State::Deleting => "DELETING",
                State::Detached => "DETACHED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "ACTIVE" => Some(Self::Active),
                "DELETING" => Some(Self::Deleting),
                "DETACHED" => Some(Self::Detached),
                _ => None,
            }
        }
    }
}
/// Reference to an underlying networking resource that can comprise a Service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceReference {
    /// Output only. The underlying resource URI (For example, URI of Forwarding
    /// Rule, URL Map, and Backend Service).
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
}
/// Properties of an underlying cloud resource that can comprise a Service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceProperties {
    /// Output only. The service project identifier that the underlying cloud
    /// resource resides in.
    #[prost(string, tag = "1")]
    pub gcp_project: ::prost::alloc::string::String,
    /// Output only. The location that the underlying resource resides in, for
    /// example, us-west1.
    #[prost(string, tag = "2")]
    pub location: ::prost::alloc::string::String,
    /// Output only. The location that the underlying resource resides in if it is
    /// zonal, for example, us-west1-a).
    #[prost(string, tag = "3")]
    pub zone: ::prost::alloc::string::String,
}
/// DiscoveredService is a network/api interface that exposes some functionality
/// to clients for consumption over the network. A discovered service can be
/// registered to a App Hub service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiscoveredService {
    /// Identifier. The resource name of the discovered service. Format:
    /// "projects/{host-project-id}/locations/{location}/discoveredServices/{uuid}""
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Reference to an underlying networking resource that can
    /// comprise a Service. These are immutable.
    #[prost(message, optional, tag = "2")]
    pub service_reference: ::core::option::Option<ServiceReference>,
    /// Output only. Properties of an underlying compute resource that can comprise
    /// a Service. These are immutable.
    #[prost(message, optional, tag = "3")]
    pub service_properties: ::core::option::Option<ServiceProperties>,
}
/// ServiceProjectAttachment represents an attachment from a service project to a
/// host project. Service projects contain the underlying cloud
/// infrastructure resources, and expose these resources to the host project
/// through a ServiceProjectAttachment. With the attachments, the host project
/// can provide an aggregated view of resources across all service projects.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceProjectAttachment {
    /// Identifier. The resource name of a ServiceProjectAttachment. Format:
    /// "projects/{host-project-id}/locations/global/serviceProjectAttachments/{service-project-id}."
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Immutable. Service project name in the format: "projects/abc" or
    /// "projects/123". As input, project name with either project id or number are
    /// accepted. As output, this field will contain project number.
    #[prost(string, tag = "2")]
    pub service_project: ::prost::alloc::string::String,
    /// Output only. Create time.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. A globally unique identifier (in UUID4 format) for the
    /// `ServiceProjectAttachment`.
    #[prost(string, tag = "4")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. ServiceProjectAttachment state.
    #[prost(enumeration = "service_project_attachment::State", tag = "5")]
    pub state: i32,
}
/// Nested message and enum types in `ServiceProjectAttachment`.
pub mod service_project_attachment {
    /// ServiceProjectAttachment state.
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
        /// The ServiceProjectAttachment is being created.
        Creating = 1,
        /// The ServiceProjectAttachment is ready.
        /// This means Services and Workloads under the corresponding
        /// ServiceProjectAttachment is ready for registration.
        Active = 2,
        /// The ServiceProjectAttachment is being deleted.
        Deleting = 3,
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
                State::Active => "ACTIVE",
                State::Deleting => "DELETING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "ACTIVE" => Some(Self::Active),
                "DELETING" => Some(Self::Deleting),
                _ => None,
            }
        }
    }
}
/// Workload is an App Hub data model that contains a discovered workload, which
/// represents a binary deployment (such as managed instance groups (MIGs) and
/// GKE deployments) that performs the smallest logical subset of business
/// functionality.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Workload {
    /// Identifier. The resource name of the Workload. Format:
    /// "projects/{host-project-id}/locations/{location}/applications/{application-id}/workloads/{workload-id}"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. User-defined name for the Workload.
    /// Can have a maximum length of 63 characters.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. User-defined description of a Workload.
    /// Can have a maximum length of 2048 characters.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Reference of an underlying compute resource represented by the
    /// Workload. These are immutable.
    #[prost(message, optional, tag = "4")]
    pub workload_reference: ::core::option::Option<WorkloadReference>,
    /// Output only. Properties of an underlying compute resource represented by
    /// the Workload. These are immutable.
    #[prost(message, optional, tag = "5")]
    pub workload_properties: ::core::option::Option<WorkloadProperties>,
    /// Required. Immutable. The resource name of the original discovered workload.
    #[prost(string, tag = "6")]
    pub discovered_workload: ::prost::alloc::string::String,
    /// Optional. Consumer provided attributes.
    #[prost(message, optional, tag = "7")]
    pub attributes: ::core::option::Option<Attributes>,
    /// Output only. Create time.
    #[prost(message, optional, tag = "8")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Update time.
    #[prost(message, optional, tag = "9")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. A universally unique identifier (UUID) for the `Workload` in
    /// the UUID4 format.
    #[prost(string, tag = "10")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. Workload state.
    #[prost(enumeration = "workload::State", tag = "11")]
    pub state: i32,
}
/// Nested message and enum types in `Workload`.
pub mod workload {
    /// Workload state.
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
        /// The Workload is being created.
        Creating = 1,
        /// The Workload is ready.
        Active = 2,
        /// The Workload is being deleted.
        Deleting = 3,
        /// The underlying compute resources have been deleted.
        Detached = 4,
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
                State::Active => "ACTIVE",
                State::Deleting => "DELETING",
                State::Detached => "DETACHED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "ACTIVE" => Some(Self::Active),
                "DELETING" => Some(Self::Deleting),
                "DETACHED" => Some(Self::Detached),
                _ => None,
            }
        }
    }
}
/// Reference of an underlying compute resource represented by the Workload.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkloadReference {
    /// Output only. The underlying compute resource uri.
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
}
/// Properties of an underlying compute resource represented by the Workload.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkloadProperties {
    /// Output only. The service project identifier that the underlying cloud
    /// resource resides in. Empty for non cloud resources.
    #[prost(string, tag = "1")]
    pub gcp_project: ::prost::alloc::string::String,
    /// Output only. The location that the underlying compute resource resides in
    /// (e.g us-west1).
    #[prost(string, tag = "2")]
    pub location: ::prost::alloc::string::String,
    /// Output only. The location that the underlying compute resource resides in
    /// if it is zonal (e.g us-west1-a).
    #[prost(string, tag = "3")]
    pub zone: ::prost::alloc::string::String,
}
/// DiscoveredWorkload is a binary deployment (such as managed instance groups
/// (MIGs) and GKE deployments) that performs the smallest logical subset of
/// business functionality. A discovered workload can be registered to an App Hub
/// Workload.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiscoveredWorkload {
    /// Identifier. The resource name of the discovered workload. Format:
    /// "projects/{host-project-id}/locations/{location}/discoveredWorkloads/{uuid}"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Reference of an underlying compute resource represented by the
    /// Workload. These are immutable.
    #[prost(message, optional, tag = "2")]
    pub workload_reference: ::core::option::Option<WorkloadReference>,
    /// Output only. Properties of an underlying compute resource represented by
    /// the Workload. These are immutable.
    #[prost(message, optional, tag = "3")]
    pub workload_properties: ::core::option::Option<WorkloadProperties>,
}
/// Request for LookupServiceProjectAttachment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LookupServiceProjectAttachmentRequest {
    /// Required. Value for name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Response for LookupServiceProjectAttachment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LookupServiceProjectAttachmentResponse {
    /// Service project attachment for a project if exists, empty otherwise.
    #[prost(message, optional, tag = "1")]
    pub service_project_attachment: ::core::option::Option<ServiceProjectAttachment>,
}
/// Request for ListServiceProjectAttachments.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServiceProjectAttachmentsRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server may return fewer items than
    /// requested. If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filtering results
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Hint for how to order the results
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response for ListServiceProjectAttachments.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServiceProjectAttachmentsResponse {
    /// List of service project attachments.
    #[prost(message, repeated, tag = "1")]
    pub service_project_attachments: ::prost::alloc::vec::Vec<ServiceProjectAttachment>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for CreateServiceProjectAttachment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateServiceProjectAttachmentRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The service project attachment identifier must contain the
    /// project_id of the service project specified in the
    /// service_project_attachment.service_project field. Hint:
    /// "projects/{project_id}"
    #[prost(string, tag = "2")]
    pub service_project_attachment_id: ::prost::alloc::string::String,
    /// Required. The resource being created.
    #[prost(message, optional, tag = "3")]
    pub service_project_attachment: ::core::option::Option<ServiceProjectAttachment>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request for GetServiceProjectAttachment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServiceProjectAttachmentRequest {
    /// Required. Value for name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for DeleteServiceProjectAttachment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteServiceProjectAttachmentRequest {
    /// Required. Value for name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request for DetachServiceProjectAttachment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetachServiceProjectAttachmentRequest {
    /// Required. Value for name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Response for DetachServiceProjectAttachment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetachServiceProjectAttachmentResponse {}
/// Request for ListServices.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServicesRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server may return fewer items than
    /// requested. If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filtering results
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Hint for how to order the results
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response for ListServices.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServicesResponse {
    /// List of Services.
    #[prost(message, repeated, tag = "1")]
    pub services: ::prost::alloc::vec::Vec<Service>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for ListDiscoveredServices.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDiscoveredServicesRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server may return fewer items than
    /// requested. If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filtering results
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Hint for how to order the results
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response for ListDiscoveredServices.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDiscoveredServicesResponse {
    /// List of discovered services.
    #[prost(message, repeated, tag = "1")]
    pub discovered_services: ::prost::alloc::vec::Vec<DiscoveredService>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for CreateService.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateServiceRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The Service identifier.
    /// Must contain only lowercase letters, numbers
    /// or hyphens, with the first character a letter, the last a letter or a
    /// number, and a 63 character maximum.
    #[prost(string, tag = "2")]
    pub service_id: ::prost::alloc::string::String,
    /// Required. The resource being created.
    #[prost(message, optional, tag = "3")]
    pub service: ::core::option::Option<Service>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request for GetService.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServiceRequest {
    /// Required. Value for name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for GetDiscoveredService.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDiscoveredServiceRequest {
    /// Required. Value for name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for LookupDiscoveredService.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LookupDiscoveredServiceRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. GCP resource URI to find service for
    /// Accepts both project number and project id and does translation when
    /// needed.
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
}
/// Response for LookupDiscoveredService.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LookupDiscoveredServiceResponse {
    /// Discovered service if exists, empty otherwise.
    #[prost(message, optional, tag = "1")]
    pub discovered_service: ::core::option::Option<DiscoveredService>,
}
/// Request for UpdateService.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateServiceRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// Service resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request.
    /// The API changes the values of the fields as specified in the update_mask.
    /// The API ignores the values of all fields not covered by the update_mask.
    /// You can also unset a field by not specifying it in the updated message, but
    /// adding the field to the mask. This clears whatever value the field
    /// previously had.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The resource being updated.
    #[prost(message, optional, tag = "2")]
    pub service: ::core::option::Option<Service>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request for DeleteService.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteServiceRequest {
    /// Required. Value for name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request for ListApplications.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListApplicationsRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server may return fewer items than
    /// requested. If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filtering results
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Hint for how to order the results
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response for ListApplications.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListApplicationsResponse {
    /// List of Applications.
    #[prost(message, repeated, tag = "1")]
    pub applications: ::prost::alloc::vec::Vec<Application>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for CreateApplication.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateApplicationRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The Application identifier.
    /// Must contain only lowercase letters, numbers
    /// or hyphens, with the first character a letter, the last a letter or a
    /// number, and a 63 character maximum.
    #[prost(string, tag = "2")]
    pub application_id: ::prost::alloc::string::String,
    /// Required. The resource being created
    #[prost(message, optional, tag = "3")]
    pub application: ::core::option::Option<Application>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request for GetApplication.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetApplicationRequest {
    /// Required. Value for name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for UpdateApplication.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateApplicationRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// Application resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request.
    /// The API changes the values of the fields as specified in the update_mask.
    /// The API ignores the values of all fields not covered by the update_mask.
    /// You can also unset a field by not specifying it in the updated message, but
    /// adding the field to the mask. This clears whatever value the field
    /// previously had.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The resource being updated.
    #[prost(message, optional, tag = "2")]
    pub application: ::core::option::Option<Application>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request for DeleteApplication.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteApplicationRequest {
    /// Required. Value for name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request for ListWorkloads.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkloadsRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server may return fewer items than
    /// requested. If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filtering results
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Hint for how to order the results
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response for ListWorkloads.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkloadsResponse {
    /// List of Workloads.
    #[prost(message, repeated, tag = "1")]
    pub workloads: ::prost::alloc::vec::Vec<Workload>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for ListDiscoveredWorkloads.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDiscoveredWorkloadsRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server may return fewer items than
    /// requested. If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filtering results
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Hint for how to order the results
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response for ListDiscoveredWorkloads.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDiscoveredWorkloadsResponse {
    /// List of discovered workloads.
    #[prost(message, repeated, tag = "1")]
    pub discovered_workloads: ::prost::alloc::vec::Vec<DiscoveredWorkload>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for CreateWorkload.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWorkloadRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The Workload identifier.
    /// Must contain only lowercase letters, numbers
    /// or hyphens, with the first character a letter, the last a letter or a
    /// number, and a 63 character maximum.
    #[prost(string, tag = "2")]
    pub workload_id: ::prost::alloc::string::String,
    /// Required. The resource being created.
    #[prost(message, optional, tag = "3")]
    pub workload: ::core::option::Option<Workload>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request for GetWorkload.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWorkloadRequest {
    /// Required. Value for name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for GetDiscoveredWorkload.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDiscoveredWorkloadRequest {
    /// Required. Value for name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for LookupDiscoveredWorkload.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LookupDiscoveredWorkloadRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. GCP resource URI to find workload for.
    /// Accepts both project number and project id and does translation when
    /// needed.
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
}
/// Response for LookupDiscoveredWorkload.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LookupDiscoveredWorkloadResponse {
    /// Discovered workload if exists, empty otherwise.
    #[prost(message, optional, tag = "1")]
    pub discovered_workload: ::core::option::Option<DiscoveredWorkload>,
}
/// Request for UpdateWorkload.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateWorkloadRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// Workload resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request.
    /// The API changes the values of the fields as specified in the update_mask.
    /// The API ignores the values of all fields not covered by the update_mask.
    /// You can also unset a field by not specifying it in the updated message, but
    /// adding the field to the mask. This clears whatever value the field
    /// previously had.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The resource being updated.
    #[prost(message, optional, tag = "2")]
    pub workload: ::core::option::Option<Workload>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request for DeleteWorkload.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteWorkloadRequest {
    /// Required. Value for name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
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
pub mod app_hub_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// The AppHub services allows users to enable toplogy and telemetry
    /// configuration.
    #[derive(Debug, Clone)]
    pub struct AppHubClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AppHubClient<tonic::transport::Channel> {
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
    impl<T> AppHubClient<T>
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
        ) -> AppHubClient<InterceptedService<T, F>>
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
            AppHubClient::new(InterceptedService::new(inner, interceptor))
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
        /// Looks up a service project attachment. You can call this API from either a
        /// host or service project.
        pub async fn lookup_service_project_attachment(
            &mut self,
            request: impl tonic::IntoRequest<
                super::LookupServiceProjectAttachmentRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::LookupServiceProjectAttachmentResponse>,
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
                "/google.cloud.apphub.v1.AppHub/LookupServiceProjectAttachment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.apphub.v1.AppHub",
                        "LookupServiceProjectAttachment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List service projects attached to the host project.
        pub async fn list_service_project_attachments(
            &mut self,
            request: impl tonic::IntoRequest<super::ListServiceProjectAttachmentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListServiceProjectAttachmentsResponse>,
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
                "/google.cloud.apphub.v1.AppHub/ListServiceProjectAttachments",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.apphub.v1.AppHub",
                        "ListServiceProjectAttachments",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Attaches a service project to the host project.
        pub async fn create_service_project_attachment(
            &mut self,
            request: impl tonic::IntoRequest<
                super::CreateServiceProjectAttachmentRequest,
            >,
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
                "/google.cloud.apphub.v1.AppHub/CreateServiceProjectAttachment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.apphub.v1.AppHub",
                        "CreateServiceProjectAttachment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a service project attached to the host project.
        pub async fn get_service_project_attachment(
            &mut self,
            request: impl tonic::IntoRequest<super::GetServiceProjectAttachmentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ServiceProjectAttachment>,
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
                "/google.cloud.apphub.v1.AppHub/GetServiceProjectAttachment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.apphub.v1.AppHub",
                        "GetServiceProjectAttachment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a service project attached to the host project.
        pub async fn delete_service_project_attachment(
            &mut self,
            request: impl tonic::IntoRequest<
                super::DeleteServiceProjectAttachmentRequest,
            >,
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
                "/google.cloud.apphub.v1.AppHub/DeleteServiceProjectAttachment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.apphub.v1.AppHub",
                        "DeleteServiceProjectAttachment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Detaches a service project from a host project. You can call this API from
        /// either a host or service project.
        pub async fn detach_service_project_attachment(
            &mut self,
            request: impl tonic::IntoRequest<
                super::DetachServiceProjectAttachmentRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::DetachServiceProjectAttachmentResponse>,
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
                "/google.cloud.apphub.v1.AppHub/DetachServiceProjectAttachment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.apphub.v1.AppHub",
                        "DetachServiceProjectAttachment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists discovered services that can be added to an application in a host
        /// project and location.
        pub async fn list_discovered_services(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDiscoveredServicesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDiscoveredServicesResponse>,
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
                "/google.cloud.apphub.v1.AppHub/ListDiscoveredServices",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.apphub.v1.AppHub",
                        "ListDiscoveredServices",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a discovered service in a host project and location.
        pub async fn get_discovered_service(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDiscoveredServiceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DiscoveredService>,
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
                "/google.cloud.apphub.v1.AppHub/GetDiscoveredService",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.apphub.v1.AppHub",
                        "GetDiscoveredService",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Looks up a discovered service in a host project and location and with a
        /// given resource URI.
        pub async fn lookup_discovered_service(
            &mut self,
            request: impl tonic::IntoRequest<super::LookupDiscoveredServiceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::LookupDiscoveredServiceResponse>,
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
                "/google.cloud.apphub.v1.AppHub/LookupDiscoveredService",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.apphub.v1.AppHub",
                        "LookupDiscoveredService",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List Services in an Application.
        pub async fn list_services(
            &mut self,
            request: impl tonic::IntoRequest<super::ListServicesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListServicesResponse>,
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
                "/google.cloud.apphub.v1.AppHub/ListServices",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.apphub.v1.AppHub", "ListServices"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a Service in an Application.
        pub async fn create_service(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateServiceRequest>,
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
                "/google.cloud.apphub.v1.AppHub/CreateService",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.apphub.v1.AppHub", "CreateService"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a Service in an Application.
        pub async fn get_service(
            &mut self,
            request: impl tonic::IntoRequest<super::GetServiceRequest>,
        ) -> std::result::Result<tonic::Response<super::Service>, tonic::Status> {
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
                "/google.cloud.apphub.v1.AppHub/GetService",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.apphub.v1.AppHub", "GetService"));
            self.inner.unary(req, path, codec).await
        }
        /// Updates a Service in an Application.
        pub async fn update_service(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateServiceRequest>,
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
                "/google.cloud.apphub.v1.AppHub/UpdateService",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.apphub.v1.AppHub", "UpdateService"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a Service in an Application.
        pub async fn delete_service(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteServiceRequest>,
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
                "/google.cloud.apphub.v1.AppHub/DeleteService",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.apphub.v1.AppHub", "DeleteService"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists discovered workloads that can be added to an application in a host
        /// project and location.
        pub async fn list_discovered_workloads(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDiscoveredWorkloadsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDiscoveredWorkloadsResponse>,
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
                "/google.cloud.apphub.v1.AppHub/ListDiscoveredWorkloads",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.apphub.v1.AppHub",
                        "ListDiscoveredWorkloads",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a discovered workload in a host project and location.
        pub async fn get_discovered_workload(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDiscoveredWorkloadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DiscoveredWorkload>,
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
                "/google.cloud.apphub.v1.AppHub/GetDiscoveredWorkload",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.apphub.v1.AppHub",
                        "GetDiscoveredWorkload",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Looks up a discovered Workload in a host project and location and with a
        /// given resource URI.
        pub async fn lookup_discovered_workload(
            &mut self,
            request: impl tonic::IntoRequest<super::LookupDiscoveredWorkloadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::LookupDiscoveredWorkloadResponse>,
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
                "/google.cloud.apphub.v1.AppHub/LookupDiscoveredWorkload",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.apphub.v1.AppHub",
                        "LookupDiscoveredWorkload",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists Workloads in an Application.
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
                "/google.cloud.apphub.v1.AppHub/ListWorkloads",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.apphub.v1.AppHub", "ListWorkloads"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a Workload in an Application.
        pub async fn create_workload(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateWorkloadRequest>,
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
                "/google.cloud.apphub.v1.AppHub/CreateWorkload",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.apphub.v1.AppHub", "CreateWorkload"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a Workload in an Application.
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
                "/google.cloud.apphub.v1.AppHub/GetWorkload",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.apphub.v1.AppHub", "GetWorkload"));
            self.inner.unary(req, path, codec).await
        }
        /// Updates a Workload in an Application.
        pub async fn update_workload(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateWorkloadRequest>,
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
                "/google.cloud.apphub.v1.AppHub/UpdateWorkload",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.apphub.v1.AppHub", "UpdateWorkload"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a Workload in an Application.
        pub async fn delete_workload(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteWorkloadRequest>,
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
                "/google.cloud.apphub.v1.AppHub/DeleteWorkload",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.apphub.v1.AppHub", "DeleteWorkload"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists Applications in a host project and location.
        pub async fn list_applications(
            &mut self,
            request: impl tonic::IntoRequest<super::ListApplicationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListApplicationsResponse>,
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
                "/google.cloud.apphub.v1.AppHub/ListApplications",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.apphub.v1.AppHub", "ListApplications"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates an Application in a host project and location.
        pub async fn create_application(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateApplicationRequest>,
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
                "/google.cloud.apphub.v1.AppHub/CreateApplication",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.apphub.v1.AppHub", "CreateApplication"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets an Application in a host project and location.
        pub async fn get_application(
            &mut self,
            request: impl tonic::IntoRequest<super::GetApplicationRequest>,
        ) -> std::result::Result<tonic::Response<super::Application>, tonic::Status> {
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
                "/google.cloud.apphub.v1.AppHub/GetApplication",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.apphub.v1.AppHub", "GetApplication"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates an Application in a host project and location.
        pub async fn update_application(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateApplicationRequest>,
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
                "/google.cloud.apphub.v1.AppHub/UpdateApplication",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.apphub.v1.AppHub", "UpdateApplication"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes an Application in a host project and location.
        pub async fn delete_application(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteApplicationRequest>,
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
                "/google.cloud.apphub.v1.AppHub/DeleteApplication",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.apphub.v1.AppHub", "DeleteApplication"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
