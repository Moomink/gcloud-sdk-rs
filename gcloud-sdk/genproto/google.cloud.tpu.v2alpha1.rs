/// A guest attributes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuestAttributes {
    /// The path to be queried. This can be the default namespace ('/') or a
    /// nested namespace ('/\<namespace\>/') or a specified key
    /// ('/\<namespace\>/\<key\>')
    #[prost(string, tag = "1")]
    pub query_path: ::prost::alloc::string::String,
    /// The value of the requested queried path.
    #[prost(message, optional, tag = "2")]
    pub query_value: ::core::option::Option<GuestAttributesValue>,
}
/// Array of guest attribute namespace/key/value tuples.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuestAttributesValue {
    /// The list of guest attributes entries.
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<GuestAttributesEntry>,
}
/// A guest attributes namespace/key/value entry.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuestAttributesEntry {
    /// Namespace for the guest attribute entry.
    #[prost(string, tag = "1")]
    pub namespace: ::prost::alloc::string::String,
    /// Key for the guest attribute entry.
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
    /// Value for the guest attribute entry.
    #[prost(string, tag = "3")]
    pub value: ::prost::alloc::string::String,
}
/// A node-attached disk resource.
/// Next ID: 8;
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttachedDisk {
    /// Specifies the full path to an existing disk.
    /// For example: "projects/my-project/zones/us-central1-c/disks/my-disk".
    #[prost(string, tag = "3")]
    pub source_disk: ::prost::alloc::string::String,
    /// The mode in which to attach this disk.
    /// If not specified, the default is READ_WRITE mode.
    /// Only applicable to data_disks.
    #[prost(enumeration = "attached_disk::DiskMode", tag = "4")]
    pub mode: i32,
}
/// Nested message and enum types in `AttachedDisk`.
pub mod attached_disk {
    /// The different mode of the attached disk.
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
    pub enum DiskMode {
        /// The disk mode is not known/set.
        Unspecified = 0,
        /// Attaches the disk in read-write mode. Only one TPU node can attach a disk
        /// in read-write mode at a time.
        ReadWrite = 1,
        /// Attaches the disk in read-only mode. Multiple TPU nodes can attach
        /// a disk in read-only mode at a time.
        ReadOnly = 2,
    }
    impl DiskMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DiskMode::Unspecified => "DISK_MODE_UNSPECIFIED",
                DiskMode::ReadWrite => "READ_WRITE",
                DiskMode::ReadOnly => "READ_ONLY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DISK_MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "READ_WRITE" => Some(Self::ReadWrite),
                "READ_ONLY" => Some(Self::ReadOnly),
                _ => None,
            }
        }
    }
}
/// Sets the scheduling options for this node.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SchedulingConfig {
    /// Defines whether the node is preemptible.
    #[prost(bool, tag = "1")]
    pub preemptible: bool,
    /// Whether the node is created under a reservation.
    #[prost(bool, tag = "2")]
    pub reserved: bool,
}
/// A network endpoint over which a TPU worker can be reached.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkEndpoint {
    /// The internal IP address of this network endpoint.
    #[prost(string, tag = "1")]
    pub ip_address: ::prost::alloc::string::String,
    /// The port of this network endpoint.
    #[prost(int32, tag = "2")]
    pub port: i32,
    /// The access config for the TPU worker.
    #[prost(message, optional, tag = "5")]
    pub access_config: ::core::option::Option<AccessConfig>,
}
/// An access config attached to the TPU worker.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessConfig {
    /// Output only. An external IP address associated with the TPU worker.
    #[prost(string, tag = "1")]
    pub external_ip: ::prost::alloc::string::String,
}
/// Network related configurations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkConfig {
    /// The name of the network for the TPU node. It must be a preexisting Google
    /// Compute Engine network. If none is provided, "default" will be used.
    #[prost(string, tag = "1")]
    pub network: ::prost::alloc::string::String,
    /// The name of the subnetwork for the TPU node. It must be a preexisting
    /// Google Compute Engine subnetwork. If none is provided, "default" will be
    /// used.
    #[prost(string, tag = "2")]
    pub subnetwork: ::prost::alloc::string::String,
    /// Indicates that external IP addresses would be associated with the TPU
    /// workers. If set to false, the specified subnetwork or network should have
    /// Private Google Access enabled.
    #[prost(bool, tag = "3")]
    pub enable_external_ips: bool,
    /// Allows the TPU node to send and receive packets with non-matching
    /// destination or source IPs. This is required if you plan to use the TPU
    /// workers to forward routes.
    #[prost(bool, tag = "4")]
    pub can_ip_forward: bool,
}
/// A service account.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceAccount {
    /// Email address of the service account. If empty, default Compute service
    /// account will be used.
    #[prost(string, tag = "1")]
    pub email: ::prost::alloc::string::String,
    /// The list of scopes to be made available for this service account. If empty,
    /// access to all Cloud APIs will be allowed.
    #[prost(string, repeated, tag = "2")]
    pub scope: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A TPU instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Node {
    /// Output only. Immutable. The name of the TPU.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The user-supplied description of the TPU. Maximum of 512 characters.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// The type of hardware accelerators associated with this node.
    #[prost(string, tag = "5")]
    pub accelerator_type: ::prost::alloc::string::String,
    /// Output only. The current state for the TPU Node.
    #[prost(enumeration = "node::State", tag = "9")]
    pub state: i32,
    /// Output only. If this field is populated, it contains a description of why
    /// the TPU Node is unhealthy.
    #[prost(string, tag = "10")]
    pub health_description: ::prost::alloc::string::String,
    /// Required. The runtime version running in the Node.
    #[prost(string, tag = "11")]
    pub runtime_version: ::prost::alloc::string::String,
    /// Network configurations for the TPU node.
    #[prost(message, optional, tag = "36")]
    pub network_config: ::core::option::Option<NetworkConfig>,
    /// The CIDR block that the TPU node will use when selecting an IP address.
    /// This CIDR block must be a /29 block; the Compute Engine networks API
    /// forbids a smaller block, and using a larger block would be wasteful (a
    /// node can only consume one IP address). Errors will occur if the CIDR block
    /// has already been used for a currently existing TPU node, the CIDR block
    /// conflicts with any subnetworks in the user's provided network, or the
    /// provided network is peered with another network that is using that CIDR
    /// block.
    #[prost(string, tag = "13")]
    pub cidr_block: ::prost::alloc::string::String,
    /// The Google Cloud Platform Service Account to be used by the TPU node VMs.
    /// If None is specified, the default compute service account will be used.
    #[prost(message, optional, tag = "37")]
    pub service_account: ::core::option::Option<ServiceAccount>,
    /// Output only. The time when the node was created.
    #[prost(message, optional, tag = "16")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The scheduling options for this node.
    #[prost(message, optional, tag = "17")]
    pub scheduling_config: ::core::option::Option<SchedulingConfig>,
    /// Output only. The network endpoints where TPU workers can be accessed and
    /// sent work. It is recommended that runtime clients of the node reach out
    /// to the 0th entry in this map first.
    #[prost(message, repeated, tag = "21")]
    pub network_endpoints: ::prost::alloc::vec::Vec<NetworkEndpoint>,
    /// The health status of the TPU node.
    #[prost(enumeration = "node::Health", tag = "22")]
    pub health: i32,
    /// Resource labels to represent user-provided metadata.
    #[prost(map = "string, string", tag = "24")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Custom metadata to apply to the TPU Node.
    /// Can set startup-script and shutdown-script
    #[prost(map = "string, string", tag = "34")]
    pub metadata: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Tags to apply to the TPU Node. Tags are used to identify valid sources or
    /// targets for network firewalls.
    #[prost(string, repeated, tag = "40")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. The unique identifier for the TPU Node.
    #[prost(int64, tag = "33")]
    pub id: i64,
    /// The additional data disks for the Node.
    #[prost(message, repeated, tag = "41")]
    pub data_disks: ::prost::alloc::vec::Vec<AttachedDisk>,
    /// Output only. The API version that created this Node.
    #[prost(enumeration = "node::ApiVersion", tag = "38")]
    pub api_version: i32,
    /// Output only. The Symptoms that have occurred to the TPU Node.
    #[prost(message, repeated, tag = "39")]
    pub symptoms: ::prost::alloc::vec::Vec<Symptom>,
    /// Output only. The qualified name of the QueuedResource that requested this
    /// Node.
    #[prost(string, tag = "43")]
    pub queued_resource: ::prost::alloc::string::String,
    /// The AccleratorConfig for the TPU Node.
    #[prost(message, optional, tag = "44")]
    pub accelerator_config: ::core::option::Option<AcceleratorConfig>,
    /// Shielded Instance options.
    #[prost(message, optional, tag = "45")]
    pub shielded_instance_config: ::core::option::Option<ShieldedInstanceConfig>,
    /// Output only. Whether the Node belongs to a Multislice group.
    #[prost(bool, tag = "47")]
    pub multislice_node: bool,
}
/// Nested message and enum types in `Node`.
pub mod node {
    /// Represents the different states of a TPU node during its lifecycle.
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
        /// TPU node state is not known/set.
        Unspecified = 0,
        /// TPU node is being created.
        Creating = 1,
        /// TPU node has been created.
        Ready = 2,
        /// TPU node is restarting.
        Restarting = 3,
        /// TPU node is undergoing reimaging.
        Reimaging = 4,
        /// TPU node is being deleted.
        Deleting = 5,
        /// TPU node is being repaired and may be unusable. Details can be
        /// found in the 'help_description' field.
        Repairing = 6,
        /// TPU node is stopped.
        Stopped = 8,
        /// TPU node is currently stopping.
        Stopping = 9,
        /// TPU node is currently starting.
        Starting = 10,
        /// TPU node has been preempted. Only applies to Preemptible TPU Nodes.
        Preempted = 11,
        /// TPU node has been terminated due to maintenance or has reached the end of
        /// its life cycle (for preemptible nodes).
        Terminated = 12,
        /// TPU node is currently hiding.
        Hiding = 13,
        /// TPU node has been hidden.
        Hidden = 14,
        /// TPU node is currently unhiding.
        Unhiding = 15,
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
                State::Ready => "READY",
                State::Restarting => "RESTARTING",
                State::Reimaging => "REIMAGING",
                State::Deleting => "DELETING",
                State::Repairing => "REPAIRING",
                State::Stopped => "STOPPED",
                State::Stopping => "STOPPING",
                State::Starting => "STARTING",
                State::Preempted => "PREEMPTED",
                State::Terminated => "TERMINATED",
                State::Hiding => "HIDING",
                State::Hidden => "HIDDEN",
                State::Unhiding => "UNHIDING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "READY" => Some(Self::Ready),
                "RESTARTING" => Some(Self::Restarting),
                "REIMAGING" => Some(Self::Reimaging),
                "DELETING" => Some(Self::Deleting),
                "REPAIRING" => Some(Self::Repairing),
                "STOPPED" => Some(Self::Stopped),
                "STOPPING" => Some(Self::Stopping),
                "STARTING" => Some(Self::Starting),
                "PREEMPTED" => Some(Self::Preempted),
                "TERMINATED" => Some(Self::Terminated),
                "HIDING" => Some(Self::Hiding),
                "HIDDEN" => Some(Self::Hidden),
                "UNHIDING" => Some(Self::Unhiding),
                _ => None,
            }
        }
    }
    /// Health defines the status of a TPU node as reported by
    /// Health Monitor.
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
    pub enum Health {
        /// Health status is unknown: not initialized or failed to retrieve.
        Unspecified = 0,
        /// The resource is healthy.
        Healthy = 1,
        /// The resource is unresponsive.
        Timeout = 3,
        /// The in-guest ML stack is unhealthy.
        UnhealthyTensorflow = 4,
        /// The node is under maintenance/priority boost caused rescheduling and
        /// will resume running once rescheduled.
        UnhealthyMaintenance = 5,
    }
    impl Health {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Health::Unspecified => "HEALTH_UNSPECIFIED",
                Health::Healthy => "HEALTHY",
                Health::Timeout => "TIMEOUT",
                Health::UnhealthyTensorflow => "UNHEALTHY_TENSORFLOW",
                Health::UnhealthyMaintenance => "UNHEALTHY_MAINTENANCE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "HEALTH_UNSPECIFIED" => Some(Self::Unspecified),
                "HEALTHY" => Some(Self::Healthy),
                "TIMEOUT" => Some(Self::Timeout),
                "UNHEALTHY_TENSORFLOW" => Some(Self::UnhealthyTensorflow),
                "UNHEALTHY_MAINTENANCE" => Some(Self::UnhealthyMaintenance),
                _ => None,
            }
        }
    }
    /// TPU API Version.
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
    pub enum ApiVersion {
        /// API version is unknown.
        Unspecified = 0,
        /// TPU API V1Alpha1 version.
        V1Alpha1 = 1,
        /// TPU API V1 version.
        V1 = 2,
        /// TPU API V2Alpha1 version.
        V2Alpha1 = 3,
    }
    impl ApiVersion {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ApiVersion::Unspecified => "API_VERSION_UNSPECIFIED",
                ApiVersion::V1Alpha1 => "V1_ALPHA1",
                ApiVersion::V1 => "V1",
                ApiVersion::V2Alpha1 => "V2_ALPHA1",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "API_VERSION_UNSPECIFIED" => Some(Self::Unspecified),
                "V1_ALPHA1" => Some(Self::V1Alpha1),
                "V1" => Some(Self::V1),
                "V2_ALPHA1" => Some(Self::V2Alpha1),
                _ => None,
            }
        }
    }
}
/// A QueuedResource represents a request for resources that will be placed
/// in a queue and fulfilled when the necessary resources are available.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueuedResource {
    /// Output only. Immutable. The name of the QueuedResource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The queueing policy of the QueuedRequest.
    #[prost(message, optional, tag = "5")]
    pub queueing_policy: ::core::option::Option<queued_resource::QueueingPolicy>,
    /// Output only. State of the QueuedResource request.
    #[prost(message, optional, tag = "6")]
    pub state: ::core::option::Option<QueuedResourceState>,
    /// Name of the reservation in which the resource should be provisioned.
    /// Format: projects/{project}/locations/{zone}/reservations/{reservation}
    #[prost(string, tag = "8")]
    pub reservation_name: ::prost::alloc::string::String,
    /// Resource specification.
    #[prost(oneof = "queued_resource::Resource", tags = "2")]
    pub resource: ::core::option::Option<queued_resource::Resource>,
    /// Tier specifies the required tier.
    #[prost(oneof = "queued_resource::Tier", tags = "3, 4, 9")]
    pub tier: ::core::option::Option<queued_resource::Tier>,
}
/// Nested message and enum types in `QueuedResource`.
pub mod queued_resource {
    /// Details of the TPU resource(s) being requested.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Tpu {
        /// The TPU node(s) being requested.
        #[prost(message, repeated, tag = "1")]
        pub node_spec: ::prost::alloc::vec::Vec<tpu::NodeSpec>,
    }
    /// Nested message and enum types in `Tpu`.
    pub mod tpu {
        /// Details of the TPU node(s) being requested. Users can request either a
        /// single node or multiple nodes.
        /// NodeSpec provides the specification for node(s) to be created.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct NodeSpec {
            /// Required. The parent resource name.
            #[prost(string, tag = "1")]
            pub parent: ::prost::alloc::string::String,
            /// The unqualified resource name. Should follow the `^\[A-Za-z0-9_.~+%-\]+$`
            /// regex format. This is only specified when requesting a single node.
            /// In case of multi-node requests, multi_node_params must be populated
            /// instead. It's an error to specify both node_id and multi_node_params.
            #[prost(string, tag = "2")]
            pub node_id: ::prost::alloc::string::String,
            /// Required. The node.
            #[prost(message, optional, tag = "3")]
            pub node: ::core::option::Option<super::super::Node>,
        }
    }
    /// BestEffort tier definition.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BestEffort {}
    /// Spot tier definition.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Spot {}
    /// Guaranteed tier definition.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Guaranteed {
        /// Optional. Defines the minimum duration of the guarantee. If specified,
        /// the requested resources will only be provisioned if they can be
        /// allocated for at least the given duration.
        #[prost(message, optional, tag = "1")]
        pub min_duration: ::core::option::Option<::prost_types::Duration>,
        /// Optional. Specifies the request should be scheduled on reserved capacity.
        #[prost(bool, tag = "2")]
        pub reserved: bool,
    }
    /// Defines the policy of the QueuedRequest.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QueueingPolicy {
        /// Time flexibility specification.
        #[prost(
            oneof = "queueing_policy::StartTimingConstraints",
            tags = "1, 2, 3, 4, 5"
        )]
        pub start_timing_constraints: ::core::option::Option<
            queueing_policy::StartTimingConstraints,
        >,
    }
    /// Nested message and enum types in `QueueingPolicy`.
    pub mod queueing_policy {
        /// Time flexibility specification.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum StartTimingConstraints {
            /// A relative time after which resources should not be created.
            /// If the request cannot be fulfilled by this time the request will be
            /// failed.
            #[prost(message, tag = "1")]
            ValidUntilDuration(::prost_types::Duration),
            /// An absolute time after which resources should not be created.
            /// If the request cannot be fulfilled by this time the request will be
            /// failed.
            #[prost(message, tag = "2")]
            ValidUntilTime(::prost_types::Timestamp),
            /// A relative time after which resources may be created.
            #[prost(message, tag = "3")]
            ValidAfterDuration(::prost_types::Duration),
            /// An absolute time at which resources may be created.
            #[prost(message, tag = "4")]
            ValidAfterTime(::prost_types::Timestamp),
            /// An absolute time interval within which resources may be created.
            #[prost(message, tag = "5")]
            ValidInterval(super::super::super::super::super::r#type::Interval),
        }
    }
    /// Resource specification.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Resource {
        /// Defines a TPU resource.
        #[prost(message, tag = "2")]
        Tpu(Tpu),
    }
    /// Tier specifies the required tier.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Tier {
        /// The BestEffort tier.
        #[prost(message, tag = "3")]
        BestEffort(BestEffort),
        /// The Guaranteed tier.
        #[prost(message, tag = "4")]
        Guaranteed(Guaranteed),
        /// Optional. The Spot tier.
        #[prost(message, tag = "9")]
        Spot(Spot),
    }
}
/// QueuedResourceState defines the details of the QueuedResource request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueuedResourceState {
    /// State of the QueuedResource request.
    #[prost(enumeration = "queued_resource_state::State", tag = "1")]
    pub state: i32,
    /// Further data for the state.
    #[prost(oneof = "queued_resource_state::StateData", tags = "2, 3, 4, 5, 6, 7, 8, 9")]
    pub state_data: ::core::option::Option<queued_resource_state::StateData>,
}
/// Nested message and enum types in `QueuedResourceState`.
pub mod queued_resource_state {
    /// Further data for the creating state.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CreatingData {}
    /// Further data for the accepted state.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AcceptedData {}
    /// Further data for the provisioning state.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProvisioningData {}
    /// Further data for the failed state.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FailedData {
        /// The error that caused the queued resource to enter the FAILED state.
        #[prost(message, optional, tag = "1")]
        pub error: ::core::option::Option<super::super::super::super::rpc::Status>,
    }
    /// Further data for the deleting state.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DeletingData {}
    /// Further data for the active state.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ActiveData {}
    /// Further data for the suspending state.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SuspendingData {}
    /// Further data for the suspended state.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SuspendedData {}
    /// Output only state of the request
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
        /// State of the QueuedResource request is not known/set.
        Unspecified = 0,
        /// The QueuedResource request has been received. We're still working on
        /// determining if we will be able to honor this request.
        Creating = 1,
        /// The QueuedResource request has passed initial validation/admission
        /// control and has been persisted in the queue.
        Accepted = 2,
        /// The QueuedResource request has been selected. The
        /// associated resources are currently being provisioned (or very soon
        /// will begin provisioning).
        Provisioning = 3,
        /// The request could not be completed. This may be due to some
        /// late-discovered problem with the request itself, or due to
        /// unavailability of resources within the constraints of the request
        /// (e.g., the 'valid until' start timing constraint expired).
        Failed = 4,
        /// The QueuedResource is being deleted.
        Deleting = 5,
        /// The resources specified in the QueuedResource request have been
        /// provisioned and are ready for use by the end-user/consumer.
        Active = 6,
        /// The resources specified in the QueuedResource request are being
        /// deleted. This may have been initiated by the user, or
        /// the Cloud TPU service. Inspect the state data for more details.
        Suspending = 7,
        /// The resources specified in the QueuedResource request have been
        /// deleted.
        Suspended = 8,
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
                State::Accepted => "ACCEPTED",
                State::Provisioning => "PROVISIONING",
                State::Failed => "FAILED",
                State::Deleting => "DELETING",
                State::Active => "ACTIVE",
                State::Suspending => "SUSPENDING",
                State::Suspended => "SUSPENDED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "ACCEPTED" => Some(Self::Accepted),
                "PROVISIONING" => Some(Self::Provisioning),
                "FAILED" => Some(Self::Failed),
                "DELETING" => Some(Self::Deleting),
                "ACTIVE" => Some(Self::Active),
                "SUSPENDING" => Some(Self::Suspending),
                "SUSPENDED" => Some(Self::Suspended),
                _ => None,
            }
        }
    }
    /// Further data for the state.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum StateData {
        /// Further data for the creating state.
        #[prost(message, tag = "2")]
        CreatingData(CreatingData),
        /// Further data for the accepted state.
        #[prost(message, tag = "3")]
        AcceptedData(AcceptedData),
        /// Further data for the provisioning state.
        #[prost(message, tag = "4")]
        ProvisioningData(ProvisioningData),
        /// Further data for the failed state.
        #[prost(message, tag = "5")]
        FailedData(FailedData),
        /// Further data for the deleting state.
        #[prost(message, tag = "6")]
        DeletingData(DeletingData),
        /// Further data for the active state.
        #[prost(message, tag = "7")]
        ActiveData(ActiveData),
        /// Further data for the suspending state.
        #[prost(message, tag = "8")]
        SuspendingData(SuspendingData),
        /// Further data for the suspended state.
        #[prost(message, tag = "9")]
        SuspendedData(SuspendedData),
    }
}
/// Request for \[ListNodes][google.cloud.tpu.v2alpha1.Tpu.ListNodes\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNodesRequest {
    /// Required. The parent resource name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response for \[ListNodes][google.cloud.tpu.v2alpha1.Tpu.ListNodes\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNodesResponse {
    /// The listed nodes.
    #[prost(message, repeated, tag = "1")]
    pub nodes: ::prost::alloc::vec::Vec<Node>,
    /// The next page token or empty if none.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for \[GetNode][google.cloud.tpu.v2alpha1.Tpu.GetNode\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNodeRequest {
    /// Required. The resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for \[CreateNode][google.cloud.tpu.v2alpha1.Tpu.CreateNode\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateNodeRequest {
    /// Required. The parent resource name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The unqualified resource name.
    #[prost(string, tag = "2")]
    pub node_id: ::prost::alloc::string::String,
    /// Required. The node.
    #[prost(message, optional, tag = "3")]
    pub node: ::core::option::Option<Node>,
    /// Idempotent request UUID.
    #[prost(string, tag = "6")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request for \[DeleteNode][google.cloud.tpu.v2alpha1.Tpu.DeleteNode\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteNodeRequest {
    /// Required. The resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Idempotent request UUID.
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request for \[StopNode][google.cloud.tpu.v2alpha1.Tpu.StopNode\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopNodeRequest {
    /// Required. The resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for \[StartNode][google.cloud.tpu.v2alpha1.Tpu.StartNode\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartNodeRequest {
    /// Required. The resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for \[UpdateNode][google.cloud.tpu.v2alpha1.Tpu.UpdateNode\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateNodeRequest {
    /// Required. Mask of fields from \[Node][Tpu.Node\] to update.
    /// Supported fields: [description, tags, labels, metadata,
    /// network_config.enable_external_ips].
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The node. Only fields specified in update_mask are updated.
    #[prost(message, optional, tag = "2")]
    pub node: ::core::option::Option<Node>,
}
/// Request for
/// \[ListQueuedResources][google.cloud.tpu.v2alpha1.Tpu.ListQueuedResources\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListQueuedResourcesRequest {
    /// Required. The parent resource name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response for
/// \[ListQueuedResources][google.cloud.tpu.v2alpha1.Tpu.ListQueuedResources\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListQueuedResourcesResponse {
    /// The listed queued resources.
    #[prost(message, repeated, tag = "1")]
    pub queued_resources: ::prost::alloc::vec::Vec<QueuedResource>,
    /// The next page token or empty if none.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for
/// \[GetQueuedResource][google.cloud.tpu.v2alpha1.Tpu.GetQueuedResource\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetQueuedResourceRequest {
    /// Required. The resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for
/// \[CreateQueuedResource][google.cloud.tpu.v2alpha1.Tpu.CreateQueuedResource\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateQueuedResourceRequest {
    /// Required. The parent resource name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The unqualified resource name. Should follow the `^\[A-Za-z0-9_.~+%-\]+$`
    /// regex format.
    #[prost(string, tag = "2")]
    pub queued_resource_id: ::prost::alloc::string::String,
    /// Required. The queued resource.
    #[prost(message, optional, tag = "3")]
    pub queued_resource: ::core::option::Option<QueuedResource>,
    /// Idempotent request UUID.
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request for
/// \[DeleteQueuedResource][google.cloud.tpu.v2alpha1.Tpu.DeleteQueuedResource\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteQueuedResourceRequest {
    /// Required. The resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Idempotent request UUID.
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// If set to true, all running nodes belonging to this queued resource will
    /// be deleted first and then the queued resource will be deleted.
    /// Otherwise (i.e. force=false), the queued resource will only be deleted if
    /// its nodes have already been deleted or the queued resource is in the
    /// ACCEPTED, FAILED, or SUSPENDED state.
    #[prost(bool, tag = "3")]
    pub force: bool,
}
/// The per-product per-project service identity for Cloud TPU service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceIdentity {
    /// The email address of the service identity.
    #[prost(string, tag = "1")]
    pub email: ::prost::alloc::string::String,
}
/// Request for
/// \[GenerateServiceIdentity][google.cloud.tpu.v2alpha1.Tpu.GenerateServiceIdentity\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateServiceIdentityRequest {
    /// Required. The parent resource name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
}
/// Response for
/// \[GenerateServiceIdentity][google.cloud.tpu.v2alpha1.Tpu.GenerateServiceIdentity\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateServiceIdentityResponse {
    /// ServiceIdentity that was created or retrieved.
    #[prost(message, optional, tag = "1")]
    pub identity: ::core::option::Option<ServiceIdentity>,
}
/// A accelerator type that a Node can be configured with.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcceleratorType {
    /// The resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The accelerator type.
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
    /// The accelerator config.
    #[prost(message, repeated, tag = "3")]
    pub accelerator_configs: ::prost::alloc::vec::Vec<AcceleratorConfig>,
}
/// Request for
/// \[GetAcceleratorType][google.cloud.tpu.v2alpha1.Tpu.GetAcceleratorType\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAcceleratorTypeRequest {
    /// Required. The resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for
