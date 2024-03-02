/// The username/password for a database user. Used for specifying initial
/// users at cluster creation time.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserPassword {
    /// The database username.
    #[prost(string, tag = "1")]
    pub user: ::prost::alloc::string::String,
    /// The initial password for the user.
    #[prost(string, tag = "2")]
    pub password: ::prost::alloc::string::String,
}
/// Subset of the source instance configuration that is available when reading
/// the cluster resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MigrationSource {
    /// Output only. The host and port of the on-premises instance in host:port
    /// format
    #[prost(string, tag = "1")]
    pub host_port: ::prost::alloc::string::String,
    /// Output only. Place holder for the external source identifier(e.g DMS job
    /// name) that created the cluster.
    #[prost(string, tag = "2")]
    pub reference_id: ::prost::alloc::string::String,
    /// Output only. Type of migration source.
    #[prost(enumeration = "migration_source::MigrationSourceType", tag = "3")]
    pub source_type: i32,
}
/// Nested message and enum types in `MigrationSource`.
pub mod migration_source {
    /// Denote the type of migration source that created this cluster.
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
    pub enum MigrationSourceType {
        /// Migration source is unknown.
        Unspecified = 0,
        /// DMS source means the cluster was created via DMS migration job.
        Dms = 1,
    }
    impl MigrationSourceType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MigrationSourceType::Unspecified => "MIGRATION_SOURCE_TYPE_UNSPECIFIED",
                MigrationSourceType::Dms => "DMS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MIGRATION_SOURCE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "DMS" => Some(Self::Dms),
                _ => None,
            }
        }
    }
}
/// EncryptionConfig describes the encryption config of a cluster or a backup
/// that is encrypted with a CMEK (customer-managed encryption key).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EncryptionConfig {
    /// The fully-qualified resource name of the KMS key.
    /// Each Cloud KMS key is regionalized and has the following format:
    /// projects/\[PROJECT\]/locations/\[REGION\]/keyRings/\[RING\]/cryptoKeys/\[KEY_NAME\]
    #[prost(string, tag = "1")]
    pub kms_key_name: ::prost::alloc::string::String,
}
/// EncryptionInfo describes the encryption information of a cluster or a backup.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EncryptionInfo {
    /// Output only. Type of encryption.
    #[prost(enumeration = "encryption_info::Type", tag = "1")]
    pub encryption_type: i32,
    /// Output only. Cloud KMS key versions that are being used to protect the
    /// database or the backup.
    #[prost(string, repeated, tag = "2")]
    pub kms_key_versions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `EncryptionInfo`.
pub mod encryption_info {
    /// Possible encryption types.
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
        /// Encryption type not specified. Defaults to GOOGLE_DEFAULT_ENCRYPTION.
        Unspecified = 0,
        /// The data is encrypted at rest with a key that is fully managed by Google.
        /// No key version will be populated. This is the default state.
        GoogleDefaultEncryption = 1,
        /// The data is encrypted at rest with a key that is managed by the customer.
        /// KMS key versions will be populated.
        CustomerManagedEncryption = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::GoogleDefaultEncryption => "GOOGLE_DEFAULT_ENCRYPTION",
                Type::CustomerManagedEncryption => "CUSTOMER_MANAGED_ENCRYPTION",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "GOOGLE_DEFAULT_ENCRYPTION" => Some(Self::GoogleDefaultEncryption),
                "CUSTOMER_MANAGED_ENCRYPTION" => Some(Self::CustomerManagedEncryption),
                _ => None,
            }
        }
    }
}
/// SSL configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SslConfig {
    /// Optional. SSL mode. Specifies client-server SSL/TLS connection behavior.
    #[prost(enumeration = "ssl_config::SslMode", tag = "1")]
    pub ssl_mode: i32,
    /// Optional. Certificate Authority (CA) source. Only CA_SOURCE_MANAGED is
    /// supported currently, and is the default value.
    #[prost(enumeration = "ssl_config::CaSource", tag = "2")]
    pub ca_source: i32,
}
/// Nested message and enum types in `SslConfig`.
pub mod ssl_config {
    /// SSL mode options.
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
    pub enum SslMode {
        /// SSL mode not specified. Defaults to ENCRYPTED_ONLY.
        Unspecified = 0,
        /// SSL connections are optional. CA verification not enforced.
        Allow = 1,
        /// SSL connections are required. CA verification not enforced.
        /// Clients may use locally self-signed certificates (default psql client
        /// behavior).
        Require = 2,
        /// SSL connections are required. CA verification enforced.
        /// Clients must have certificates signed by a Cluster CA, e.g. via
        /// GenerateClientCertificate.
        VerifyCa = 3,
        /// SSL connections are optional. CA verification not enforced.
        AllowUnencryptedAndEncrypted = 4,
        /// SSL connections are required. CA verification not enforced.
        EncryptedOnly = 5,
    }
    impl SslMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SslMode::Unspecified => "SSL_MODE_UNSPECIFIED",
                SslMode::Allow => "SSL_MODE_ALLOW",
                SslMode::Require => "SSL_MODE_REQUIRE",
                SslMode::VerifyCa => "SSL_MODE_VERIFY_CA",
                SslMode::AllowUnencryptedAndEncrypted => {
                    "ALLOW_UNENCRYPTED_AND_ENCRYPTED"
                }
                SslMode::EncryptedOnly => "ENCRYPTED_ONLY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SSL_MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "SSL_MODE_ALLOW" => Some(Self::Allow),
                "SSL_MODE_REQUIRE" => Some(Self::Require),
                "SSL_MODE_VERIFY_CA" => Some(Self::VerifyCa),
                "ALLOW_UNENCRYPTED_AND_ENCRYPTED" => {
                    Some(Self::AllowUnencryptedAndEncrypted)
                }
                "ENCRYPTED_ONLY" => Some(Self::EncryptedOnly),
                _ => None,
            }
        }
    }
    /// Certificate Authority (CA) source for SSL/TLS certificates.
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
    pub enum CaSource {
        /// Certificate Authority (CA) source not specified. Defaults to
        /// CA_SOURCE_MANAGED.
        Unspecified = 0,
        /// Certificate Authority (CA) managed by the AlloyDB Cluster.
        Managed = 1,
    }
    impl CaSource {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CaSource::Unspecified => "CA_SOURCE_UNSPECIFIED",
                CaSource::Managed => "CA_SOURCE_MANAGED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CA_SOURCE_UNSPECIFIED" => Some(Self::Unspecified),
                "CA_SOURCE_MANAGED" => Some(Self::Managed),
                _ => None,
            }
        }
    }
}
/// Message describing the user-specified automated backup policy.
///
/// All fields in the automated backup policy are optional. Defaults for each
/// field are provided if they are not set.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutomatedBackupPolicy {
    /// Whether automated automated backups are enabled. If not set, defaults to
    /// true.
    #[prost(bool, optional, tag = "1")]
    pub enabled: ::core::option::Option<bool>,
    /// The length of the time window during which a backup can be
    /// taken. If a backup does not succeed within this time window, it will be
    /// canceled and considered failed.
    ///
    /// The backup window must be at least 5 minutes long. There is no upper bound
    /// on the window. If not set, it defaults to 1 hour.
    #[prost(message, optional, tag = "3")]
    pub backup_window: ::core::option::Option<::prost_types::Duration>,
    /// Optional. The encryption config can be specified to encrypt the
    /// backups with a customer-managed encryption key (CMEK). When this field is
    /// not specified, the backup will then use default encryption scheme to
    /// protect the user data.
    #[prost(message, optional, tag = "8")]
    pub encryption_config: ::core::option::Option<EncryptionConfig>,
    /// The location where the backup will be stored. Currently, the only supported
    /// option is to store the backup in the same region as the cluster.
    ///
    /// If empty, defaults to the region of the cluster.
    #[prost(string, tag = "6")]
    pub location: ::prost::alloc::string::String,
    /// Labels to apply to backups created using this configuration.
    #[prost(map = "string, string", tag = "7")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The schedule for this automated backup policy.
    ///
    /// A schedule specifies times at which to start a backup. If a backup
    /// window is also provided, the backup is guaranteed to be started and
    /// completed within the start time plus the backup window. If the backup is
    /// not completed within the backup window it is marked as failed.
    ///
    /// If not set, the schedule defaults to a weekly schedule with one backup
    /// per day and a start time chosen arbitrarily.
    #[prost(oneof = "automated_backup_policy::Schedule", tags = "2")]
    pub schedule: ::core::option::Option<automated_backup_policy::Schedule>,
    /// The retention policy for automated backups.
    ///
    /// The retention policy for a backup is fixed at the time the backup is
    /// created. Changes to this field only apply to new backups taken with the
    /// policy; the retentions of existing backups remain unchanged.
    ///
    /// If no retention policy is set, a default of 14 days is used.
    #[prost(oneof = "automated_backup_policy::Retention", tags = "4, 5")]
    pub retention: ::core::option::Option<automated_backup_policy::Retention>,
}
/// Nested message and enum types in `AutomatedBackupPolicy`.
pub mod automated_backup_policy {
    /// A weekly schedule starts a backup at prescribed start times within a
    /// day, for the specified days of the week.
    ///
    /// The weekly schedule message is flexible and can be used to create many
    /// types of schedules. For example, to have a daily backup that starts at
    /// 22:00, configure the `start_times` field to have one element "22:00" and
    /// the `days_of_week` field to have all seven days of the week.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WeeklySchedule {
        /// The times during the day to start a backup. The start times are assumed
        /// to be in UTC and to be an exact hour (e.g., 04:00:00).
        ///
        /// If no start times are provided, a single fixed start time is chosen
        /// arbitrarily.
        #[prost(message, repeated, tag = "1")]
        pub start_times: ::prost::alloc::vec::Vec<
            super::super::super::super::r#type::TimeOfDay,
        >,
        /// The days of the week to perform a backup.
        ///
        /// If this field is left empty, the default of every day of the week is
        /// used.
        #[prost(
            enumeration = "super::super::super::super::r#type::DayOfWeek",
            repeated,
            tag = "2"
        )]
        pub days_of_week: ::prost::alloc::vec::Vec<i32>,
    }
    /// A time based retention policy specifies that all backups within a certain
    /// time period should be retained.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TimeBasedRetention {
        /// The retention period.
        #[prost(message, optional, tag = "1")]
        pub retention_period: ::core::option::Option<::prost_types::Duration>,
    }
    /// A quantity based policy specifies that a certain number of the most recent
    /// successful backups should be retained.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QuantityBasedRetention {
        /// The number of backups to retain.
        #[prost(int32, tag = "1")]
        pub count: i32,
    }
    /// The schedule for this automated backup policy.
    ///
    /// A schedule specifies times at which to start a backup. If a backup
    /// window is also provided, the backup is guaranteed to be started and
    /// completed within the start time plus the backup window. If the backup is
    /// not completed within the backup window it is marked as failed.
    ///
    /// If not set, the schedule defaults to a weekly schedule with one backup
    /// per day and a start time chosen arbitrarily.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Schedule {
        /// Weekly schedule for the Backup.
        #[prost(message, tag = "2")]
        WeeklySchedule(WeeklySchedule),
    }
    /// The retention policy for automated backups.
    ///
    /// The retention policy for a backup is fixed at the time the backup is
    /// created. Changes to this field only apply to new backups taken with the
    /// policy; the retentions of existing backups remain unchanged.
    ///
    /// If no retention policy is set, a default of 14 days is used.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Retention {
        /// Time-based Backup retention policy.
        #[prost(message, tag = "4")]
        TimeBasedRetention(TimeBasedRetention),
        /// Quantity-based Backup retention policy to retain recent backups.
        #[prost(message, tag = "5")]
        QuantityBasedRetention(QuantityBasedRetention),
    }
}
/// ContinuousBackupConfig describes the continuous backups recovery
/// configurations of a cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContinuousBackupConfig {
    /// Whether ContinuousBackup is enabled.
    #[prost(bool, optional, tag = "1")]
    pub enabled: ::core::option::Option<bool>,
    /// The number of days that are eligible to restore from using PITR. To support
    /// the entire recovery window, backups and logs are retained for one day more
    /// than the recovery window. If not set, defaults to 14 days.
    #[prost(int32, tag = "4")]
    pub recovery_window_days: i32,
    /// The encryption config can be specified to encrypt the
    /// backups with a customer-managed encryption key (CMEK). When this field is
    /// not specified, the backup will then use default encryption scheme to
    /// protect the user data.
    #[prost(message, optional, tag = "3")]
    pub encryption_config: ::core::option::Option<EncryptionConfig>,
}
/// ContinuousBackupInfo describes the continuous backup properties of a
/// cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContinuousBackupInfo {
    /// Output only. The encryption information for the WALs and backups required
    /// for ContinuousBackup.
    #[prost(message, optional, tag = "1")]
    pub encryption_info: ::core::option::Option<EncryptionInfo>,
    /// Output only. When ContinuousBackup was most recently enabled. Set to null
    /// if ContinuousBackup is not enabled.
    #[prost(message, optional, tag = "2")]
    pub enabled_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Days of the week on which a continuous backup is taken. Output
    /// only field. Ignored if passed into the request.
    #[prost(
        enumeration = "super::super::super::r#type::DayOfWeek",
        repeated,
        packed = "false",
        tag = "3"
    )]
    pub schedule: ::prost::alloc::vec::Vec<i32>,
    /// Output only. The earliest restorable time that can be restored to. Output
    /// only field.
    #[prost(message, optional, tag = "4")]
    pub earliest_restorable_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Message describing a BackupSource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupSource {
    /// Output only. The system-generated UID of the backup which was used to
    /// create this resource. The UID is generated when the backup is created, and
    /// it is retained until the backup is deleted.
    #[prost(string, tag = "2")]
    pub backup_uid: ::prost::alloc::string::String,
    /// Required. The name of the backup resource with the format:
    ///   * projects/{project}/locations/{region}/backups/{backup_id}
    #[prost(string, tag = "1")]
    pub backup_name: ::prost::alloc::string::String,
}
/// Message describing a ContinuousBackupSource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContinuousBackupSource {
    /// Required. The source cluster from which to restore. This cluster must have
    /// continuous backup enabled for this operation to succeed. For the required
    /// format, see the comment on the Cluster.name field.
    #[prost(string, tag = "1")]
    pub cluster: ::prost::alloc::string::String,
    /// Required. The point in time to restore to.
    #[prost(message, optional, tag = "2")]
    pub point_in_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// A cluster is a collection of regional AlloyDB resources. It can include a
