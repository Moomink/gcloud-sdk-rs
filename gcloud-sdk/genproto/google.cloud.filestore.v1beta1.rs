/// Network configuration for the instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkConfig {
    /// The name of the Google Compute Engine
    /// [VPC network](<https://cloud.google.com/vpc/docs/vpc>) to which the
    /// instance is connected.
    #[prost(string, tag = "1")]
    pub network: ::prost::alloc::string::String,
    /// Internet protocol versions for which the instance has IP addresses
    /// assigned. For this version, only MODE_IPV4 is supported.
    #[prost(enumeration = "network_config::AddressMode", repeated, tag = "3")]
    pub modes: ::prost::alloc::vec::Vec<i32>,
    /// Optional, reserved_ip_range can have one of the following two types of
    /// values.
    ///
    /// * CIDR range value when using DIRECT_PEERING connect mode.
    /// * [Allocated IP address
    /// range](<https://cloud.google.com/compute/docs/ip-addresses/reserve-static-internal-ip-address>)
    /// when using PRIVATE_SERVICE_ACCESS connect mode.
    ///
    /// When the name of an allocated IP address range is specified, it must be one
    /// of the ranges associated with the private service access connection.
    /// When specified as a direct CIDR value, it must be a /29 CIDR block for
    /// Basic tier, a /24 CIDR block for High Scale tier, or a /26 CIDR block for
    /// Enterprise tier in one of the [internal IP address
    /// ranges](<https://www.arin.net/reference/research/statistics/address_filters/>)
    /// that identifies the range of IP addresses reserved for this instance. For
    /// example, 10.0.0.0/29, 192.168.0.0/24, or 192.168.0.0/26, respectively. The
    /// range you specify can't overlap with either existing subnets or assigned IP
    /// address ranges for other Filestore instances in the selected VPC
    /// network.
    #[prost(string, tag = "4")]
    pub reserved_ip_range: ::prost::alloc::string::String,
    /// Output only. IPv4 addresses in the format
    /// `{octet1}.{octet2}.{octet3}.{octet4}` or IPv6 addresses in the format
    /// `{block1}:{block2}:{block3}:{block4}:{block5}:{block6}:{block7}:{block8}`.
    #[prost(string, repeated, tag = "5")]
    pub ip_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The network connect mode of the Filestore instance.
    /// If not provided, the connect mode defaults to DIRECT_PEERING.
    #[prost(enumeration = "network_config::ConnectMode", tag = "6")]
    pub connect_mode: i32,
}
/// Nested message and enum types in `NetworkConfig`.
pub mod network_config {
    /// Internet protocol versions supported by Filestore.
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
    pub enum AddressMode {
        /// Internet protocol not set.
        Unspecified = 0,
        /// Use the IPv4 internet protocol.
        ModeIpv4 = 1,
    }
    impl AddressMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AddressMode::Unspecified => "ADDRESS_MODE_UNSPECIFIED",
                AddressMode::ModeIpv4 => "MODE_IPV4",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ADDRESS_MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "MODE_IPV4" => Some(Self::ModeIpv4),
                _ => None,
            }
        }
    }
    /// Available connection modes.
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
    pub enum ConnectMode {
        /// ConnectMode not set.
        Unspecified = 0,
        /// Connect via direct peering to the Filestore service.
        DirectPeering = 1,
        /// Connect to your Filestore instance using Private Service
        /// Access. Private services access provides an IP address range for multiple
        /// Google Cloud services, including Filestore.
        PrivateServiceAccess = 2,
    }
    impl ConnectMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ConnectMode::Unspecified => "CONNECT_MODE_UNSPECIFIED",
                ConnectMode::DirectPeering => "DIRECT_PEERING",
                ConnectMode::PrivateServiceAccess => "PRIVATE_SERVICE_ACCESS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CONNECT_MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "DIRECT_PEERING" => Some(Self::DirectPeering),
                "PRIVATE_SERVICE_ACCESS" => Some(Self::PrivateServiceAccess),
                _ => None,
            }
        }
    }
}
/// File share configuration for the instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileShareConfig {
    /// The name of the file share (must be 32 characters or less for
    /// Enterprise and High Scale SSD tiers and 16 characters or less for all other
    /// tiers).
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// File share capacity in gigabytes (GB).
    /// Filestore defines 1 GB as 1024^3 bytes.
    #[prost(int64, tag = "2")]
    pub capacity_gb: i64,
    /// Nfs Export Options.
    /// There is a limit of 10 export options per file share.
    #[prost(message, repeated, tag = "8")]
    pub nfs_export_options: ::prost::alloc::vec::Vec<NfsExportOptions>,
    /// The source that this file share has been restored from. Empty if the file
    /// share is created from scratch.
    #[prost(oneof = "file_share_config::Source", tags = "9")]
    pub source: ::core::option::Option<file_share_config::Source>,
}
/// Nested message and enum types in `FileShareConfig`.
pub mod file_share_config {
    /// The source that this file share has been restored from. Empty if the file
    /// share is created from scratch.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// The resource name of the backup, in the format
        /// `projects/{project_id}/locations/{location_id}/backups/{backup_id}`, that
        /// this file share has been restored from.
        #[prost(string, tag = "9")]
        SourceBackup(::prost::alloc::string::String),
    }
}
/// NFS export options specifications.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NfsExportOptions {
    /// List of either an IPv4 addresses in the format
    /// `{octet1}.{octet2}.{octet3}.{octet4}` or CIDR ranges in the format
    /// `{octet1}.{octet2}.{octet3}.{octet4}/{mask size}` which may mount the
    /// file share.
    /// Overlapping IP ranges are not allowed, both within and across
    /// NfsExportOptions. An error will be returned.
    /// The limit is 64 IP ranges/addresses for each FileShareConfig among all
    /// NfsExportOptions.
    #[prost(string, repeated, tag = "1")]
    pub ip_ranges: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Either READ_ONLY, for allowing only read requests on the exported
    /// directory, or READ_WRITE, for allowing both read and write requests.
    /// The default is READ_WRITE.
    #[prost(enumeration = "nfs_export_options::AccessMode", tag = "2")]
    pub access_mode: i32,
    /// Either NO_ROOT_SQUASH, for allowing root access on the exported directory,
    /// or ROOT_SQUASH, for not allowing root access. The default is
    /// NO_ROOT_SQUASH.
    #[prost(enumeration = "nfs_export_options::SquashMode", tag = "3")]
    pub squash_mode: i32,
    /// An integer representing the anonymous user id with a default value of
    /// 65534.
    /// Anon_uid may only be set with squash_mode of ROOT_SQUASH.  An error will be
    /// returned if this field is specified for other squash_mode settings.
    #[prost(int64, tag = "4")]
    pub anon_uid: i64,
    /// An integer representing the anonymous group id with a default value of
    /// 65534.
    /// Anon_gid may only be set with squash_mode of ROOT_SQUASH.  An error will be
    /// returned if this field is specified for other squash_mode settings.
    #[prost(int64, tag = "5")]
    pub anon_gid: i64,
    /// The security flavors allowed for mount operations.
    /// The default is AUTH_SYS.
    #[prost(enumeration = "nfs_export_options::SecurityFlavor", repeated, tag = "6")]
    pub security_flavors: ::prost::alloc::vec::Vec<i32>,
}
/// Nested message and enum types in `NfsExportOptions`.
pub mod nfs_export_options {
    /// The access mode.
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
    pub enum AccessMode {
        /// AccessMode not set.
        Unspecified = 0,
        /// The client can only read the file share.
        ReadOnly = 1,
        /// The client can read and write the file share (default).
        ReadWrite = 2,
    }
    impl AccessMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AccessMode::Unspecified => "ACCESS_MODE_UNSPECIFIED",
                AccessMode::ReadOnly => "READ_ONLY",
                AccessMode::ReadWrite => "READ_WRITE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ACCESS_MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "READ_ONLY" => Some(Self::ReadOnly),
                "READ_WRITE" => Some(Self::ReadWrite),
                _ => None,
            }
        }
    }
    /// The squash mode.
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
    pub enum SquashMode {
        /// SquashMode not set.
        Unspecified = 0,
        /// The Root user has root access to the file share (default).
        NoRootSquash = 1,
        /// The Root user has squashed access to the anonymous uid/gid.
        RootSquash = 2,
    }
    impl SquashMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SquashMode::Unspecified => "SQUASH_MODE_UNSPECIFIED",
                SquashMode::NoRootSquash => "NO_ROOT_SQUASH",
                SquashMode::RootSquash => "ROOT_SQUASH",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SQUASH_MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "NO_ROOT_SQUASH" => Some(Self::NoRootSquash),
                "ROOT_SQUASH" => Some(Self::RootSquash),
                _ => None,
            }
        }
    }
    /// The security flavor. In general, a "flavor" represents a designed process
    /// or system. A "security flavor" is a system designed for the purpose of
    /// authenticating a data originator (client), recipient (server), and the data
    /// they transmit between one another.
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
    pub enum SecurityFlavor {
        /// SecurityFlavor not set.
        Unspecified = 0,
        /// The user's UNIX user-id and group-ids are transferred "in the clear" (not
        /// encrypted) on the network, unauthenticated by the NFS server (default).
        AuthSys = 1,
        /// End-user authentication through Kerberos V5.
        Krb5 = 2,
        /// krb5 plus integrity protection (data packets are tamper proof).
        Krb5i = 3,
        /// krb5i plus privacy protection (data packets are tamper proof and
        /// encrypted).
        Krb5p = 4,
    }
    impl SecurityFlavor {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SecurityFlavor::Unspecified => "SECURITY_FLAVOR_UNSPECIFIED",
                SecurityFlavor::AuthSys => "AUTH_SYS",
                SecurityFlavor::Krb5 => "KRB5",
                SecurityFlavor::Krb5i => "KRB5I",
                SecurityFlavor::Krb5p => "KRB5P",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SECURITY_FLAVOR_UNSPECIFIED" => Some(Self::Unspecified),
                "AUTH_SYS" => Some(Self::AuthSys),
                "KRB5" => Some(Self::Krb5),
                "KRB5I" => Some(Self::Krb5i),
                "KRB5P" => Some(Self::Krb5p),
                _ => None,
            }
        }
    }
}
/// ManagedActiveDirectoryConfig contains all the parameters for connecting
/// to Managed Active Directory.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManagedActiveDirectoryConfig {
    /// Fully qualified domain name.
    #[prost(string, tag = "1")]
    pub domain: ::prost::alloc::string::String,
    /// The computer name is used as a prefix to the mount remote target.
    /// Example: if the computer_name is `my-computer`, the mount command will
    /// look like: `$mount -o vers=4,sec=krb5
    /// my-computer.filestore.<domain>:<share>`.
    #[prost(string, tag = "2")]
    pub computer: ::prost::alloc::string::String,
}
/// Directory Services configuration for Kerberos-based authentication.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectoryServicesConfig {
    #[prost(oneof = "directory_services_config::Config", tags = "1")]
    pub config: ::core::option::Option<directory_services_config::Config>,
}
/// Nested message and enum types in `DirectoryServicesConfig`.
pub mod directory_services_config {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Config {
        /// Configuration for Managed Service for Microsoft Active Directory.
        #[prost(message, tag = "1")]
        ManagedActiveDirectory(super::ManagedActiveDirectoryConfig),
    }
}
/// A Filestore instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instance {
    /// Output only. The resource name of the instance, in the format
    /// `projects/{project_id}/locations/{location_id}/instances/{instance_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The description of the instance (2048 characters or less).
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The instance state.
    #[prost(enumeration = "instance::State", tag = "5")]
    pub state: i32,
    /// Output only. Additional information about the instance state, if available.
    #[prost(string, tag = "6")]
    pub status_message: ::prost::alloc::string::String,
    /// Output only. The time when the instance was created.
    #[prost(message, optional, tag = "7")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The service tier of the instance.
    #[prost(enumeration = "instance::Tier", tag = "8")]
    pub tier: i32,
    /// Resource labels to represent user provided metadata.
    #[prost(map = "string, string", tag = "9")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// File system shares on the instance.
    /// For this version, only a single file share is supported.
    #[prost(message, repeated, tag = "10")]
    pub file_shares: ::prost::alloc::vec::Vec<FileShareConfig>,
    /// VPC networks to which the instance is connected.
    /// For this version, only a single network is supported.
    #[prost(message, repeated, tag = "11")]
    pub networks: ::prost::alloc::vec::Vec<NetworkConfig>,
    /// Server-specified ETag for the instance resource to prevent simultaneous
    /// updates from overwriting each other.
    #[prost(string, tag = "12")]
    pub etag: ::prost::alloc::string::String,
    /// Output only. Reserved for future use.
    #[prost(message, optional, tag = "13")]
    pub satisfies_pzs: ::core::option::Option<bool>,
    /// KMS key name used for data encryption.
    #[prost(string, tag = "14")]
    pub kms_key_name: ::prost::alloc::string::String,
    /// Output only. Field indicates all the reasons the instance is in "SUSPENDED"
    /// state.
    #[prost(
        enumeration = "instance::SuspensionReason",
        repeated,
        packed = "false",
        tag = "15"
    )]
    pub suspension_reasons: ::prost::alloc::vec::Vec<i32>,
    /// Output only. The max capacity of the instance.
    #[prost(int64, tag = "16")]
    pub max_capacity_gb: i64,
    /// Output only. The increase/decrease capacity step size.
    #[prost(int64, tag = "17")]
    pub capacity_step_size_gb: i64,
    /// The max number of shares allowed.
    #[prost(int64, tag = "18")]
    pub max_share_count: i64,
    /// The storage capacity of the instance in gigabytes (GB = 1024^3 bytes).
    /// This capacity can be increased up to `max_capacity_gb` GB in multipliers
    /// of `capacity_step_size_gb` GB.
    #[prost(int64, tag = "19")]
    pub capacity_gb: i64,
    /// Indicates whether this instance uses a multi-share configuration with which
    /// it can have more than one file-share or none at all. File-shares are added,
    /// updated and removed through the separate file-share APIs.
    #[prost(bool, tag = "20")]
    pub multi_share_enabled: bool,
    /// Immutable. The protocol indicates the access protocol for all shares in the
    /// instance. This field is immutable and it cannot be changed after the
    /// instance has been created. Default value: `NFS_V3`.
    #[prost(enumeration = "instance::FileProtocol", tag = "21")]
    pub protocol: i32,
    /// Directory Services configuration for Kerberos-based authentication.
    /// Should only be set if protocol is "NFS_V4_1".
    #[prost(message, optional, tag = "24")]
    pub directory_services: ::core::option::Option<DirectoryServicesConfig>,
}
/// Nested message and enum types in `Instance`.
pub mod instance {
    /// The instance state.
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
        /// State not set.
        Unspecified = 0,
        /// The instance is being created.
        Creating = 1,
        /// The instance is available for use.
        Ready = 2,
        /// Work is being done on the instance. You can get further details from the
        /// `statusMessage` field of the `Instance` resource.
        Repairing = 3,
        /// The instance is shutting down.
        Deleting = 4,
        /// The instance is experiencing an issue and might be unusable. You can get
        /// further details from the `statusMessage` field of the `Instance`
        /// resource.
        Error = 6,
        /// The instance is restoring a snapshot or backup to an existing file share
        /// and may be unusable during this time.
        Restoring = 7,
        /// The instance is suspended. You can get further details from
        /// the `suspension_reasons` field of the `Instance` resource.
        Suspended = 8,
        /// The instance is reverting to a snapshot.
        Reverting = 9,
        /// The instance is in the process of becoming suspended.
        Suspending = 10,
        /// The instance is in the process of becoming active.
        Resuming = 11,
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
                State::Repairing => "REPAIRING",
                State::Deleting => "DELETING",
                State::Error => "ERROR",
                State::Restoring => "RESTORING",
                State::Suspended => "SUSPENDED",
                State::Reverting => "REVERTING",
                State::Suspending => "SUSPENDING",
                State::Resuming => "RESUMING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "READY" => Some(Self::Ready),
                "REPAIRING" => Some(Self::Repairing),
                "DELETING" => Some(Self::Deleting),
                "ERROR" => Some(Self::Error),
                "RESTORING" => Some(Self::Restoring),
                "SUSPENDED" => Some(Self::Suspended),
                "REVERTING" => Some(Self::Reverting),
                "SUSPENDING" => Some(Self::Suspending),
                "RESUMING" => Some(Self::Resuming),
                _ => None,
            }
        }
    }
    /// Available service tiers.
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
    pub enum Tier {
        /// Not set.
        Unspecified = 0,
        /// STANDARD tier. BASIC_HDD is the preferred term for this tier.
        Standard = 1,
        /// PREMIUM tier. BASIC_SSD is the preferred term for this tier.
        Premium = 2,
        /// BASIC instances offer a maximum capacity of 63.9 TB.
        /// BASIC_HDD is an alias for STANDARD Tier, offering economical
        /// performance backed by HDD.
        BasicHdd = 3,
        /// BASIC instances offer a maximum capacity of 63.9 TB.
        /// BASIC_SSD is an alias for PREMIUM Tier, and offers improved
        /// performance backed by SSD.
        BasicSsd = 4,
        /// HIGH_SCALE instances offer expanded capacity and performance scaling
        /// capabilities.
        HighScaleSsd = 6,
        /// ENTERPRISE instances offer the features and availability needed for
        /// mission-critical workloads.
        Enterprise = 7,
        /// ZONAL instances offer expanded capacity and performance scaling
        /// capabilities.
        Zonal = 8,
    }
    impl Tier {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Tier::Unspecified => "TIER_UNSPECIFIED",
                Tier::Standard => "STANDARD",
                Tier::Premium => "PREMIUM",
                Tier::BasicHdd => "BASIC_HDD",
                Tier::BasicSsd => "BASIC_SSD",
                Tier::HighScaleSsd => "HIGH_SCALE_SSD",
                Tier::Enterprise => "ENTERPRISE",
                Tier::Zonal => "ZONAL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TIER_UNSPECIFIED" => Some(Self::Unspecified),
                "STANDARD" => Some(Self::Standard),
                "PREMIUM" => Some(Self::Premium),
                "BASIC_HDD" => Some(Self::BasicHdd),
                "BASIC_SSD" => Some(Self::BasicSsd),
                "HIGH_SCALE_SSD" => Some(Self::HighScaleSsd),
                "ENTERPRISE" => Some(Self::Enterprise),
                "ZONAL" => Some(Self::Zonal),
                _ => None,
            }
        }
    }
    /// SuspensionReason contains the possible reasons for a suspension.
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
    pub enum SuspensionReason {
        /// Not set.
        Unspecified = 0,
        /// The KMS key used by the instance is either revoked or denied access to.
        KmsKeyIssue = 1,
    }
    impl SuspensionReason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SuspensionReason::Unspecified => "SUSPENSION_REASON_UNSPECIFIED",
                SuspensionReason::KmsKeyIssue => "KMS_KEY_ISSUE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SUSPENSION_REASON_UNSPECIFIED" => Some(Self::Unspecified),
                "KMS_KEY_ISSUE" => Some(Self::KmsKeyIssue),
                _ => None,
            }
        }
    }
    /// File access protocol.
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
    pub enum FileProtocol {
        /// FILE_PROTOCOL_UNSPECIFIED serves a "not set" default value when
        /// a FileProtocol is a separate field in a message.
        Unspecified = 0,
        /// NFS 3.0.
        NfsV3 = 1,
        /// NFS 4.1.
        NfsV41 = 2,
    }
    impl FileProtocol {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FileProtocol::Unspecified => "FILE_PROTOCOL_UNSPECIFIED",
                FileProtocol::NfsV3 => "NFS_V3",
                FileProtocol::NfsV41 => "NFS_V4_1",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FILE_PROTOCOL_UNSPECIFIED" => Some(Self::Unspecified),
                "NFS_V3" => Some(Self::NfsV3),
                "NFS_V4_1" => Some(Self::NfsV41),
                _ => None,
            }
        }
    }
}
/// CreateInstanceRequest creates an instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInstanceRequest {
    /// Required. The instance's project and location, in the format
    /// `projects/{project_id}/locations/{location}`. In Filestore,
    /// locations map to Google Cloud zones, for example **us-west1-b**.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID of the instance to create.
    /// The ID must be unique within the specified project and location.
    ///
    /// This value must start with a lowercase letter followed by up to 62
    /// lowercase letters, numbers, or hyphens, and cannot end with a hyphen.
    #[prost(string, tag = "2")]
    pub instance_id: ::prost::alloc::string::String,
    /// Required. An [instance resource][google.cloud.filestore.v1beta1.Instance]
    #[prost(message, optional, tag = "3")]
    pub instance: ::core::option::Option<Instance>,
}
/// GetInstanceRequest gets the state of an instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstanceRequest {
    /// Required. The instance resource name, in the format
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// UpdateInstanceRequest updates the settings of an instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInstanceRequest {
    /// Required. Mask of fields to update.  At least one path must be supplied in
    /// this field.  The elements of the repeated paths field may only include
    /// these fields:
    ///
    /// * "description"
    /// * "file_shares"
    /// * "labels"
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. Only fields specified in update_mask are updated.
    #[prost(message, optional, tag = "2")]
    pub instance: ::core::option::Option<Instance>,
}
/// RestoreInstanceRequest restores an existing instance's file share from a
/// backup.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreInstanceRequest {
    /// Required. The resource name of the instance, in the format
    /// `projects/{project_id}/locations/{location_id}/instances/{instance_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Name of the file share in the Filestore instance that the backup
    /// is being restored to.
    #[prost(string, tag = "2")]
    pub file_share: ::prost::alloc::string::String,
    #[prost(oneof = "restore_instance_request::Source", tags = "3, 4")]
    pub source: ::core::option::Option<restore_instance_request::Source>,
}
/// Nested message and enum types in `RestoreInstanceRequest`.
pub mod restore_instance_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// The resource name of the snapshot, in the format
        /// `projects/{project_id}/locations/{location_id}/snapshots/{snapshot_id}`.
        #[prost(string, tag = "3")]
        SourceSnapshot(::prost::alloc::string::String),
        /// The resource name of the backup, in the format
        /// `projects/{project_id}/locations/{location_id}/backups/{backup_id}`.
        #[prost(string, tag = "4")]
        SourceBackup(::prost::alloc::string::String),
    }
}
/// RevertInstanceRequest reverts the given instance's file share to the
/// specified snapshot.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevertInstanceRequest {
    /// Required.
    /// projects/{project_id}/locations/{location_id}/instances/{instance_id}. The
    /// resource name of the instance, in the format
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The snapshot resource ID, in the format 'my-snapshot', where the
    /// specified ID is the {snapshot_id} of the fully qualified name like
    /// projects/{project_id}/locations/{location_id}/instances/{instance_id}/snapshots/{snapshot_id}
    #[prost(string, tag = "2")]
    pub target_snapshot_id: ::prost::alloc::string::String,
}
/// DeleteInstanceRequest deletes an instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteInstanceRequest {
    /// Required. The instance resource name, in the format
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If set to true, any snapshots of the instance will also be deleted.
    /// (Otherwise, the request will only work if the instance has no snapshots.)
    #[prost(bool, tag = "2")]
    pub force: bool,
}
/// ListInstancesRequest lists instances.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesRequest {
    /// Required. The project and location for which to retrieve instance
    /// information, in the format `projects/{project_id}/locations/{location}`. In
    /// Cloud Filestore, locations map to Google Cloud zones, for example
    /// **us-west1-b**. To retrieve instance information for all locations, use "-"
    /// for the
    /// `{location}` value.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value to use if there are additional
    /// results to retrieve for this list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Sort results. Supported values are "name", "name desc" or "" (unsorted).
    #[prost(string, tag = "4")]
    pub order_by: ::prost::alloc::string::String,
    /// List filter.
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
}
/// ListInstancesResponse is the result of ListInstancesRequest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesResponse {
    /// A list of instances in the project for the specified location.
    ///
    /// If the `{location}` value in the request is "-", the response contains a
    /// list of instances from all locations. If any location is unreachable, the
    /// response will only return instances in reachable locations and the
    /// "unreachable" field will be populated with a list of unreachable locations.
    #[prost(message, repeated, tag = "1")]
    pub instances: ::prost::alloc::vec::Vec<Instance>,
    /// The token you can use to retrieve the next page of results. Not returned
    /// if there are no more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A Filestore snapshot.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Snapshot {
    /// Output only. The resource name of the snapshot, in the format
    /// `projects/{project_id}/locations/{location_id}/instances/{instance_id}/snapshots/{snapshot_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A description of the snapshot with 2048 characters or less.
    /// Requests with longer descriptions will be rejected.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The snapshot state.
    #[prost(enumeration = "snapshot::State", tag = "3")]
    pub state: i32,
    /// Output only. The time when the snapshot was created.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Resource labels to represent user provided metadata.
    #[prost(map = "string, string", tag = "5")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. The amount of bytes needed to allocate a full copy of the
    /// snapshot content
    #[prost(int64, tag = "12")]
    pub filesystem_used_bytes: i64,
}
/// Nested message and enum types in `Snapshot`.
pub mod snapshot {
    /// The snapshot state.
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
        /// State not set.
        Unspecified = 0,
        /// Snapshot is being created.
        Creating = 1,
        /// Snapshot is available for use.
        Ready = 3,
        /// Snapshot is being deleted.
        Deleting = 4,
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
                State::Deleting => "DELETING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "READY" => Some(Self::Ready),
                "DELETING" => Some(Self::Deleting),
                _ => None,
            }
        }
    }
}
/// CreateSnapshotRequest creates a snapshot.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSnapshotRequest {
    /// Required. The Filestore Instance to create the snapshots of, in the format
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID to use for the snapshot.
    /// The ID must be unique within the specified instance.
    ///
    /// This value must start with a lowercase letter followed by up to 62
    /// lowercase letters, numbers, or hyphens, and cannot end with a hyphen.
    #[prost(string, tag = "2")]
    pub snapshot_id: ::prost::alloc::string::String,
    /// Required. A snapshot resource
    #[prost(message, optional, tag = "3")]
    pub snapshot: ::core::option::Option<Snapshot>,
}
/// GetSnapshotRequest gets the state of a snapshot.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSnapshotRequest {
    /// Required. The snapshot resource name, in the format
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}/snapshots/{snapshot_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// DeleteSnapshotRequest deletes a snapshot.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSnapshotRequest {
    /// Required. The snapshot resource name, in the format
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}/snapshots/{snapshot_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// UpdateSnapshotRequest updates description and/or labels for a snapshot.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSnapshotRequest {
    /// Required. Mask of fields to update.  At least one path must be supplied in
    /// this field.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. A snapshot resource
    #[prost(message, optional, tag = "2")]
    pub snapshot: ::core::option::Option<Snapshot>,
}
/// ListSnapshotsRequest lists snapshots.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSnapshotsRequest {
    /// Required. The instance for which to retrieve snapshot information,
    /// in the format
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value to use if there are additional
    /// results to retrieve for this list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Sort results. Supported values are "name", "name desc" or "" (unsorted).
    #[prost(string, tag = "4")]
    pub order_by: ::prost::alloc::string::String,
    /// List filter.
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
}
/// ListSnapshotsResponse is the result of ListSnapshotsRequest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSnapshotsResponse {
    /// A list of snapshots in the project for the specified instance.
    #[prost(message, repeated, tag = "1")]
    pub snapshots: ::prost::alloc::vec::Vec<Snapshot>,
    /// The token you can use to retrieve the next page of results. Not returned
    /// if there are no more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// A Filestore backup.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Backup {
    /// Output only. The resource name of the backup, in the format
    /// `projects/{project_id}/locations/{location_id}/backups/{backup_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A description of the backup with 2048 characters or less.
    /// Requests with longer descriptions will be rejected.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The backup state.
    #[prost(enumeration = "backup::State", tag = "3")]
    pub state: i32,
    /// Output only. The time when the backup was created.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Resource labels to represent user provided metadata.
    #[prost(map = "string, string", tag = "5")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. Capacity of the source file share when the backup was created.
    #[prost(int64, tag = "6")]
    pub capacity_gb: i64,
    /// Output only. The size of the storage used by the backup. As backups share
    /// storage, this number is expected to change with backup creation/deletion.
    #[prost(int64, tag = "7")]
    pub storage_bytes: i64,
    /// The resource name of the source Filestore instance, in the format
    /// `projects/{project_id}/locations/{location_id}/instances/{instance_id}`,
    /// used to create this backup.
    #[prost(string, tag = "8")]
    pub source_instance: ::prost::alloc::string::String,
    /// Name of the file share in the source Filestore instance that the
    /// backup is created from.
    #[prost(string, tag = "9")]
    pub source_file_share: ::prost::alloc::string::String,
    /// Output only. The service tier of the source Filestore instance that this
    /// backup is created from.
    #[prost(enumeration = "instance::Tier", tag = "10")]
    pub source_instance_tier: i32,
    /// Output only. Amount of bytes that will be downloaded if the backup is
    /// restored
    #[prost(int64, tag = "11")]
    pub download_bytes: i64,
    /// Output only. Reserved for future use.
    #[prost(message, optional, tag = "12")]
    pub satisfies_pzs: ::core::option::Option<bool>,
    /// Immutable. KMS key name used for data encryption.
    #[prost(string, tag = "13")]
    pub kms_key_name: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Backup`.
pub mod backup {
    /// The backup state.
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
        /// State not set.
        Unspecified = 0,
        /// Backup is being created.
        Creating = 1,
        /// Backup has been taken and the operation is being finalized. At this
        /// point, changes to the file share will not be reflected in the backup.
        Finalizing = 2,
        /// Backup is available for use.
        Ready = 3,
        /// Backup is being deleted.
        Deleting = 4,
        /// Backup is not valid and cannot be used for creating new instances or
        /// restoring existing instances.
        Invalid = 5,
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
                State::Finalizing => "FINALIZING",
                State::Ready => "READY",
                State::Deleting => "DELETING",
                State::Invalid => "INVALID",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "FINALIZING" => Some(Self::Finalizing),
                "READY" => Some(Self::Ready),
                "DELETING" => Some(Self::Deleting),
                "INVALID" => Some(Self::Invalid),
                _ => None,
            }
        }
    }
}
/// CreateBackupRequest creates a backup.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBackupRequest {
    /// Required. The backup's project and location, in the format
    /// `projects/{project_id}/locations/{location}`. In Filestore,
    /// backup locations map to Google Cloud regions, for example **us-west1**.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. A [backup resource][google.cloud.filestore.v1beta1.Backup]
    #[prost(message, optional, tag = "2")]
    pub backup: ::core::option::Option<Backup>,
    /// Required. The ID to use for the backup.
    /// The ID must be unique within the specified project and location.
    ///
    /// This value must start with a lowercase letter followed by up to 62
    /// lowercase letters, numbers, or hyphens, and cannot end with a hyphen.
    #[prost(string, tag = "3")]
    pub backup_id: ::prost::alloc::string::String,
}
/// DeleteBackupRequest deletes a backup.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteBackupRequest {
    /// Required. The backup resource name, in the format
    /// `projects/{project_id}/locations/{location}/backups/{backup_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// UpdateBackupRequest updates description and/or labels for a backup.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBackupRequest {
    /// Required. A [backup resource][google.cloud.filestore.v1beta1.Backup]
    #[prost(message, optional, tag = "1")]
    pub backup: ::core::option::Option<Backup>,
    /// Required. Mask of fields to update.  At least one path must be supplied in
    /// this field.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// GetBackupRequest gets the state of a backup.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBackupRequest {
    /// Required. The backup resource name, in the format
    /// `projects/{project_id}/locations/{location}/backups/{backup_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// ListBackupsRequest lists backups.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBackupsRequest {
    /// Required. The project and location for which to retrieve backup
    /// information, in the format `projects/{project_id}/locations/{location}`. In
    /// Filestore, backup locations map to Google Cloud regions, for example
    /// **us-west1**. To retrieve backup information for all locations, use "-" for
    /// the
    /// `{location}` value.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value to use if there are additional
    /// results to retrieve for this list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Sort results. Supported values are "name", "name desc" or "" (unsorted).
    #[prost(string, tag = "4")]
    pub order_by: ::prost::alloc::string::String,
    /// List filter.
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
}
/// ListBackupsResponse is the result of ListBackupsRequest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBackupsResponse {
    /// A list of backups in the project for the specified location.
    ///
    /// If the `{location}` value in the request is "-", the response contains a
    /// list of backups from all locations. If any location is unreachable, the
    /// response will only return backups in reachable locations and the
    /// "unreachable" field will be populated with a list of unreachable
    /// locations.
    #[prost(message, repeated, tag = "1")]
    pub backups: ::prost::alloc::vec::Vec<Backup>,
    /// The token you can use to retrieve the next page of results. Not returned
    /// if there are no more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A Filestore share.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Share {
    /// Output only. The resource name of the share, in the format
    /// `projects/{project_id}/locations/{location_id}/instances/{instance_id}/shares/{share_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The mount name of the share. Must be 63 characters or less and consist of
    /// uppercase or lowercase letters, numbers, and underscores.
    #[prost(string, tag = "2")]
    pub mount_name: ::prost::alloc::string::String,
    /// A description of the share with 2048 characters or less. Requests with
    /// longer descriptions will be rejected.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// File share capacity in gigabytes (GB). Filestore defines 1 GB as
    /// 1024^3 bytes. Must be greater than 0.
    #[prost(int64, tag = "4")]
    pub capacity_gb: i64,
    /// Nfs Export Options.
    /// There is a limit of 10 export options per file share.
    #[prost(message, repeated, tag = "5")]
    pub nfs_export_options: ::prost::alloc::vec::Vec<NfsExportOptions>,
    /// Output only. The share state.
    #[prost(enumeration = "share::State", tag = "6")]
    pub state: i32,
    /// Output only. The time when the share was created.
    #[prost(message, optional, tag = "7")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Resource labels to represent user provided metadata.
    #[prost(map = "string, string", tag = "8")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The source that this Share has been restored from. Empty if the Share is
    /// created from scratch.
    #[prost(oneof = "share::Source", tags = "9")]
    pub source: ::core::option::Option<share::Source>,
}
/// Nested message and enum types in `Share`.
pub mod share {
    /// The share state.
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
        /// State not set.
        Unspecified = 0,
        /// Share is being created.
        Creating = 1,
        /// Share is ready for use.
        Ready = 3,
        /// Share is being deleted.
        Deleting = 4,
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
                State::Deleting => "DELETING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "READY" => Some(Self::Ready),
                "DELETING" => Some(Self::Deleting),
                _ => None,
            }
        }
    }
    /// The source that this Share has been restored from. Empty if the Share is
    /// created from scratch.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// Immutable. Full name of the Cloud Filestore Backup resource that this
        /// Share is restored from, in the format of
        /// projects/{project_id}/locations/{location_id}/backups/{backup_id}.
        /// Empty, if the Share is created from scratch and not restored from a
        /// backup.
        #[prost(string, tag = "9")]
        Backup(::prost::alloc::string::String),
    }
}
/// CreateShareRequest creates a share.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateShareRequest {
    /// Required. The Filestore Instance to create the share for, in the format
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID to use for the share.
    /// The ID must be unique within the specified instance.
    ///
    /// This value must start with a lowercase letter followed by up to 62
    /// lowercase letters, numbers, or hyphens, and cannot end with a hyphen.
    #[prost(string, tag = "2")]
    pub share_id: ::prost::alloc::string::String,
    /// Required. A share resource
    #[prost(message, optional, tag = "3")]
    pub share: ::core::option::Option<Share>,
}
/// GetShareRequest gets the state of a share.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetShareRequest {
    /// Required. The share resource name, in the format
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}/shares/{share_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// DeleteShareRequest deletes a share.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteShareRequest {
    /// Required. The share resource name, in the format
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}/share/{share_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// ListSharesRequest lists shares.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSharesRequest {
    /// Required. The instance for which to retrieve share information,
    /// in the format
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value to use if there are additional
    /// results to retrieve for this list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Sort results. Supported values are "name", "name desc" or "" (unsorted).
    #[prost(string, tag = "4")]
    pub order_by: ::prost::alloc::string::String,
    /// List filter.
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
}
/// ListSharesResponse is the result of ListSharesRequest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSharesResponse {
    /// A list of shares in the project for the specified instance.
    #[prost(message, repeated, tag = "1")]
    pub shares: ::prost::alloc::vec::Vec<Share>,
    /// The token you can use to retrieve the next page of results. Not returned
    /// if there are no more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// UpdateShareRequest updates the settings of a share.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateShareRequest {
    /// Required. A share resource.
    /// Only fields specified in update_mask are updated.
    #[prost(message, optional, tag = "1")]
    pub share: ::core::option::Option<Share>,
    /// Required. Mask of fields to update. At least one path must be supplied in
    /// this field. The elements of the repeated paths field may only include these
    /// fields:
    ///
    /// * "description"
    /// * "capacity_gb"
    /// * "labels"
    /// * "nfs_export_options"
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Generated client implementations.
pub mod cloud_filestore_manager_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Configures and manages Filestore resources.
    ///
    /// Filestore Manager v1beta1.
    ///
    /// The `file.googleapis.com` service implements the Filestore API and
    /// defines the following model for managing resources:
    /// * The service works with a collection of cloud projects, named: `/projects/*`
    /// * Each project has a collection of available locations, named: `/locations/*`
    /// * Each location has a collection of instances and backups, named:
    /// `/instances/*` and `/backups/*` respectively.
    /// * As such, Filestore instances are resources of the form:
    ///   `/projects/{project_id}/locations/{location_id}/instances/{instance_id}`
    ///   backups are resources of the form:
    ///   `/projects/{project_id}/locations/{location_id}/backup/{backup_id}`
    ///
    /// Note that location_id can represent a Google Cloud `zone` or `region`
    /// depending on the resource. for example: A zonal Filestore instance:
    /// * `projects/my-project/locations/us-central1-c/instances/my-basic-tier-filer`
    /// A regional Filestore instance:
    /// * `projects/my-project/locations/us-central1/instances/my-enterprise-filer`
    #[derive(Debug, Clone)]
    pub struct CloudFilestoreManagerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CloudFilestoreManagerClient<tonic::transport::Channel> {
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
    impl<T> CloudFilestoreManagerClient<T>
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
        ) -> CloudFilestoreManagerClient<InterceptedService<T, F>>
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
            CloudFilestoreManagerClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists all instances in a project for either a specified location
        /// or for all locations.
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
                "/google.cloud.filestore.v1beta1.CloudFilestoreManager/ListInstances",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.filestore.v1beta1.CloudFilestoreManager",
                        "ListInstances",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the details of a specific instance.
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
                "/google.cloud.filestore.v1beta1.CloudFilestoreManager/GetInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.filestore.v1beta1.CloudFilestoreManager",
                        "GetInstance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates an instance.
        /// When creating from a backup, the capacity of the new instance needs to be
        /// equal to or larger than the capacity of the backup (and also equal to or
        /// larger than the minimum capacity of the tier).
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
                "/google.cloud.filestore.v1beta1.CloudFilestoreManager/CreateInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.filestore.v1beta1.CloudFilestoreManager",
                        "CreateInstance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the settings of a specific instance.
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
                "/google.cloud.filestore.v1beta1.CloudFilestoreManager/UpdateInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.filestore.v1beta1.CloudFilestoreManager",
                        "UpdateInstance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Restores an existing instance's file share from a backup.
        ///
        /// The capacity of the instance needs to be equal to or larger than the
        /// capacity of the backup (and also equal to or larger than the minimum
        /// capacity of the tier).
        pub async fn restore_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::RestoreInstanceRequest>,
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
                "/google.cloud.filestore.v1beta1.CloudFilestoreManager/RestoreInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.filestore.v1beta1.CloudFilestoreManager",
                        "RestoreInstance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Revert an existing instance's file system to a specified snapshot.
        pub async fn revert_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::RevertInstanceRequest>,
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
                "/google.cloud.filestore.v1beta1.CloudFilestoreManager/RevertInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.filestore.v1beta1.CloudFilestoreManager",
                        "RevertInstance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes an instance.
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
                "/google.cloud.filestore.v1beta1.CloudFilestoreManager/DeleteInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.filestore.v1beta1.CloudFilestoreManager",
                        "DeleteInstance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all snapshots in a project for either a specified location
        /// or for all locations.
        pub async fn list_snapshots(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSnapshotsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListSnapshotsResponse>,
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
                "/google.cloud.filestore.v1beta1.CloudFilestoreManager/ListSnapshots",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.filestore.v1beta1.CloudFilestoreManager",
                        "ListSnapshots",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the details of a specific snapshot.
        pub async fn get_snapshot(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSnapshotRequest>,
        ) -> std::result::Result<tonic::Response<super::Snapshot>, tonic::Status> {
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
                "/google.cloud.filestore.v1beta1.CloudFilestoreManager/GetSnapshot",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.filestore.v1beta1.CloudFilestoreManager",
                        "GetSnapshot",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a snapshot.
        pub async fn create_snapshot(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSnapshotRequest>,
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
                "/google.cloud.filestore.v1beta1.CloudFilestoreManager/CreateSnapshot",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.filestore.v1beta1.CloudFilestoreManager",
                        "CreateSnapshot",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a snapshot.
        pub async fn delete_snapshot(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSnapshotRequest>,
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
                "/google.cloud.filestore.v1beta1.CloudFilestoreManager/DeleteSnapshot",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.filestore.v1beta1.CloudFilestoreManager",
                        "DeleteSnapshot",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the settings of a specific snapshot.
        pub async fn update_snapshot(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSnapshotRequest>,
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
                "/google.cloud.filestore.v1beta1.CloudFilestoreManager/UpdateSnapshot",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.filestore.v1beta1.CloudFilestoreManager",
                        "UpdateSnapshot",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all backups in a project for either a specified location or for all
        /// locations.
        pub async fn list_backups(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBackupsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListBackupsResponse>,
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
                "/google.cloud.filestore.v1beta1.CloudFilestoreManager/ListBackups",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.filestore.v1beta1.CloudFilestoreManager",
                        "ListBackups",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the details of a specific backup.
        pub async fn get_backup(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBackupRequest>,
        ) -> std::result::Result<tonic::Response<super::Backup>, tonic::Status> {
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
                "/google.cloud.filestore.v1beta1.CloudFilestoreManager/GetBackup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.filestore.v1beta1.CloudFilestoreManager",
                        "GetBackup",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a backup.
        pub async fn create_backup(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateBackupRequest>,
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
                "/google.cloud.filestore.v1beta1.CloudFilestoreManager/CreateBackup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.filestore.v1beta1.CloudFilestoreManager",
                        "CreateBackup",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a backup.
        pub async fn delete_backup(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteBackupRequest>,
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
                "/google.cloud.filestore.v1beta1.CloudFilestoreManager/DeleteBackup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.filestore.v1beta1.CloudFilestoreManager",
                        "DeleteBackup",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the settings of a specific backup.
        pub async fn update_backup(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateBackupRequest>,
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
                "/google.cloud.filestore.v1beta1.CloudFilestoreManager/UpdateBackup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.filestore.v1beta1.CloudFilestoreManager",
                        "UpdateBackup",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all shares for a specified instance.
        pub async fn list_shares(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSharesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListSharesResponse>,
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
                "/google.cloud.filestore.v1beta1.CloudFilestoreManager/ListShares",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.filestore.v1beta1.CloudFilestoreManager",
                        "ListShares",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the details of a specific share.
        pub async fn get_share(
            &mut self,
            request: impl tonic::IntoRequest<super::GetShareRequest>,
        ) -> std::result::Result<tonic::Response<super::Share>, tonic::Status> {
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
                "/google.cloud.filestore.v1beta1.CloudFilestoreManager/GetShare",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.filestore.v1beta1.CloudFilestoreManager",
                        "GetShare",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a share.
        pub async fn create_share(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateShareRequest>,
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
                "/google.cloud.filestore.v1beta1.CloudFilestoreManager/CreateShare",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.filestore.v1beta1.CloudFilestoreManager",
                        "CreateShare",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a share.
        pub async fn delete_share(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteShareRequest>,
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
                "/google.cloud.filestore.v1beta1.CloudFilestoreManager/DeleteShare",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.filestore.v1beta1.CloudFilestoreManager",
                        "DeleteShare",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the settings of a specific share.
        pub async fn update_share(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateShareRequest>,
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
                "/google.cloud.filestore.v1beta1.CloudFilestoreManager/UpdateShare",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.filestore.v1beta1.CloudFilestoreManager",
                        "UpdateShare",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
