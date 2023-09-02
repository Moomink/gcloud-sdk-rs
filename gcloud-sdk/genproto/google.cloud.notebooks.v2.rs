/// Defines flags that are used to run the diagnostic tool
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiagnosticConfig {
    /// Required. User Cloud Storage bucket location (REQUIRED).
    /// Must be formatted with path prefix (`gs://$GCS_BUCKET`).
    ///
    /// Permissions:
    /// User Managed Notebooks:
    /// - storage.buckets.writer: Must be given to the project's service account
    ///    attached to VM.
    /// Google Managed Notebooks:
    /// - storage.buckets.writer: Must be given to the project's service account or
    ///    user credentials attached to VM depending on authentication mode.
    ///
    /// Cloud Storage bucket Log file will be written to
    /// `gs://$GCS_BUCKET/$RELATIVE_PATH/$VM_DATE_$TIME.tar.gz`
    #[prost(string, tag = "1")]
    pub gcs_bucket: ::prost::alloc::string::String,
    /// Optional. Defines the relative storage path in the Cloud Storage bucket
    /// where the diagnostic logs will be written: Default path will be the root
    /// directory of the Cloud Storage bucket
    /// (`gs://$GCS_BUCKET/$DATE_$TIME.tar.gz`) Example of full path where Log file
    /// will be written: `gs://$GCS_BUCKET/$RELATIVE_PATH/`
    #[prost(string, tag = "2")]
    pub relative_path: ::prost::alloc::string::String,
    /// Optional. Enables flag to repair service for instance
    #[prost(bool, tag = "3")]
    pub enable_repair_flag: bool,
    /// Optional. Enables flag to capture packets from the instance for 30 seconds
    #[prost(bool, tag = "4")]
    pub enable_packet_capture_flag: bool,
    /// Optional. Enables flag to copy all `/home/jupyter` folder contents
    #[prost(bool, tag = "5")]
    pub enable_copy_home_files_flag: bool,
}
/// The definition of an Event for a managed / semi-managed notebook instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    /// Optional. Event report time.
    #[prost(message, optional, tag = "1")]
    pub report_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Event type.
    #[prost(enumeration = "event::EventType", tag = "2")]
    pub r#type: i32,
    /// Optional. Event details. This field is used to pass event information.
    #[prost(map = "string, string", tag = "3")]
    pub details: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Nested message and enum types in `Event`.