/// primary instance and one or more read pool instances.
/// All cluster resources share a storage layer, which scales as needed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cluster {
    /// Output only. The name of the cluster resource with the format:
    ///   * projects/{project}/locations/{region}/clusters/{cluster_id}
    /// where the cluster ID segment should satisfy the regex expression
    /// `\[a-z0-9-\]+`. For more details see <https://google.aip.dev/122.>
    /// The prefix of the cluster resource name is the name of the parent resource:
    ///   * projects/{project}/locations/{region}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// User-settable and human-readable display name for the Cluster.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. The system-generated UID of the resource. The UID is assigned
    /// when the resource is created, and it is retained until it is deleted.
    #[prost(string, tag = "3")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. Create time stamp
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Update time stamp
    #[prost(message, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Delete time stamp
    #[prost(message, optional, tag = "6")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Labels as key value pairs
    #[prost(map = "string, string", tag = "7")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. The current serving state of the cluster.
    #[prost(enumeration = "cluster::State", tag = "8")]
    pub state: i32,
    /// Output only. The type of the cluster. This is an output-only field and it's
    /// populated at the Cluster creation time or the Cluster promotion
    /// time. The cluster type is determined by which RPC was used to create
    /// the cluster (i.e. `CreateCluster` vs. `CreateSecondaryCluster`
    #[prost(enumeration = "cluster::ClusterType", tag = "24")]
    pub cluster_type: i32,
    /// Optional. The database engine major version. This is an optional field and
    /// it is populated at the Cluster creation time. If a database version is not
    /// supplied at cluster creation time, then a default database version will
    /// be used.
    #[prost(enumeration = "DatabaseVersion", tag = "9")]
    pub database_version: i32,
    #[prost(message, optional, tag = "29")]
    pub network_config: ::core::option::Option<cluster::NetworkConfig>,
    /// Required. The resource link for the VPC network in which cluster resources
    /// are created and from which they are accessible via Private IP. The network
    /// must belong to the same project as the cluster. It is specified in the
    /// form: "projects/{project}/global/networks/{network_id}". This is required
    /// to create a cluster. Deprecated, use network_config.network instead.
    #[deprecated]
    #[prost(string, tag = "10")]
    pub network: ::prost::alloc::string::String,
    /// For Resource freshness validation (<https://google.aip.dev/154>)
    #[prost(string, tag = "11")]
    pub etag: ::prost::alloc::string::String,
    /// Annotations to allow client tools to store small amount of arbitrary data.
    /// This is distinct from labels.
    /// <https://google.aip.dev/128>
    #[prost(map = "string, string", tag = "12")]
    pub annotations: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. Reconciling (<https://google.aip.dev/128#reconciliation>).
    /// Set to true if the current state of Cluster does not match the user's
    /// intended state, and the service is actively updating the resource to
    /// reconcile them. This can happen due to user-triggered updates or
    /// system actions like failover or maintenance.
    #[prost(bool, tag = "13")]
    pub reconciling: bool,
    /// Input only. Initial user to setup during cluster creation. Required.
    /// If used in `RestoreCluster` this is ignored.
    #[prost(message, optional, tag = "14")]
    pub initial_user: ::core::option::Option<UserPassword>,
    /// The automated backup policy for this cluster.
    ///
    /// If no policy is provided then the default policy will be used. If backups
    /// are supported for the cluster, the default policy takes one backup a day,
    /// has a backup window of 1 hour, and retains backups for 14 days.
    /// For more information on the defaults, consult the
    /// documentation for the message type.
    #[prost(message, optional, tag = "17")]
    pub automated_backup_policy: ::core::option::Option<AutomatedBackupPolicy>,
    /// SSL configuration for this AlloyDB cluster.
    #[deprecated]
    #[prost(message, optional, tag = "18")]
    pub ssl_config: ::core::option::Option<SslConfig>,
    /// Optional. The encryption config can be specified to encrypt the data disks
    /// and other persistent data resources of a cluster with a
    /// customer-managed encryption key (CMEK). When this field is not
    /// specified, the cluster will then use default encryption scheme to
    /// protect the user data.
    #[prost(message, optional, tag = "19")]
    pub encryption_config: ::core::option::Option<EncryptionConfig>,
    /// Output only. The encryption information for the cluster.
    #[prost(message, optional, tag = "20")]
    pub encryption_info: ::core::option::Option<EncryptionInfo>,
    /// Optional. Continuous backup configuration for this cluster.
    #[prost(message, optional, tag = "27")]
    pub continuous_backup_config: ::core::option::Option<ContinuousBackupConfig>,
    /// Output only. Continuous backup properties for this cluster.
    #[prost(message, optional, tag = "28")]
    pub continuous_backup_info: ::core::option::Option<ContinuousBackupInfo>,
    /// Cross Region replication config specific to SECONDARY cluster.
    #[prost(message, optional, tag = "22")]
    pub secondary_config: ::core::option::Option<cluster::SecondaryConfig>,
    /// Output only. Cross Region replication config specific to PRIMARY cluster.
    #[prost(message, optional, tag = "23")]
    pub primary_config: ::core::option::Option<cluster::PrimaryConfig>,
    /// Output only. Reserved for future use.
    #[prost(bool, tag = "33")]
    pub satisfies_pzi: bool,
    /// Output only. Reserved for future use.
    #[prost(bool, tag = "30")]
    pub satisfies_pzs: bool,
    /// Optional. The configuration for Private Service Connect (PSC) for the
    /// cluster.
    #[prost(message, optional, tag = "31")]
    pub psc_config: ::core::option::Option<cluster::PscConfig>,
    /// In case of an imported cluster, this field contains information about the
    /// source this cluster was imported from.
    #[prost(oneof = "cluster::Source", tags = "15, 16")]
    pub source: ::core::option::Option<cluster::Source>,
}
/// Nested message and enum types in `Cluster`.
pub mod cluster {
    /// Metadata related to network configuration.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NetworkConfig {
        /// Optional. The resource link for the VPC network in which cluster
        /// resources are created and from which they are accessible via Private IP.
        /// The network must belong to the same project as the cluster. It is
        /// specified in the form:
        /// "projects/{project_number}/global/networks/{network_id}". This is
        /// required to create a cluster.
        #[prost(string, tag = "1")]
        pub network: ::prost::alloc::string::String,
        /// Optional. Name of the allocated IP range for the private IP AlloyDB
        /// cluster, for example: "google-managed-services-default". If set, the
        /// instance IPs for this cluster will be created in the allocated range. The
        /// range name must comply with RFC 1035. Specifically, the name must be 1-63
        /// characters long and match the regular expression
        /// `[a-z](\[-a-z0-9\]*[a-z0-9])?`.
        /// Field name is intended to be consistent with Cloud SQL.
        #[prost(string, tag = "2")]
        pub allocated_ip_range: ::prost::alloc::string::String,
    }
    /// Configuration information for the secondary cluster. This should be set
    /// if and only if the cluster is of type SECONDARY.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SecondaryConfig {
        /// The name of the primary cluster name with the format:
        /// * projects/{project}/locations/{region}/clusters/{cluster_id}
        #[prost(string, tag = "1")]
        pub primary_cluster_name: ::prost::alloc::string::String,
    }
    /// Configuration for the primary cluster. It has the list of clusters that are
    /// replicating from this cluster. This should be set if and only if the
    /// cluster is of type PRIMARY.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PrimaryConfig {
        /// Output only. Names of the clusters that are replicating from this
        /// cluster.
        #[prost(string, repeated, tag = "1")]
        pub secondary_cluster_names: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
    }
    /// PscConfig contains PSC related configuration at a cluster level.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PscConfig {
        /// Optional. Create an instance that allows connections from Private Service
        /// Connect endpoints to the instance.
        #[prost(bool, tag = "1")]
        pub psc_enabled: bool,
    }
    /// Cluster State
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
        /// The state of the cluster is unknown.
        Unspecified = 0,
        /// The cluster is active and running.
        Ready = 1,
        /// The cluster is stopped. All instances in the cluster are stopped.
        /// Customers can start a stopped cluster at any point and all their
        /// instances will come back to life with same names and IP resources. In
        /// this state, customer pays for storage.
        /// Associated backups could also be present in a stopped cluster.
        Stopped = 2,
        /// The cluster is empty and has no associated resources.
        /// All instances, associated storage and backups have been deleted.
        Empty = 3,
        /// The cluster is being created.
        Creating = 4,
        /// The cluster is being deleted.
        Deleting = 5,
        /// The creation of the cluster failed.
        Failed = 6,
        /// The cluster is bootstrapping with data from some other source.
        /// Direct mutations to the cluster (e.g. adding read pool) are not allowed.
        Bootstrapping = 7,
        /// The cluster is under maintenance. AlloyDB regularly performs maintenance
        /// and upgrades on customer clusters. Updates on the cluster are
        /// not allowed while the cluster is in this state.
        Maintenance = 8,
        /// The cluster is being promoted.
        Promoting = 9,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Ready => "READY",
                State::Stopped => "STOPPED",
                State::Empty => "EMPTY",
                State::Creating => "CREATING",
                State::Deleting => "DELETING",
                State::Failed => "FAILED",
                State::Bootstrapping => "BOOTSTRAPPING",
                State::Maintenance => "MAINTENANCE",
                State::Promoting => "PROMOTING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "READY" => Some(Self::Ready),
                "STOPPED" => Some(Self::Stopped),
                "EMPTY" => Some(Self::Empty),
                "CREATING" => Some(Self::Creating),
                "DELETING" => Some(Self::Deleting),
                "FAILED" => Some(Self::Failed),
                "BOOTSTRAPPING" => Some(Self::Bootstrapping),
                "MAINTENANCE" => Some(Self::Maintenance),
                "PROMOTING" => Some(Self::Promoting),
                _ => None,
            }
        }
    }
    /// Type of Cluster
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
    pub enum ClusterType {
        /// The type of the cluster is unknown.
        Unspecified = 0,
        /// Primary cluster that support read and write operations.
        Primary = 1,
        /// Secondary cluster that is replicating from another region.
        /// This only supports read.
        Secondary = 2,
    }
    impl ClusterType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ClusterType::Unspecified => "CLUSTER_TYPE_UNSPECIFIED",
                ClusterType::Primary => "PRIMARY",
                ClusterType::Secondary => "SECONDARY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CLUSTER_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "PRIMARY" => Some(Self::Primary),
                "SECONDARY" => Some(Self::Secondary),
                _ => None,
            }
        }
    }
    /// In case of an imported cluster, this field contains information about the
    /// source this cluster was imported from.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// Output only. Cluster created from backup.
        #[prost(message, tag = "15")]
        BackupSource(super::BackupSource),
        /// Output only. Cluster created via DMS migration.
        #[prost(message, tag = "16")]
        MigrationSource(super::MigrationSource),
    }
}
/// An Instance is a computing unit that an end customer can connect to.
/// It's the main unit of computing resources in AlloyDB.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instance {
    /// Output only. The name of the instance resource with the format:
    ///   * projects/{project}/locations/{region}/clusters/{cluster_id}/instances/{instance_id}
    /// where the cluster and instance ID segments should satisfy the regex
    /// expression `[a-z](\[a-z0-9-\]{0,61}\[a-z0-9\])?`, e.g. 1-63 characters of
    /// lowercase letters, numbers, and dashes, starting with a letter, and ending
    /// with a letter or number. For more details see <https://google.aip.dev/122.>
    /// The prefix of the instance resource name is the name of the parent
    /// resource:
    ///   * projects/{project}/locations/{region}/clusters/{cluster_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// User-settable and human-readable display name for the Instance.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. The system-generated UID of the resource. The UID is assigned
    /// when the resource is created, and it is retained until it is deleted.
    #[prost(string, tag = "3")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. Create time stamp
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Update time stamp
    #[prost(message, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Delete time stamp
    #[prost(message, optional, tag = "6")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Labels as key value pairs
    #[prost(map = "string, string", tag = "7")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. The current serving state of the instance.
    #[prost(enumeration = "instance::State", tag = "8")]
    pub state: i32,
    /// Required. The type of the instance. Specified at creation time.
    #[prost(enumeration = "instance::InstanceType", tag = "9")]
    pub instance_type: i32,
    /// Configurations for the machines that host the underlying
    /// database engine.
    #[prost(message, optional, tag = "10")]
    pub machine_config: ::core::option::Option<instance::MachineConfig>,
    /// Availability type of an Instance.
    /// If empty, defaults to REGIONAL for primary instances.
    /// For read pools, availability_type is always UNSPECIFIED. Instances in the
    /// read pools are evenly distributed across available zones within the region
    /// (i.e. read pools with more than one node will have a node in at
    /// least two zones).
    #[prost(enumeration = "instance::AvailabilityType", tag = "11")]
    pub availability_type: i32,
    /// The Compute Engine zone that the instance should serve from, per
    /// <https://cloud.google.com/compute/docs/regions-zones>
    /// This can ONLY be specified for ZONAL instances.
    /// If present for a REGIONAL instance, an error will be thrown.
    /// If this is absent for a ZONAL instance, instance is created in a random
    /// zone with available capacity.
    #[prost(string, tag = "12")]
    pub gce_zone: ::prost::alloc::string::String,
    /// Database flags. Set at instance level.
    ///   * They are copied from primary instance on read instance creation.
    ///   * Read instances can set new or override existing flags that are relevant
    ///     for reads, e.g. for enabling columnar cache on a read instance. Flags
    ///     set on read instance may or may not be present on primary.
    ///
    ///
    /// This is a list of "key": "value" pairs.
    /// "key": The name of the flag. These flags are passed at instance setup time,
    /// so include both server options and system variables for Postgres. Flags are
    /// specified with underscores, not hyphens.
    /// "value": The value of the flag. Booleans are set to **on** for true
    /// and **off** for false. This field must be omitted if the flag
    /// doesn't take a value.
    #[prost(map = "string, string", tag = "13")]
    pub database_flags: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. This is set for the read-write VM of the PRIMARY instance
    /// only.
    #[prost(message, optional, tag = "19")]
    pub writable_node: ::core::option::Option<instance::Node>,
    /// Output only. List of available read-only VMs in this instance, including
    /// the standby for a PRIMARY instance.
    #[prost(message, repeated, tag = "20")]
    pub nodes: ::prost::alloc::vec::Vec<instance::Node>,
    /// Configuration for query insights.
    #[prost(message, optional, tag = "21")]
    pub query_insights_config: ::core::option::Option<
        instance::QueryInsightsInstanceConfig,
    >,
    /// Read pool instance configuration.
    /// This is required if the value of instanceType is READ_POOL.
    #[prost(message, optional, tag = "14")]
    pub read_pool_config: ::core::option::Option<instance::ReadPoolConfig>,
    /// Output only. The IP address for the Instance.
    /// This is the connection endpoint for an end-user application.
    #[prost(string, tag = "15")]
    pub ip_address: ::prost::alloc::string::String,
    /// Output only. The public IP addresses for the Instance. This is available
    /// ONLY when enable_public_ip is set. This is the connection endpoint for an
    /// end-user application.
    #[prost(string, tag = "27")]
    pub public_ip_address: ::prost::alloc::string::String,
    /// Output only. Reconciling (<https://google.aip.dev/128#reconciliation>).
    /// Set to true if the current state of Instance does not match the user's
    /// intended state, and the service is actively updating the resource to
    /// reconcile them. This can happen due to user-triggered updates or
    /// system actions like failover or maintenance.
    #[prost(bool, tag = "16")]
    pub reconciling: bool,
    /// For Resource freshness validation (<https://google.aip.dev/154>)
    #[prost(string, tag = "17")]
    pub etag: ::prost::alloc::string::String,
    /// Annotations to allow client tools to store small amount of arbitrary data.
    /// This is distinct from labels.
    /// <https://google.aip.dev/128>
    #[prost(map = "string, string", tag = "18")]
    pub annotations: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Update policy that will be applied during instance update.
    /// This field is not persisted when you update the instance.
    /// To use a non-default update policy, you must
    /// specify explicitly specify the value in each update request.
    #[prost(message, optional, tag = "22")]
    pub update_policy: ::core::option::Option<instance::UpdatePolicy>,
    /// Optional. Client connection specific configurations
    #[prost(message, optional, tag = "23")]
    pub client_connection_config: ::core::option::Option<
        instance::ClientConnectionConfig,
    >,
    /// Output only. Reserved for future use.
    #[prost(bool, tag = "30")]
    pub satisfies_pzi: bool,
    /// Output only. Reserved for future use.
    #[prost(bool, tag = "24")]
    pub satisfies_pzs: bool,
    /// Optional. The configuration for Private Service Connect (PSC) for the
    /// instance.
    #[prost(message, optional, tag = "28")]
    pub psc_instance_config: ::core::option::Option<instance::PscInstanceConfig>,
    /// Optional. Instance level network configuration.
    #[prost(message, optional, tag = "29")]
    pub network_config: ::core::option::Option<instance::InstanceNetworkConfig>,
}
/// Nested message and enum types in `Instance`.
pub mod instance {
    /// MachineConfig describes the configuration of a machine.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MachineConfig {
        /// The number of CPU's in the VM instance.
        #[prost(int32, tag = "1")]
        pub cpu_count: i32,
    }
    /// Details of a single node in the instance.
    /// Nodes in an AlloyDB instance are ephemereal, they can change during
    /// update, failover, autohealing and resize operations.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Node {
        /// The Compute Engine zone of the VM e.g. "us-central1-b".
        #[prost(string, tag = "1")]
        pub zone_id: ::prost::alloc::string::String,
        /// The identifier of the VM e.g. "test-read-0601-407e52be-ms3l".
        #[prost(string, tag = "2")]
        pub id: ::prost::alloc::string::String,
        /// The private IP address of the VM e.g. "10.57.0.34".
        #[prost(string, tag = "3")]
        pub ip: ::prost::alloc::string::String,
        /// Determined by state of the compute VM and postgres-service health.
        /// Compute VM state can have values listed in
        /// <https://cloud.google.com/compute/docs/instances/instance-life-cycle> and
        /// postgres-service health can have values: HEALTHY and UNHEALTHY.
        #[prost(string, tag = "4")]
        pub state: ::prost::alloc::string::String,
    }
    /// QueryInsights Instance specific configuration.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QueryInsightsInstanceConfig {
        /// Record application tags for an instance.
        /// This flag is turned "on" by default.
        #[prost(bool, optional, tag = "2")]
        pub record_application_tags: ::core::option::Option<bool>,
        /// Record client address for an instance. Client address is PII information.
        /// This flag is turned "on" by default.
        #[prost(bool, optional, tag = "3")]
        pub record_client_address: ::core::option::Option<bool>,
        /// Query string length. The default value is 1024.
        /// Any integer between 256 and 4500 is considered valid.
        #[prost(uint32, tag = "4")]
        pub query_string_length: u32,
        /// Number of query execution plans captured by Insights per minute
        /// for all queries combined. The default value is 5.
        /// Any integer between 0 and 20 is considered valid.
        #[prost(uint32, optional, tag = "5")]
        pub query_plans_per_minute: ::core::option::Option<u32>,
    }
    /// Configuration for a read pool instance.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ReadPoolConfig {
        /// Read capacity, i.e. number of nodes in a read pool instance.
        #[prost(int32, tag = "1")]
        pub node_count: i32,
    }
    /// Policy to be used while updating the instance.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UpdatePolicy {
        /// Mode for updating the instance.
        #[prost(enumeration = "update_policy::Mode", tag = "1")]
        pub mode: i32,
    }
    /// Nested message and enum types in `UpdatePolicy`.
    pub mod update_policy {
        /// Specifies the available modes of update.
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
        pub enum Mode {
            /// Mode is unknown.
            Unspecified = 0,
            /// Least disruptive way to apply the update.
            Default = 1,
            /// Performs a forced update when applicable. This will be fast but may
            /// incur a downtime.
            ForceApply = 2,
        }
        impl Mode {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Mode::Unspecified => "MODE_UNSPECIFIED",
                    Mode::Default => "DEFAULT",
                    Mode::ForceApply => "FORCE_APPLY",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "MODE_UNSPECIFIED" => Some(Self::Unspecified),
                    "DEFAULT" => Some(Self::Default),
                    "FORCE_APPLY" => Some(Self::ForceApply),
                    _ => None,
                }
            }
        }
    }
    /// Client connection configuration
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ClientConnectionConfig {
        /// Optional. Configuration to enforce connectors only (ex: AuthProxy)
        /// connections to the database.
        #[prost(bool, tag = "1")]
        pub require_connectors: bool,
        /// Optional. SSL config option for this instance.
        #[prost(message, optional, tag = "2")]
        pub ssl_config: ::core::option::Option<super::SslConfig>,
    }
    /// Configuration for setting up a PSC interface. This information needs to be
    /// provided by the customer.
    /// PSC interfaces will be created and added to VMs via SLM (adding a network
    /// interface will require recreating the VM). For HA instances this will be
    /// done via LDTM.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PscInterfaceConfig {
        /// A list of endpoints in the consumer VPC the interface might initiate
        /// outbound connections to. This list has to be provided when the PSC
        /// interface is created.
        #[prost(string, repeated, tag = "1")]
        pub consumer_endpoint_ips: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
        /// The NetworkAttachment resource created in the consumer VPC to which the
        /// PSC interface will be linked, in the form of:
        /// "projects/${CONSUMER_PROJECT}/regions/${REGION}/networkAttachments/${NETWORK_ATTACHMENT_NAME}".
        /// NetworkAttachment has to be provided when the PSC interface is created.
        #[prost(string, tag = "2")]
        pub network_attachment: ::prost::alloc::string::String,
    }
    /// PscInstanceConfig contains PSC related configuration at an
    /// instance level.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PscInstanceConfig {
        /// Output only. The service attachment created when Private
        /// Service Connect (PSC) is enabled for the instance.
        /// The name of the resource will be in the format of
        /// projects/<alloydb-tenant-project-number>/regions/<region-name>/serviceAttachments/<service-attachment-name>
        #[prost(string, tag = "1")]
        pub service_attachment_link: ::prost::alloc::string::String,
        /// Optional. List of consumer projects that are allowed to create
        /// PSC endpoints to service-attachments to this instance.
        #[prost(string, repeated, tag = "2")]
        pub allowed_consumer_projects: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
        /// Optional. List of consumer networks that are allowed to create
        /// PSC endpoints to service-attachments to this instance.
        #[prost(string, repeated, tag = "3")]
        pub allowed_consumer_networks: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
        /// Optional. Configurations for setting up PSC interfaces attached to the
        /// instance which are used for outbound connectivity. Only primary instances
        /// can have PSC interface attached. All the VMs created for the primary
        /// instance will share the same configurations. Currently we only support 0
        /// or 1 PSC interface.
        #[prost(message, repeated, tag = "4")]
        pub psc_interface_configs: ::prost::alloc::vec::Vec<PscInterfaceConfig>,
        /// Optional. List of service attachments that this instance has created
        /// endpoints to connect with. Currently, only a single outgoing service
        /// attachment is supported per instance.
        #[prost(string, repeated, tag = "5")]
        pub outgoing_service_attachment_links: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
        /// Optional. Whether PSC connectivity is enabled for this instance.
        /// This is populated by referencing the value from the parent cluster.
        #[prost(bool, tag = "6")]
        pub psc_enabled: bool,
    }
    /// Metadata related to instance level network configuration.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InstanceNetworkConfig {
        /// Optional. A list of external network authorized to access this instance.
        #[prost(message, repeated, tag = "1")]
        pub authorized_external_networks: ::prost::alloc::vec::Vec<
            instance_network_config::AuthorizedNetwork,
        >,
        /// Optional. Enabling public ip for the instance.
        #[prost(bool, tag = "2")]
        pub enable_public_ip: bool,
    }
    /// Nested message and enum types in `InstanceNetworkConfig`.
    pub mod instance_network_config {
        /// AuthorizedNetwork contains metadata for an authorized network.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct AuthorizedNetwork {
            /// CIDR range for one authorzied network of the instance.
            #[prost(string, tag = "1")]
            pub cidr_range: ::prost::alloc::string::String,
        }
    }
    /// Instance State
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
        /// The state of the instance is unknown.
        Unspecified = 0,
        /// The instance is active and running.
        Ready = 1,
        /// The instance is stopped. Instance name and IP resources are preserved.
        Stopped = 2,
        /// The instance is being created.
        Creating = 3,
        /// The instance is being deleted.
        Deleting = 4,
        /// The instance is down for maintenance.
        Maintenance = 5,
        /// The creation of the instance failed or a fatal error occurred during
        /// an operation on the instance.
        /// Note: Instances in this state would tried to be auto-repaired. And
        /// Customers should be able to restart, update or delete these instances.
        Failed = 6,
        /// Index 7 is used in the producer apis for ROLLED_BACK state. Keeping that
        /// index unused in case that state also needs to exposed via consumer apis
        /// in future.
        /// The instance has been configured to sync data from some other source.
        Bootstrapping = 8,
        /// The instance is being promoted.
        Promoting = 9,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Ready => "READY",
                State::Stopped => "STOPPED",
                State::Creating => "CREATING",
                State::Deleting => "DELETING",
                State::Maintenance => "MAINTENANCE",
                State::Failed => "FAILED",
                State::Bootstrapping => "BOOTSTRAPPING",
                State::Promoting => "PROMOTING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "READY" => Some(Self::Ready),
                "STOPPED" => Some(Self::Stopped),
                "CREATING" => Some(Self::Creating),
                "DELETING" => Some(Self::Deleting),
                "MAINTENANCE" => Some(Self::Maintenance),
                "FAILED" => Some(Self::Failed),
                "BOOTSTRAPPING" => Some(Self::Bootstrapping),
                "PROMOTING" => Some(Self::Promoting),
                _ => None,
            }
        }
    }
    /// Type of an Instance
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
    pub enum InstanceType {
        /// The type of the instance is unknown.
        Unspecified = 0,
        /// PRIMARY instances support read and write operations.
        Primary = 1,
        /// READ POOL instances support read operations only. Each read pool instance
        /// consists of one or more homogeneous nodes.
        ///   * Read pool of size 1 can only have zonal availability.
        ///   * Read pools with node count of 2 or more can have regional
        ///     availability (nodes are present in 2 or more zones in a region).
        ReadPool = 2,
        /// SECONDARY instances support read operations only. SECONDARY instance
        /// is a cross-region read replica
        Secondary = 3,
    }
    impl InstanceType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                InstanceType::Unspecified => "INSTANCE_TYPE_UNSPECIFIED",
                InstanceType::Primary => "PRIMARY",
                InstanceType::ReadPool => "READ_POOL",
                InstanceType::Secondary => "SECONDARY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INSTANCE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "PRIMARY" => Some(Self::Primary),
                "READ_POOL" => Some(Self::ReadPool),
                "SECONDARY" => Some(Self::Secondary),
                _ => None,
            }
        }
    }
    /// The Availability type of an instance. Potential values:
    ///
    /// - ZONAL: The instance serves data from only one zone. Outages in that
    ///      zone affect instance availability.
    /// - REGIONAL: The instance can serve data from more than one zone in a
    ///      region (it is highly available).
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
    pub enum AvailabilityType {
        /// This is an unknown Availability type.
        Unspecified = 0,
        /// Zonal available instance.
        Zonal = 1,
        /// Regional (or Highly) available instance.
        Regional = 2,
    }
    impl AvailabilityType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AvailabilityType::Unspecified => "AVAILABILITY_TYPE_UNSPECIFIED",
                AvailabilityType::Zonal => "ZONAL",
                AvailabilityType::Regional => "REGIONAL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "AVAILABILITY_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "ZONAL" => Some(Self::Zonal),
                "REGIONAL" => Some(Self::Regional),
                _ => None,
            }
        }
    }
}
/// ConnectionInfo singleton resource.
/// <https://google.aip.dev/156>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionInfo {
    /// The name of the ConnectionInfo singleton resource, e.g.:
    /// projects/{project}/locations/{location}/clusters/*/instances/*/connectionInfo
    /// This field currently has no semantic meaning.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The private network IP address for the Instance. This is the
    /// default IP for the instance and is always created (even if enable_public_ip
    /// is set). This is the connection endpoint for an end-user application.
    #[prost(string, tag = "2")]
    pub ip_address: ::prost::alloc::string::String,
    /// Output only. The public IP addresses for the Instance. This is available
    /// ONLY when enable_public_ip is set. This is the connection endpoint for an
    /// end-user application.
    #[prost(string, tag = "5")]
    pub public_ip_address: ::prost::alloc::string::String,
    /// Output only. The pem-encoded chain that may be used to verify the X.509
    /// certificate. Expected to be in issuer-to-root order according to RFC 5246.
    #[deprecated]
    #[prost(string, repeated, tag = "3")]
    pub pem_certificate_chain: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. The unique ID of the Instance.
    #[prost(string, tag = "4")]
    pub instance_uid: ::prost::alloc::string::String,
    /// Output only. The DNS name to use with PSC for the Instance.
    #[prost(string, tag = "6")]
    pub psc_dns_name: ::prost::alloc::string::String,
}
/// Message describing Backup object
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Backup {
    /// Output only. The name of the backup resource with the format:
    ///   * projects/{project}/locations/{region}/backups/{backup_id}
    /// where the cluster and backup ID segments should satisfy the regex
    /// expression `[a-z](\[a-z0-9-\]{0,61}\[a-z0-9\])?`, e.g. 1-63 characters of
    /// lowercase letters, numbers, and dashes, starting with a letter, and ending
    /// with a letter or number. For more details see <https://google.aip.dev/122.>
    /// The prefix of the backup resource name is the name of the parent
    /// resource:
    ///   * projects/{project}/locations/{region}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// User-settable and human-readable display name for the Backup.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. The system-generated UID of the resource. The UID is assigned
    /// when the resource is created, and it is retained until it is deleted.
    #[prost(string, tag = "3")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. Create time stamp
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Update time stamp
    #[prost(message, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Delete time stamp
    #[prost(message, optional, tag = "15")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Labels as key value pairs
    #[prost(map = "string, string", tag = "6")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. The current state of the backup.
    #[prost(enumeration = "backup::State", tag = "7")]
    pub state: i32,
    /// The backup type, which suggests the trigger for the backup.
    #[prost(enumeration = "backup::Type", tag = "8")]
    pub r#type: i32,
    /// User-provided description of the backup.
    #[prost(string, tag = "9")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The system-generated UID of the cluster which was used to
    /// create this resource.
    #[prost(string, tag = "18")]
    pub cluster_uid: ::prost::alloc::string::String,
    /// Required. The full resource name of the backup source cluster
    /// (e.g., projects/{project}/locations/{region}/clusters/{cluster_id}).
    #[prost(string, tag = "10")]
    pub cluster_name: ::prost::alloc::string::String,
    /// Output only. Reconciling (<https://google.aip.dev/128#reconciliation>), if
    /// true, indicates that the service is actively updating the resource. This
    /// can happen due to user-triggered updates or system actions like failover or
    /// maintenance.
    #[prost(bool, tag = "11")]
    pub reconciling: bool,
    /// Optional. The encryption config can be specified to encrypt the
    /// backup with a customer-managed encryption key (CMEK). When this field is
    /// not specified, the backup will then use default encryption scheme to
    /// protect the user data.
    #[prost(message, optional, tag = "12")]
    pub encryption_config: ::core::option::Option<EncryptionConfig>,
    /// Output only. The encryption information for the backup.
    #[prost(message, optional, tag = "13")]
    pub encryption_info: ::core::option::Option<EncryptionInfo>,
    /// For Resource freshness validation (<https://google.aip.dev/154>)
    #[prost(string, tag = "14")]
    pub etag: ::prost::alloc::string::String,
    /// Annotations to allow client tools to store small amount of arbitrary data.
    /// This is distinct from labels.
    /// <https://google.aip.dev/128>
    #[prost(map = "string, string", tag = "16")]
    pub annotations: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. The size of the backup in bytes.
    #[prost(int64, tag = "17")]
    pub size_bytes: i64,
    /// Output only. The time at which after the backup is eligible to be garbage
    /// collected. It is the duration specified by the backup's retention policy,
    /// added to the backup's create_time.
    #[prost(message, optional, tag = "19")]
    pub expiry_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The QuantityBasedExpiry of the backup, specified by the
    /// backup's retention policy. Once the expiry quantity is over retention, the
    /// backup is eligible to be garbage collected.
    #[prost(message, optional, tag = "20")]
    pub expiry_quantity: ::core::option::Option<backup::QuantityBasedExpiry>,
    /// Output only. Reserved for future use.
    #[prost(bool, tag = "23")]
    pub satisfies_pzi: bool,
    /// Output only. Reserved for future use.
    #[prost(bool, tag = "21")]
    pub satisfies_pzs: bool,
    /// Output only. The database engine major version of the cluster this backup
    /// was created from. Any restored cluster created from this backup will have
    /// the same database version.
    #[prost(enumeration = "DatabaseVersion", tag = "22")]
    pub database_version: i32,
}
/// Nested message and enum types in `Backup`.
pub mod backup {
    /// A backup's position in a quantity-based retention queue, of backups with
    /// the same source cluster and type, with length, retention, specified by the
    /// backup's retention policy.
    /// Once the position is greater than the retention, the backup is eligible to
    /// be garbage collected.
    ///
    /// Example: 5 backups from the same source cluster and type with a
    /// quantity-based retention of 3 and denoted by backup_id (position,
    /// retention).
    ///
    /// Safe: backup_5 (1, 3), backup_4, (2, 3), backup_3 (3, 3).
    /// Awaiting garbage collection: backup_2 (4, 3), backup_1 (5, 3)
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QuantityBasedExpiry {
        /// Output only. The backup's position among its backups with the same source
        /// cluster and type, by descending chronological order create time(i.e.
        /// newest first).
        #[prost(int32, tag = "1")]
        pub retention_count: i32,
        /// Output only. The length of the quantity-based queue, specified by the
        /// backup's retention policy.
        #[prost(int32, tag = "2")]
        pub total_retention_count: i32,
    }
    /// Backup State
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
        /// The state of the backup is unknown.
        Unspecified = 0,
        /// The backup is ready.
        Ready = 1,
        /// The backup is creating.
        Creating = 2,
        /// The backup failed.
        Failed = 3,
        /// The backup is being deleted.
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
                State::Ready => "READY",
                State::Creating => "CREATING",
                State::Failed => "FAILED",
                State::Deleting => "DELETING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "READY" => Some(Self::Ready),
                "CREATING" => Some(Self::Creating),
                "FAILED" => Some(Self::Failed),
                "DELETING" => Some(Self::Deleting),
                _ => None,
            }
        }
    }
    /// Backup Type
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
        /// Backup Type is unknown.
        Unspecified = 0,
        /// ON_DEMAND backups that were triggered by the customer (e.g., not
        /// AUTOMATED).
        OnDemand = 1,
        /// AUTOMATED backups triggered by the automated backups scheduler pursuant
        /// to an automated backup policy.
        Automated = 2,
        /// CONTINUOUS backups triggered by the automated backups scheduler
        /// due to a continuous backup policy.
        Continuous = 3,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::OnDemand => "ON_DEMAND",
                Type::Automated => "AUTOMATED",
                Type::Continuous => "CONTINUOUS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "ON_DEMAND" => Some(Self::OnDemand),
                "AUTOMATED" => Some(Self::Automated),
                "CONTINUOUS" => Some(Self::Continuous),
                _ => None,
            }
        }
    }
}
/// SupportedDatabaseFlag gives general information about a database flag,
/// like type and allowed values. This is a static value that is defined
/// on the server side, and it cannot be modified by callers.
/// To set the Database flags on a particular Instance, a caller should modify
/// the Instance.database_flags field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SupportedDatabaseFlag {
    /// The name of the flag resource, following Google Cloud conventions, e.g.:
    ///   * projects/{project}/locations/{location}/flags/{flag}
    /// This field currently has no semantic meaning.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The name of the database flag, e.g. "max_allowed_packets".
    /// The is a possibly key for the Instance.database_flags map field.
    #[prost(string, tag = "2")]
    pub flag_name: ::prost::alloc::string::String,
    #[prost(enumeration = "supported_database_flag::ValueType", tag = "3")]
    pub value_type: i32,
    /// Whether the database flag accepts multiple values. If true,
    /// a comma-separated list of stringified values may be specified.
    #[prost(bool, tag = "4")]
    pub accepts_multiple_values: bool,
    /// Major database engine versions for which this flag is supported.
    #[prost(enumeration = "DatabaseVersion", repeated, tag = "5")]
    pub supported_db_versions: ::prost::alloc::vec::Vec<i32>,
    /// Whether setting or updating this flag on an Instance requires a database
    /// restart. If a flag that requires database restart is set, the backend
    /// will automatically restart the database (making sure to satisfy any
    /// availability SLO's).
    #[prost(bool, tag = "6")]
    pub requires_db_restart: bool,
    /// The restrictions on the flag value per type.
    #[prost(oneof = "supported_database_flag::Restrictions", tags = "7, 8")]
    pub restrictions: ::core::option::Option<supported_database_flag::Restrictions>,
}
/// Nested message and enum types in `SupportedDatabaseFlag`.
pub mod supported_database_flag {
    /// Restrictions on STRING type values
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StringRestrictions {
        /// The list of allowed values, if bounded. This field will be empty
        /// if there is a unbounded number of allowed values.
        #[prost(string, repeated, tag = "1")]
        pub allowed_values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// Restrictions on INTEGER type values.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IntegerRestrictions {
        /// The minimum value that can be specified, if applicable.
        #[prost(message, optional, tag = "1")]
        pub min_value: ::core::option::Option<i64>,
        /// The maximum value that can be specified, if applicable.
        #[prost(message, optional, tag = "2")]
        pub max_value: ::core::option::Option<i64>,
    }
    /// ValueType describes the semantic type of the value that the flag accepts.
    /// Regardless of the ValueType, the Instance.database_flags field accepts the
    /// stringified version of the value, i.e. "20" or "3.14".
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
    pub enum ValueType {
        /// This is an unknown flag type.
        Unspecified = 0,
        /// String type flag.
        String = 1,
        /// Integer type flag.
        Integer = 2,
        /// Float type flag.
        Float = 3,
        /// Denotes that the flag does not accept any values.
        None = 4,
    }
    impl ValueType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ValueType::Unspecified => "VALUE_TYPE_UNSPECIFIED",
                ValueType::String => "STRING",
                ValueType::Integer => "INTEGER",
                ValueType::Float => "FLOAT",
                ValueType::None => "NONE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "VALUE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "STRING" => Some(Self::String),
                "INTEGER" => Some(Self::Integer),
                "FLOAT" => Some(Self::Float),
                "NONE" => Some(Self::None),
                _ => None,
            }
        }
    }
    /// The restrictions on the flag value per type.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Restrictions {
        /// Restriction on STRING type value.
        #[prost(message, tag = "7")]
        StringRestrictions(StringRestrictions),
        /// Restriction on INTEGER type value.
        #[prost(message, tag = "8")]
        IntegerRestrictions(IntegerRestrictions),
    }
}
/// Message describing User object.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    /// Output only. Name of the resource in the form of
    /// projects/{project}/locations/{location}/cluster/{cluster}/users/{user}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Input only. Password for the user.
    #[prost(string, tag = "2")]
    pub password: ::prost::alloc::string::String,
    /// Optional. List of database roles this user has.
    /// The database role strings are subject to the PostgreSQL naming conventions.
    #[prost(string, repeated, tag = "4")]
    pub database_roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. Type of this user.
    #[prost(enumeration = "user::UserType", tag = "5")]
    pub user_type: i32,
}
/// Nested message and enum types in `User`.
pub mod user {
    /// Enum that details the user type.
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
    pub enum UserType {
        /// Unspecified user type.
        Unspecified = 0,
        /// The default user type that authenticates via password-based
        /// authentication.
        AlloydbBuiltIn = 1,
        /// Database user that can authenticate via IAM-Based authentication.
        AlloydbIamUser = 2,
    }
    impl UserType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                UserType::Unspecified => "USER_TYPE_UNSPECIFIED",
                UserType::AlloydbBuiltIn => "ALLOYDB_BUILT_IN",
                UserType::AlloydbIamUser => "ALLOYDB_IAM_USER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "USER_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "ALLOYDB_BUILT_IN" => Some(Self::AlloydbBuiltIn),
                "ALLOYDB_IAM_USER" => Some(Self::AlloydbIamUser),
                _ => None,
            }
        }
    }
}
/// Message describing Database object.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Database {
    /// Identifier. Name of the resource in the form of
    /// projects/{project}/locations/{location}/clusters/{cluster}/databases/{database}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Charset for the database.
    /// This field can contain any PostgreSQL supported charset name.
    /// Example values include "UTF8", "SQL_ASCII", etc.
    #[prost(string, tag = "2")]
    pub charset: ::prost::alloc::string::String,
    /// Optional. Collation for the database.
    /// Name of the custom or native collation for postgres.
    /// Example values include "C", "POSIX", etc
    #[prost(string, tag = "3")]
    pub collation: ::prost::alloc::string::String,
}
/// View on Instance. Pass this enum to rpcs that returns an Instance message to
/// control which subsets of fields to get.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InstanceView {
    /// INSTANCE_VIEW_UNSPECIFIED Not specified, equivalent to BASIC.
    Unspecified = 0,
    /// BASIC server responses for a primary or read instance include all the
    /// relevant instance details, excluding the details of each node in the
    /// instance. The default value.
    Basic = 1,
    /// FULL response is equivalent to BASIC for primary instance (for now).
    /// For read pool instance, this includes details of each node in the pool.
    Full = 2,
}
impl InstanceView {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            InstanceView::Unspecified => "INSTANCE_VIEW_UNSPECIFIED",
            InstanceView::Basic => "INSTANCE_VIEW_BASIC",
            InstanceView::Full => "INSTANCE_VIEW_FULL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INSTANCE_VIEW_UNSPECIFIED" => Some(Self::Unspecified),
            "INSTANCE_VIEW_BASIC" => Some(Self::Basic),
            "INSTANCE_VIEW_FULL" => Some(Self::Full),
            _ => None,
        }
    }
}
/// View on Cluster. Pass this enum to rpcs that returns a cluster message to
/// control which subsets of fields to get.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ClusterView {
    /// CLUSTER_VIEW_UNSPECIFIED Not specified, equivalent to BASIC.
    Unspecified = 0,
    /// BASIC server responses include all the relevant cluster details, excluding
    /// Cluster.ContinuousBackupInfo.EarliestRestorableTime and other view-specific
    /// fields. The default value.
    Basic = 1,
    /// CONTINUOUS_BACKUP response returns all the fields from BASIC plus
    /// the earliest restorable time if continuous backups are enabled.
    /// May increase latency.
    ContinuousBackup = 2,
}
impl ClusterView {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ClusterView::Unspecified => "CLUSTER_VIEW_UNSPECIFIED",
            ClusterView::Basic => "CLUSTER_VIEW_BASIC",
            ClusterView::ContinuousBackup => "CLUSTER_VIEW_CONTINUOUS_BACKUP",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CLUSTER_VIEW_UNSPECIFIED" => Some(Self::Unspecified),
            "CLUSTER_VIEW_BASIC" => Some(Self::Basic),
            "CLUSTER_VIEW_CONTINUOUS_BACKUP" => Some(Self::ContinuousBackup),
            _ => None,
        }
    }
}
/// The supported database engine versions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DatabaseVersion {
    /// This is an unknown database version.
    Unspecified = 0,
    /// DEPRECATED - The database version is Postgres 13.
    Postgres13 = 1,
    /// The database version is Postgres 14.
    Postgres14 = 2,
    /// The database version is Postgres 15.
    Postgres15 = 3,
}
impl DatabaseVersion {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DatabaseVersion::Unspecified => "DATABASE_VERSION_UNSPECIFIED",
            DatabaseVersion::Postgres13 => "POSTGRES_13",
            DatabaseVersion::Postgres14 => "POSTGRES_14",
            DatabaseVersion::Postgres15 => "POSTGRES_15",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DATABASE_VERSION_UNSPECIFIED" => Some(Self::Unspecified),
            "POSTGRES_13" => Some(Self::Postgres13),
            "POSTGRES_14" => Some(Self::Postgres14),
            "POSTGRES_15" => Some(Self::Postgres15),
            _ => None,
        }
    }
}
/// Message for requesting list of Clusters
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListClustersRequest {
    /// Required. The name of the parent resource. For the required format, see the
    /// comment on the Cluster.name field. Additionally, you can perform an
    /// aggregated list operation by specifying a value with the following format:
    ///   * projects/{project}/locations/-
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server may return fewer items than
    /// requested. If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filtering results
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Hint for how to order the results
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Message for response to listing Clusters
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListClustersResponse {
    /// The list of Cluster
    #[prost(message, repeated, tag = "1")]
    pub clusters: ::prost::alloc::vec::Vec<Cluster>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for getting a Cluster
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClusterRequest {
    /// Required. The name of the resource. For the required format, see the
    /// comment on the Cluster.name field.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The view of the cluster to return. Returns all default fields if
    /// not set.
    #[prost(enumeration = "ClusterView", tag = "2")]
    pub view: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSecondaryClusterRequest {
    /// Required. The location of the new cluster. For the required
    /// format, see the comment on the Cluster.name field.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. ID of the requesting object (the secondary cluster).
    #[prost(string, tag = "2")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Required. Configuration of the requesting object (the secondary cluster).
    #[prost(message, optional, tag = "3")]
    pub cluster: ::core::option::Option<Cluster>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "5")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set, performs request validation (e.g. permission checks and
    /// any other type of validation), but do not actually execute the create
    /// request.
    #[prost(bool, tag = "6")]
    pub validate_only: bool,
}
/// Message for creating a Cluster
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateClusterRequest {
    /// Required. The location of the new cluster. For the required format, see the
    /// comment on the Cluster.name field.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. ID of the requesting object.
    #[prost(string, tag = "2")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Required. The resource being created
    #[prost(message, optional, tag = "3")]
    pub cluster: ::core::option::Option<Cluster>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set, performs request validation (e.g. permission checks and
    /// any other type of validation), but do not actually execute the create
    /// request.
    #[prost(bool, tag = "5")]
    pub validate_only: bool,
}
/// Message for updating a Cluster
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateClusterRequest {
    /// Optional. Field mask is used to specify the fields to be overwritten in the
    /// Cluster resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The resource being updated
    #[prost(message, optional, tag = "2")]
    pub cluster: ::core::option::Option<Cluster>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set, performs request validation (e.g. permission checks and
    /// any other type of validation), but do not actually execute the update
    /// request.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
    /// Optional. If set to true, update succeeds even if cluster is not found. In
    /// that case, a new cluster is created and `update_mask` is ignored.
    #[prost(bool, tag = "5")]
    pub allow_missing: bool,
}
/// Message for deleting a Cluster
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteClusterRequest {
    /// Required. The name of the resource. For the required format, see the
    /// comment on the Cluster.name field.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. The current etag of the Cluster.
    /// If an etag is provided and does not match the current etag of the Cluster,
    /// deletion will be blocked and an ABORTED error will be returned.
    #[prost(string, tag = "3")]
    pub etag: ::prost::alloc::string::String,
    /// Optional. If set, performs request validation (e.g. permission checks and
    /// any other type of validation), but do not actually execute the delete.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
    /// Optional. Whether to cascade delete child instances for given cluster.
    #[prost(bool, tag = "5")]
    pub force: bool,
}
/// Message for promoting a Cluster
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PromoteClusterRequest {
    /// Required. The name of the resource. For the required format, see the
    /// comment on the Cluster.name field
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. The current etag of the Cluster.
    /// If an etag is provided and does not match the current etag of the Cluster,
    /// deletion will be blocked and an ABORTED error will be returned.
    #[prost(string, tag = "3")]
    pub etag: ::prost::alloc::string::String,
    /// Optional. If set, performs request validation (e.g. permission checks and
    /// any other type of validation), but do not actually execute the delete.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// Message for restoring a Cluster from a backup or another cluster at a given
/// point in time.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreClusterRequest {
    /// Required. The name of the parent resource. For the required format, see the
    /// comment on the Cluster.name field.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. ID of the requesting object.
    #[prost(string, tag = "2")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Required. The resource being created
    #[prost(message, optional, tag = "3")]
    pub cluster: ::core::option::Option<Cluster>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "5")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set, performs request validation (e.g. permission checks and
    /// any other type of validation), but do not actually execute the import
    /// request.
    #[prost(bool, tag = "6")]
    pub validate_only: bool,
    /// Required.
    /// The source to import from.
    #[prost(oneof = "restore_cluster_request::Source", tags = "4, 8")]
    pub source: ::core::option::Option<restore_cluster_request::Source>,
}
/// Nested message and enum types in `RestoreClusterRequest`.
pub mod restore_cluster_request {
    /// Required.
    /// The source to import from.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// Backup source.
        #[prost(message, tag = "4")]
        BackupSource(super::BackupSource),
        /// ContinuousBackup source. Continuous backup needs to be enabled in the
        /// source cluster for this operation to succeed.
        #[prost(message, tag = "8")]
        ContinuousBackupSource(super::ContinuousBackupSource),
    }
}
/// Message for requesting list of Instances
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesRequest {
    /// Required. The name of the parent resource. For the required format, see the
    /// comment on the Instance.name field. Additionally, you can perform an
    /// aggregated list operation by specifying a value with one of the following
    /// formats:
    ///   * projects/{project}/locations/-/clusters/-
    ///   * projects/{project}/locations/{region}/clusters/-
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server may return fewer items than
    /// requested. If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filtering results
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Hint for how to order the results
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Message for response to listing Instances
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesResponse {
    /// The list of Instance
    #[prost(message, repeated, tag = "1")]
    pub instances: ::prost::alloc::vec::Vec<Instance>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for getting a Instance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstanceRequest {
    /// Required. The name of the resource. For the required format, see the
    /// comment on the Instance.name field.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The view of the instance to return.
    #[prost(enumeration = "InstanceView", tag = "2")]
    pub view: i32,
}
/// Message for creating a Instance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInstanceRequest {
    /// Required. The name of the parent resource. For the required format, see the
    /// comment on the Instance.name field.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. ID of the requesting object.
    #[prost(string, tag = "2")]
    pub instance_id: ::prost::alloc::string::String,
    /// Required. The resource being created
    #[prost(message, optional, tag = "3")]
    pub instance: ::core::option::Option<Instance>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set, performs request validation (e.g. permission checks and
    /// any other type of validation), but do not actually execute the create
    /// request.
    #[prost(bool, tag = "5")]
    pub validate_only: bool,
}
/// Message for creating a Secondary Instance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSecondaryInstanceRequest {
    /// Required. The name of the parent resource. For the required format, see the
    /// comment on the Instance.name field.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. ID of the requesting object.
    #[prost(string, tag = "2")]
    pub instance_id: ::prost::alloc::string::String,
    /// Required. The resource being created
    #[prost(message, optional, tag = "3")]
    pub instance: ::core::option::Option<Instance>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set, performs request validation (e.g. permission checks and
    /// any other type of validation), but do not actually execute the create
    /// request.
    #[prost(bool, tag = "5")]
    pub validate_only: bool,
}
/// See usage below for notes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInstanceRequests {
    /// Required. Primary and read replica instances to be created. This list
    /// should not be empty.
    #[prost(message, repeated, tag = "1")]
    pub create_instance_requests: ::prost::alloc::vec::Vec<CreateInstanceRequest>,
}
/// Message for creating a batch of instances under the specified cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateInstancesRequest {
    /// Required. The name of the parent resource.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Resources being created.
    #[prost(message, optional, tag = "2")]
    pub requests: ::core::option::Option<CreateInstanceRequests>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for creating batches of instances in a cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateInstancesResponse {
    /// Created instances.
    #[prost(message, repeated, tag = "1")]
    pub instances: ::prost::alloc::vec::Vec<Instance>,
}
/// Message for metadata that is specific to BatchCreateInstances API.
/// NEXT_ID: 3
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateInstancesMetadata {
    /// The instances being created in the API call. Each string in this list
    /// is the server defined resource path for target instances in the request
    /// and for the format of each string, see the comment on the Instance.name
    /// field.
    #[prost(string, repeated, tag = "1")]
    pub instance_targets: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A map representing state of the instances involved in the
    /// BatchCreateInstances operation during the operation execution.
    /// The instance state will be in STATE_UNSPECIFIED state if the instance has
    /// not yet been picked up for processing.
    /// The key of the map is the name of the instance resource.
    /// For the format, see the comment on the Instance.name field.
    #[prost(map = "string, message", tag = "2")]
    pub instance_statuses: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        BatchCreateInstanceStatus,
    >,
}
/// Message for current status of an instance in the BatchCreateInstances
/// operation.
/// For example, lets say a BatchCreateInstances workflow has 4 instances,
/// Instance1 through Instance4. Lets also assume that 2 instances succeeded
/// but the third failed to create and the 4th was never picked up for creation
/// because of failure of the previous one. Then, resulting states would look
/// something like:
///    1. Instance1 = ROLLED_BACK
///    2. Instance2 = ROLLED_BACK
///    3. Instance3 = FAILED
///    4. Instance4 = FAILED
///
/// However, while the operation is running, the instance might be in other
/// states including PENDING_CREATE, ACTIVE, DELETING and CREATING. The states
/// / do not get further updated once the operation is done.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateInstanceStatus {
    /// The current state of an instance involved in the batch create operation.
    /// Once the operation is complete, the final state of the instances in the
    /// LRO can be one of:
    ///    1. ACTIVE, indicating that instances were created successfully
    ///    2. FAILED, indicating that a particular instance failed creation
    ///    3. ROLLED_BACK indicating that although the instance was created
    ///       successfully, it had to be rolled back and deleted due to failure in
    ///       other steps of the workflow.
    #[prost(enumeration = "batch_create_instance_status::State", tag = "1")]
    pub state: i32,
    /// DEPRECATED - Use the error field instead.
    /// Error, if any error occurred and is available, during instance creation.
    #[prost(string, tag = "2")]
    pub error_msg: ::prost::alloc::string::String,
    /// The RPC status of the instance creation operation. This field will be
    /// present if an error happened during the instance creation.
    #[prost(message, optional, tag = "4")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
    #[prost(enumeration = "instance::InstanceType", tag = "3")]
    pub r#type: i32,
}
/// Nested message and enum types in `BatchCreateInstanceStatus`.
pub mod batch_create_instance_status {
    /// State contains all valid instance states for the BatchCreateInstances
    /// operation. This is mainly used for status reporting through the LRO
    /// metadata.
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
        /// The state of the instance is unknown.
        Unspecified = 0,
        /// Instance is pending creation and has not yet been picked up for
        /// processsing in the backend.
        PendingCreate = 1,
        /// The instance is active and running.
        Ready = 2,
        /// The instance is being created.
        Creating = 3,
        /// The instance is being deleted.
        Deleting = 4,
        /// The creation of the instance failed or a fatal error occurred during
        /// an operation on the instance or a batch of instances.
        Failed = 5,
        /// The instance was created successfully, but was rolled back and deleted
        /// due to some other failure during BatchCreateInstances operation.
        RolledBack = 6,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::PendingCreate => "PENDING_CREATE",
                State::Ready => "READY",
                State::Creating => "CREATING",
                State::Deleting => "DELETING",
                State::Failed => "FAILED",
                State::RolledBack => "ROLLED_BACK",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "PENDING_CREATE" => Some(Self::PendingCreate),
                "READY" => Some(Self::Ready),
                "CREATING" => Some(Self::Creating),
                "DELETING" => Some(Self::Deleting),
                "FAILED" => Some(Self::Failed),
                "ROLLED_BACK" => Some(Self::RolledBack),
                _ => None,
            }
        }
    }
}
/// Message for updating a Instance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInstanceRequest {
    /// Optional. Field mask is used to specify the fields to be overwritten in the
    /// Instance resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The resource being updated
    #[prost(message, optional, tag = "2")]
    pub instance: ::core::option::Option<Instance>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set, performs request validation (e.g. permission checks and
    /// any other type of validation), but do not actually execute the update
    /// request.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
    /// Optional. If set to true, update succeeds even if instance is not found. In
    /// that case, a new instance is created and `update_mask` is ignored.
    #[prost(bool, tag = "5")]
    pub allow_missing: bool,
}
/// Message for deleting a Instance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteInstanceRequest {
    /// Required. The name of the resource. For the required format, see the
    /// comment on the Instance.name field.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. The current etag of the Instance.
    /// If an etag is provided and does not match the current etag of the Instance,
    /// deletion will be blocked and an ABORTED error will be returned.
    #[prost(string, tag = "3")]
    pub etag: ::prost::alloc::string::String,
    /// Optional. If set, performs request validation (e.g. permission checks and
    /// any other type of validation), but do not actually execute the delete.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// Message for triggering failover on an Instance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FailoverInstanceRequest {
    /// Required. The name of the resource. For the required format, see the
    /// comment on the Instance.name field.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set, performs request validation (e.g. permission checks and
    /// any other type of validation), but do not actually execute the failover.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
}
/// Message for triggering fault injection on an instance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InjectFaultRequest {
    /// Required. The type of fault to be injected in an instance.
    #[prost(enumeration = "inject_fault_request::FaultType", tag = "1")]
    pub fault_type: i32,
    /// Required. The name of the resource. For the required format, see the
    /// comment on the Instance.name field.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set, performs request validation (e.g. permission checks and
    /// any other type of validation), but do not actually execute the fault
    /// injection.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// Nested message and enum types in `InjectFaultRequest`.
