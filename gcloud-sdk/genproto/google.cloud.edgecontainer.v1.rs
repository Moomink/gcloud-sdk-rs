/// A Google Distributed Cloud Edge Kubernetes cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cluster {
    /// Required. The resource name of the cluster.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The time when the cluster was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the cluster was last updated.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Labels associated with this resource.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Required. Fleet configuration.
    #[prost(message, optional, tag = "11")]
    pub fleet: ::core::option::Option<Fleet>,
    /// Required. Cluster-wide networking configuration.
    #[prost(message, optional, tag = "7")]
    pub networking: ::core::option::Option<ClusterNetworking>,
    /// Required. Immutable. RBAC policy that will be applied and managed by GEC.
    #[prost(message, optional, tag = "9")]
    pub authorization: ::core::option::Option<Authorization>,
    /// Optional. The default maximum number of pods per node used if a maximum
    /// value is not specified explicitly for a node pool in this cluster. If
    /// unspecified, the Kubernetes default value will be used.
    #[prost(int32, tag = "8")]
    pub default_max_pods_per_node: i32,
    /// Output only. The IP address of the Kubernetes API server.
    #[prost(string, tag = "6")]
    pub endpoint: ::prost::alloc::string::String,
    /// Output only. The port number of the Kubernetes API server.
    #[prost(int32, tag = "19")]
    pub port: i32,
    /// Output only. The PEM-encoded public certificate of the cluster's CA.
    #[prost(string, tag = "10")]
    pub cluster_ca_certificate: ::prost::alloc::string::String,
    /// Optional. Cluster-wide maintenance policy configuration.
    #[prost(message, optional, tag = "12")]
    pub maintenance_policy: ::core::option::Option<MaintenancePolicy>,
    /// Output only. The control plane release version
    #[prost(string, tag = "13")]
    pub control_plane_version: ::prost::alloc::string::String,
    /// Output only. The lowest release version among all worker nodes. This field
    /// can be empty if the cluster does not have any worker nodes.
    #[prost(string, tag = "14")]
    pub node_version: ::prost::alloc::string::String,
    /// Optional. The configuration of the cluster control plane.
    #[prost(message, optional, tag = "15")]
    pub control_plane: ::core::option::Option<cluster::ControlPlane>,
    /// Optional. The configuration of the system add-ons.
    #[prost(message, optional, tag = "16")]
    pub system_addons_config: ::core::option::Option<cluster::SystemAddonsConfig>,
    /// Optional. IPv4 address pools for cluster data plane external load
    /// balancing.
    #[prost(string, repeated, tag = "17")]
    pub external_load_balancer_ipv4_address_pools: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// Optional. Remote control plane disk encryption options. This field is only
    /// used when enabling CMEK support.
    #[prost(message, optional, tag = "18")]
    pub control_plane_encryption: ::core::option::Option<
        cluster::ControlPlaneEncryption,
    >,
    /// Output only. The current status of the cluster.
    #[prost(enumeration = "cluster::Status", tag = "20")]
    pub status: i32,
    /// Output only. All the maintenance events scheduled for the cluster,
    /// including the ones ongoing, planned for the future and done in the past (up
    /// to 90 days).
    #[prost(message, repeated, tag = "21")]
    pub maintenance_events: ::prost::alloc::vec::Vec<cluster::MaintenanceEvent>,
    /// Optional. The target cluster version. For example: "1.5.0".
    #[prost(string, tag = "22")]
    pub target_version: ::prost::alloc::string::String,
    /// Optional. The release channel a cluster is subscribed to.
    #[prost(enumeration = "cluster::ReleaseChannel", tag = "23")]
    pub release_channel: i32,
    /// Optional. Configuration of the cluster survivability, e.g., for the case
    /// when network connectivity is lost. Note: This only applies to local control
    /// plane clusters.
    #[prost(message, optional, tag = "24")]
    pub survivability_config: ::core::option::Option<cluster::SurvivabilityConfig>,
    /// Optional. IPv6 address pools for cluster data plane external load
    /// balancing.
    #[prost(string, repeated, tag = "25")]
    pub external_load_balancer_ipv6_address_pools: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
}
/// Nested message and enum types in `Cluster`.
pub mod cluster {
    /// Configuration of the cluster control plane.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ControlPlane {
        #[prost(oneof = "control_plane::Config", tags = "1, 2")]
        pub config: ::core::option::Option<control_plane::Config>,
    }
    /// Nested message and enum types in `ControlPlane`.
    pub mod control_plane {
        /// Configuration specific to clusters with a control plane hosted remotely.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Remote {}
        /// Configuration specific to clusters with a control plane hosted locally.
        ///
        /// Warning: Local control plane clusters must be created in their own
        /// project. Local control plane clusters cannot coexist in the same
        /// project with any other type of clusters, including non-GDCE clusters.
        /// Mixing local control plane GDCE clusters with any other type of
        /// clusters in the same project can result in data loss.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Local {
            /// Name of the Google Distributed Cloud Edge zones where this node pool
            /// will be created. For example: `us-central1-edge-customer-a`.
            #[prost(string, tag = "1")]
            pub node_location: ::prost::alloc::string::String,
            /// The number of nodes to serve as replicas of the Control Plane.
            #[prost(int32, tag = "2")]
            pub node_count: i32,
            /// Only machines matching this filter will be allowed to host control
            /// plane nodes. The filtering language accepts strings like "name=<name>",
            /// and is documented here: [AIP-160](<https://google.aip.dev/160>).
            #[prost(string, tag = "3")]
            pub machine_filter: ::prost::alloc::string::String,
            /// Policy configuration about how user applications are deployed.
            #[prost(enumeration = "SharedDeploymentPolicy", tag = "4")]
            pub shared_deployment_policy: i32,
        }
        /// Represents the policy configuration about how user applications are
        /// deployed.
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
        pub enum SharedDeploymentPolicy {
            /// Unspecified.
            Unspecified = 0,
            /// User applications can be deployed both on control plane and worker
            /// nodes.
            Allowed = 1,
            /// User applications can not be deployed on control plane nodes and can
            /// only be deployed on worker nodes.
            Disallowed = 2,
        }
        impl SharedDeploymentPolicy {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    SharedDeploymentPolicy::Unspecified => {
                        "SHARED_DEPLOYMENT_POLICY_UNSPECIFIED"
                    }
                    SharedDeploymentPolicy::Allowed => "ALLOWED",
                    SharedDeploymentPolicy::Disallowed => "DISALLOWED",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "SHARED_DEPLOYMENT_POLICY_UNSPECIFIED" => Some(Self::Unspecified),
                    "ALLOWED" => Some(Self::Allowed),
                    "DISALLOWED" => Some(Self::Disallowed),
                    _ => None,
                }
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Config {
            /// Remote control plane configuration.
            #[prost(message, tag = "1")]
            Remote(Remote),
            /// Local control plane configuration.
            ///
            /// Warning: Local control plane clusters must be created in their own
            /// project. Local control plane clusters cannot coexist in the same
            /// project with any other type of clusters, including non-GDCE clusters.
            /// Mixing local control plane GDCE clusters with any other type of
            /// clusters in the same project can result in data loss.
            #[prost(message, tag = "2")]
            Local(Local),
        }
    }
    /// Config that customers are allowed to define for GDCE system add-ons.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SystemAddonsConfig {
        /// Optional. Config for Ingress.
        #[prost(message, optional, tag = "1")]
        pub ingress: ::core::option::Option<system_addons_config::Ingress>,
    }
    /// Nested message and enum types in `SystemAddonsConfig`.
    pub mod system_addons_config {
        /// Config for the Ingress add-on which allows customers to create an Ingress
        /// object to manage external access to the servers in a cluster. The add-on
        /// consists of istiod and istio-ingress.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Ingress {
            /// Optional. Whether Ingress is disabled.
            #[prost(bool, tag = "1")]
            pub disabled: bool,
            /// Optional. Ingress VIP.
            #[prost(string, tag = "2")]
            pub ipv4_vip: ::prost::alloc::string::String,
        }
    }
    /// Configuration for Customer-managed KMS key support for remote control plane
    /// cluster disk encryption.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ControlPlaneEncryption {
        /// Immutable. The Cloud KMS CryptoKey e.g.
        /// projects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{cryptoKey}
        /// to use for protecting control plane disks. If not specified, a
        /// Google-managed key will be used instead.
        #[prost(string, tag = "1")]
        pub kms_key: ::prost::alloc::string::String,
        /// Output only. The Cloud KMS CryptoKeyVersion currently in use for
        /// protecting control plane disks. Only applicable if kms_key is set.
        #[prost(string, tag = "2")]
        pub kms_key_active_version: ::prost::alloc::string::String,
        /// Output only. Availability of the Cloud KMS CryptoKey. If not
        /// `KEY_AVAILABLE`, then nodes may go offline as they cannot access their
        /// local data. This can be caused by a lack of permissions to use the key,
        /// or if the key is disabled or deleted.
        #[prost(enumeration = "super::KmsKeyState", tag = "3")]
        pub kms_key_state: i32,
        /// Output only. Error status returned by Cloud KMS when using this key. This
        /// field may be populated only if `kms_key_state` is not
        /// `KMS_KEY_STATE_KEY_AVAILABLE`. If populated, this field contains the
        /// error status reported by Cloud KMS.
        #[prost(message, optional, tag = "4")]
        pub kms_status: ::core::option::Option<super::super::super::super::rpc::Status>,
    }
    /// A Maintenance Event is an operation that could cause temporary disruptions
    /// to the cluster workloads, including Google-driven or user-initiated cluster
    /// upgrades, user-initiated cluster configuration changes that require
    /// restarting nodes, etc.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MaintenanceEvent {
        /// Output only. UUID of the maintenance event.
        #[prost(string, tag = "1")]
        pub uuid: ::prost::alloc::string::String,
        /// Output only. The target version of the cluster.
        #[prost(string, tag = "2")]
        pub target_version: ::prost::alloc::string::String,
        /// Output only. The operation for running the maintenance event. Specified
        /// in the format projects/*/locations/*/operations/*. If the maintenance
        /// event is split into multiple operations (e.g. due to maintenance
        /// windows), the latest one is recorded.
        #[prost(string, tag = "3")]
        pub operation: ::prost::alloc::string::String,
        /// Output only. The type of the maintenance event.
        #[prost(enumeration = "maintenance_event::Type", tag = "4")]
        pub r#type: i32,
        /// Output only. The schedule of the maintenance event.
        #[prost(enumeration = "maintenance_event::Schedule", tag = "5")]
        pub schedule: i32,
        /// Output only. The state of the maintenance event.
        #[prost(enumeration = "maintenance_event::State", tag = "6")]
        pub state: i32,
        /// Output only. The time when the maintenance event request was created.
        #[prost(message, optional, tag = "7")]
        pub create_time: ::core::option::Option<::prost_types::Timestamp>,
        /// Output only. The time when the maintenance event started.
        #[prost(message, optional, tag = "8")]
        pub start_time: ::core::option::Option<::prost_types::Timestamp>,
        /// Output only. The time when the maintenance event ended, either
        /// successfully or not. If the maintenance event is split into multiple
        /// maintenance windows, end_time is only updated when the whole flow ends.
        #[prost(message, optional, tag = "9")]
        pub end_time: ::core::option::Option<::prost_types::Timestamp>,
        /// Output only. The time when the maintenance event message was updated.
        #[prost(message, optional, tag = "10")]
        pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    }
    /// Nested message and enum types in `MaintenanceEvent`.
    pub mod maintenance_event {
        /// Indicates the maintenance event type.
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
            /// Unspecified.
            Unspecified = 0,
            /// Upgrade initiated by users.
            UserInitiatedUpgrade = 1,
            /// Upgrade driven by Google.
            GoogleDrivenUpgrade = 2,
        }
        impl Type {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Type::Unspecified => "TYPE_UNSPECIFIED",
                    Type::UserInitiatedUpgrade => "USER_INITIATED_UPGRADE",
                    Type::GoogleDrivenUpgrade => "GOOGLE_DRIVEN_UPGRADE",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                    "USER_INITIATED_UPGRADE" => Some(Self::UserInitiatedUpgrade),
                    "GOOGLE_DRIVEN_UPGRADE" => Some(Self::GoogleDrivenUpgrade),
                    _ => None,
                }
            }
        }
        /// Indicates when the maintenance event should be performed.
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
        pub enum Schedule {
            /// Unspecified.
            Unspecified = 0,
            /// Immediately after receiving the request.
            Immediately = 1,
        }
        impl Schedule {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Schedule::Unspecified => "SCHEDULE_UNSPECIFIED",
                    Schedule::Immediately => "IMMEDIATELY",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "SCHEDULE_UNSPECIFIED" => Some(Self::Unspecified),
                    "IMMEDIATELY" => Some(Self::Immediately),
                    _ => None,
                }
            }
        }
        /// Indicates the maintenance event state.
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
            /// Unspecified.
            Unspecified = 0,
            /// The maintenance event is ongoing. The cluster might be unusable.
            Reconciling = 1,
            /// The maintenance event succeeded.
            Succeeded = 2,
            /// The maintenance event failed.
            Failed = 3,
        }
        impl State {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    State::Unspecified => "STATE_UNSPECIFIED",
                    State::Reconciling => "RECONCILING",
                    State::Succeeded => "SUCCEEDED",
                    State::Failed => "FAILED",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                    "RECONCILING" => Some(Self::Reconciling),
                    "SUCCEEDED" => Some(Self::Succeeded),
                    "FAILED" => Some(Self::Failed),
                    _ => None,
                }
            }
        }
    }
    /// Configuration of the cluster survivability, e.g., for the case when network
    /// connectivity is lost.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SurvivabilityConfig {
        /// Optional. Time period that allows the cluster nodes to be rebooted and
        /// become functional without network connectivity to Google. The default 0
        /// means not allowed. The maximum is 7 days.
        #[prost(message, optional, tag = "1")]
        pub offline_reboot_ttl: ::core::option::Option<::prost_types::Duration>,
    }
    /// Indicates the status of the cluster.
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
    pub enum Status {
        /// Status unknown.
        Unspecified = 0,
        /// The cluster is being created.
        Provisioning = 1,
        /// The cluster is created and fully usable.
        Running = 2,
        /// The cluster is being deleted.
        Deleting = 3,
        /// The status indicates that some errors occurred while reconciling/deleting
        /// the cluster.
        Error = 4,
        /// The cluster is undergoing some work such as version upgrades, etc.
        Reconciling = 5,
    }
    impl Status {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Status::Unspecified => "STATUS_UNSPECIFIED",
                Status::Provisioning => "PROVISIONING",
                Status::Running => "RUNNING",
                Status::Deleting => "DELETING",
                Status::Error => "ERROR",
                Status::Reconciling => "RECONCILING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATUS_UNSPECIFIED" => Some(Self::Unspecified),
                "PROVISIONING" => Some(Self::Provisioning),
                "RUNNING" => Some(Self::Running),
                "DELETING" => Some(Self::Deleting),
                "ERROR" => Some(Self::Error),
                "RECONCILING" => Some(Self::Reconciling),
                _ => None,
            }
        }
    }
    /// The release channel a cluster is subscribed to.
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
    pub enum ReleaseChannel {
        /// Unspecified release channel. This will default to the REGULAR channel.
        Unspecified = 0,
        /// No release channel.
        None = 1,
        /// Regular release channel.
        Regular = 2,
    }
    impl ReleaseChannel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ReleaseChannel::Unspecified => "RELEASE_CHANNEL_UNSPECIFIED",
                ReleaseChannel::None => "NONE",
                ReleaseChannel::Regular => "REGULAR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RELEASE_CHANNEL_UNSPECIFIED" => Some(Self::Unspecified),
                "NONE" => Some(Self::None),
                "REGULAR" => Some(Self::Regular),
                _ => None,
            }
        }
    }
}
/// Cluster-wide networking configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterNetworking {
    /// Required. All pods in the cluster are assigned an RFC1918 IPv4 address from
    /// these blocks. Only a single block is supported. This field cannot be
    /// changed after creation.
    #[prost(string, repeated, tag = "1")]
    pub cluster_ipv4_cidr_blocks: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// Required. All services in the cluster are assigned an RFC1918 IPv4 address
    /// from these blocks. Only a single block is supported. This field cannot be
    /// changed after creation.
    #[prost(string, repeated, tag = "2")]
    pub services_ipv4_cidr_blocks: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
}
/// Fleet related configuration.
///
/// Fleets are a Google Cloud concept for logically organizing clusters,
/// letting you use and manage multi-cluster capabilities and apply
/// consistent policies across your systems.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fleet {
    /// Required. The name of the Fleet host project where this cluster will be
    /// registered.
    ///
    /// Project names are formatted as
    /// `projects/<project-number>`.
    #[prost(string, tag = "1")]
    pub project: ::prost::alloc::string::String,
    /// Output only. The name of the managed Hub Membership resource associated to
    /// this cluster.
    ///
    /// Membership names are formatted as
    /// `projects/<project-number>/locations/global/membership/<cluster-id>`.
    #[prost(string, tag = "2")]
    pub membership: ::prost::alloc::string::String,
}
/// A user principal for an RBAC policy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterUser {
    /// Required. An active Google username.
    #[prost(string, tag = "1")]
    pub username: ::prost::alloc::string::String,
}
/// RBAC policy that will be applied and managed by GEC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Authorization {
    /// Required. User that will be granted the cluster-admin role on the cluster,
    /// providing full access to the cluster. Currently, this is a singular field,
    /// but will be expanded to allow multiple admins in the future.
    #[prost(message, optional, tag = "1")]
    pub admin_users: ::core::option::Option<ClusterUser>,
}
/// A set of Kubernetes nodes in a cluster with common configuration and
/// specification.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodePool {
    /// Required. The resource name of the node pool.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The time when the node pool was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the node pool was last updated.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Labels associated with this resource.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Name of the Google Distributed Cloud Edge zone where this node pool will be
    /// created. For example: `us-central1-edge-customer-a`.
    #[prost(string, tag = "8")]
    pub node_location: ::prost::alloc::string::String,
    /// Required. The number of nodes in the pool.
    #[prost(int32, tag = "6")]
    pub node_count: i32,
    /// Only machines matching this filter will be allowed to join the node pool.
    /// The filtering language accepts strings like "name=<name>", and is
    /// documented in more detail in [AIP-160](<https://google.aip.dev/160>).
    #[prost(string, tag = "7")]
    pub machine_filter: ::prost::alloc::string::String,
    /// Optional. Local disk encryption options. This field is only used when
    /// enabling CMEK support.
    #[prost(message, optional, tag = "9")]
    pub local_disk_encryption: ::core::option::Option<node_pool::LocalDiskEncryption>,
    /// Output only. The lowest release version among all worker nodes.
    #[prost(string, tag = "10")]
    pub node_version: ::prost::alloc::string::String,
    /// Optional. Configuration for each node in the NodePool
    #[prost(message, optional, tag = "11")]
    pub node_config: ::core::option::Option<node_pool::NodeConfig>,
}
/// Nested message and enum types in `NodePool`.
pub mod node_pool {
    /// Configuration for CMEK support for edge machine local disk encryption.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LocalDiskEncryption {
        /// Immutable. The Cloud KMS CryptoKey e.g.
        /// projects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{cryptoKey}
        /// to use for protecting node local disks. If not specified, a
        /// Google-managed key will be used instead.
        #[prost(string, tag = "1")]
        pub kms_key: ::prost::alloc::string::String,
        /// Output only. The Cloud KMS CryptoKeyVersion currently in use for
        /// protecting node local disks. Only applicable if kms_key is set.
        #[prost(string, tag = "2")]
        pub kms_key_active_version: ::prost::alloc::string::String,
        /// Output only. Availability of the Cloud KMS CryptoKey. If not
        /// `KEY_AVAILABLE`, then nodes may go offline as they cannot access their
        /// local data. This can be caused by a lack of permissions to use the key,
        /// or if the key is disabled or deleted.
        #[prost(enumeration = "super::KmsKeyState", tag = "3")]
        pub kms_key_state: i32,
        /// Output only. Error status returned by Cloud KMS when using this key. This
        /// field may be populated only if `kms_key_state` is not
        /// `KMS_KEY_STATE_KEY_AVAILABLE`. If populated, this field contains the
        /// error status reported by Cloud KMS.
        #[prost(message, optional, tag = "4")]
        pub kms_status: ::core::option::Option<super::super::super::super::rpc::Status>,
    }
    /// Configuration for each node in the NodePool
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NodeConfig {
        /// Optional. The Kubernetes node labels
        #[prost(map = "string, string", tag = "1")]
        pub labels: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
    }
}
/// A Google Distributed Cloud Edge machine capable of acting as a Kubernetes
/// node.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Machine {
    /// Required. The resource name of the machine.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The time when the node pool was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the node pool was last updated.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Labels associated with this resource.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Canonical resource name of the node that this machine is responsible for
    /// hosting e.g.
    /// projects/{project}/locations/{location}/clusters/{cluster_id}/nodePools/{pool_id}/{node},
    /// Or empty if the machine is not assigned to assume the role of a node.
    ///
    /// For control plane nodes hosted on edge machines, this will return
    /// the following format:
    ///    "projects/{project}/locations/{location}/clusters/{cluster_id}/controlPlaneNodes/{node}".
    #[prost(string, tag = "5")]
    pub hosted_node: ::prost::alloc::string::String,
    /// The Google Distributed Cloud Edge zone of this machine.
    #[prost(string, tag = "6")]
    pub zone: ::prost::alloc::string::String,
    /// Output only. The software version of the machine.
    #[prost(string, tag = "7")]
    pub version: ::prost::alloc::string::String,
    /// Output only. Whether the machine is disabled. If disabled, the machine is
    /// unable to enter service.
    #[prost(bool, tag = "8")]
    pub disabled: bool,
}
/// A VPN connection .
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VpnConnection {
    /// Required. The resource name of VPN connection
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The time when the VPN connection was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the VPN connection was last updated.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Labels associated with this resource.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// NAT gateway IP, or WAN IP address. If a customer has multiple NAT IPs, the
    /// customer needs to configure NAT such that only one external IP maps to the
    /// GMEC Anthos cluster. This is empty if NAT is not used.
    #[prost(string, tag = "5")]
    pub nat_gateway_ip: ::prost::alloc::string::String,
    /// Dynamic routing mode of the VPC network, `regional` or `global`.
    #[deprecated]
    #[prost(enumeration = "vpn_connection::BgpRoutingMode", tag = "6")]
    pub bgp_routing_mode: i32,
    /// The canonical Cluster name to connect to. It is in the form of
    /// projects/{project}/locations/{location}/clusters/{cluster}.
    #[prost(string, tag = "7")]
    pub cluster: ::prost::alloc::string::String,
    /// The network ID of VPC to connect to.
    #[prost(string, tag = "8")]
    pub vpc: ::prost::alloc::string::String,
    /// Optional. Project detail of the VPC network. Required if VPC is in a
    /// different project than the cluster project.
    #[prost(message, optional, tag = "11")]
    pub vpc_project: ::core::option::Option<vpn_connection::VpcProject>,
    /// Whether this VPN connection has HA enabled on cluster side. If enabled,
    /// when creating VPN connection we will attempt to use 2 ANG floating IPs.
    #[prost(bool, tag = "9")]
    pub enable_high_availability: bool,
    /// Optional. The VPN connection Cloud Router name.
    #[prost(string, tag = "12")]
    pub router: ::prost::alloc::string::String,
    /// Output only. The created connection details.
    #[prost(message, optional, tag = "10")]
    pub details: ::core::option::Option<vpn_connection::Details>,
}
/// Nested message and enum types in `VpnConnection`.
pub mod vpn_connection {
    /// Project detail of the VPC network.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct VpcProject {
        /// The project of the VPC to connect to. If not specified, it is the same as
        /// the cluster project.
        #[prost(string, tag = "1")]
        pub project_id: ::prost::alloc::string::String,
        /// Optional. The service account in the VPC project configured by user. It
        /// is used to create/delete Cloud Router and Cloud HA VPNs for VPN
        /// connection. If this SA is changed during/after a VPN connection is
        /// created, you need to remove the Cloud Router and Cloud VPN resources in
        /// |project_id|. It is in the form of
        /// service-{project_number}@gcp-sa-edgecontainer.iam.gserviceaccount.com.
        #[deprecated]
        #[prost(string, tag = "2")]
        pub service_account: ::prost::alloc::string::String,
    }
    /// The created connection details.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Details {
        /// The state of this connection.
        #[prost(enumeration = "details::State", tag = "1")]
        pub state: i32,
        /// The error message. This is only populated when state=ERROR.
        #[prost(string, tag = "2")]
        pub error: ::prost::alloc::string::String,
        /// The Cloud Router info.
        #[prost(message, optional, tag = "3")]
        pub cloud_router: ::core::option::Option<details::CloudRouter>,
        /// Each connection has multiple Cloud VPN gateways.
        #[prost(message, repeated, tag = "4")]
        pub cloud_vpns: ::prost::alloc::vec::Vec<details::CloudVpn>,
    }
    /// Nested message and enum types in `Details`.
    pub mod details {
        /// The Cloud Router info.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct CloudRouter {
            /// The associated Cloud Router name.
            #[prost(string, tag = "1")]
            pub name: ::prost::alloc::string::String,
        }
        /// The Cloud VPN info.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct CloudVpn {
            /// The created Cloud VPN gateway name.
            #[prost(string, tag = "1")]
            pub gateway: ::prost::alloc::string::String,
        }
        /// The current connection state.
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
            /// Unknown.
            Unspecified = 0,
            /// Connected.
            Connected = 1,
            /// Still connecting.
            Connecting = 2,
            /// Error occurred.
            Error = 3,
        }
        impl State {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    State::Unspecified => "STATE_UNSPECIFIED",
                    State::Connected => "STATE_CONNECTED",
                    State::Connecting => "STATE_CONNECTING",
                    State::Error => "STATE_ERROR",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                    "STATE_CONNECTED" => Some(Self::Connected),
                    "STATE_CONNECTING" => Some(Self::Connecting),
                    "STATE_ERROR" => Some(Self::Error),
                    _ => None,
                }
            }
        }
    }
    /// Routing mode.
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
    pub enum BgpRoutingMode {
        /// Unknown.
        Unspecified = 0,
        /// Regional mode.
        Regional = 1,
        /// Global mode.
        Global = 2,
    }
    impl BgpRoutingMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                BgpRoutingMode::Unspecified => "BGP_ROUTING_MODE_UNSPECIFIED",
                BgpRoutingMode::Regional => "REGIONAL",
                BgpRoutingMode::Global => "GLOBAL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BGP_ROUTING_MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "REGIONAL" => Some(Self::Regional),
                "GLOBAL" => Some(Self::Global),
                _ => None,
            }
        }
    }
}
/// Metadata for a given
/// [google.cloud.location.Location][google.cloud.location.Location].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationMetadata {
    /// The set of available Google Distributed Cloud Edge zones in the location.
    /// The map is keyed by the lowercase ID of each zone.
    #[prost(map = "string, message", tag = "1")]
    pub available_zones: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ZoneMetadata,
    >,
}
/// A Google Distributed Cloud Edge zone where edge machines are located.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ZoneMetadata {
    /// Quota for resources in this zone.
    #[prost(message, repeated, tag = "1")]
    pub quota: ::prost::alloc::vec::Vec<Quota>,
    /// The map keyed by rack name and has value of RackType.
    #[prost(map = "string, enumeration(zone_metadata::RackType)", tag = "2")]
    pub rack_types: ::std::collections::HashMap<::prost::alloc::string::String, i32>,
}
/// Nested message and enum types in `ZoneMetadata`.
pub mod zone_metadata {
    /// Type of the rack.
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
    pub enum RackType {
        /// Unspecified rack type, single rack also belongs to this type.
        Unspecified = 0,
        /// Base rack type, a pair of two modified Config-1 racks containing
        /// Aggregation switches.
        Base = 1,
        /// Expansion rack type, also known as standalone racks,
        /// added by customers on demand.
        Expansion = 2,
    }
    impl RackType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RackType::Unspecified => "RACK_TYPE_UNSPECIFIED",
                RackType::Base => "BASE",
                RackType::Expansion => "EXPANSION",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RACK_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "BASE" => Some(Self::Base),
                "EXPANSION" => Some(Self::Expansion),
                _ => None,
            }
        }
    }
}
/// Represents quota for Edge Container resources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Quota {
    /// Name of the quota metric.
    #[prost(string, tag = "1")]
    pub metric: ::prost::alloc::string::String,
    /// Quota limit for this metric.
    #[prost(double, tag = "2")]
    pub limit: f64,
    /// Current usage of this metric.
    #[prost(double, tag = "3")]
    pub usage: f64,
}
/// Maintenance policy configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaintenancePolicy {
    /// Specifies the maintenance window in which maintenance may be performed.
    #[prost(message, optional, tag = "1")]
    pub window: ::core::option::Option<MaintenanceWindow>,
}
/// Maintenance window configuration
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaintenanceWindow {
    /// Configuration of a recurring maintenance window.
    #[prost(message, optional, tag = "1")]
    pub recurring_window: ::core::option::Option<RecurringTimeWindow>,
}
/// Represents an arbitrary window of time that recurs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecurringTimeWindow {
    /// The window of the first recurrence.
    #[prost(message, optional, tag = "1")]
    pub window: ::core::option::Option<TimeWindow>,
    /// An RRULE (<https://tools.ietf.org/html/rfc5545#section-3.8.5.3>) for how
    /// this window recurs. They go on for the span of time between the start and
    /// end time.
    #[prost(string, tag = "2")]
    pub recurrence: ::prost::alloc::string::String,
}
/// Represents an arbitrary window of time.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeWindow {
    /// The time that the window first starts.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time that the window ends. The end time must take place after the
    /// start time.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Server configuration for supported versions and release channels.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerConfig {
    /// Output only. Mapping from release channel to channel config.
    #[prost(map = "string, message", tag = "1")]
    pub channels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ChannelConfig,
    >,
    /// Output only. Supported versions, e.g.: \["1.4.0", "1.5.0"\].
    #[prost(message, repeated, tag = "2")]
    pub versions: ::prost::alloc::vec::Vec<Version>,
    /// Output only. Default version, e.g.: "1.4.0".
    #[prost(string, tag = "3")]
    pub default_version: ::prost::alloc::string::String,
}
/// Configuration for a release channel.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelConfig {
    /// Output only. Default version for this release channel, e.g.: "1.4.0".
    #[prost(string, tag = "1")]
    pub default_version: ::prost::alloc::string::String,
}
/// Version of a cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Version {
    /// Output only. Name of the version, e.g.: "1.4.0".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Represents the accessibility state of a customer-managed KMS key used for