pub mod event {
    /// The definition of the event types.
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
    pub enum EventType {
        /// Event is not specified.
        Unspecified = 0,
        /// The instance / runtime is idle
        Idle = 1,
        /// The instance / runtime is available.
        /// This event indicates that instance / runtime underlying compute is
        /// operational.
        Heartbeat = 2,
        /// The instance / runtime health is available.
        /// This event indicates that instance / runtime health information.
        Health = 3,
        /// The instance / runtime is available.
        /// This event allows instance / runtime to send Host maintenance
        /// information to Control Plane.
        /// <https://cloud.google.com/compute/docs/gpus/gpu-host-maintenance>
        Maintenance = 4,
        /// The instance / runtime is available.
        /// This event indicates that the instance had metadata that needs to be
        /// modified.
        MetadataChange = 5,
    }
    impl EventType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EventType::Unspecified => "EVENT_TYPE_UNSPECIFIED",
                EventType::Idle => "IDLE",
                EventType::Heartbeat => "HEARTBEAT",
                EventType::Health => "HEALTH",
                EventType::Maintenance => "MAINTENANCE",
                EventType::MetadataChange => "METADATA_CHANGE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "EVENT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "IDLE" => Some(Self::Idle),
                "HEARTBEAT" => Some(Self::Heartbeat),
                "HEALTH" => Some(Self::Health),
                "MAINTENANCE" => Some(Self::Maintenance),
                "METADATA_CHANGE" => Some(Self::MetadataChange),
                _ => None,
            }
        }
    }
}
/// The definition of a network interface resource attached to a VM.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkInterface {
    /// Optional. The name of the VPC that this VM instance is in.
    /// Format:
    /// `projects/{project_id}/global/networks/{network_id}`
    #[prost(string, tag = "1")]
    pub network: ::prost::alloc::string::String,
    /// Optional. The name of the subnet that this VM instance is in.
    /// Format:
    /// `projects/{project_id}/regions/{region}/subnetworks/{subnetwork_id}`
    #[prost(string, tag = "2")]
    pub subnet: ::prost::alloc::string::String,
    /// Optional. The type of vNIC to be used on this interface. This may be gVNIC
    /// or VirtioNet.
    #[prost(enumeration = "network_interface::NicType", tag = "3")]
    pub nic_type: i32,
}
/// Nested message and enum types in `NetworkInterface`.
pub mod network_interface {
    /// The type of vNIC driver.
    /// Default should be NIC_TYPE_UNSPECIFIED.
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
    pub enum NicType {
        /// No type specified.
        Unspecified = 0,
        /// VIRTIO
        VirtioNet = 1,
        /// GVNIC
        Gvnic = 2,
    }
    impl NicType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                NicType::Unspecified => "NIC_TYPE_UNSPECIFIED",
                NicType::VirtioNet => "VIRTIO_NET",
                NicType::Gvnic => "GVNIC",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NIC_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "VIRTIO_NET" => Some(Self::VirtioNet),
                "GVNIC" => Some(Self::Gvnic),
                _ => None,
            }
        }
    }
}
/// Definition of a custom Compute Engine virtual machine image for starting a
/// notebook instance with the environment installed directly on the VM.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VmImage {
    /// Required. The name of the Google Cloud project that this VM image belongs
    /// to. Format: `{project_id}`
    #[prost(string, tag = "1")]
    pub project: ::prost::alloc::string::String,
    /// The reference to an external Compute Engine VM image.
    #[prost(oneof = "vm_image::Image", tags = "2, 3")]
    pub image: ::core::option::Option<vm_image::Image>,
}
/// Nested message and enum types in `VmImage`.
pub mod vm_image {
    /// The reference to an external Compute Engine VM image.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Image {
        /// Optional. Use VM image name to find the image.
        #[prost(string, tag = "2")]
        Name(::prost::alloc::string::String),
        /// Optional. Use this VM image family to find the image; the newest image in
        /// this family will be used.
        #[prost(string, tag = "3")]
        Family(::prost::alloc::string::String),
    }
}
/// Definition of a container image for starting a notebook instance with the
/// environment installed in a container.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContainerImage {
    /// Required. The path to the container image repository. For example:
    /// `gcr.io/{project_id}/{image_name}`
    #[prost(string, tag = "1")]
    pub repository: ::prost::alloc::string::String,
    /// Optional. The tag of the container image. If not specified, this defaults
    /// to the latest tag.
    #[prost(string, tag = "2")]
    pub tag: ::prost::alloc::string::String,
}
/// An accelerator configuration for a VM instance
/// Definition of a hardware accelerator. Note that there is no check on `type`
/// and `core_count` combinations. TPUs are not supported.
/// See [GPUs on Compute
/// Engine](<https://cloud.google.com/compute/docs/gpus/#gpus-list>) to find a
/// valid combination.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcceleratorConfig {
    /// Optional. Type of this accelerator.
    #[prost(enumeration = "accelerator_config::AcceleratorType", tag = "1")]
    pub r#type: i32,
    /// Optional. Count of cores of this accelerator.
    #[prost(int64, tag = "2")]
    pub core_count: i64,
}
/// Nested message and enum types in `AcceleratorConfig`.
pub mod accelerator_config {
    /// Definition of the types of hardware accelerators that can be used on
    /// this instance.
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
    pub enum AcceleratorType {
        /// Accelerator type is not specified.
        Unspecified = 0,
        /// Accelerator type is Nvidia Tesla P100.
        NvidiaTeslaP100 = 2,
        /// Accelerator type is Nvidia Tesla V100.
        NvidiaTeslaV100 = 3,
        /// Accelerator type is Nvidia Tesla P4.
        NvidiaTeslaP4 = 4,
        /// Accelerator type is Nvidia Tesla T4.
        NvidiaTeslaT4 = 5,
        /// Accelerator type is Nvidia Tesla A100 - 40GB.
        NvidiaTeslaA100 = 11,
        /// Accelerator type is Nvidia Tesla A100 - 80GB.
        NvidiaA10080gb = 12,
        /// Accelerator type is Nvidia Tesla L4.
        NvidiaL4 = 13,
        /// Accelerator type is NVIDIA Tesla T4 Virtual Workstations.
        NvidiaTeslaT4Vws = 8,
        /// Accelerator type is NVIDIA Tesla P100 Virtual Workstations.
        NvidiaTeslaP100Vws = 9,
        /// Accelerator type is NVIDIA Tesla P4 Virtual Workstations.
        NvidiaTeslaP4Vws = 10,
    }
    impl AcceleratorType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AcceleratorType::Unspecified => "ACCELERATOR_TYPE_UNSPECIFIED",
                AcceleratorType::NvidiaTeslaP100 => "NVIDIA_TESLA_P100",
                AcceleratorType::NvidiaTeslaV100 => "NVIDIA_TESLA_V100",
                AcceleratorType::NvidiaTeslaP4 => "NVIDIA_TESLA_P4",
                AcceleratorType::NvidiaTeslaT4 => "NVIDIA_TESLA_T4",
                AcceleratorType::NvidiaTeslaA100 => "NVIDIA_TESLA_A100",
                AcceleratorType::NvidiaA10080gb => "NVIDIA_A100_80GB",
                AcceleratorType::NvidiaL4 => "NVIDIA_L4",
                AcceleratorType::NvidiaTeslaT4Vws => "NVIDIA_TESLA_T4_VWS",
                AcceleratorType::NvidiaTeslaP100Vws => "NVIDIA_TESLA_P100_VWS",
                AcceleratorType::NvidiaTeslaP4Vws => "NVIDIA_TESLA_P4_VWS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ACCELERATOR_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "NVIDIA_TESLA_P100" => Some(Self::NvidiaTeslaP100),
                "NVIDIA_TESLA_V100" => Some(Self::NvidiaTeslaV100),
                "NVIDIA_TESLA_P4" => Some(Self::NvidiaTeslaP4),
                "NVIDIA_TESLA_T4" => Some(Self::NvidiaTeslaT4),
                "NVIDIA_TESLA_A100" => Some(Self::NvidiaTeslaA100),
                "NVIDIA_A100_80GB" => Some(Self::NvidiaA10080gb),
                "NVIDIA_L4" => Some(Self::NvidiaL4),
                "NVIDIA_TESLA_T4_VWS" => Some(Self::NvidiaTeslaT4Vws),
                "NVIDIA_TESLA_P100_VWS" => Some(Self::NvidiaTeslaP100Vws),
                "NVIDIA_TESLA_P4_VWS" => Some(Self::NvidiaTeslaP4Vws),
                _ => None,
            }
        }
    }
}
/// A set of Shielded Instance options.
/// See [Images using supported Shielded VM
/// features](<https://cloud.google.com/compute/docs/instances/modifying-shielded-vm>).
/// Not all combinations are valid.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShieldedInstanceConfig {
    /// Optional. Defines whether the VM instance has Secure Boot enabled.
    ///
    /// Secure Boot helps ensure that the system only runs authentic software by
    /// verifying the digital signature of all boot components, and halting the
    /// boot process if signature verification fails. Disabled by default.
    #[prost(bool, tag = "1")]
    pub enable_secure_boot: bool,
    /// Optional. Defines whether the VM instance has the vTPM enabled. Enabled by
    /// default.
    #[prost(bool, tag = "2")]
    pub enable_vtpm: bool,
    /// Optional. Defines whether the VM instance has integrity monitoring enabled.
    ///
    /// Enables monitoring and attestation of the boot integrity of the VM
    /// instance. The attestation is performed against the integrity policy
    /// baseline. This baseline is initially derived from the implicitly trusted
    /// boot image when the VM instance is created. Enabled by default.
    #[prost(bool, tag = "3")]
    pub enable_integrity_monitoring: bool,
}
/// A GPU driver configuration
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GpuDriverConfig {
    /// Optional. Whether the end user authorizes Google Cloud to install GPU
    /// driver on this VM instance. If this field is empty or set to false, the GPU
    /// driver won't be installed. Only applicable to instances with GPUs.
    #[prost(bool, tag = "1")]
    pub enable_gpu_driver: bool,
    /// Optional. Specify a custom Cloud Storage path where the GPU driver is
    /// stored. If not specified, we'll automatically choose from official GPU
    /// drivers.
    #[prost(string, tag = "2")]
    pub custom_gpu_driver_path: ::prost::alloc::string::String,
}
/// An instance-attached disk resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataDisk {
    /// Optional. The size of the disk in GB attached to this VM instance, up to a
    /// maximum of 64000 GB (64 TB). If not specified, this defaults to 100.
    #[prost(int64, tag = "1")]
    pub disk_size_gb: i64,
    /// Optional. Input only. Indicates the type of the disk.
    #[prost(enumeration = "DiskType", tag = "2")]
    pub disk_type: i32,
    /// Optional. Input only. Disk encryption method used on the boot and data
    /// disks, defaults to GMEK.
    #[prost(enumeration = "DiskEncryption", tag = "5")]
    pub disk_encryption: i32,
    /// Optional. Input only. The KMS key used to encrypt the disks, only
    /// applicable if disk_encryption is CMEK. Format:
    /// `projects/{project_id}/locations/{location}/keyRings/{key_ring_id}/cryptoKeys/{key_id}`
    ///
    /// Learn more about using your own encryption keys.
    #[prost(string, tag = "6")]
    pub kms_key: ::prost::alloc::string::String,
}
/// The definition of a boot disk.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BootDisk {
    /// Optional. The size of the boot disk in GB attached to this instance, up to
    /// a maximum of 64000 GB (64 TB). If not specified, this defaults to the
    /// recommended value of 150GB.
    #[prost(int64, tag = "1")]
    pub disk_size_gb: i64,
    /// Optional. Indicates the type of the disk.
    #[prost(enumeration = "DiskType", tag = "2")]
    pub disk_type: i32,
    /// Optional. Input only. Disk encryption method used on the boot and data
    /// disks, defaults to GMEK.
    #[prost(enumeration = "DiskEncryption", tag = "3")]
    pub disk_encryption: i32,
    /// Optional. Input only. The KMS key used to encrypt the disks, only
    /// applicable if disk_encryption is CMEK. Format:
    /// `projects/{project_id}/locations/{location}/keyRings/{key_ring_id}/cryptoKeys/{key_id}`
    ///
    /// Learn more about using your own encryption keys.
    #[prost(string, tag = "4")]
    pub kms_key: ::prost::alloc::string::String,
}
/// A service account that acts as an identity.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceAccount {
    /// Optional. Email address of the service account.
    #[prost(string, tag = "1")]
    pub email: ::prost::alloc::string::String,
    /// Output only. The list of scopes to be made available for this service
    /// account. Set by the CLH to <https://www.googleapis.com/auth/cloud-platform>
    #[prost(string, repeated, tag = "2")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The definition of how to configure a VM instance outside of Resources and