/// \[ListAcceleratorTypes][google.cloud.tpu.v2alpha1.Tpu.ListAcceleratorTypes\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAcceleratorTypesRequest {
    /// Required. The parent resource name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// List filter.
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
    /// Sort results.
    #[prost(string, tag = "6")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response for
/// \[ListAcceleratorTypes][google.cloud.tpu.v2alpha1.Tpu.ListAcceleratorTypes\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAcceleratorTypesResponse {
    /// The listed nodes.
    #[prost(message, repeated, tag = "1")]
    pub accelerator_types: ::prost::alloc::vec::Vec<AcceleratorType>,
    /// The next page token or empty if none.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A runtime version that a Node can be configured with.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuntimeVersion {
    /// The resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The runtime version.
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
}
/// Request for
/// \[GetRuntimeVersion][google.cloud.tpu.v2alpha1.Tpu.GetRuntimeVersion\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRuntimeVersionRequest {
    /// Required. The resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for
/// \[ListRuntimeVersions][google.cloud.tpu.v2alpha1.Tpu.ListRuntimeVersions\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRuntimeVersionsRequest {
    /// Required. The parent resource name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// List filter.
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
    /// Sort results.
    #[prost(string, tag = "6")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response for
/// \[ListRuntimeVersions][google.cloud.tpu.v2alpha1.Tpu.ListRuntimeVersions\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRuntimeVersionsResponse {
    /// The listed nodes.
    #[prost(message, repeated, tag = "1")]
    pub runtime_versions: ::prost::alloc::vec::Vec<RuntimeVersion>,
    /// The next page token or empty if none.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Metadata describing an \[Operation][google.longrunning.Operation\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Target of the operation - for example
    /// projects/project-1/connectivityTests/test-1
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: ::prost::alloc::string::String,
    /// Human-readable status of the operation, if any.
    #[prost(string, tag = "5")]
    pub status_detail: ::prost::alloc::string::String,
    /// Specifies if cancellation was requested for the operation.
    #[prost(bool, tag = "6")]
    pub cancel_requested: bool,
    /// API version.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