/// CMEK integration.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum KmsKeyState {
    /// Unspecified.
    Unspecified = 0,
    /// The key is available for use, and dependent resources should be accessible.
    KeyAvailable = 1,
    /// The key is unavailable for an unspecified reason. Dependent resources may
    /// be inaccessible.
    KeyUnavailable = 2,
}
impl KmsKeyState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            KmsKeyState::Unspecified => "KMS_KEY_STATE_UNSPECIFIED",
            KmsKeyState::KeyAvailable => "KMS_KEY_STATE_KEY_AVAILABLE",
            KmsKeyState::KeyUnavailable => "KMS_KEY_STATE_KEY_UNAVAILABLE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "KMS_KEY_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "KMS_KEY_STATE_KEY_AVAILABLE" => Some(Self::KeyAvailable),
            "KMS_KEY_STATE_KEY_UNAVAILABLE" => Some(Self::KeyUnavailable),
            _ => None,
        }
    }
}
/// Long-running operation metadata for Edge Container API methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Server-defined resource path for the target of the operation.
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// The verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: ::prost::alloc::string::String,
    /// Human-readable status of the operation, if any.
    #[prost(string, tag = "5")]
    pub status_message: ::prost::alloc::string::String,
    /// Identifies whether the user has requested cancellation of the operation.
    /// Operations that have successfully been cancelled have [Operation.error][]
    /// value with a [google.rpc.Status.code][google.rpc.Status.code] of 1,
    /// corresponding to `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
    /// Warnings that do not block the operation, but still hold relevant
    /// information for the end user to receive.
    #[prost(string, repeated, tag = "8")]
    pub warnings: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Lists clusters in a location.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListClustersRequest {
    /// Required. The parent location, which owns this collection of clusters.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to list.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token received from previous list request.
    /// A page token received from previous list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Only resources matching this filter will be listed.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Specifies the order in which resources will be listed.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// List of clusters in a location.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListClustersResponse {
    /// Clusters in the location.
    #[prost(message, repeated, tag = "1")]
    pub clusters: ::prost::alloc::vec::Vec<Cluster>,
    /// A token to retrieve next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Gets a cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClusterRequest {
    /// Required. The resource name of the cluster.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Creates a cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateClusterRequest {
    /// Required. The parent location where this cluster will be created.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. A client-specified unique identifier for the cluster.
    #[prost(string, tag = "2")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Required. The cluster to create.
    #[prost(message, optional, tag = "3")]
    pub cluster: ::core::option::Option<Cluster>,
    /// A unique identifier for this request. Restricted to 36 ASCII characters. A
    /// random UUID is recommended. This request is only idempotent if
    /// `request_id` is provided.
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Updates a cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateClusterRequest {
    /// Field mask is used to specify the fields to be overwritten in the
    /// Cluster resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The updated cluster.
    #[prost(message, optional, tag = "2")]
    pub cluster: ::core::option::Option<Cluster>,
    /// A unique identifier for this request. Restricted to 36 ASCII characters.
    /// A random UUID is recommended.
    /// This request is only idempotent if `request_id` is provided.
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Upgrades a cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpgradeClusterRequest {
    /// Required. The resource name of the cluster.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The version the cluster is going to be upgraded to.
    #[prost(string, tag = "2")]
    pub target_version: ::prost::alloc::string::String,
    /// The schedule for the upgrade.
    #[prost(enumeration = "upgrade_cluster_request::Schedule", tag = "3")]
    pub schedule: i32,
    /// A unique identifier for this request. Restricted to 36 ASCII characters. A
    /// random UUID is recommended. This request is only idempotent if
    /// `request_id` is provided.
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Nested message and enum types in `UpgradeClusterRequest`.
pub mod upgrade_cluster_request {
    /// Represents the schedule about when the cluster is going to be upgraded.
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
    pub enum Schedule {
        /// Unspecified. The default is to upgrade the cluster immediately which is
        /// the only option today.
        Unspecified = 0,
        /// The cluster is going to be upgraded immediately after receiving the
        /// request.
        Immediately = 1,
    }
    impl Schedule {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Schedule::Unspecified => "SCHEDULE_UNSPECIFIED",
                Schedule::Immediately => "IMMEDIATELY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SCHEDULE_UNSPECIFIED" => Some(Self::Unspecified),
                "IMMEDIATELY" => Some(Self::Immediately),
                _ => None,
            }
        }
    }
}
/// Deletes a cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteClusterRequest {
    /// Required. The resource name of the cluster.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A unique identifier for this request. Restricted to 36 ASCII characters. A
    /// random UUID is recommended. This request is only idempotent if
    /// `request_id` is provided.
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Generates an access token for a cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateAccessTokenRequest {
    /// Required. The resource name of the cluster.
    #[prost(string, tag = "1")]
    pub cluster: ::prost::alloc::string::String,
}
/// An access token for a cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateAccessTokenResponse {
    /// Output only. Access token to authenticate to k8s api-server.
    #[prost(string, tag = "1")]
    pub access_token: ::prost::alloc::string::String,
    /// Output only. Timestamp at which the token will expire.
    #[prost(message, optional, tag = "2")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Generates an offline credential(offline) for a cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateOfflineCredentialRequest {
    /// Required. The resource name of the cluster.
    #[prost(string, tag = "1")]
    pub cluster: ::prost::alloc::string::String,
}
/// An offline credential for a cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateOfflineCredentialResponse {
    /// Output only. Client certificate to authenticate to k8s api-server.
    #[prost(string, tag = "1")]
    pub client_certificate: ::prost::alloc::string::String,
    /// Output only. Client private key to authenticate to k8s api-server.
    #[prost(string, tag = "2")]
    pub client_key: ::prost::alloc::string::String,
    /// Output only. Client's identity.
    #[prost(string, tag = "3")]
    pub user_id: ::prost::alloc::string::String,
    /// Output only. Timestamp at which this credential will expire.
    #[prost(message, optional, tag = "4")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Lists node pools in a cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNodePoolsRequest {
    /// Required. The parent cluster, which owns this collection of node pools.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to list.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token received from previous list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Only resources matching this filter will be listed.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Specifies the order in which resources will be listed.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// List of node pools in a cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNodePoolsResponse {
    /// Node pools in the cluster.
    #[prost(message, repeated, tag = "1")]
    pub node_pools: ::prost::alloc::vec::Vec<NodePool>,
    /// A token to retrieve next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Gets a node pool.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNodePoolRequest {
    /// Required. The resource name of the node pool.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Creates a node pool.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateNodePoolRequest {
    /// Required. The parent cluster where this node pool will be created.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. A client-specified unique identifier for the node pool.
    #[prost(string, tag = "2")]
    pub node_pool_id: ::prost::alloc::string::String,
    /// Required. The node pool to create.
    #[prost(message, optional, tag = "3")]
    pub node_pool: ::core::option::Option<NodePool>,
    /// A unique identifier for this request. Restricted to 36 ASCII characters. A
    /// random UUID is recommended. This request is only idempotent if
    /// `request_id` is provided.
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Updates a node pool.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateNodePoolRequest {
    /// Field mask is used to specify the fields to be overwritten in the
    /// NodePool resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The updated node pool.
    #[prost(message, optional, tag = "2")]
    pub node_pool: ::core::option::Option<NodePool>,
    /// A unique identifier for this request. Restricted to 36 ASCII characters. A
    /// random UUID is recommended. This request is only idempotent if
    /// `request_id` is provided.
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Deletes a node pool.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteNodePoolRequest {
    /// Required. The resource name of the node pool.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A unique identifier for this request. Restricted to 36 ASCII characters. A
    /// random UUID is recommended. This request is only idempotent if
    /// `request_id` is provided.
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Lists machines in a site.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMachinesRequest {
    /// Required. The parent site, which owns this collection of machines.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to list.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token received from previous list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Only resources matching this filter will be listed.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Specifies the order in which resources will be listed.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// List of machines in a site.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMachinesResponse {
    /// Machines in the site.
    #[prost(message, repeated, tag = "1")]
    pub machines: ::prost::alloc::vec::Vec<Machine>,
    /// A token to retrieve next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Gets a machine.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMachineRequest {
    /// Required. The resource name of the machine.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Lists VPN connections.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVpnConnectionsRequest {
    /// Required. The parent location, which owns this collection of VPN
    /// connections.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to list.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token received from previous list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Only resources matching this filter will be listed.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Specifies the order in which resources will be listed.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// List of VPN connections in a location.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVpnConnectionsResponse {
    /// VpnConnections in the location.
    #[prost(message, repeated, tag = "1")]
    pub vpn_connections: ::prost::alloc::vec::Vec<VpnConnection>,
    /// A token to retrieve next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Gets a VPN connection.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVpnConnectionRequest {
    /// Required. The resource name of the vpn connection.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Creates a VPN connection.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateVpnConnectionRequest {
    /// Required. The parent location where this vpn connection will be created.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The VPN connection identifier.
    #[prost(string, tag = "2")]
    pub vpn_connection_id: ::prost::alloc::string::String,
    /// Required. The VPN connection to create.
    #[prost(message, optional, tag = "3")]
    pub vpn_connection: ::core::option::Option<VpnConnection>,
    /// A unique identifier for this request. Restricted to 36 ASCII characters. A
    /// random UUID is recommended. This request is only idempotent if
    /// `request_id` is provided.
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Deletes a vpn connection.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteVpnConnectionRequest {
    /// Required. The resource name of the vpn connection.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A unique identifier for this request. Restricted to 36 ASCII characters. A
    /// random UUID is recommended. This request is only idempotent if
    /// `request_id` is provided.
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Gets the server config.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServerConfigRequest {
    /// Required. The name (project and location) of the server config to get,
    /// specified in the format `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod edge_container_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// EdgeContainer API provides management of Kubernetes Clusters on Google Edge
    /// Cloud deployments.
    #[derive(Debug, Clone)]
    pub struct EdgeContainerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl EdgeContainerClient<tonic::transport::Channel> {
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
    impl<T> EdgeContainerClient<T>
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
        ) -> EdgeContainerClient<InterceptedService<T, F>>
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
            EdgeContainerClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists Clusters in a given project and location.
        pub async fn list_clusters(
            &mut self,
            request: impl tonic::IntoRequest<super::ListClustersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListClustersResponse>,
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
                "/google.cloud.edgecontainer.v1.EdgeContainer/ListClusters",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgecontainer.v1.EdgeContainer",
                        "ListClusters",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single Cluster.
        pub async fn get_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::GetClusterRequest>,
        ) -> std::result::Result<tonic::Response<super::Cluster>, tonic::Status> {
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
                "/google.cloud.edgecontainer.v1.EdgeContainer/GetCluster",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgecontainer.v1.EdgeContainer",
                        "GetCluster",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new Cluster in a given project and location.
        pub async fn create_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateClusterRequest>,
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
                "/google.cloud.edgecontainer.v1.EdgeContainer/CreateCluster",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgecontainer.v1.EdgeContainer",
                        "CreateCluster",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the parameters of a single Cluster.
        pub async fn update_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateClusterRequest>,
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
                "/google.cloud.edgecontainer.v1.EdgeContainer/UpdateCluster",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgecontainer.v1.EdgeContainer",
                        "UpdateCluster",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Upgrades a single cluster.
        pub async fn upgrade_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::UpgradeClusterRequest>,
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
                "/google.cloud.edgecontainer.v1.EdgeContainer/UpgradeCluster",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgecontainer.v1.EdgeContainer",
                        "UpgradeCluster",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single Cluster.
        pub async fn delete_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteClusterRequest>,
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
                "/google.cloud.edgecontainer.v1.EdgeContainer/DeleteCluster",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgecontainer.v1.EdgeContainer",
                        "DeleteCluster",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Generates an access token for a Cluster.
        pub async fn generate_access_token(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateAccessTokenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateAccessTokenResponse>,
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
                "/google.cloud.edgecontainer.v1.EdgeContainer/GenerateAccessToken",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgecontainer.v1.EdgeContainer",
                        "GenerateAccessToken",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Generates an offline credential for a Cluster.
        pub async fn generate_offline_credential(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateOfflineCredentialRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateOfflineCredentialResponse>,
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
                "/google.cloud.edgecontainer.v1.EdgeContainer/GenerateOfflineCredential",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgecontainer.v1.EdgeContainer",
                        "GenerateOfflineCredential",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists NodePools in a given project and location.
        pub async fn list_node_pools(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNodePoolsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListNodePoolsResponse>,
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
                "/google.cloud.edgecontainer.v1.EdgeContainer/ListNodePools",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgecontainer.v1.EdgeContainer",
                        "ListNodePools",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single NodePool.
        pub async fn get_node_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNodePoolRequest>,
        ) -> std::result::Result<tonic::Response<super::NodePool>, tonic::Status> {
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
                "/google.cloud.edgecontainer.v1.EdgeContainer/GetNodePool",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgecontainer.v1.EdgeContainer",
                        "GetNodePool",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new NodePool in a given project and location.
        pub async fn create_node_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateNodePoolRequest>,
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
                "/google.cloud.edgecontainer.v1.EdgeContainer/CreateNodePool",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgecontainer.v1.EdgeContainer",
                        "CreateNodePool",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the parameters of a single NodePool.
        pub async fn update_node_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateNodePoolRequest>,
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
                "/google.cloud.edgecontainer.v1.EdgeContainer/UpdateNodePool",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgecontainer.v1.EdgeContainer",
                        "UpdateNodePool",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single NodePool.
        pub async fn delete_node_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteNodePoolRequest>,
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
                "/google.cloud.edgecontainer.v1.EdgeContainer/DeleteNodePool",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgecontainer.v1.EdgeContainer",
                        "DeleteNodePool",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists Machines in a given project and location.
        pub async fn list_machines(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMachinesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMachinesResponse>,
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
                "/google.cloud.edgecontainer.v1.EdgeContainer/ListMachines",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgecontainer.v1.EdgeContainer",
                        "ListMachines",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single Machine.
        pub async fn get_machine(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMachineRequest>,
        ) -> std::result::Result<tonic::Response<super::Machine>, tonic::Status> {
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
                "/google.cloud.edgecontainer.v1.EdgeContainer/GetMachine",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgecontainer.v1.EdgeContainer",
                        "GetMachine",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists VPN connections in a given project and location.
        pub async fn list_vpn_connections(
            &mut self,
            request: impl tonic::IntoRequest<super::ListVpnConnectionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListVpnConnectionsResponse>,
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
                "/google.cloud.edgecontainer.v1.EdgeContainer/ListVpnConnections",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgecontainer.v1.EdgeContainer",
                        "ListVpnConnections",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single VPN connection.
        pub async fn get_vpn_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVpnConnectionRequest>,
        ) -> std::result::Result<tonic::Response<super::VpnConnection>, tonic::Status> {
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
                "/google.cloud.edgecontainer.v1.EdgeContainer/GetVpnConnection",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgecontainer.v1.EdgeContainer",
                        "GetVpnConnection",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new VPN connection in a given project and location.
        pub async fn create_vpn_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateVpnConnectionRequest>,
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
                "/google.cloud.edgecontainer.v1.EdgeContainer/CreateVpnConnection",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgecontainer.v1.EdgeContainer",
                        "CreateVpnConnection",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single VPN connection.
        pub async fn delete_vpn_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteVpnConnectionRequest>,
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
                "/google.cloud.edgecontainer.v1.EdgeContainer/DeleteVpnConnection",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgecontainer.v1.EdgeContainer",
                        "DeleteVpnConnection",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the server config.
        pub async fn get_server_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetServerConfigRequest>,
        ) -> std::result::Result<tonic::Response<super::ServerConfig>, tonic::Status> {
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
                "/google.cloud.edgecontainer.v1.EdgeContainer/GetServerConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgecontainer.v1.EdgeContainer",
                        "GetServerConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