/// Identity.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GceSetup {
    /// Optional. The machine type of the VM instance.
    /// <https://cloud.google.com/compute/docs/machine-resource>
    #[prost(string, tag = "1")]
    pub machine_type: ::prost::alloc::string::String,
    /// Optional. The hardware accelerators used on this instance. If you use
    /// accelerators, make sure that your configuration has
    /// [enough vCPUs and memory to support the `machine_type` you have
    /// selected](<https://cloud.google.com/compute/docs/gpus/#gpus-list>).
    /// Currently supports only one accelerator configuration.
    #[prost(message, repeated, tag = "2")]
    pub accelerator_configs: ::prost::alloc::vec::Vec<AcceleratorConfig>,
    /// Optional. The service account that serves as an identity for the VM
    /// instance. Currently supports only one service account.
    #[prost(message, repeated, tag = "3")]
    pub service_accounts: ::prost::alloc::vec::Vec<ServiceAccount>,
    /// Optional. The boot disk for the VM.
    #[prost(message, optional, tag = "6")]
    pub boot_disk: ::core::option::Option<BootDisk>,
    /// Optional. Data disks attached to the VM instance.
    /// Currently supports only one data disk.
    #[prost(message, repeated, tag = "7")]
    pub data_disks: ::prost::alloc::vec::Vec<DataDisk>,
    /// Optional. Shielded VM configuration.
    /// [Images using supported Shielded VM
    /// features](<https://cloud.google.com/compute/docs/instances/modifying-shielded-vm>).
    #[prost(message, optional, tag = "8")]
    pub shielded_instance_config: ::core::option::Option<ShieldedInstanceConfig>,
    /// Optional. The network interfaces for the VM.
    /// Supports only one interface.
    #[prost(message, repeated, tag = "9")]
    pub network_interfaces: ::prost::alloc::vec::Vec<NetworkInterface>,
    /// Optional. If true, no external IP will be assigned to this VM instance.
    #[prost(bool, tag = "10")]
    pub disable_public_ip: bool,
    /// Optional. The Compute Engine tags to add to runtime (see [Tagging
    /// instances](<https://cloud.google.com/compute/docs/label-or-tag-resources#tags>)).
    #[prost(string, repeated, tag = "11")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. Custom metadata to apply to this instance.
    #[prost(map = "string, string", tag = "12")]
    pub metadata: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Optional. Flag to enable ip forwarding or not, default false/off.
    /// <https://cloud.google.com/vpc/docs/using-routes#canipforward>
    #[prost(bool, tag = "13")]
    pub enable_ip_forwarding: bool,
    /// Optional. Configuration for GPU drivers.
    #[prost(message, optional, tag = "14")]
    pub gpu_driver_config: ::core::option::Option<GpuDriverConfig>,
    /// Type of the image; can be one of VM image, or container image.
    #[prost(oneof = "gce_setup::Image", tags = "4, 5")]
    pub image: ::core::option::Option<gce_setup::Image>,
}
/// Nested message and enum types in `GceSetup`.
pub mod gce_setup {
    /// Type of the image; can be one of VM image, or container image.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Image {
        /// Optional. Use a Compute Engine VM image to start the notebook instance.
        #[prost(message, tag = "4")]
        VmImage(super::VmImage),
        /// Optional. Use a container image to start the notebook instance.
        #[prost(message, tag = "5")]
        ContainerImage(super::ContainerImage),
    }
}
/// Definition of the disk encryption options.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DiskEncryption {
    /// Disk encryption is not specified.
    Unspecified = 0,
    /// Use Google managed encryption keys to encrypt the boot disk.
    Gmek = 1,
    /// Use customer managed encryption keys to encrypt the boot disk.
    Cmek = 2,
}
impl DiskEncryption {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DiskEncryption::Unspecified => "DISK_ENCRYPTION_UNSPECIFIED",
            DiskEncryption::Gmek => "GMEK",
            DiskEncryption::Cmek => "CMEK",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DISK_ENCRYPTION_UNSPECIFIED" => Some(Self::Unspecified),
            "GMEK" => Some(Self::Gmek),
            "CMEK" => Some(Self::Cmek),
            _ => None,
        }
    }
}
/// Possible disk types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DiskType {
    /// Disk type not set.
    Unspecified = 0,
    /// Standard persistent disk type.
    PdStandard = 1,
    /// SSD persistent disk type.
    PdSsd = 2,
    /// Balanced persistent disk type.
    PdBalanced = 3,
    /// Extreme persistent disk type.
    PdExtreme = 4,
}
impl DiskType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DiskType::Unspecified => "DISK_TYPE_UNSPECIFIED",
            DiskType::PdStandard => "PD_STANDARD",
            DiskType::PdSsd => "PD_SSD",
            DiskType::PdBalanced => "PD_BALANCED",
            DiskType::PdExtreme => "PD_EXTREME",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DISK_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "PD_STANDARD" => Some(Self::PdStandard),
            "PD_SSD" => Some(Self::PdSsd),
            "PD_BALANCED" => Some(Self::PdBalanced),
            "PD_EXTREME" => Some(Self::PdExtreme),
            _ => None,
        }
    }
}
/// The entry of VM image upgrade history.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpgradeHistoryEntry {
    /// Optional. The snapshot of the boot disk of this notebook instance before
    /// upgrade.
    #[prost(string, tag = "1")]
    pub snapshot: ::prost::alloc::string::String,
    /// Optional. The VM image before this instance upgrade.
    #[prost(string, tag = "2")]
    pub vm_image: ::prost::alloc::string::String,
    /// Optional. The container image before this instance upgrade.
    #[prost(string, tag = "3")]
    pub container_image: ::prost::alloc::string::String,
    /// Optional. The framework of this notebook instance.
    #[prost(string, tag = "4")]
    pub framework: ::prost::alloc::string::String,
    /// Optional. The version of the notebook instance before this upgrade.
    #[prost(string, tag = "5")]
    pub version: ::prost::alloc::string::String,
    /// Output only. The state of this instance upgrade history entry.
    #[prost(enumeration = "upgrade_history_entry::State", tag = "6")]
    pub state: i32,
    /// Immutable. The time that this instance upgrade history entry is created.
    #[prost(message, optional, tag = "7")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Action. Rolloback or Upgrade.
    #[prost(enumeration = "upgrade_history_entry::Action", tag = "8")]
    pub action: i32,
    /// Optional. Target VM Version, like m63.
    #[prost(string, tag = "9")]
    pub target_version: ::prost::alloc::string::String,
}
/// Nested message and enum types in `UpgradeHistoryEntry`.
pub mod upgrade_history_entry {
    /// The definition of the states of this upgrade history entry.
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
        /// State is not specified.
        Unspecified = 0,
        /// The instance upgrade is started.
        Started = 1,
        /// The instance upgrade is succeeded.
        Succeeded = 2,
        /// The instance upgrade is failed.
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
                State::Started => "STARTED",
                State::Succeeded => "SUCCEEDED",
                State::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "STARTED" => Some(Self::Started),
                "SUCCEEDED" => Some(Self::Succeeded),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
    /// The definition of operations of this upgrade history entry.
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
    pub enum Action {
        /// Operation is not specified.
        Unspecified = 0,
        /// Upgrade.
        Upgrade = 1,
        /// Rollback.
        Rollback = 2,
    }
    impl Action {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Action::Unspecified => "ACTION_UNSPECIFIED",
                Action::Upgrade => "UPGRADE",
                Action::Rollback => "ROLLBACK",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ACTION_UNSPECIFIED" => Some(Self::Unspecified),
                "UPGRADE" => Some(Self::Upgrade),
                "ROLLBACK" => Some(Self::Rollback),
                _ => None,
            }
        }
    }
}
/// The definition of a notebook instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instance {
    /// Output only. The name of this notebook instance. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The proxy endpoint that is used to access the Jupyter
    /// notebook.
    #[prost(string, tag = "3")]
    pub proxy_uri: ::prost::alloc::string::String,
    /// Optional. Input only. The owner of this instance after creation. Format:
    /// `alias@example.com`
    ///
    /// Currently supports one owner only. If not specified, all of the service
    /// account users of your VM instance's service account can use
    /// the instance.
    #[prost(string, repeated, tag = "4")]
    pub instance_owners: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. Email address of entity that sent original CreateInstance
    /// request.
    #[prost(string, tag = "5")]
    pub creator: ::prost::alloc::string::String,
    /// Output only. The state of this instance.
    #[prost(enumeration = "State", tag = "6")]
    pub state: i32,
    /// Output only. The upgrade history of this instance.
    #[prost(message, repeated, tag = "7")]
    pub upgrade_history: ::prost::alloc::vec::Vec<UpgradeHistoryEntry>,
    /// Output only. Unique ID of the resource.
    #[prost(string, tag = "8")]
    pub id: ::prost::alloc::string::String,
    /// Output only. Instance health_state.
    #[prost(enumeration = "HealthState", tag = "9")]
    pub health_state: i32,
    /// Output only. Additional information about instance health.
    /// Example:
    ///
    ///      healthInfo": {
    ///        "docker_proxy_agent_status": "1",
    ///        "docker_status": "1",
    ///        "jupyterlab_api_status": "-1",
    ///        "jupyterlab_status": "-1",
    ///        "updated": "2020-10-18 09:40:03.573409"
    ///      }
    #[prost(map = "string, string", tag = "10")]
    pub health_info: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. Instance creation time.
    #[prost(message, optional, tag = "11")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Instance update time.
    #[prost(message, optional, tag = "12")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. If true, the notebook instance will not register with the proxy.
    #[prost(bool, tag = "13")]
    pub disable_proxy_access: bool,
    /// Optional. Labels to apply to this instance.
    /// These can be later modified by the UpdateInstance method.
    #[prost(map = "string, string", tag = "14")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Setup for the Notebook instance.
    #[prost(oneof = "instance::Infrastructure", tags = "2")]
    pub infrastructure: ::core::option::Option<instance::Infrastructure>,
}
/// Nested message and enum types in `Instance`.
pub mod instance {
    /// Setup for the Notebook instance.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Infrastructure {
        /// Optional. Compute Engine setup for the notebook. Uses notebook-defined
        /// fields.
        #[prost(message, tag = "2")]
        GceSetup(super::GceSetup),
    }
}
/// The definition of the states of this instance.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum State {
    /// State is not specified.
    Unspecified = 0,
    /// The control logic is starting the instance.
    Starting = 1,
    /// The control logic is installing required frameworks and registering the
    /// instance with notebook proxy
    Provisioning = 2,
    /// The instance is running.
    Active = 3,
    /// The control logic is stopping the instance.
    Stopping = 4,
    /// The instance is stopped.
    Stopped = 5,
    /// The instance is deleted.
    Deleted = 6,
    /// The instance is upgrading.
    Upgrading = 7,
    /// The instance is being created.
    Initializing = 8,
    /// The instance is suspending.
    Suspending = 9,
    /// The instance is suspended.
    Suspended = 10,
}
impl State {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            State::Unspecified => "STATE_UNSPECIFIED",
            State::Starting => "STARTING",
            State::Provisioning => "PROVISIONING",
            State::Active => "ACTIVE",
            State::Stopping => "STOPPING",
            State::Stopped => "STOPPED",
            State::Deleted => "DELETED",
            State::Upgrading => "UPGRADING",
            State::Initializing => "INITIALIZING",
            State::Suspending => "SUSPENDING",
            State::Suspended => "SUSPENDED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "STARTING" => Some(Self::Starting),
            "PROVISIONING" => Some(Self::Provisioning),
            "ACTIVE" => Some(Self::Active),
            "STOPPING" => Some(Self::Stopping),
            "STOPPED" => Some(Self::Stopped),
            "DELETED" => Some(Self::Deleted),
            "UPGRADING" => Some(Self::Upgrading),
            "INITIALIZING" => Some(Self::Initializing),
            "SUSPENDING" => Some(Self::Suspending),
            "SUSPENDED" => Some(Self::Suspended),
            _ => None,
        }
    }
}
/// The instance health state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HealthState {
    /// The instance substate is unknown.
    Unspecified = 0,
    /// The instance is known to be in an healthy state
    /// (for example, critical daemons are running)
    /// Applies to ACTIVE state.
    Healthy = 1,
    /// The instance is known to be in an unhealthy state
    /// (for example, critical daemons are not running)
    /// Applies to ACTIVE state.
    Unhealthy = 2,
    /// The instance has not installed health monitoring agent.
    /// Applies to ACTIVE state.
    AgentNotInstalled = 3,
    /// The instance health monitoring agent is not running.
    /// Applies to ACTIVE state.
    AgentNotRunning = 4,
}
impl HealthState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            HealthState::Unspecified => "HEALTH_STATE_UNSPECIFIED",
            HealthState::Healthy => "HEALTHY",
            HealthState::Unhealthy => "UNHEALTHY",
            HealthState::AgentNotInstalled => "AGENT_NOT_INSTALLED",
            HealthState::AgentNotRunning => "AGENT_NOT_RUNNING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "HEALTH_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "HEALTHY" => Some(Self::Healthy),
            "UNHEALTHY" => Some(Self::Unhealthy),
            "AGENT_NOT_INSTALLED" => Some(Self::AgentNotInstalled),
            "AGENT_NOT_RUNNING" => Some(Self::AgentNotRunning),
            _ => None,
        }
    }
}
/// Represents the metadata of the long-running operation.
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
    /// Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: ::prost::alloc::string::String,
    /// Human-readable status of the operation, if any.
    #[prost(string, tag = "5")]
    pub status_message: ::prost::alloc::string::String,
    /// Identifies whether the user has requested cancellation
    /// of the operation. Operations that have successfully been cancelled
    /// have \[Operation.error][\] value with a
    /// \[google.rpc.Status.code][google.rpc.Status.code\] of 1, corresponding to
    /// `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
    /// API endpoint name of this operation.
    #[prost(string, tag = "8")]
    pub endpoint: ::prost::alloc::string::String,
}
/// Request for listing notebook instances.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesRequest {
    /// Required. Format:
    /// `parent=projects/{project_id}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Maximum return size of the list call.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A previous returned page token that can be used to continue
    /// listing from the last result.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Sort results. Supported values are "name", "name desc" or ""
    /// (unsorted).
    #[prost(string, tag = "4")]
    pub order_by: ::prost::alloc::string::String,
    /// Optional. List filter.
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
}
/// Response for listing notebook instances.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesResponse {
    /// A list of returned instances.
    #[prost(message, repeated, tag = "1")]
    pub instances: ::prost::alloc::vec::Vec<Instance>,
    /// Page token that can be used to continue listing from the last result in the
    /// next list call.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached. For example,
    /// ['us-west1-a', 'us-central1-b'].
    /// A ListInstancesResponse will only contain either instances or unreachables,
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for getting a notebook instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstanceRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for creating a notebook instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInstanceRequest {
    /// Required. Format:
    /// `parent=projects/{project_id}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. User-defined unique ID of this instance.
    #[prost(string, tag = "2")]
    pub instance_id: ::prost::alloc::string::String,
    /// Required. The instance to be created.
    #[prost(message, optional, tag = "3")]
    pub instance: ::core::option::Option<Instance>,
    /// Optional. Idempotent request UUID.
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request for updating a notebook instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInstanceRequest {
    /// Required. A representation of an instance.
    #[prost(message, optional, tag = "1")]
    pub instance: ::core::option::Option<Instance>,
    /// Required. Mask used to update an instance
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Optional. Idempotent request UUID.
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request for deleting a notebook instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteInstanceRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Idempotent request UUID.
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request for starting a notebook instance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartInstanceRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for stopping a notebook instance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopInstanceRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for resetting a notebook instance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetInstanceRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for checking if a notebook instance is upgradeable.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckInstanceUpgradabilityRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub notebook_instance: ::prost::alloc::string::String,
}
/// Response for checking if a notebook instance is upgradeable.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckInstanceUpgradabilityResponse {
    /// If an instance is upgradeable.
    #[prost(bool, tag = "1")]
    pub upgradeable: bool,
    /// The version this instance will be upgraded to if calling the upgrade
    /// endpoint. This field will only be populated if field upgradeable is true.
    #[prost(string, tag = "2")]
    pub upgrade_version: ::prost::alloc::string::String,
    /// Additional information about upgrade.
    #[prost(string, tag = "3")]
    pub upgrade_info: ::prost::alloc::string::String,
    /// The new image self link this instance will be upgraded to if calling the
    /// upgrade endpoint. This field will only be populated if field upgradeable
    /// is true.
    #[prost(string, tag = "4")]
    pub upgrade_image: ::prost::alloc::string::String,
}
/// Request for upgrading a notebook instance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpgradeInstanceRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for rollbacking a notebook instance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollbackInstanceRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The snapshot for rollback.
    /// Example: "projects/test-project/global/snapshots/krwlzipynril".
    #[prost(string, tag = "2")]
    pub target_snapshot: ::prost::alloc::string::String,
    /// Required. Output only. Revision Id
    #[prost(string, tag = "3")]
    pub revision_id: ::prost::alloc::string::String,
}
/// Request for creating a notebook instance diagnostic file.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiagnoseInstanceRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Defines flags that are used to run the diagnostic tool
    #[prost(message, optional, tag = "2")]
    pub diagnostic_config: ::core::option::Option<DiagnosticConfig>,
    /// Optional. Maxmium amount of time in minutes before the operation times out.
    #[prost(int32, tag = "3")]
    pub timeout_minutes: i32,
}
/// Generated client implementations.
pub mod notebook_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// API v2 service for Workbench Notebooks Instances.
    #[derive(Debug, Clone)]
    pub struct NotebookServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl NotebookServiceClient<tonic::transport::Channel> {
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
    impl<T> NotebookServiceClient<T>
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
        ) -> NotebookServiceClient<InterceptedService<T, F>>
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
            NotebookServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists instances in a given project and location.
        pub async fn list_instances(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInstancesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListInstancesResponse>,
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
                "/google.cloud.notebooks.v2.NotebookService/ListInstances",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.notebooks.v2.NotebookService",
                        "ListInstances",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single Instance.
        pub async fn get_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInstanceRequest>,
        ) -> std::result::Result<tonic::Response<super::Instance>, tonic::Status> {
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
                "/google.cloud.notebooks.v2.NotebookService/GetInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.notebooks.v2.NotebookService",
                        "GetInstance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new Instance in a given project and location.
        pub async fn create_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateInstanceRequest>,
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
                "/google.cloud.notebooks.v2.NotebookService/CreateInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.notebooks.v2.NotebookService",
                        "CreateInstance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// UpdateInstance updates an Instance.
        pub async fn update_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateInstanceRequest>,
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
                "/google.cloud.notebooks.v2.NotebookService/UpdateInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.notebooks.v2.NotebookService",
                        "UpdateInstance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single Instance.
        pub async fn delete_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteInstanceRequest>,
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
                "/google.cloud.notebooks.v2.NotebookService/DeleteInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.notebooks.v2.NotebookService",
                        "DeleteInstance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Starts a notebook instance.
        pub async fn start_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::StartInstanceRequest>,
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
                "/google.cloud.notebooks.v2.NotebookService/StartInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.notebooks.v2.NotebookService",
                        "StartInstance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Stops a notebook instance.
        pub async fn stop_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::StopInstanceRequest>,
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
                "/google.cloud.notebooks.v2.NotebookService/StopInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.notebooks.v2.NotebookService",
                        "StopInstance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Resets a notebook instance.
        pub async fn reset_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::ResetInstanceRequest>,
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
                "/google.cloud.notebooks.v2.NotebookService/ResetInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.notebooks.v2.NotebookService",
                        "ResetInstance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Checks whether a notebook instance is upgradable.
        pub async fn check_instance_upgradability(
            &mut self,
            request: impl tonic::IntoRequest<super::CheckInstanceUpgradabilityRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CheckInstanceUpgradabilityResponse>,
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
                "/google.cloud.notebooks.v2.NotebookService/CheckInstanceUpgradability",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.notebooks.v2.NotebookService",
                        "CheckInstanceUpgradability",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Upgrades a notebook instance to the latest version.
        pub async fn upgrade_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::UpgradeInstanceRequest>,
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
                "/google.cloud.notebooks.v2.NotebookService/UpgradeInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.notebooks.v2.NotebookService",
                        "UpgradeInstance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Rollbacks a notebook instance to the previous version.
        pub async fn rollback_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::RollbackInstanceRequest>,
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
                "/google.cloud.notebooks.v2.NotebookService/RollbackInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.notebooks.v2.NotebookService",
                        "RollbackInstance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a Diagnostic File and runs Diagnostic Tool given an Instance.
        pub async fn diagnose_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::DiagnoseInstanceRequest>,
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
                "/google.cloud.notebooks.v2.NotebookService/DiagnoseInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.notebooks.v2.NotebookService",
                        "DiagnoseInstance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
