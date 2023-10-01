/// A workstation cluster resource in the Cloud Workstations API.
///
/// Defines a group of workstations in a particular region and the
/// VPC network they're attached to.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkstationCluster {
    /// Full name of this workstation cluster.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Human-readable name for this workstation cluster.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. A system-assigned unique identifier for this workstation
    /// cluster.
    #[prost(string, tag = "3")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. Indicates whether this workstation cluster is currently being
    /// updated to match its intended state.
    #[prost(bool, tag = "4")]
    pub reconciling: bool,
    /// Optional. Client-specified annotations.
    #[prost(map = "string, string", tag = "5")]
    pub annotations: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Optional.
    /// [Labels](<https://cloud.google.com/workstations/docs/label-resources>) that
    /// are applied to the workstation cluster and that are also propagated to the
    /// underlying Compute Engine resources.
    #[prost(map = "string, string", tag = "15")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. Time when this workstation cluster was created.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when this workstation cluster was most recently updated.
    #[prost(message, optional, tag = "7")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when this workstation cluster was soft-deleted.
    #[prost(message, optional, tag = "8")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Checksum computed by the server. May be sent on update and delete
    /// requests to make sure that the client has an up-to-date value before
    /// proceeding.
    #[prost(string, tag = "9")]
    pub etag: ::prost::alloc::string::String,
    /// Immutable. Name of the Compute Engine network in which instances associated
    /// with this workstation cluster will be created.
    #[prost(string, tag = "10")]
    pub network: ::prost::alloc::string::String,
    /// Immutable. Name of the Compute Engine subnetwork in which instances
    /// associated with this workstation cluster will be created. Must be part of
    /// the subnetwork specified for this workstation cluster.
    #[prost(string, tag = "11")]
    pub subnetwork: ::prost::alloc::string::String,
    /// Output only. The private IP address of the control plane for this
    /// workstation cluster. Workstation VMs need access to this IP address to work
    /// with the service, so make sure that your firewall rules allow egress from
    /// the workstation VMs to this address.
    #[prost(string, tag = "16")]
    pub control_plane_ip: ::prost::alloc::string::String,
    /// Optional. Configuration for private workstation cluster.
    #[prost(message, optional, tag = "12")]
    pub private_cluster_config: ::core::option::Option<
        workstation_cluster::PrivateClusterConfig,
    >,
    /// Output only. Whether this workstation cluster is in degraded mode, in which
    /// case it may require user action to restore full functionality. Details can
    /// be found in
    /// [conditions][google.cloud.workstations.v1.WorkstationCluster.conditions].
    #[prost(bool, tag = "13")]
    pub degraded: bool,
    /// Output only. Status conditions describing the workstation cluster's current
    /// state.
    #[prost(message, repeated, tag = "14")]
    pub conditions: ::prost::alloc::vec::Vec<super::super::super::rpc::Status>,
}
/// Nested message and enum types in `WorkstationCluster`.
pub mod workstation_cluster {
    /// Configuration options for private workstation clusters.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PrivateClusterConfig {
        /// Immutable. Whether Workstations endpoint is private.
        #[prost(bool, tag = "1")]
        pub enable_private_endpoint: bool,
        /// Output only. Hostname for the workstation cluster. This field will be
        /// populated only when private endpoint is enabled. To access workstations
        /// in the workstation cluster, create a new DNS zone mapping this domain
        /// name to an internal IP address and a forwarding rule mapping that address
        /// to the service attachment.
        #[prost(string, tag = "2")]
        pub cluster_hostname: ::prost::alloc::string::String,
        /// Output only. Service attachment URI for the workstation cluster. The
        /// service attachemnt is created when private endpoint is enabled. To access
        /// workstations in the workstation cluster, configure access to the managed
        /// service using [Private Service
        /// Connect](<https://cloud.google.com/vpc/docs/configure-private-service-connect-services>).
        #[prost(string, tag = "3")]
        pub service_attachment_uri: ::prost::alloc::string::String,
        /// Optional. Additional projects that are allowed to attach to the
        /// workstation cluster's service attachment. By default, the workstation
        /// cluster's project and the VPC host project (if different) are allowed.
        #[prost(string, repeated, tag = "4")]
        pub allowed_projects: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
/// A workstation configuration resource in the Cloud Workstations API.
///
/// Workstation configurations act as templates for workstations. The workstation
/// configuration defines details such as the workstation virtual machine (VM)
/// instance type, persistent storage, container image defining environment,
/// which IDE or Code Editor to use, and more. Administrators and platform teams
/// can also use [Identity and Access Management
/// (IAM)](<https://cloud.google.com/iam/docs/overview>) rules to grant access to
/// teams or to individual developers.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkstationConfig {
    /// Full name of this workstation configuration.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Human-readable name for this workstation configuration.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. A system-assigned unique identifier for this workstation
    /// configuration.
    #[prost(string, tag = "3")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. Indicates whether this workstation configuration is currently
    /// being updated to match its intended state.
    #[prost(bool, tag = "4")]
    pub reconciling: bool,
    /// Optional. Client-specified annotations.
    #[prost(map = "string, string", tag = "5")]
    pub annotations: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Optional.
    /// [Labels](<https://cloud.google.com/workstations/docs/label-resources>) that
    /// are applied to the workstation configuration and that are also propagated
    /// to the underlying Compute Engine resources.
    #[prost(map = "string, string", tag = "18")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. Time when this workstation configuration was created.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when this workstation configuration was most recently
    /// updated.
    #[prost(message, optional, tag = "7")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when this workstation configuration was soft-deleted.
    #[prost(message, optional, tag = "8")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Checksum computed by the server. May be sent on update and delete
    /// requests to make sure that the client has an up-to-date value before
    /// proceeding.
    #[prost(string, tag = "9")]
    pub etag: ::prost::alloc::string::String,
    /// Optional. Number of seconds to wait before automatically stopping a
    /// workstation after it last received user traffic.
    ///
    /// A value of `"0s"` indicates that Cloud Workstations VMs created with this
    /// configuration should never time out due to idleness.
    /// Provide
    /// [duration](<https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#duration>)
    /// terminated by `s` for seconds—for example, `"7200s"` (2 hours).
    /// The default is `"1200s"` (20 minutes).
    #[prost(message, optional, tag = "10")]
    pub idle_timeout: ::core::option::Option<::prost_types::Duration>,
    /// Optional. Number of seconds that a workstation can run until it is
    /// automatically shut down. We recommend that workstations be shut down daily
    /// to reduce costs and so that security updates can be applied upon restart.
    /// The
    /// [idle_timeout][google.cloud.workstations.v1.WorkstationConfig.idle_timeout]
    /// and
    /// [running_timeout][google.cloud.workstations.v1.WorkstationConfig.running_timeout]
    /// fields are independent of each other. Note that the
    /// [running_timeout][google.cloud.workstations.v1.WorkstationConfig.running_timeout]
    /// field shuts down VMs after the specified time, regardless of whether or not
    /// the VMs are idle.
    ///
    /// Provide duration terminated by `s` for seconds—for example, `"54000s"`
    /// (15 hours). Defaults to `"43200s"` (12 hours). A value of `"0s"` indicates
    /// that workstations using this configuration should never time out. If
    /// [encryption_key][google.cloud.workstations.v1.WorkstationConfig.encryption_key]
    /// is set, it must be greater than `"0s"` and less than
    /// `"86400s"` (24 hours).
    ///
    /// Warning: A value of `"0s"` indicates that Cloud Workstations VMs created
    /// with this configuration have no maximum running time. This is strongly
    /// discouraged because you incur costs and will not pick up security updates.
    #[prost(message, optional, tag = "11")]
    pub running_timeout: ::core::option::Option<::prost_types::Duration>,
    /// Optional. Runtime host for the workstation.
    #[prost(message, optional, tag = "12")]
    pub host: ::core::option::Option<workstation_config::Host>,
    /// Optional. Directories to persist across workstation sessions.
    #[prost(message, repeated, tag = "13")]
    pub persistent_directories: ::prost::alloc::vec::Vec<
        workstation_config::PersistentDirectory,
    >,
    /// Optional. Container that runs upon startup for each workstation using this
    /// workstation configuration.
    #[prost(message, optional, tag = "14")]
    pub container: ::core::option::Option<workstation_config::Container>,
    /// Immutable. Encrypts resources of this workstation configuration using a
    /// customer-managed encryption key (CMEK).
    ///
    /// If specified, the boot disk of the Compute Engine instance and the
    /// persistent disk are encrypted using this encryption key. If
    /// this field is not set, the disks are encrypted using a generated
    /// key. Customer-managed encryption keys do not protect disk metadata.
    ///
    /// If the customer-managed encryption key is rotated, when the workstation
    /// instance is stopped, the system attempts to recreate the
    /// persistent disk with the new version of the key. Be sure to keep
    /// older versions of the key until the persistent disk is recreated.
    /// Otherwise, data on the persistent disk might be lost.
    ///
    /// If the encryption key is revoked, the workstation session automatically
    /// stops within 7 hours.
    ///
    /// Immutable after the workstation configuration is created.
    #[prost(message, optional, tag = "17")]
    pub encryption_key: ::core::option::Option<
        workstation_config::CustomerEncryptionKey,
    >,
    /// Optional. Readiness checks to perform when starting a workstation using
    /// this workstation configuration. Mark a workstation as running only after
    /// all specified readiness checks return 200 status codes.
    #[prost(message, repeated, tag = "19")]
    pub readiness_checks: ::prost::alloc::vec::Vec<workstation_config::ReadinessCheck>,
    /// Optional. Immutable. Specifies the zones used to replicate the VM and disk
    /// resources within the region. If set, exactly two zones within the
    /// workstation cluster's region must be specified—for example,
    /// `\['us-central1-a', 'us-central1-f'\]`. If this field is empty, two default
    /// zones within the region are used.
    ///
    /// Immutable after the workstation configuration is created.
    #[prost(string, repeated, tag = "23")]
    pub replica_zones: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. Whether this resource is degraded, in which case it may
    /// require user action to restore full functionality. See also the
    /// [conditions][google.cloud.workstations.v1.WorkstationConfig.conditions]
    /// field.
    #[prost(bool, tag = "15")]
    pub degraded: bool,
    /// Output only. Status conditions describing the current resource state.
    #[prost(message, repeated, tag = "16")]
    pub conditions: ::prost::alloc::vec::Vec<super::super::super::rpc::Status>,
}
/// Nested message and enum types in `WorkstationConfig`.
pub mod workstation_config {
    /// Runtime host for a workstation.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Host {
        /// Type of host that will be used for the workstation's runtime.
        #[prost(oneof = "host::Config", tags = "1")]
        pub config: ::core::option::Option<host::Config>,
    }
    /// Nested message and enum types in `Host`.
    pub mod host {
        /// A runtime using a Compute Engine instance.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct GceInstance {
            /// Optional. The type of machine to use for VM instances—for example,
            /// `"e2-standard-4"`. For more information about machine types that
            /// Cloud Workstations supports, see the list of
            /// [available machine
            /// types](<https://cloud.google.com/workstations/docs/available-machine-types>).
            #[prost(string, tag = "1")]
            pub machine_type: ::prost::alloc::string::String,
            /// Optional. The email address of the service account for Cloud
            /// Workstations VMs created with this configuration. When specified, be
            /// sure that the service account has `logginglogEntries.create` permission
            /// on the project so it can write logs out to Cloud Logging. If using a
            /// custom container image, the service account must have permissions to
            /// pull the specified image.
            ///
            /// If you as the administrator want to be able to `ssh` into the
            /// underlying VM, you need to set this value to a service account
            /// for which you have the `iam.serviceAccounts.actAs` permission.
            /// Conversely, if you don't want anyone to be able to `ssh` into the
            /// underlying VM, use a service account where no one has that
            /// permission.
            ///
            /// If not set, VMs run with a service account provided by the
            /// Cloud Workstations service, and the image must be publicly
            /// accessible.
            #[prost(string, tag = "2")]
            pub service_account: ::prost::alloc::string::String,
            /// Optional. Scopes to grant to the
            /// [service_account][google.cloud.workstations.v1.WorkstationConfig.Host.GceInstance.service_account].
            /// Various scopes are automatically added based on feature usage. When
            /// specified, users of workstations under this configuration must have
            /// `iam.serviceAccounts.actAs` on the service account.
            #[prost(string, repeated, tag = "3")]
            pub service_account_scopes: ::prost::alloc::vec::Vec<
                ::prost::alloc::string::String,
            >,
            /// Optional. Network tags to add to the Compute Engine VMs backing the
            /// workstations. This option applies
            /// [network
            /// tags](<https://cloud.google.com/vpc/docs/add-remove-network-tags>) to VMs
            /// created with this configuration. These network tags enable the creation
            /// of [firewall
            /// rules](<https://cloud.google.com/workstations/docs/configure-firewall-rules>).
            #[prost(string, repeated, tag = "4")]
            pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            /// Optional. The number of VMs that the system should keep idle so that
            /// new workstations can be started quickly for new users. Defaults to `0`
            /// in the API.
            #[prost(int32, tag = "5")]
            pub pool_size: i32,
            /// Output only. Number of instances currently available in the pool for
            /// faster workstation startup.
            #[prost(int32, tag = "12")]
            pub pooled_instances: i32,
            /// Optional. When set to true, disables public IP addresses for VMs. If
            /// you disable public IP addresses, you must set up Private Google Access
            /// or Cloud NAT on your network. If you use Private Google Access and you
            /// use `private.googleapis.com` or `restricted.googleapis.com` for
            /// Container Registry and Artifact Registry, make sure that you set
            /// up DNS records for domains `*.gcr.io` and `*.pkg.dev`.
            /// Defaults to false (VMs have public IP addresses).
            #[prost(bool, tag = "6")]
            pub disable_public_ip_addresses: bool,
            /// Optional. Whether to enable nested virtualization on Cloud Workstations
            /// VMs created under this workstation configuration.
            ///
            /// Nested virtualization lets you run virtual machine (VM) instances
            /// inside your workstation. Before enabling nested virtualization,
            /// consider the following important considerations. Cloud Workstations
            /// instances are subject to the [same restrictions as Compute Engine
            /// instances](<https://cloud.google.com/compute/docs/instances/nested-virtualization/overview#restrictions>):
            ///
            /// * **Organization policy**: projects, folders, or
            /// organizations may be restricted from creating nested VMs if the
            /// **Disable VM nested virtualization** constraint is enforced in
            /// the organization policy. For more information, see the
            /// Compute Engine section,
            /// [Checking whether nested virtualization is
            /// allowed](<https://cloud.google.com/compute/docs/instances/nested-virtualization/managing-constraint#checking_whether_nested_virtualization_is_allowed>).
            /// * **Performance**: nested VMs might experience a 10% or greater
            /// decrease in performance for workloads that are CPU-bound and
            /// possibly greater than a 10% decrease for workloads that are
            /// input/output bound.
            /// * **Machine Type**: nested virtualization can only be enabled on
            /// workstation configurations that specify a
            /// [machine_type][google.cloud.workstations.v1.WorkstationConfig.Host.GceInstance.machine_type]
            /// in the N1 or N2 machine series.
            /// * **GPUs**: nested virtualization may not be enabled on workstation
            /// configurations with accelerators.
            /// * **Operating System**: Because
            /// [Container-Optimized
            /// OS](<https://cloud.google.com/compute/docs/images/os-details#container-optimized_os_cos>)
            /// does not support nested virtualization, when nested virtualization is
            /// enabled, the underlying Compute Engine VM instances boot from an
            /// [Ubuntu
            /// LTS](<https://cloud.google.com/compute/docs/images/os-details#ubuntu_lts>)
            /// image.
            #[prost(bool, tag = "7")]
            pub enable_nested_virtualization: bool,
            /// Optional. A set of Compute Engine Shielded instance options.
            #[prost(message, optional, tag = "8")]
            pub shielded_instance_config: ::core::option::Option<
                gce_instance::GceShieldedInstanceConfig,
            >,
            /// Optional. A set of Compute Engine Confidential VM instance options.
            #[prost(message, optional, tag = "10")]
            pub confidential_instance_config: ::core::option::Option<
                gce_instance::GceConfidentialInstanceConfig,
            >,
            /// Optional. The size of the boot disk for the VM in gigabytes (GB).
            /// The minimum boot disk size is `30` GB. Defaults to `50` GB.
            #[prost(int32, tag = "9")]
            pub boot_disk_size_gb: i32,
        }
        /// Nested message and enum types in `GceInstance`.
        pub mod gce_instance {
            /// A set of Compute Engine Shielded instance options.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct GceShieldedInstanceConfig {
                /// Optional. Whether the instance has Secure Boot enabled.
                #[prost(bool, tag = "1")]
                pub enable_secure_boot: bool,
                /// Optional. Whether the instance has the vTPM enabled.
                #[prost(bool, tag = "2")]
                pub enable_vtpm: bool,
                /// Optional. Whether the instance has integrity monitoring enabled.
                #[prost(bool, tag = "3")]
                pub enable_integrity_monitoring: bool,
            }
            /// A set of Compute Engine Confidential VM instance options.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct GceConfidentialInstanceConfig {
                /// Optional. Whether the instance has confidential compute enabled.
                #[prost(bool, tag = "1")]
                pub enable_confidential_compute: bool,
            }
        }
        /// Type of host that will be used for the workstation's runtime.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Config {
            /// Specifies a Compute Engine instance as the host.
            #[prost(message, tag = "1")]
            GceInstance(GceInstance),
        }
    }
    /// A directory to persist across workstation sessions.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PersistentDirectory {
        /// Optional. Location of this directory in the running workstation.
        #[prost(string, tag = "1")]
        pub mount_path: ::prost::alloc::string::String,
        /// How a persistent directory should be implemented.
        #[prost(oneof = "persistent_directory::DirectoryType", tags = "2")]
        pub directory_type: ::core::option::Option<persistent_directory::DirectoryType>,
    }
    /// Nested message and enum types in `PersistentDirectory`.
    pub mod persistent_directory {
        /// A PersistentDirectory backed by a Compute Engine regional persistent
        /// disk. The
        /// [persistent_directories][google.cloud.workstations.v1.WorkstationConfig.persistent_directories]
        /// field is repeated, but it may contain only one entry. It creates a
        /// [persistent
        /// disk](<https://cloud.google.com/compute/docs/disks/persistent-disks>) that
        /// mounts to the workstation VM at `/home` when the session starts and
        /// detaches when the session ends. If this field is empty, workstations
        /// created with this configuration do not have a persistent home
        /// directory.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct GceRegionalPersistentDisk {
            /// Optional. The GB capacity of a persistent home directory for each
            /// workstation created with this configuration. Must be empty if
            /// [source_snapshot][google.cloud.workstations.v1.WorkstationConfig.PersistentDirectory.GceRegionalPersistentDisk.source_snapshot]
            /// is set.
            ///
            /// Valid values are `10`, `50`, `100`, `200`, `500`, or `1000`.
            /// Defaults to `200`. If less than `200` GB, the
            /// [disk_type][google.cloud.workstations.v1.WorkstationConfig.PersistentDirectory.GceRegionalPersistentDisk.disk_type]
            /// must be
            /// `"pd-balanced"` or `"pd-ssd"`.
            #[prost(int32, tag = "1")]
            pub size_gb: i32,
            /// Optional. Type of file system that the disk should be formatted with.
            /// The workstation image must support this file system type. Must be empty
            /// if
            /// [source_snapshot][google.cloud.workstations.v1.WorkstationConfig.PersistentDirectory.GceRegionalPersistentDisk.source_snapshot]
            /// is set. Defaults to `"ext4"`.
            #[prost(string, tag = "2")]
            pub fs_type: ::prost::alloc::string::String,
            /// Optional. The [type of the persistent
            /// disk](<https://cloud.google.com/compute/docs/disks#disk-types>) for the
            /// home directory. Defaults to `"pd-standard"`.
            #[prost(string, tag = "3")]
            pub disk_type: ::prost::alloc::string::String,
            /// Optional. Name of the snapshot to use as the source for the disk. If
            /// set,
            /// [size_gb][google.cloud.workstations.v1.WorkstationConfig.PersistentDirectory.GceRegionalPersistentDisk.size_gb]
            /// and
            /// [fs_type][google.cloud.workstations.v1.WorkstationConfig.PersistentDirectory.GceRegionalPersistentDisk.fs_type]
            /// must be empty.
            #[prost(string, tag = "5")]
            pub source_snapshot: ::prost::alloc::string::String,
            /// Optional. Whether the persistent disk should be deleted when the
            /// workstation is deleted. Valid values are `DELETE` and `RETAIN`.
            /// Defaults to `DELETE`.
            #[prost(
                enumeration = "gce_regional_persistent_disk::ReclaimPolicy",
                tag = "4"
            )]
            pub reclaim_policy: i32,
        }
        /// Nested message and enum types in `GceRegionalPersistentDisk`.
        pub mod gce_regional_persistent_disk {
            /// Value representing what should happen to the disk after the workstation
            /// is deleted.
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
            pub enum ReclaimPolicy {
                /// Do not use.
                Unspecified = 0,
                /// Delete the persistent disk when deleting the workstation.
                Delete = 1,
                /// Keep the persistent disk when deleting the workstation.
                /// An administrator must manually delete the disk.
                Retain = 2,
            }
            impl ReclaimPolicy {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        ReclaimPolicy::Unspecified => "RECLAIM_POLICY_UNSPECIFIED",
                        ReclaimPolicy::Delete => "DELETE",
                        ReclaimPolicy::Retain => "RETAIN",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "RECLAIM_POLICY_UNSPECIFIED" => Some(Self::Unspecified),
                        "DELETE" => Some(Self::Delete),
                        "RETAIN" => Some(Self::Retain),
                        _ => None,
                    }
                }
            }
        }
        /// How a persistent directory should be implemented.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum DirectoryType {
            /// A PersistentDirectory backed by a Compute Engine persistent disk.
            #[prost(message, tag = "2")]
            GcePd(GceRegionalPersistentDisk),
        }
    }
    /// A Docker container.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Container {
        /// Optional. A Docker container image that defines a custom environment.
        ///
        /// Cloud Workstations provides a number of
        /// [preconfigured
        /// images](<https://cloud.google.com/workstations/docs/preconfigured-base-images>),
        /// but you can create your own
        /// [custom container
        /// images](<https://cloud.google.com/workstations/docs/custom-container-images>).
        /// If using a private image, the `host.gceInstance.serviceAccount` field
        /// must be specified in the workstation configuration and must have
        /// permission to pull the specified image. Otherwise, the image must be
        /// publicly accessible.
        #[prost(string, tag = "1")]
        pub image: ::prost::alloc::string::String,
        /// Optional. If set, overrides the default ENTRYPOINT specified by the
        /// image.
        #[prost(string, repeated, tag = "2")]
        pub command: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Optional. Arguments passed to the entrypoint.
        #[prost(string, repeated, tag = "3")]
        pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Optional. Environment variables passed to the container's entrypoint.
        #[prost(map = "string, string", tag = "4")]
        pub env: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
        /// Optional. If set, overrides the default DIR specified by the image.
        #[prost(string, tag = "5")]
        pub working_dir: ::prost::alloc::string::String,
        /// Optional. If set, overrides the USER specified in the image with the
        /// given uid.
        #[prost(int32, tag = "6")]
        pub run_as_user: i32,
    }
    /// A customer-managed encryption key (CMEK) for the Compute Engine
    /// resources of the associated workstation configuration. Specify the name of
    /// your Cloud KMS encryption key and the default service account.
    /// We recommend that you use a separate service account and follow
    /// [Cloud KMS best
    /// practices](<https://cloud.google.com/kms/docs/separation-of-duties>).
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CustomerEncryptionKey {
        /// Immutable. The name of the Google Cloud KMS encryption key. For example,
        /// `"projects/PROJECT_ID/locations/REGION/keyRings/KEY_RING/cryptoKeys/KEY_NAME"`.
        /// The key must be in the same region as the workstation configuration.
        #[prost(string, tag = "1")]
        pub kms_key: ::prost::alloc::string::String,
        /// Immutable. The service account to use with the specified
        /// KMS key. We recommend that you use a separate service account
        /// and follow KMS best practices. For more information, see
        /// [Separation of
        /// duties](<https://cloud.google.com/kms/docs/separation-of-duties>) and
        /// `gcloud kms keys add-iam-policy-binding`
        /// [`--member`](<https://cloud.google.com/sdk/gcloud/reference/kms/keys/add-iam-policy-binding#--member>).
        #[prost(string, tag = "2")]
        pub kms_key_service_account: ::prost::alloc::string::String,
    }
    /// A readiness check to be performed on a workstation.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ReadinessCheck {
        /// Optional. Path to which the request should be sent.
        #[prost(string, tag = "1")]
        pub path: ::prost::alloc::string::String,
        /// Optional. Port to which the request should be sent.
        #[prost(int32, tag = "2")]
        pub port: i32,
    }
}
/// A single instance of a developer workstation with its own persistent storage.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Workstation {
    /// Full name of this workstation.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Human-readable name for this workstation.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. A system-assigned unique identifier for this workstation.
    #[prost(string, tag = "3")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. Indicates whether this workstation is currently being updated
    /// to match its intended state.
    #[prost(bool, tag = "4")]
    pub reconciling: bool,
    /// Optional. Client-specified annotations.
    #[prost(map = "string, string", tag = "5")]
    pub annotations: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Optional.
    /// [Labels](<https://cloud.google.com/workstations/docs/label-resources>) that
    /// are applied to the workstation and that are also propagated to the
    /// underlying Compute Engine resources.
    #[prost(map = "string, string", tag = "13")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. Time when this workstation was created.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when this workstation was most recently updated.
    #[prost(message, optional, tag = "7")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when this workstation was most recently successfully
    /// started, regardless of the workstation's initial state.
    #[prost(message, optional, tag = "14")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when this workstation was soft-deleted.
    #[prost(message, optional, tag = "8")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Checksum computed by the server. May be sent on update and delete
    /// requests to make sure that the client has an up-to-date value before
    /// proceeding.
    #[prost(string, tag = "9")]
    pub etag: ::prost::alloc::string::String,
    /// Output only. Current state of the workstation.
    #[prost(enumeration = "workstation::State", tag = "10")]
    pub state: i32,
    /// Output only. Host to which clients can send HTTPS traffic that will be
    /// received by the workstation. Authorized traffic will be received to the
    /// workstation as HTTP on port 80. To send traffic to a different port,
    /// clients may prefix the host with the destination port in the format
    /// `{port}-{host}`.
    #[prost(string, tag = "11")]
    pub host: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Workstation`.
pub mod workstation {
    /// Whether a workstation is running and ready to receive user requests.
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
        /// Do not use.
        Unspecified = 0,
        /// The workstation is not yet ready to accept requests from users but will
        /// be soon.
        Starting = 1,
        /// The workstation is ready to accept requests from users.
        Running = 2,
        /// The workstation is being stopped.
        Stopping = 3,
        /// The workstation is stopped and will not be able to receive requests until
        /// it is started.
        Stopped = 4,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Starting => "STATE_STARTING",
                State::Running => "STATE_RUNNING",
                State::Stopping => "STATE_STOPPING",
                State::Stopped => "STATE_STOPPED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "STATE_STARTING" => Some(Self::Starting),
                "STATE_RUNNING" => Some(Self::Running),
                "STATE_STOPPING" => Some(Self::Stopping),
                "STATE_STOPPED" => Some(Self::Stopped),
                _ => None,
            }
        }
    }
}
/// Request message for GetWorkstationCluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWorkstationClusterRequest {
    /// Required. Name of the requested resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListWorkstationClusters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkstationClustersRequest {
    /// Required. Parent resource name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. next_page_token value returned from a previous List request, if
    /// any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListWorkstationClusters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkstationClustersResponse {
    /// The requested workstation clusters.
    #[prost(message, repeated, tag = "1")]
    pub workstation_clusters: ::prost::alloc::vec::Vec<WorkstationCluster>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Unreachable resources.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for creating a CreateWorkstationCluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWorkstationClusterRequest {
    /// Required. Parent resource name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. ID to use for the workstation cluster.
    #[prost(string, tag = "2")]
    pub workstation_cluster_id: ::prost::alloc::string::String,
    /// Required. Workstation cluster to create.
    #[prost(message, optional, tag = "3")]
    pub workstation_cluster: ::core::option::Option<WorkstationCluster>,
    /// Optional. If set, validate the request and preview the review, but do not
    /// actually apply it.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// Request message for UpdateWorkstationCluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateWorkstationClusterRequest {
    /// Required. Workstation cluster to update.
    #[prost(message, optional, tag = "1")]
    pub workstation_cluster: ::core::option::Option<WorkstationCluster>,
    /// Required. Mask that specifies which fields in the workstation cluster
    /// should be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Optional. If set, validate the request and preview the review, but do not
    /// actually apply it.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
    /// Optional. If set, and the workstation cluster is not found, a new
    /// workstation cluster will be created. In this situation, update_mask is
    /// ignored.
    #[prost(bool, tag = "4")]
    pub allow_missing: bool,
}
/// Message for deleting a workstation cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteWorkstationClusterRequest {
    /// Required. Name of the workstation cluster to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. If set, validate the request and preview the review, but do not
    /// apply it.
    #[prost(bool, tag = "2")]
    pub validate_only: bool,
    /// Optional. If set, the request will be rejected if the latest version of the
    /// workstation cluster on the server does not have this ETag.
    #[prost(string, tag = "3")]
    pub etag: ::prost::alloc::string::String,
    /// Optional. If set, any workstation configurations and workstations in the
    /// workstation cluster are also deleted. Otherwise, the request only
    /// works if the workstation cluster has no configurations or workstations.
    #[prost(bool, tag = "4")]
    pub force: bool,
}
/// Request message for GetWorkstationConfig.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWorkstationConfigRequest {
    /// Required. Name of the requested resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListWorkstationConfigs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkstationConfigsRequest {
    /// Required. Parent resource name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. next_page_token value returned from a previous List request, if
    /// any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListWorkstationConfigs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkstationConfigsResponse {
    /// The requested configs.
    #[prost(message, repeated, tag = "1")]
    pub workstation_configs: ::prost::alloc::vec::Vec<WorkstationConfig>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Unreachable resources.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for ListUsableWorkstationConfigs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUsableWorkstationConfigsRequest {
    /// Required. Parent resource name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. next_page_token value returned from a previous List request, if
    /// any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListUsableWorkstationConfigs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUsableWorkstationConfigsResponse {
    /// The requested configs.
    #[prost(message, repeated, tag = "1")]
    pub workstation_configs: ::prost::alloc::vec::Vec<WorkstationConfig>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Unreachable resources.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for creating a CreateWorkstationConfig.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWorkstationConfigRequest {
    /// Required. Parent resource name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. ID to use for the workstation configuration.
    #[prost(string, tag = "2")]
    pub workstation_config_id: ::prost::alloc::string::String,
    /// Required. Config to create.
    #[prost(message, optional, tag = "3")]
    pub workstation_config: ::core::option::Option<WorkstationConfig>,
    /// Optional. If set, validate the request and preview the review, but do not
    /// actually apply it.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// Request message for UpdateWorkstationConfig.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateWorkstationConfigRequest {
    /// Required. Config to update.
    #[prost(message, optional, tag = "1")]
    pub workstation_config: ::core::option::Option<WorkstationConfig>,
    /// Required. Mask specifying which fields in the workstation configuration
    /// should be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Optional. If set, validate the request and preview the review, but do not
    /// actually apply it.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
    /// Optional. If set and the workstation configuration is not found, a new
    /// workstation configuration will be created. In this situation,
    /// update_mask is ignored.
    #[prost(bool, tag = "4")]
    pub allow_missing: bool,
}
/// Message for deleting a workstation configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteWorkstationConfigRequest {
    /// Required. Name of the workstation configuration to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. If set, validate the request and preview the review, but do not
    /// actually apply it.
    #[prost(bool, tag = "2")]
    pub validate_only: bool,
    /// Optional. If set, the request is rejected if the latest version of the
    /// workstation configuration on the server does not have this ETag.
    #[prost(string, tag = "3")]
    pub etag: ::prost::alloc::string::String,
    /// Optional. If set, any workstations in the workstation configuration are
    /// also deleted. Otherwise, the request works only if the workstation
    /// configuration has no workstations.
    #[prost(bool, tag = "4")]
    pub force: bool,
}
/// Request message for GetWorkstation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWorkstationRequest {
    /// Required. Name of the requested resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListWorkstations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkstationsRequest {
    /// Required. Parent resource name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. next_page_token value returned from a previous List request, if
    /// any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListWorkstations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkstationsResponse {
    /// The requested workstations.
    #[prost(message, repeated, tag = "1")]
    pub workstations: ::prost::alloc::vec::Vec<Workstation>,
    /// Optional. Token to retrieve the next page of results, or empty if there are
    /// no more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Optional. Unreachable resources.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for ListUsableWorkstations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUsableWorkstationsRequest {
    /// Required. Parent resource name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. next_page_token value returned from a previous List request, if
    /// any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListUsableWorkstations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUsableWorkstationsResponse {
    /// The requested workstations.
    #[prost(message, repeated, tag = "1")]
    pub workstations: ::prost::alloc::vec::Vec<Workstation>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Unreachable resources.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for creating a CreateWorkstation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWorkstationRequest {
    /// Required. Parent resource name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. ID to use for the workstation.
    #[prost(string, tag = "2")]
    pub workstation_id: ::prost::alloc::string::String,
    /// Required. Workstation to create.
    #[prost(message, optional, tag = "3")]
    pub workstation: ::core::option::Option<Workstation>,
    /// Optional. If set, validate the request and preview the review, but do not
    /// actually apply it.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// Request message for UpdateWorkstation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateWorkstationRequest {
    /// Required. Workstation to update.
    #[prost(message, optional, tag = "1")]
    pub workstation: ::core::option::Option<Workstation>,
    /// Required. Mask specifying which fields in the workstation configuration
    /// should be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Optional. If set, validate the request and preview the review, but do not
    /// actually apply it.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
    /// Optional. If set and the workstation configuration is not found, a new
    /// workstation configuration is created. In this situation, update_mask
    /// is ignored.
    #[prost(bool, tag = "4")]
    pub allow_missing: bool,
}
/// Request message for DeleteWorkstation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteWorkstationRequest {
    /// Required. Name of the workstation to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. If set, validate the request and preview the review, but do not
    /// actually apply it.
    #[prost(bool, tag = "2")]
    pub validate_only: bool,
    /// Optional. If set, the request will be rejected if the latest version of the
    /// workstation on the server does not have this ETag.
    #[prost(string, tag = "3")]
    pub etag: ::prost::alloc::string::String,
}
/// Request message for StartWorkstation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartWorkstationRequest {
    /// Required. Name of the workstation to start.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. If set, validate the request and preview the review, but do not
    /// actually apply it.
    #[prost(bool, tag = "2")]
    pub validate_only: bool,
    /// Optional. If set, the request will be rejected if the latest version of the
    /// workstation on the server does not have this ETag.
    #[prost(string, tag = "3")]
    pub etag: ::prost::alloc::string::String,
}
/// Request message for StopWorkstation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopWorkstationRequest {
    /// Required. Name of the workstation to stop.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. If set, validate the request and preview the review, but do not
    /// actually apply it.
    #[prost(bool, tag = "2")]
    pub validate_only: bool,
    /// Optional. If set, the request will be rejected if the latest version of the
    /// workstation on the server does not have this ETag.
    #[prost(string, tag = "3")]
    pub etag: ::prost::alloc::string::String,
}
/// Request message for GenerateAccessToken.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateAccessTokenRequest {
    /// Required. Name of the workstation for which the access token should be
    /// generated.
    #[prost(string, tag = "1")]
    pub workstation: ::prost::alloc::string::String,
    /// Desired expiration or lifetime of the access token.
    #[prost(oneof = "generate_access_token_request::Expiration", tags = "2, 3")]
    pub expiration: ::core::option::Option<generate_access_token_request::Expiration>,
}
/// Nested message and enum types in `GenerateAccessTokenRequest`.
pub mod generate_access_token_request {
    /// Desired expiration or lifetime of the access token.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Expiration {
        /// Desired expiration time of the access token. This value must
        /// be at most 24 hours in the future. If a value is not specified, the
        /// token's expiration time will be set to a default value of 1 hour in the
        /// future.
        #[prost(message, tag = "2")]
        ExpireTime(::prost_types::Timestamp),
        /// Desired lifetime duration of the access token. This value must
        /// be at most 24 hours. If a value is not specified, the token's lifetime
        /// will be set to a default value of 1 hour.
        #[prost(message, tag = "3")]
        Ttl(::prost_types::Duration),
    }
}
/// Response message for GenerateAccessToken.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateAccessTokenResponse {
    /// The generated bearer access token. To use this token, include it in an
    /// Authorization header of an HTTP request sent to the associated
    /// workstation's hostname—for example, `Authorization: Bearer
    /// <access_token>`.
    #[prost(string, tag = "1")]
    pub access_token: ::prost::alloc::string::String,
    /// Time at which the generated token will expire.
    #[prost(message, optional, tag = "2")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Metadata for long-running operations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// Output only. Time that the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time that the operation finished running.
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
    /// of the operation.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod workstations_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for interacting with Cloud Workstations.
    #[derive(Debug, Clone)]
    pub struct WorkstationsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl WorkstationsClient<tonic::transport::Channel> {
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
    impl<T> WorkstationsClient<T>
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
        ) -> WorkstationsClient<InterceptedService<T, F>>
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
            WorkstationsClient::new(InterceptedService::new(inner, interceptor))
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
        /// Returns the requested workstation cluster.
        pub async fn get_workstation_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::GetWorkstationClusterRequest>,
        ) -> std::result::Result<
            tonic::Response<super::WorkstationCluster>,
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
                "/google.cloud.workstations.v1.Workstations/GetWorkstationCluster",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workstations.v1.Workstations",
                        "GetWorkstationCluster",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns all workstation clusters in the specified location.
        pub async fn list_workstation_clusters(
            &mut self,
            request: impl tonic::IntoRequest<super::ListWorkstationClustersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListWorkstationClustersResponse>,
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
                "/google.cloud.workstations.v1.Workstations/ListWorkstationClusters",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workstations.v1.Workstations",
                        "ListWorkstationClusters",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new workstation cluster.
        pub async fn create_workstation_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateWorkstationClusterRequest>,
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
                "/google.cloud.workstations.v1.Workstations/CreateWorkstationCluster",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workstations.v1.Workstations",
                        "CreateWorkstationCluster",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates an existing workstation cluster.
        pub async fn update_workstation_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateWorkstationClusterRequest>,
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
                "/google.cloud.workstations.v1.Workstations/UpdateWorkstationCluster",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workstations.v1.Workstations",
                        "UpdateWorkstationCluster",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the specified workstation cluster.
        pub async fn delete_workstation_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteWorkstationClusterRequest>,
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
                "/google.cloud.workstations.v1.Workstations/DeleteWorkstationCluster",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workstations.v1.Workstations",
                        "DeleteWorkstationCluster",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the requested workstation configuration.
        pub async fn get_workstation_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetWorkstationConfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::WorkstationConfig>,
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
                "/google.cloud.workstations.v1.Workstations/GetWorkstationConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workstations.v1.Workstations",
                        "GetWorkstationConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns all workstation configurations in the specified cluster.
        pub async fn list_workstation_configs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListWorkstationConfigsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListWorkstationConfigsResponse>,
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
                "/google.cloud.workstations.v1.Workstations/ListWorkstationConfigs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workstations.v1.Workstations",
                        "ListWorkstationConfigs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns all workstation configurations in the specified cluster on which
        /// the caller has the "workstations.workstation.create" permission.
        pub async fn list_usable_workstation_configs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListUsableWorkstationConfigsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListUsableWorkstationConfigsResponse>,
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
                "/google.cloud.workstations.v1.Workstations/ListUsableWorkstationConfigs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workstations.v1.Workstations",
                        "ListUsableWorkstationConfigs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new workstation configuration.
        pub async fn create_workstation_config(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateWorkstationConfigRequest>,
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
                "/google.cloud.workstations.v1.Workstations/CreateWorkstationConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workstations.v1.Workstations",
                        "CreateWorkstationConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates an existing workstation configuration.
        pub async fn update_workstation_config(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateWorkstationConfigRequest>,
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
                "/google.cloud.workstations.v1.Workstations/UpdateWorkstationConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workstations.v1.Workstations",
                        "UpdateWorkstationConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the specified workstation configuration.
        pub async fn delete_workstation_config(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteWorkstationConfigRequest>,
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
                "/google.cloud.workstations.v1.Workstations/DeleteWorkstationConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workstations.v1.Workstations",
                        "DeleteWorkstationConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the requested workstation.
        pub async fn get_workstation(
            &mut self,
            request: impl tonic::IntoRequest<super::GetWorkstationRequest>,
        ) -> std::result::Result<tonic::Response<super::Workstation>, tonic::Status> {
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
                "/google.cloud.workstations.v1.Workstations/GetWorkstation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workstations.v1.Workstations",
                        "GetWorkstation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns all Workstations using the specified workstation configuration.
        pub async fn list_workstations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListWorkstationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListWorkstationsResponse>,
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
                "/google.cloud.workstations.v1.Workstations/ListWorkstations",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workstations.v1.Workstations",
                        "ListWorkstations",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns all workstations using the specified workstation configuration
        /// on which the caller has the "workstations.workstations.use" permission.
        pub async fn list_usable_workstations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListUsableWorkstationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListUsableWorkstationsResponse>,
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
                "/google.cloud.workstations.v1.Workstations/ListUsableWorkstations",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workstations.v1.Workstations",
                        "ListUsableWorkstations",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new workstation.
        pub async fn create_workstation(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateWorkstationRequest>,
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
                "/google.cloud.workstations.v1.Workstations/CreateWorkstation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workstations.v1.Workstations",
                        "CreateWorkstation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates an existing workstation.
        pub async fn update_workstation(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateWorkstationRequest>,
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
                "/google.cloud.workstations.v1.Workstations/UpdateWorkstation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workstations.v1.Workstations",
                        "UpdateWorkstation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the specified workstation.
        pub async fn delete_workstation(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteWorkstationRequest>,
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
                "/google.cloud.workstations.v1.Workstations/DeleteWorkstation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workstations.v1.Workstations",
                        "DeleteWorkstation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Starts running a workstation so that users can connect to it.
        pub async fn start_workstation(
            &mut self,
            request: impl tonic::IntoRequest<super::StartWorkstationRequest>,
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
                "/google.cloud.workstations.v1.Workstations/StartWorkstation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workstations.v1.Workstations",
                        "StartWorkstation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Stops running a workstation, reducing costs.
        pub async fn stop_workstation(
            &mut self,
            request: impl tonic::IntoRequest<super::StopWorkstationRequest>,
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
                "/google.cloud.workstations.v1.Workstations/StopWorkstation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workstations.v1.Workstations",
                        "StopWorkstation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns a short-lived credential that can be used to send authenticated and
        /// authorized traffic to a workstation.
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
                "/google.cloud.workstations.v1.Workstations/GenerateAccessToken",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workstations.v1.Workstations",
                        "GenerateAccessToken",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