pub mod inject_fault_request {
    /// FaultType contains all valid types of faults that can be injected to an
    /// instance.
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
    pub enum FaultType {
        /// The fault type is unknown.
        Unspecified = 0,
        /// Stop the VM
        StopVm = 1,
    }
    impl FaultType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FaultType::Unspecified => "FAULT_TYPE_UNSPECIFIED",
                FaultType::StopVm => "STOP_VM",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FAULT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "STOP_VM" => Some(Self::StopVm),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestartInstanceRequest {
    /// Required. The name of the resource. For the required format, see the
    /// comment on the Instance.name field.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set, performs request validation (e.g. permission checks and
    /// any other type of validation), but do not actually execute the restart.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
}
/// Message for requesting list of Backups
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBackupsRequest {
    /// Required. Parent value for ListBackupsRequest
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filtering results
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Hint for how to order the results
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Message for response to listing Backups
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBackupsResponse {
    /// The list of Backup
    #[prost(message, repeated, tag = "1")]
    pub backups: ::prost::alloc::vec::Vec<Backup>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for getting a Backup
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBackupRequest {
    /// Required. Name of the resource
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for creating a Backup
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBackupRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. ID of the requesting object.
    #[prost(string, tag = "2")]
    pub backup_id: ::prost::alloc::string::String,
    /// Required. The resource being created
    #[prost(message, optional, tag = "3")]
    pub backup: ::core::option::Option<Backup>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set, the backend validates the request, but doesn't actually
    /// execute it.
    #[prost(bool, tag = "5")]
    pub validate_only: bool,
}
/// Message for updating a Backup
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBackupRequest {
    /// Optional. Field mask is used to specify the fields to be overwritten in the
    /// Backup resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The resource being updated
    #[prost(message, optional, tag = "2")]
    pub backup: ::core::option::Option<Backup>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set, the backend validates the request, but doesn't actually
    /// execute it.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
    /// Optional. If set to true, update succeeds even if instance is not found. In
    /// that case, a new backup is created and `update_mask` is ignored.
    #[prost(bool, tag = "5")]
    pub allow_missing: bool,
}
/// Message for deleting a Backup
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteBackupRequest {
    /// Required. Name of the resource. For the required format, see the comment on
    /// the Backup.name field.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set, the backend validates the request, but doesn't actually
    /// execute it.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
    /// Optional. The current etag of the Backup.
    /// If an etag is provided and does not match the current etag of the Backup,
    /// deletion will be blocked and an ABORTED error will be returned.
    #[prost(string, tag = "4")]
    pub etag: ::prost::alloc::string::String,
}
/// Message for listing the information about the supported Database flags.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSupportedDatabaseFlagsRequest {
    /// Required. The name of the parent resource. The required format is:
    ///   * projects/{project}/locations/{location}
    ///
    /// Regardless of the parent specified here, as long it is contains a valid
    /// project and location, the service will return a static list of supported
    /// flags resources. Note that we do not yet support region-specific
    /// flags.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Message for response to listing SupportedDatabaseFlags.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSupportedDatabaseFlagsResponse {
    /// The list of SupportedDatabaseFlags.
    #[prost(message, repeated, tag = "1")]
    pub supported_database_flags: ::prost::alloc::vec::Vec<SupportedDatabaseFlag>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Message for requests to generate a client certificate signed by the Cluster
/// CA.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateClientCertificateRequest {
    /// Required. The name of the parent resource. The required format is:
    ///   * projects/{project}/locations/{location}/clusters/{cluster}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. A pem-encoded X.509 certificate signing request (CSR). It is
    /// recommended to use public_key instead.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub pem_csr: ::prost::alloc::string::String,
    /// Optional. An optional hint to the endpoint to generate the client
    /// certificate with the requested duration. The duration can be from 1 hour to
    /// 24 hours. The endpoint may or may not honor the hint. If the hint is left
    /// unspecified or is not honored, then the endpoint will pick an appropriate
    /// default duration.
    #[prost(message, optional, tag = "4")]
    pub cert_duration: ::core::option::Option<::prost_types::Duration>,
    /// Optional. The public key from the client.
    #[prost(string, tag = "5")]
    pub public_key: ::prost::alloc::string::String,
    /// Optional. An optional hint to the endpoint to generate a client
    /// ceritificate that can be used by AlloyDB connectors to exchange additional
    /// metadata with the server after TLS handshake.
    #[prost(bool, tag = "6")]
    pub use_metadata_exchange: bool,
}
/// Message returned by a GenerateClientCertificate operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateClientCertificateResponse {
    /// Output only. The pem-encoded, signed X.509 certificate.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub pem_certificate: ::prost::alloc::string::String,
    /// Output only. The pem-encoded chain that may be used to verify the X.509
    /// certificate. Expected to be in issuer-to-root order according to RFC 5246.
    #[prost(string, repeated, tag = "2")]
    pub pem_certificate_chain: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. The pem-encoded cluster ca X.509 certificate.
    #[prost(string, tag = "3")]
    pub ca_cert: ::prost::alloc::string::String,
}
/// Request message for GetConnectionInfo.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConnectionInfoRequest {
    /// Required. The name of the parent resource. The required format is:
    /// projects/{project}/locations/{location}/clusters/{cluster}/instances/{instance}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
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
    /// of the operation. Operations that have successfully been cancelled
    /// have [Operation.error][] value with a
    /// [google.rpc.Status.code][google.rpc.Status.code] of 1, corresponding to
    /// `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
    /// Request specific metadata, if any.
    #[prost(oneof = "operation_metadata::RequestSpecific", tags = "8")]
    pub request_specific: ::core::option::Option<operation_metadata::RequestSpecific>,
}
/// Nested message and enum types in `OperationMetadata`.
pub mod operation_metadata {
    /// Request specific metadata, if any.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RequestSpecific {
        /// Output only. BatchCreateInstances related metadata.
        #[prost(message, tag = "8")]
        BatchCreateInstancesMetadata(super::BatchCreateInstancesMetadata),
    }
}
/// Message for requesting list of Users
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUsersRequest {
    /// Required. Parent value for ListUsersRequest
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
/// Message for response to listing Users
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUsersResponse {
    /// The list of User
    #[prost(message, repeated, tag = "1")]
    pub users: ::prost::alloc::vec::Vec<User>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for getting a User
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserRequest {
    /// Required. The name of the resource. For the required format, see the
    /// comment on the User.name field.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for creating a User
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUserRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. ID of the requesting object.
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
    /// Required. The resource being created
    #[prost(message, optional, tag = "3")]
    pub user: ::core::option::Option<User>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set, the backend validates the request, but doesn't actually
    /// execute it.
    #[prost(bool, tag = "5")]
    pub validate_only: bool,
}
/// Message for updating a User
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserRequest {
    /// Optional. Field mask is used to specify the fields to be overwritten in the
    /// User resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The resource being updated
    #[prost(message, optional, tag = "2")]
    pub user: ::core::option::Option<User>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set, the backend validates the request, but doesn't actually
    /// execute it.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
    /// Optional. Allow missing fields in the update mask.
    #[prost(bool, tag = "5")]
    pub allow_missing: bool,
}
/// Message for deleting a User
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteUserRequest {
    /// Required. The name of the resource. For the required format, see the
    /// comment on the User.name field.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set, the backend validates the request, but doesn't actually
    /// execute it.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
}
/// Message for requesting list of Databases.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDatabasesRequest {
    /// Required. Parent value for ListDatabasesRequest.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of databases to return. The service may return
    /// fewer than this value. If unspecified, an appropriate number of databases
    /// will be returned. The max value will be 2000, values above max will be
    /// coerced to max.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListDatabases` call.
    /// This should be provided to retrieve the subsequent page.
    /// This field is currently not supported, its value will be ignored if passed.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filtering results.
    /// This field is currently not supported, its value will be ignored if passed.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// Message for response to listing Databases.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDatabasesResponse {
    /// The list of databases
    #[prost(message, repeated, tag = "1")]
    pub databases: ::prost::alloc::vec::Vec<Database>,
    /// A token identifying the next page of results the server should return.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod alloy_db_admin_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service describing handlers for resources
    #[derive(Debug, Clone)]
    pub struct AlloyDbAdminClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AlloyDbAdminClient<tonic::transport::Channel> {
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
    impl<T> AlloyDbAdminClient<T>
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
        ) -> AlloyDbAdminClient<InterceptedService<T, F>>
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
            AlloyDbAdminClient::new(InterceptedService::new(inner, interceptor))
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
                "/google.cloud.alloydb.v1alpha.AlloyDBAdmin/ListClusters",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.alloydb.v1alpha.AlloyDBAdmin",
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
                "/google.cloud.alloydb.v1alpha.AlloyDBAdmin/GetCluster",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.alloydb.v1alpha.AlloyDBAdmin",
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
                "/google.cloud.alloydb.v1alpha.AlloyDBAdmin/CreateCluster",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.alloydb.v1alpha.AlloyDBAdmin",
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
                "/google.cloud.alloydb.v1alpha.AlloyDBAdmin/UpdateCluster",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.alloydb.v1alpha.AlloyDBAdmin",
                        "UpdateCluster",
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
                "/google.cloud.alloydb.v1alpha.AlloyDBAdmin/DeleteCluster",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.alloydb.v1alpha.AlloyDBAdmin",
                        "DeleteCluster",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Promotes a SECONDARY cluster. This turns down replication
        /// from the PRIMARY cluster and promotes a secondary cluster
        /// into its own standalone cluster.
        /// Imperative only.
        pub async fn promote_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::PromoteClusterRequest>,
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
                "/google.cloud.alloydb.v1alpha.AlloyDBAdmin/PromoteCluster",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.alloydb.v1alpha.AlloyDBAdmin",
                        "PromoteCluster",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new Cluster in a given project and location, with a volume
        /// restored from the provided source, either a backup ID or a point-in-time
        /// and a source cluster.
        pub async fn restore_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::RestoreClusterRequest>,
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
                "/google.cloud.alloydb.v1alpha.AlloyDBAdmin/RestoreCluster",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.alloydb.v1alpha.AlloyDBAdmin",
                        "RestoreCluster",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a cluster of type SECONDARY in the given location using
        /// the primary cluster as the source.
        pub async fn create_secondary_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSecondaryClusterRequest>,
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
                "/google.cloud.alloydb.v1alpha.AlloyDBAdmin/CreateSecondaryCluster",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.alloydb.v1alpha.AlloyDBAdmin",
                        "CreateSecondaryCluster",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists Instances in a given project and location.
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
                "/google.cloud.alloydb.v1alpha.AlloyDBAdmin/ListInstances",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.alloydb.v1alpha.AlloyDBAdmin",
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
                "/google.cloud.alloydb.v1alpha.AlloyDBAdmin/GetInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.alloydb.v1alpha.AlloyDBAdmin",
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
                "/google.cloud.alloydb.v1alpha.AlloyDBAdmin/CreateInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.alloydb.v1alpha.AlloyDBAdmin",
                        "CreateInstance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new SECONDARY Instance in a given project and location.
        pub async fn create_secondary_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSecondaryInstanceRequest>,
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
                "/google.cloud.alloydb.v1alpha.AlloyDBAdmin/CreateSecondaryInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.alloydb.v1alpha.AlloyDBAdmin",
                        "CreateSecondaryInstance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates new instances under the given project, location and cluster.
        /// There can be only one primary instance in a cluster. If the primary
        /// instance exists in the cluster as well as this request, then API will
        /// throw an error.
        /// The primary instance should exist before any read pool instance is
        /// created. If the primary instance is a part of the request payload, then
        /// the API will take care of creating instances in the correct order.
        /// This method is here to support Google-internal use cases, and is not meant
        /// for external customers to consume. Please do not start relying on it; its
        /// behavior is subject to change without notice.
        pub async fn batch_create_instances(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchCreateInstancesRequest>,
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
                "/google.cloud.alloydb.v1alpha.AlloyDBAdmin/BatchCreateInstances",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.alloydb.v1alpha.AlloyDBAdmin",
                        "BatchCreateInstances",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the parameters of a single Instance.
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
                "/google.cloud.alloydb.v1alpha.AlloyDBAdmin/UpdateInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.alloydb.v1alpha.AlloyDBAdmin",
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
                "/google.cloud.alloydb.v1alpha.AlloyDBAdmin/DeleteInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.alloydb.v1alpha.AlloyDBAdmin",
                        "DeleteInstance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Forces a Failover for a highly available instance.
        /// Failover promotes the HA standby instance as the new primary.
        /// Imperative only.
        pub async fn failover_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::FailoverInstanceRequest>,
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
                "/google.cloud.alloydb.v1alpha.AlloyDBAdmin/FailoverInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.alloydb.v1alpha.AlloyDBAdmin",
                        "FailoverInstance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Injects fault in an instance.
        /// Imperative only.
        pub async fn inject_fault(
            &mut self,
            request: impl tonic::IntoRequest<super::InjectFaultRequest>,
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
                "/google.cloud.alloydb.v1alpha.AlloyDBAdmin/InjectFault",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.alloydb.v1alpha.AlloyDBAdmin",
                        "InjectFault",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Restart an Instance in a cluster.
        /// Imperative only.
        pub async fn restart_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::RestartInstanceRequest>,
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
                "/google.cloud.alloydb.v1alpha.AlloyDBAdmin/RestartInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.alloydb.v1alpha.AlloyDBAdmin",
                        "RestartInstance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists Backups in a given project and location.
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
                "/google.cloud.alloydb.v1alpha.AlloyDBAdmin/ListBackups",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.alloydb.v1alpha.AlloyDBAdmin",
                        "ListBackups",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single Backup.
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
                "/google.cloud.alloydb.v1alpha.AlloyDBAdmin/GetBackup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.alloydb.v1alpha.AlloyDBAdmin",
                        "GetBackup",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new Backup in a given project and location.
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
                "/google.cloud.alloydb.v1alpha.AlloyDBAdmin/CreateBackup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.alloydb.v1alpha.AlloyDBAdmin",
                        "CreateBackup",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the parameters of a single Backup.
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
                "/google.cloud.alloydb.v1alpha.AlloyDBAdmin/UpdateBackup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.alloydb.v1alpha.AlloyDBAdmin",
                        "UpdateBackup",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single Backup.
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
                "/google.cloud.alloydb.v1alpha.AlloyDBAdmin/DeleteBackup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.alloydb.v1alpha.AlloyDBAdmin",
                        "DeleteBackup",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists SupportedDatabaseFlags for a given project and location.
        pub async fn list_supported_database_flags(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSupportedDatabaseFlagsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListSupportedDatabaseFlagsResponse>,
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
                "/google.cloud.alloydb.v1alpha.AlloyDBAdmin/ListSupportedDatabaseFlags",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.alloydb.v1alpha.AlloyDBAdmin",
                        "ListSupportedDatabaseFlags",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Generate a client certificate signed by a Cluster CA.
        /// The sole purpose of this endpoint is to support AlloyDB connectors and the
        /// Auth Proxy client. The endpoint's behavior is subject to change without
        /// notice, so do not rely on its behavior remaining constant. Future changes
        /// will not break AlloyDB connectors or the Auth Proxy client.
        pub async fn generate_client_certificate(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateClientCertificateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateClientCertificateResponse>,
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
                "/google.cloud.alloydb.v1alpha.AlloyDBAdmin/GenerateClientCertificate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.alloydb.v1alpha.AlloyDBAdmin",
                        "GenerateClientCertificate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get instance metadata used for a connection.
        pub async fn get_connection_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConnectionInfoRequest>,
        ) -> std::result::Result<tonic::Response<super::ConnectionInfo>, tonic::Status> {
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
                "/google.cloud.alloydb.v1alpha.AlloyDBAdmin/GetConnectionInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.alloydb.v1alpha.AlloyDBAdmin",
                        "GetConnectionInfo",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists Users in a given project and location.
        pub async fn list_users(
            &mut self,
            request: impl tonic::IntoRequest<super::ListUsersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListUsersResponse>,
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
                "/google.cloud.alloydb.v1alpha.AlloyDBAdmin/ListUsers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.alloydb.v1alpha.AlloyDBAdmin",
                        "ListUsers",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single User.
        pub async fn get_user(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserRequest>,
        ) -> std::result::Result<tonic::Response<super::User>, tonic::Status> {
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
                "/google.cloud.alloydb.v1alpha.AlloyDBAdmin/GetUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.alloydb.v1alpha.AlloyDBAdmin",
                        "GetUser",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new User in a given project, location, and cluster.
        pub async fn create_user(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateUserRequest>,
        ) -> std::result::Result<tonic::Response<super::User>, tonic::Status> {
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
                "/google.cloud.alloydb.v1alpha.AlloyDBAdmin/CreateUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.alloydb.v1alpha.AlloyDBAdmin",
                        "CreateUser",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the parameters of a single User.
        pub async fn update_user(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateUserRequest>,
        ) -> std::result::Result<tonic::Response<super::User>, tonic::Status> {
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
                "/google.cloud.alloydb.v1alpha.AlloyDBAdmin/UpdateUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.alloydb.v1alpha.AlloyDBAdmin",
                        "UpdateUser",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single User.
        pub async fn delete_user(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteUserRequest>,
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
                "/google.cloud.alloydb.v1alpha.AlloyDBAdmin/DeleteUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.alloydb.v1alpha.AlloyDBAdmin",
                        "DeleteUser",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists Databases in a given project and location.
        pub async fn list_databases(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDatabasesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDatabasesResponse>,
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
                "/google.cloud.alloydb.v1alpha.AlloyDBAdmin/ListDatabases",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.alloydb.v1alpha.AlloyDBAdmin",
                        "ListDatabases",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