/// A Symptom instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Symptom {
    /// Timestamp when the Symptom is created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Type of the Symptom.
    #[prost(enumeration = "symptom::SymptomType", tag = "2")]
    pub symptom_type: i32,
    /// Detailed information of the current Symptom.
    #[prost(string, tag = "3")]
    pub details: ::prost::alloc::string::String,
    /// A string used to uniquely distinguish a worker within a TPU node.
    #[prost(string, tag = "4")]
    pub worker_id: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Symptom`.
pub mod symptom {
    /// SymptomType represents the different types of Symptoms that a TPU can be
    /// at.
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
    pub enum SymptomType {
        /// Unspecified symptom.
        Unspecified = 0,
        /// TPU VM memory is low.
        LowMemory = 1,
        /// TPU runtime is out of memory.
        OutOfMemory = 2,
        /// TPU runtime execution has timed out.
        ExecuteTimedOut = 3,
        /// TPU runtime fails to construct a mesh that recognizes each TPU device's
        /// neighbors.
        MeshBuildFail = 4,
        /// TPU HBM is out of memory.
        HbmOutOfMemory = 5,
        /// Abusive behaviors have been identified on the current project.
        ProjectAbuse = 6,
    }
    impl SymptomType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SymptomType::Unspecified => "SYMPTOM_TYPE_UNSPECIFIED",
                SymptomType::LowMemory => "LOW_MEMORY",
                SymptomType::OutOfMemory => "OUT_OF_MEMORY",
                SymptomType::ExecuteTimedOut => "EXECUTE_TIMED_OUT",
                SymptomType::MeshBuildFail => "MESH_BUILD_FAIL",
                SymptomType::HbmOutOfMemory => "HBM_OUT_OF_MEMORY",
                SymptomType::ProjectAbuse => "PROJECT_ABUSE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SYMPTOM_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "LOW_MEMORY" => Some(Self::LowMemory),
                "OUT_OF_MEMORY" => Some(Self::OutOfMemory),
                "EXECUTE_TIMED_OUT" => Some(Self::ExecuteTimedOut),
                "MESH_BUILD_FAIL" => Some(Self::MeshBuildFail),
                "HBM_OUT_OF_MEMORY" => Some(Self::HbmOutOfMemory),
                "PROJECT_ABUSE" => Some(Self::ProjectAbuse),
                _ => None,
            }
        }
    }
}
/// Request for
/// \[GetGuestAttributes][google.cloud.tpu.v2alpha1.Tpu.GetGuestAttributes\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGuestAttributesRequest {
    /// Required. The resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The guest attributes path to be queried.
    #[prost(string, tag = "2")]
    pub query_path: ::prost::alloc::string::String,
    /// The 0-based worker ID. If it is empty, all workers' GuestAttributes will be
    /// returned.
    #[prost(string, repeated, tag = "3")]
    pub worker_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Response for
/// \[GetGuestAttributes][google.cloud.tpu.v2alpha1.Tpu.GetGuestAttributes\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGuestAttributesResponse {
    /// The guest attributes for the TPU workers.
    #[prost(message, repeated, tag = "1")]
    pub guest_attributes: ::prost::alloc::vec::Vec<GuestAttributes>,
}
/// Request for
/// \[SimulateMaintenanceEvent][google.cloud.tpu.v2alpha1.Tpu.SimulateMaintenanceEvent\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimulateMaintenanceEventRequest {
    /// Required. The resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The 0-based worker ID. If it is empty, worker ID 0 will be selected for
    /// maintenance event simulation. A maintenance event will only be fired on the
    /// first specified worker ID. Future implementations may support firing on
    /// multiple workers.
    #[prost(string, repeated, tag = "2")]
    pub worker_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A TPU accelerator configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcceleratorConfig {
    /// Required. Type of TPU.
    #[prost(enumeration = "accelerator_config::Type", tag = "1")]
    pub r#type: i32,
    /// Required. Topology of TPU in chips.
    #[prost(string, tag = "2")]
    pub topology: ::prost::alloc::string::String,
}
/// Nested message and enum types in `AcceleratorConfig`.
pub mod accelerator_config {
    /// TPU type.
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
        /// Unspecified version.
        Unspecified = 0,
        /// TPU v2.
        V2 = 2,
        /// TPU v3.
        V3 = 4,
        /// TPU v4.
        V4 = 7,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::V2 => "V2",
                Type::V3 => "V3",
                Type::V4 => "V4",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "V2" => Some(Self::V2),
                "V3" => Some(Self::V3),
                "V4" => Some(Self::V4),
                _ => None,
            }
        }
    }
}
/// A set of Shielded Instance options.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShieldedInstanceConfig {
    /// Defines whether the instance has Secure Boot enabled.
    #[prost(bool, tag = "1")]
    pub enable_secure_boot: bool,
}
/// Generated client implementations.
pub mod tpu_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Manages TPU nodes and other resources
    ///
    /// TPU API v2alpha1
    #[derive(Debug, Clone)]
    pub struct TpuClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TpuClient<tonic::transport::Channel> {
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
    impl<T> TpuClient<T>
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
        ) -> TpuClient<InterceptedService<T, F>>
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
            TpuClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists nodes.
        pub async fn list_nodes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNodesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListNodesResponse>,
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
                "/google.cloud.tpu.v2alpha1.Tpu/ListNodes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.tpu.v2alpha1.Tpu", "ListNodes"));
            self.inner.unary(req, path, codec).await
        }
        /// Gets the details of a node.
        pub async fn get_node(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNodeRequest>,
        ) -> std::result::Result<tonic::Response<super::Node>, tonic::Status> {
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
                "/google.cloud.tpu.v2alpha1.Tpu/GetNode",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.tpu.v2alpha1.Tpu", "GetNode"));
            self.inner.unary(req, path, codec).await
        }
        /// Creates a node.
        pub async fn create_node(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateNodeRequest>,
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
                "/google.cloud.tpu.v2alpha1.Tpu/CreateNode",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.tpu.v2alpha1.Tpu", "CreateNode"));
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a node.
        pub async fn delete_node(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteNodeRequest>,
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
                "/google.cloud.tpu.v2alpha1.Tpu/DeleteNode",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.tpu.v2alpha1.Tpu", "DeleteNode"));
            self.inner.unary(req, path, codec).await
        }
        /// Stops a node. This operation is only available with single TPU nodes.
        pub async fn stop_node(
            &mut self,
            request: impl tonic::IntoRequest<super::StopNodeRequest>,
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
                "/google.cloud.tpu.v2alpha1.Tpu/StopNode",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.tpu.v2alpha1.Tpu", "StopNode"));
            self.inner.unary(req, path, codec).await
        }
        /// Starts a node.
        pub async fn start_node(
            &mut self,
            request: impl tonic::IntoRequest<super::StartNodeRequest>,
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
                "/google.cloud.tpu.v2alpha1.Tpu/StartNode",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.tpu.v2alpha1.Tpu", "StartNode"));
            self.inner.unary(req, path, codec).await
        }
        /// Updates the configurations of a node.
        pub async fn update_node(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateNodeRequest>,
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
                "/google.cloud.tpu.v2alpha1.Tpu/UpdateNode",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.tpu.v2alpha1.Tpu", "UpdateNode"));
            self.inner.unary(req, path, codec).await
        }
        /// Lists queued resources.
        pub async fn list_queued_resources(
            &mut self,
            request: impl tonic::IntoRequest<super::ListQueuedResourcesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListQueuedResourcesResponse>,
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
                "/google.cloud.tpu.v2alpha1.Tpu/ListQueuedResources",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.tpu.v2alpha1.Tpu",
                        "ListQueuedResources",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a queued resource.
        pub async fn get_queued_resource(
            &mut self,
            request: impl tonic::IntoRequest<super::GetQueuedResourceRequest>,
        ) -> std::result::Result<tonic::Response<super::QueuedResource>, tonic::Status> {
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
                "/google.cloud.tpu.v2alpha1.Tpu/GetQueuedResource",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.tpu.v2alpha1.Tpu", "GetQueuedResource"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a QueuedResource TPU instance.
        pub async fn create_queued_resource(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateQueuedResourceRequest>,
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
                "/google.cloud.tpu.v2alpha1.Tpu/CreateQueuedResource",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.tpu.v2alpha1.Tpu",
                        "CreateQueuedResource",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a QueuedResource TPU instance.
        pub async fn delete_queued_resource(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteQueuedResourceRequest>,
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
                "/google.cloud.tpu.v2alpha1.Tpu/DeleteQueuedResource",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.tpu.v2alpha1.Tpu",
                        "DeleteQueuedResource",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Generates the Cloud TPU service identity for the project.
        pub async fn generate_service_identity(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateServiceIdentityRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateServiceIdentityResponse>,
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
                "/google.cloud.tpu.v2alpha1.Tpu/GenerateServiceIdentity",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.tpu.v2alpha1.Tpu",
                        "GenerateServiceIdentity",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists accelerator types supported by this API.
        pub async fn list_accelerator_types(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAcceleratorTypesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAcceleratorTypesResponse>,
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
                "/google.cloud.tpu.v2alpha1.Tpu/ListAcceleratorTypes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.tpu.v2alpha1.Tpu",
                        "ListAcceleratorTypes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets AcceleratorType.
        pub async fn get_accelerator_type(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAcceleratorTypeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AcceleratorType>,
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
                "/google.cloud.tpu.v2alpha1.Tpu/GetAcceleratorType",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.tpu.v2alpha1.Tpu",
                        "GetAcceleratorType",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists runtime versions supported by this API.
        pub async fn list_runtime_versions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRuntimeVersionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRuntimeVersionsResponse>,
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
                "/google.cloud.tpu.v2alpha1.Tpu/ListRuntimeVersions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.tpu.v2alpha1.Tpu",
                        "ListRuntimeVersions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a runtime version.
        pub async fn get_runtime_version(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRuntimeVersionRequest>,
        ) -> std::result::Result<tonic::Response<super::RuntimeVersion>, tonic::Status> {
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
                "/google.cloud.tpu.v2alpha1.Tpu/GetRuntimeVersion",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.tpu.v2alpha1.Tpu", "GetRuntimeVersion"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves the guest attributes for the node.
        pub async fn get_guest_attributes(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGuestAttributesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetGuestAttributesResponse>,
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
                "/google.cloud.tpu.v2alpha1.Tpu/GetGuestAttributes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.tpu.v2alpha1.Tpu",
                        "GetGuestAttributes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Simulates a maintenance event.
        pub async fn simulate_maintenance_event(
            &mut self,
            request: impl tonic::IntoRequest<super::SimulateMaintenanceEventRequest>,
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
                "/google.cloud.tpu.v2alpha1.Tpu/SimulateMaintenanceEvent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.tpu.v2alpha1.Tpu",
                        "SimulateMaintenanceEvent",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
