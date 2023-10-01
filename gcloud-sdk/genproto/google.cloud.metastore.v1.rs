/// A managed metastore service that serves metadata queries.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Service {
    /// Immutable. The relative resource name of the metastore service, in the
    /// following format:
    ///
    /// `projects/{project_number}/locations/{location_id}/services/{service_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The time when the metastore service was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the metastore service was last updated.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// User-defined labels for the metastore service.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Immutable. The relative resource name of the VPC network on which the
    /// instance can be accessed. It is specified in the following form:
    ///
    /// `projects/{project_number}/global/networks/{network_id}`.
    #[prost(string, tag = "7")]
    pub network: ::prost::alloc::string::String,
    /// Output only. The URI of the endpoint used to access the metastore service.
    #[prost(string, tag = "8")]
    pub endpoint_uri: ::prost::alloc::string::String,
    /// The TCP port at which the metastore service is reached. Default: 9083.
    #[prost(int32, tag = "9")]
    pub port: i32,
    /// Output only. The current state of the metastore service.
    #[prost(enumeration = "service::State", tag = "10")]
    pub state: i32,
    /// Output only. Additional information about the current state of the
    /// metastore service, if available.
    #[prost(string, tag = "11")]
    pub state_message: ::prost::alloc::string::String,
    /// Output only. A Cloud Storage URI (starting with `gs://`) that specifies
    /// where artifacts related to the metastore service are stored.
    #[prost(string, tag = "12")]
    pub artifact_gcs_uri: ::prost::alloc::string::String,
    /// The tier of the service.
    #[prost(enumeration = "service::Tier", tag = "13")]
    pub tier: i32,
    /// The one hour maintenance window of the metastore service. This specifies
    /// when the service can be restarted for maintenance purposes in UTC time.
    /// Maintenance window is not needed for services with the SPANNER
    /// database type.
    #[prost(message, optional, tag = "15")]
    pub maintenance_window: ::core::option::Option<MaintenanceWindow>,
    /// Output only. The globally unique resource identifier of the metastore
    /// service.
    #[prost(string, tag = "16")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. The metadata management activities of the metastore service.
    #[prost(message, optional, tag = "17")]
    pub metadata_management_activity: ::core::option::Option<MetadataManagementActivity>,
    /// Immutable. The release channel of the service.
    /// If unspecified, defaults to `STABLE`.
    #[prost(enumeration = "service::ReleaseChannel", tag = "19")]
    pub release_channel: i32,
    /// Immutable. Information used to configure the Dataproc Metastore service to
    /// encrypt customer data at rest. Cannot be updated.
    #[prost(message, optional, tag = "20")]
    pub encryption_config: ::core::option::Option<EncryptionConfig>,
    /// The configuration specifying the network settings for the
    /// Dataproc Metastore service.
    #[prost(message, optional, tag = "21")]
    pub network_config: ::core::option::Option<NetworkConfig>,
    /// Immutable. The database type that the Metastore service stores its data.
    #[prost(enumeration = "service::DatabaseType", tag = "22")]
    pub database_type: i32,
    /// The configuration specifying telemetry settings for the Dataproc Metastore
    /// service. If unspecified defaults to `JSON`.
    #[prost(message, optional, tag = "23")]
    pub telemetry_config: ::core::option::Option<TelemetryConfig>,
    /// Scaling configuration of the metastore service.
    #[prost(message, optional, tag = "24")]
    pub scaling_config: ::core::option::Option<ScalingConfig>,
    /// Configuration properties specific to the underlying metastore service
    /// technology (the software that serves metastore queries).
    #[prost(oneof = "service::MetastoreConfig", tags = "5")]
    pub metastore_config: ::core::option::Option<service::MetastoreConfig>,
}
/// Nested message and enum types in `Service`.
pub mod service {
    /// The current state of the metastore service.
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
        /// The state of the metastore service is unknown.
        Unspecified = 0,
        /// The metastore service is in the process of being created.
        Creating = 1,
        /// The metastore service is running and ready to serve queries.
        Active = 2,
        /// The metastore service is entering suspension. Its query-serving
        /// availability may cease unexpectedly.
        Suspending = 3,
        /// The metastore service is suspended and unable to serve queries.
        Suspended = 4,
        /// The metastore service is being updated. It remains usable but cannot
        /// accept additional update requests or be deleted at this time.
        Updating = 5,
        /// The metastore service is undergoing deletion. It cannot be used.
        Deleting = 6,
        /// The metastore service has encountered an error and cannot be used. The
        /// metastore service should be deleted.
        Error = 7,
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
                State::Suspending => "SUSPENDING",
                State::Suspended => "SUSPENDED",
                State::Updating => "UPDATING",
                State::Deleting => "DELETING",
                State::Error => "ERROR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "ACTIVE" => Some(Self::Active),
                "SUSPENDING" => Some(Self::Suspending),
                "SUSPENDED" => Some(Self::Suspended),
                "UPDATING" => Some(Self::Updating),
                "DELETING" => Some(Self::Deleting),
                "ERROR" => Some(Self::Error),
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
        /// The tier is not set.
        Unspecified = 0,
        /// The developer tier provides limited scalability and no fault tolerance.
        /// Good for low-cost proof-of-concept.
        Developer = 1,
        /// The enterprise tier provides multi-zone high availability, and sufficient
        /// scalability for enterprise-level Dataproc Metastore workloads.
        Enterprise = 3,
    }
    impl Tier {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Tier::Unspecified => "TIER_UNSPECIFIED",
                Tier::Developer => "DEVELOPER",
                Tier::Enterprise => "ENTERPRISE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TIER_UNSPECIFIED" => Some(Self::Unspecified),
                "DEVELOPER" => Some(Self::Developer),
                "ENTERPRISE" => Some(Self::Enterprise),
                _ => None,
            }
        }
    }
    /// Release channels bundle features of varying levels of stability. Newer
    /// features may be introduced initially into less stable release channels and
    /// can be automatically promoted into more stable release channels.
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
        /// Release channel is not specified.
        Unspecified = 0,
        /// The `CANARY` release channel contains the newest features, which may be
        /// unstable and subject to unresolved issues with no known workarounds.
        /// Services using the `CANARY` release channel are not subject to any SLAs.
        Canary = 1,
        /// The `STABLE` release channel contains features that are considered stable
        /// and have been validated for production use.
        Stable = 2,
    }
    impl ReleaseChannel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ReleaseChannel::Unspecified => "RELEASE_CHANNEL_UNSPECIFIED",
                ReleaseChannel::Canary => "CANARY",
                ReleaseChannel::Stable => "STABLE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RELEASE_CHANNEL_UNSPECIFIED" => Some(Self::Unspecified),
                "CANARY" => Some(Self::Canary),
                "STABLE" => Some(Self::Stable),
                _ => None,
            }
        }
    }
    /// The backend database type for the metastore service.
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
    pub enum DatabaseType {
        /// The DATABASE_TYPE is not set.
        Unspecified = 0,
        /// MySQL is used to persist the metastore data.
        Mysql = 1,
        /// Spanner is used to persist the metastore data.
        Spanner = 2,
    }
    impl DatabaseType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DatabaseType::Unspecified => "DATABASE_TYPE_UNSPECIFIED",
                DatabaseType::Mysql => "MYSQL",
                DatabaseType::Spanner => "SPANNER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DATABASE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "MYSQL" => Some(Self::Mysql),
                "SPANNER" => Some(Self::Spanner),
                _ => None,
            }
        }
    }
    /// Configuration properties specific to the underlying metastore service
    /// technology (the software that serves metastore queries).
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MetastoreConfig {
        /// Configuration information specific to running Hive metastore
        /// software as the metastore service.
        #[prost(message, tag = "5")]
        HiveMetastoreConfig(super::HiveMetastoreConfig),
    }
}
/// Maintenance window. This specifies when Dataproc Metastore
/// may perform system maintenance operation to the service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaintenanceWindow {
    /// The hour of day (0-23) when the window starts.
    #[prost(message, optional, tag = "1")]
    pub hour_of_day: ::core::option::Option<i32>,
    /// The day of week, when the window starts.
    #[prost(enumeration = "super::super::super::r#type::DayOfWeek", tag = "2")]
    pub day_of_week: i32,
}
/// Specifies configuration information specific to running Hive metastore
/// software as the metastore service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HiveMetastoreConfig {
    /// Immutable. The Hive metastore schema version.
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
    /// A mapping of Hive metastore configuration key-value pairs to apply to the
    /// Hive metastore (configured in `hive-site.xml`). The mappings
    /// override system defaults (some keys cannot be overridden). These
    /// overrides are also applied to auxiliary versions and can be further
    /// customized in the auxiliary version's `AuxiliaryVersionConfig`.
    #[prost(map = "string, string", tag = "2")]
    pub config_overrides: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Information used to configure the Hive metastore service as a service
    /// principal in a Kerberos realm. To disable Kerberos, use the `UpdateService`
    /// method and specify this field's path
    /// (`hive_metastore_config.kerberos_config`) in the request's `update_mask`
    /// while omitting this field from the request's `service`.
    #[prost(message, optional, tag = "3")]
    pub kerberos_config: ::core::option::Option<KerberosConfig>,
    /// The protocol to use for the metastore service endpoint. If unspecified,
    /// defaults to `THRIFT`.
    #[prost(enumeration = "hive_metastore_config::EndpointProtocol", tag = "4")]
    pub endpoint_protocol: i32,
    /// A mapping of Hive metastore version to the auxiliary version
    /// configuration. When specified, a secondary Hive metastore service is
    /// created along with the primary service. All auxiliary versions must be less
    /// than the service's primary version. The key is the auxiliary service name
    /// and it must match the regular expression [a-z](\[-a-z0-9\]*[a-z0-9])?. This
    /// means that the first character must be a lowercase letter, and all the
    /// following characters must be hyphens, lowercase letters, or digits, except
    /// the last character, which cannot be a hyphen.
    #[prost(map = "string, message", tag = "5")]
    pub auxiliary_versions: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        AuxiliaryVersionConfig,
    >,
}
/// Nested message and enum types in `HiveMetastoreConfig`.
pub mod hive_metastore_config {
    /// Protocols available for serving the metastore service endpoint.
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
    pub enum EndpointProtocol {
        /// The protocol is not set.
        Unspecified = 0,
        /// Use the legacy Apache Thrift protocol for the metastore service endpoint.
        Thrift = 1,
        /// Use the modernized gRPC protocol for the metastore service endpoint.
        Grpc = 2,
    }
    impl EndpointProtocol {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EndpointProtocol::Unspecified => "ENDPOINT_PROTOCOL_UNSPECIFIED",
                EndpointProtocol::Thrift => "THRIFT",
                EndpointProtocol::Grpc => "GRPC",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ENDPOINT_PROTOCOL_UNSPECIFIED" => Some(Self::Unspecified),
                "THRIFT" => Some(Self::Thrift),
                "GRPC" => Some(Self::Grpc),
                _ => None,
            }
        }
    }
}
/// Configuration information for a Kerberos principal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KerberosConfig {
    /// A Kerberos keytab file that can be used to authenticate a service principal
    /// with a Kerberos Key Distribution Center (KDC).
    #[prost(message, optional, tag = "1")]
    pub keytab: ::core::option::Option<Secret>,
    /// A Kerberos principal that exists in the both the keytab the KDC
    /// to authenticate as. A typical principal is of the form
    /// `primary/instance@REALM`, but there is no exact format.
    #[prost(string, tag = "2")]
    pub principal: ::prost::alloc::string::String,
    /// A Cloud Storage URI that specifies the path to a
    /// krb5.conf file. It is of the form `gs://{bucket_name}/path/to/krb5.conf`,
    /// although the file does not need to be named krb5.conf explicitly.
    #[prost(string, tag = "3")]
    pub krb5_config_gcs_uri: ::prost::alloc::string::String,
}
/// A securely stored value.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Secret {
    #[prost(oneof = "secret::Value", tags = "2")]
    pub value: ::core::option::Option<secret::Value>,
}
/// Nested message and enum types in `Secret`.
pub mod secret {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// The relative resource name of a Secret Manager secret version, in the
        /// following form:
        ///
        /// `projects/{project_number}/secrets/{secret_id}/versions/{version_id}`.
        #[prost(string, tag = "2")]
        CloudSecret(::prost::alloc::string::String),
    }
}
/// Encryption settings for the service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EncryptionConfig {
    /// The fully qualified customer provided Cloud KMS key name to use for
    /// customer data encryption, in the following form:
    ///
    /// `projects/{project_number}/locations/{location_id}/keyRings/{key_ring_id}/cryptoKeys/{crypto_key_id}`.
    #[prost(string, tag = "1")]
    pub kms_key: ::prost::alloc::string::String,
}
/// Configuration information for the auxiliary service versions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuxiliaryVersionConfig {
    /// The Hive metastore version of the auxiliary service. It must be less
    /// than the primary Hive metastore service's version.
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
    /// A mapping of Hive metastore configuration key-value pairs to apply to the
    /// auxiliary Hive metastore (configured in `hive-site.xml`) in addition to
    /// the primary version's overrides. If keys are present in both the auxiliary
    /// version's overrides and the primary version's overrides, the value from
    /// the auxiliary version's overrides takes precedence.
    #[prost(map = "string, string", tag = "2")]
    pub config_overrides: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. The network configuration contains the endpoint URI(s) of the
    /// auxiliary Hive metastore service.
    #[prost(message, optional, tag = "3")]
    pub network_config: ::core::option::Option<NetworkConfig>,
}
/// Network configuration for the Dataproc Metastore service.
///
/// Next available ID: 4
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkConfig {
    /// Immutable. The consumer-side network configuration for the Dataproc
    /// Metastore instance.
    #[prost(message, repeated, tag = "1")]
    pub consumers: ::prost::alloc::vec::Vec<network_config::Consumer>,
}
/// Nested message and enum types in `NetworkConfig`.
pub mod network_config {
    /// Contains information of the customer's network configurations.
    ///
    /// Next available ID: 5
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Consumer {
        /// Output only. The URI of the endpoint used to access the metastore
        /// service.
        #[prost(string, tag = "3")]
        pub endpoint_uri: ::prost::alloc::string::String,
        /// Output only. The location of the endpoint URI. Format:
        /// `projects/{project}/locations/{location}`.
        #[prost(string, tag = "4")]
        pub endpoint_location: ::prost::alloc::string::String,
        #[prost(oneof = "consumer::VpcResource", tags = "1")]
        pub vpc_resource: ::core::option::Option<consumer::VpcResource>,
    }
    /// Nested message and enum types in `Consumer`.
    pub mod consumer {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum VpcResource {
            /// Immutable. The subnetwork of the customer project from which an IP
            /// address is reserved and used as the Dataproc Metastore service's
            /// endpoint. It is accessible to hosts in the subnet and to all
            /// hosts in a subnet in the same region and same network. There must
            /// be at least one IP address available in the subnet's primary range. The
            /// subnet is specified in the following form:
            ///
            /// `projects/{project_number}/regions/{region_id}/subnetworks/{subnetwork_id}`
            #[prost(string, tag = "1")]
            Subnetwork(::prost::alloc::string::String),
        }
    }
}
/// Telemetry Configuration for the Dataproc Metastore service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TelemetryConfig {
    /// The output format of the Dataproc Metastore service's logs.
    #[prost(enumeration = "telemetry_config::LogFormat", tag = "1")]
    pub log_format: i32,
}
/// Nested message and enum types in `TelemetryConfig`.
pub mod telemetry_config {
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
    pub enum LogFormat {
        /// The LOG_FORMAT is not set.
        Unspecified = 0,
        /// Logging output uses the legacy `textPayload` format.
        Legacy = 1,
        /// Logging output uses the `jsonPayload` format.
        Json = 2,
    }
    impl LogFormat {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LogFormat::Unspecified => "LOG_FORMAT_UNSPECIFIED",
                LogFormat::Legacy => "LEGACY",
                LogFormat::Json => "JSON",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LOG_FORMAT_UNSPECIFIED" => Some(Self::Unspecified),
                "LEGACY" => Some(Self::Legacy),
                "JSON" => Some(Self::Json),
                _ => None,
            }
        }
    }
}
/// The metadata management activities of the metastore service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetadataManagementActivity {
    /// Output only. The latest metadata exports of the metastore service.
    #[prost(message, repeated, tag = "1")]
    pub metadata_exports: ::prost::alloc::vec::Vec<MetadataExport>,
    /// Output only. The latest restores of the metastore service.
    #[prost(message, repeated, tag = "2")]
    pub restores: ::prost::alloc::vec::Vec<Restore>,
}
/// A metastore resource that imports metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetadataImport {
    /// Immutable. The relative resource name of the metadata import, of the form:
    ///
    /// `projects/{project_number}/locations/{location_id}/services/{service_id}/metadataImports/{metadata_import_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The description of the metadata import.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The time when the metadata import was started.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the metadata import was last updated.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the metadata import finished.
    #[prost(message, optional, tag = "7")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The current state of the metadata import.
    #[prost(enumeration = "metadata_import::State", tag = "5")]
    pub state: i32,
    /// The metadata to be imported.
    #[prost(oneof = "metadata_import::Metadata", tags = "6")]
    pub metadata: ::core::option::Option<metadata_import::Metadata>,
}
/// Nested message and enum types in `MetadataImport`.
pub mod metadata_import {
    /// A specification of the location of and metadata about a database dump from
    /// a relational database management system.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DatabaseDump {
        /// The type of the database.
        #[deprecated]
        #[prost(enumeration = "database_dump::DatabaseType", tag = "1")]
        pub database_type: i32,
        /// A Cloud Storage object or folder URI that specifies the source from which
        /// to import metadata. It must begin with `gs://`.
        #[prost(string, tag = "2")]
        pub gcs_uri: ::prost::alloc::string::String,
        /// The name of the source database.
        #[deprecated]
        #[prost(string, tag = "3")]
        pub source_database: ::prost::alloc::string::String,
        /// Optional. The type of the database dump. If unspecified, defaults to
        /// `MYSQL`.
        #[prost(enumeration = "super::database_dump_spec::Type", tag = "4")]
        pub r#type: i32,
    }
    /// Nested message and enum types in `DatabaseDump`.
    pub mod database_dump {
        /// The type of the database.
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
        pub enum DatabaseType {
            /// The type of the source database is unknown.
            Unspecified = 0,
            /// The type of the source database is MySQL.
            Mysql = 1,
        }
        impl DatabaseType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    DatabaseType::Unspecified => "DATABASE_TYPE_UNSPECIFIED",
                    DatabaseType::Mysql => "MYSQL",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "DATABASE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                    "MYSQL" => Some(Self::Mysql),
                    _ => None,
                }
            }
        }
    }
    /// The current state of the metadata import.
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
        /// The state of the metadata import is unknown.
        Unspecified = 0,
        /// The metadata import is running.
        Running = 1,
        /// The metadata import completed successfully.
        Succeeded = 2,
        /// The metadata import is being updated.
        Updating = 3,
        /// The metadata import failed, and attempted metadata changes were rolled
        /// back.
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
                State::Running => "RUNNING",
                State::Succeeded => "SUCCEEDED",
                State::Updating => "UPDATING",
                State::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "RUNNING" => Some(Self::Running),
                "SUCCEEDED" => Some(Self::Succeeded),
                "UPDATING" => Some(Self::Updating),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
    /// The metadata to be imported.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Metadata {
        /// Immutable. A database dump from a pre-existing metastore's database.
        #[prost(message, tag = "6")]
        DatabaseDump(DatabaseDump),
    }
}
/// The details of a metadata export operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetadataExport {
    /// Output only. The time when the export started.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the export ended.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The current state of the export.
    #[prost(enumeration = "metadata_export::State", tag = "3")]
    pub state: i32,
    /// Output only. The type of the database dump.
    #[prost(enumeration = "database_dump_spec::Type", tag = "5")]
    pub database_dump_type: i32,
    #[prost(oneof = "metadata_export::Destination", tags = "4")]
    pub destination: ::core::option::Option<metadata_export::Destination>,
}
/// Nested message and enum types in `MetadataExport`.
pub mod metadata_export {
    /// The current state of the metadata export.
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
        /// The state of the metadata export is unknown.
        Unspecified = 0,
        /// The metadata export is running.
        Running = 1,
        /// The metadata export completed successfully.
        Succeeded = 2,
        /// The metadata export failed.
        Failed = 3,
        /// The metadata export is cancelled.
        Cancelled = 4,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Running => "RUNNING",
                State::Succeeded => "SUCCEEDED",
                State::Failed => "FAILED",
                State::Cancelled => "CANCELLED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "RUNNING" => Some(Self::Running),
                "SUCCEEDED" => Some(Self::Succeeded),
                "FAILED" => Some(Self::Failed),
                "CANCELLED" => Some(Self::Cancelled),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// Output only. A Cloud Storage URI of a folder that metadata are exported
        /// to, in the form of
        /// `gs://<bucket_name>/<path_inside_bucket>/<export_folder>`, where
        /// `<export_folder>` is automatically generated.
        #[prost(string, tag = "4")]
        DestinationGcsUri(::prost::alloc::string::String),
    }
}
/// The details of a backup resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Backup {
    /// Immutable. The relative resource name of the backup, in the following form:
    ///
    /// `projects/{project_number}/locations/{location_id}/services/{service_id}/backups/{backup_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The time when the backup was started.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the backup finished creating.
    #[prost(message, optional, tag = "3")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The current state of the backup.
    #[prost(enumeration = "backup::State", tag = "4")]
    pub state: i32,
    /// Output only. The revision of the service at the time of backup.
    #[prost(message, optional, tag = "5")]
    pub service_revision: ::core::option::Option<Service>,
    /// The description of the backup.
    #[prost(string, tag = "6")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Services that are restoring from the backup.
    #[prost(string, repeated, tag = "7")]
    pub restoring_services: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `Backup`.
pub mod backup {
    /// The current state of the backup.
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
        /// The backup is being created.
        Creating = 1,
        /// The backup is being deleted.
        Deleting = 2,
        /// The backup is active and ready to use.
        Active = 3,
        /// The backup failed.
        Failed = 4,
        /// The backup is being restored.
        Restoring = 5,
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
                State::Deleting => "DELETING",
                State::Active => "ACTIVE",
                State::Failed => "FAILED",
                State::Restoring => "RESTORING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "DELETING" => Some(Self::Deleting),
                "ACTIVE" => Some(Self::Active),
                "FAILED" => Some(Self::Failed),
                "RESTORING" => Some(Self::Restoring),
                _ => None,
            }
        }
    }
}
/// The details of a metadata restore operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Restore {
    /// Output only. The time when the restore started.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the restore ended.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The current state of the restore.
    #[prost(enumeration = "restore::State", tag = "3")]
    pub state: i32,
    /// Output only. The relative resource name of the metastore service backup to
    /// restore from, in the following form:
    ///
    /// `projects/{project_id}/locations/{location_id}/services/{service_id}/backups/{backup_id}`.
    #[prost(string, tag = "4")]
    pub backup: ::prost::alloc::string::String,
    /// Output only. The type of restore.
    #[prost(enumeration = "restore::RestoreType", tag = "5")]
    pub r#type: i32,
    /// Output only. The restore details containing the revision of the service to
    /// be restored to, in format of JSON.
    #[prost(string, tag = "6")]
    pub details: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Restore`.
pub mod restore {
    /// The current state of the restore.
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
        /// The state of the metadata restore is unknown.
        Unspecified = 0,
        /// The metadata restore is running.
        Running = 1,
        /// The metadata restore completed successfully.
        Succeeded = 2,
        /// The metadata restore failed.
        Failed = 3,
        /// The metadata restore is cancelled.
        Cancelled = 4,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Running => "RUNNING",
                State::Succeeded => "SUCCEEDED",
                State::Failed => "FAILED",
                State::Cancelled => "CANCELLED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "RUNNING" => Some(Self::Running),
                "SUCCEEDED" => Some(Self::Succeeded),
                "FAILED" => Some(Self::Failed),
                "CANCELLED" => Some(Self::Cancelled),
                _ => None,
            }
        }
    }
    /// The type of restore. If unspecified, defaults to `METADATA_ONLY`.
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
    pub enum RestoreType {
        /// The restore type is unknown.
        Unspecified = 0,
        /// The service's metadata and configuration are restored.
        Full = 1,
        /// Only the service's metadata is restored.
        MetadataOnly = 2,
    }
    impl RestoreType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RestoreType::Unspecified => "RESTORE_TYPE_UNSPECIFIED",
                RestoreType::Full => "FULL",
                RestoreType::MetadataOnly => "METADATA_ONLY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RESTORE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "FULL" => Some(Self::Full),
                "METADATA_ONLY" => Some(Self::MetadataOnly),
                _ => None,
            }
        }
    }
}
/// Represents the scaling configuration of a metastore service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScalingConfig {
    /// Represents either a predetermined instance size or a numeric
    /// scaling factor.
    #[prost(oneof = "scaling_config::ScalingModel", tags = "1, 2")]
    pub scaling_model: ::core::option::Option<scaling_config::ScalingModel>,
}
/// Nested message and enum types in `ScalingConfig`.
pub mod scaling_config {
    /// Metastore instance sizes.
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
    pub enum InstanceSize {
        /// Unspecified instance size
        Unspecified = 0,
        /// Extra small instance size, maps to a scaling factor of 0.1.
        ExtraSmall = 1,
        /// Small instance size, maps to a scaling factor of 0.5.
        Small = 2,
        /// Medium instance size, maps to a scaling factor of 1.0.
        Medium = 3,
        /// Large instance size, maps to a scaling factor of 3.0.
        Large = 4,
        /// Extra large instance size, maps to a scaling factor of 6.0.
        ExtraLarge = 5,
    }
    impl InstanceSize {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                InstanceSize::Unspecified => "INSTANCE_SIZE_UNSPECIFIED",
                InstanceSize::ExtraSmall => "EXTRA_SMALL",
                InstanceSize::Small => "SMALL",
                InstanceSize::Medium => "MEDIUM",
                InstanceSize::Large => "LARGE",
                InstanceSize::ExtraLarge => "EXTRA_LARGE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INSTANCE_SIZE_UNSPECIFIED" => Some(Self::Unspecified),
                "EXTRA_SMALL" => Some(Self::ExtraSmall),
                "SMALL" => Some(Self::Small),
                "MEDIUM" => Some(Self::Medium),
                "LARGE" => Some(Self::Large),
                "EXTRA_LARGE" => Some(Self::ExtraLarge),
                _ => None,
            }
        }
    }
    /// Represents either a predetermined instance size or a numeric
    /// scaling factor.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ScalingModel {
        /// An enum of readable instance sizes, with each instance size mapping to a
        /// float value (e.g. InstanceSize.EXTRA_SMALL = scaling_factor(0.1))
        #[prost(enumeration = "InstanceSize", tag = "1")]
        InstanceSize(i32),
        /// Scaling factor, increments of 0.1 for values less than 1.0, and
        /// increments of 1.0 for values greater than 1.0.
        #[prost(float, tag = "2")]
        ScalingFactor(f32),
    }
}
/// Request message for
/// [DataprocMetastore.ListServices][google.cloud.metastore.v1.DataprocMetastore.ListServices].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServicesRequest {
    /// Required. The relative resource name of the location of metastore services
    /// to list, in the following form:
    ///
    /// `projects/{project_number}/locations/{location_id}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of services to return. The response may
    /// contain less than the maximum number. If unspecified, no more than 500
    /// services are returned. The maximum value is 1000; values above 1000 are
    /// changed to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous
    /// [DataprocMetastore.ListServices][google.cloud.metastore.v1.DataprocMetastore.ListServices]
    /// call. Provide this token to retrieve the subsequent page.
    ///
    /// To retrieve the first page, supply an empty page token.
    ///
    /// When paginating, other parameters provided to
    /// [DataprocMetastore.ListServices][google.cloud.metastore.v1.DataprocMetastore.ListServices]
    /// must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The filter to apply to list results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Specify the ordering of results as described in [Sorting
    /// Order](<https://cloud.google.com/apis/design/design_patterns#sorting_order>).
    /// If not specified, the results will be sorted in the default order.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for
/// [DataprocMetastore.ListServices][google.cloud.metastore.v1.DataprocMetastore.ListServices].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServicesResponse {
    /// The services in the specified location.
    #[prost(message, repeated, tag = "1")]
    pub services: ::prost::alloc::vec::Vec<Service>,
    /// A token that can be sent as `page_token` to retrieve the next page. If this
    /// field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for
/// [DataprocMetastore.GetService][google.cloud.metastore.v1.DataprocMetastore.GetService].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServiceRequest {
    /// Required. The relative resource name of the metastore service to retrieve,
    /// in the following form:
    ///
    /// `projects/{project_number}/locations/{location_id}/services/{service_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [DataprocMetastore.CreateService][google.cloud.metastore.v1.DataprocMetastore.CreateService].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateServiceRequest {
    /// Required. The relative resource name of the location in which to create a
    /// metastore service, in the following form:
    ///
    /// `projects/{project_number}/locations/{location_id}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID of the metastore service, which is used as the final
    /// component of the metastore service's name.
    ///
    /// This value must be between 2 and 63 characters long inclusive, begin with a
    /// letter, end with a letter or number, and consist of alpha-numeric
    /// ASCII characters or hyphens.
    #[prost(string, tag = "2")]
    pub service_id: ::prost::alloc::string::String,
    /// Required. The Metastore service to create. The `name` field is
    /// ignored. The ID of the created metastore service must be provided in
    /// the request's `service_id` field.
    #[prost(message, optional, tag = "3")]
    pub service: ::core::option::Option<Service>,
    /// Optional. A request ID. Specify a unique request ID to allow the server to
    /// ignore the request if it has completed. The server will ignore subsequent
    /// requests that provide a duplicate request ID for at least 60 minutes after
    /// the first request.
    ///
    /// For example, if an initial request times out, followed by another request
    /// with the same request ID, the server ignores the second request to prevent
    /// the creation of duplicate commitments.
    ///
    /// The request ID must be a valid
    /// [UUID](<https://en.wikipedia.org/wiki/Universally_unique_identifier#Format>)
    /// A zero UUID (00000000-0000-0000-0000-000000000000) is not supported.
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [DataprocMetastore.UpdateService][google.cloud.metastore.v1.DataprocMetastore.UpdateService].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateServiceRequest {
    /// Required. A field mask used to specify the fields to be overwritten in the
    /// metastore service resource by the update.
    /// Fields specified in the `update_mask` are relative to the resource (not
    /// to the full request). A field is overwritten if it is in the mask.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The metastore service to update. The server only merges fields
    /// in the service if they are specified in `update_mask`.
    ///
    /// The metastore service's `name` field is used to identify the metastore
    /// service to be updated.
    #[prost(message, optional, tag = "2")]
    pub service: ::core::option::Option<Service>,
    /// Optional. A request ID. Specify a unique request ID to allow the server to
    /// ignore the request if it has completed. The server will ignore subsequent
    /// requests that provide a duplicate request ID for at least 60 minutes after
    /// the first request.
    ///
    /// For example, if an initial request times out, followed by another request
    /// with the same request ID, the server ignores the second request to prevent
    /// the creation of duplicate commitments.
    ///
    /// The request ID must be a valid
    /// [UUID](<https://en.wikipedia.org/wiki/Universally_unique_identifier#Format>)
    /// A zero UUID (00000000-0000-0000-0000-000000000000) is not supported.
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [DataprocMetastore.DeleteService][google.cloud.metastore.v1.DataprocMetastore.DeleteService].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteServiceRequest {
    /// Required. The relative resource name of the metastore service to delete, in
    /// the following form:
    ///
    /// `projects/{project_number}/locations/{location_id}/services/{service_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A request ID. Specify a unique request ID to allow the server to
    /// ignore the request if it has completed. The server will ignore subsequent
    /// requests that provide a duplicate request ID for at least 60 minutes after
    /// the first request.
    ///
    /// For example, if an initial request times out, followed by another request
    /// with the same request ID, the server ignores the second request to prevent
    /// the creation of duplicate commitments.
    ///
    /// The request ID must be a valid
    /// [UUID](<https://en.wikipedia.org/wiki/Universally_unique_identifier#Format>)
    /// A zero UUID (00000000-0000-0000-0000-000000000000) is not supported.
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [DataprocMetastore.ListMetadataImports][google.cloud.metastore.v1.DataprocMetastore.ListMetadataImports].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMetadataImportsRequest {
    /// Required. The relative resource name of the service whose metadata imports
    /// to list, in the following form:
    ///
    /// `projects/{project_number}/locations/{location_id}/services/{service_id}/metadataImports`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of imports to return. The response may contain
    /// less than the maximum number. If unspecified, no more than 500 imports are
    /// returned. The maximum value is 1000; values above 1000 are changed to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous
    /// [DataprocMetastore.ListServices][google.cloud.metastore.v1.DataprocMetastore.ListServices]
    /// call. Provide this token to retrieve the subsequent page.
    ///
    /// To retrieve the first page, supply an empty page token.
    ///
    /// When paginating, other parameters provided to
    /// [DataprocMetastore.ListServices][google.cloud.metastore.v1.DataprocMetastore.ListServices]
    /// must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The filter to apply to list results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Specify the ordering of results as described in [Sorting
    /// Order](<https://cloud.google.com/apis/design/design_patterns#sorting_order>).
    /// If not specified, the results will be sorted in the default order.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for
/// [DataprocMetastore.ListMetadataImports][google.cloud.metastore.v1.DataprocMetastore.ListMetadataImports].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMetadataImportsResponse {
    /// The imports in the specified service.
    #[prost(message, repeated, tag = "1")]
    pub metadata_imports: ::prost::alloc::vec::Vec<MetadataImport>,
    /// A token that can be sent as `page_token` to retrieve the next page. If this
    /// field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for
/// [DataprocMetastore.GetMetadataImport][google.cloud.metastore.v1.DataprocMetastore.GetMetadataImport].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMetadataImportRequest {
    /// Required. The relative resource name of the metadata import to retrieve, in
    /// the following form:
    ///
    /// `projects/{project_number}/locations/{location_id}/services/{service_id}/metadataImports/{import_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [DataprocMetastore.CreateMetadataImport][google.cloud.metastore.v1.DataprocMetastore.CreateMetadataImport].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMetadataImportRequest {
    /// Required. The relative resource name of the service in which to create a
    /// metastore import, in the following form:
    ///
    /// `projects/{project_number}/locations/{location_id}/services/{service_id}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID of the metadata import, which is used as the final
    /// component of the metadata import's name.
    ///
    /// This value must be between 1 and 64 characters long, begin with a letter,
    /// end with a letter or number, and consist of alpha-numeric ASCII characters
    /// or hyphens.
    #[prost(string, tag = "2")]
    pub metadata_import_id: ::prost::alloc::string::String,
    /// Required. The metadata import to create. The `name` field is ignored. The
    /// ID of the created metadata import must be provided in the request's
    /// `metadata_import_id` field.
    #[prost(message, optional, tag = "3")]
    pub metadata_import: ::core::option::Option<MetadataImport>,
    /// Optional. A request ID. Specify a unique request ID to allow the server to
    /// ignore the request if it has completed. The server will ignore subsequent
    /// requests that provide a duplicate request ID for at least 60 minutes after
    /// the first request.
    ///
    /// For example, if an initial request times out, followed by another request
    /// with the same request ID, the server ignores the second request to prevent
    /// the creation of duplicate commitments.
    ///
    /// The request ID must be a valid
    /// [UUID](<https://en.wikipedia.org/wiki/Universally_unique_identifier#Format>)
    /// A zero UUID (00000000-0000-0000-0000-000000000000) is not supported.
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [DataprocMetastore.UpdateMetadataImport][google.cloud.metastore.v1.DataprocMetastore.UpdateMetadataImport].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMetadataImportRequest {
    /// Required. A field mask used to specify the fields to be overwritten in the
    /// metadata import resource by the update.
    /// Fields specified in the `update_mask` are relative to the resource (not
    /// to the full request). A field is overwritten if it is in the mask.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The metadata import to update. The server only merges fields
    /// in the import if they are specified in `update_mask`.
    ///
    /// The metadata import's `name` field is used to identify the metastore
    /// import to be updated.
    #[prost(message, optional, tag = "2")]
    pub metadata_import: ::core::option::Option<MetadataImport>,
    /// Optional. A request ID. Specify a unique request ID to allow the server to
    /// ignore the request if it has completed. The server will ignore subsequent
    /// requests that provide a duplicate request ID for at least 60 minutes after
    /// the first request.
    ///
    /// For example, if an initial request times out, followed by another request
    /// with the same request ID, the server ignores the second request to prevent
    /// the creation of duplicate commitments.
    ///
    /// The request ID must be a valid
    /// [UUID](<https://en.wikipedia.org/wiki/Universally_unique_identifier#Format>)
    /// A zero UUID (00000000-0000-0000-0000-000000000000) is not supported.
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [DataprocMetastore.ListBackups][google.cloud.metastore.v1.DataprocMetastore.ListBackups].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBackupsRequest {
    /// Required. The relative resource name of the service whose backups to
    /// list, in the following form:
    ///
    /// `projects/{project_number}/locations/{location_id}/services/{service_id}/backups`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of backups to return. The response may contain
    /// less than the maximum number. If unspecified, no more than 500 backups are
    /// returned. The maximum value is 1000; values above 1000 are changed to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous
    /// [DataprocMetastore.ListBackups][google.cloud.metastore.v1.DataprocMetastore.ListBackups]
    /// call. Provide this token to retrieve the subsequent page.
    ///
    /// To retrieve the first page, supply an empty page token.
    ///
    /// When paginating, other parameters provided to
    /// [DataprocMetastore.ListBackups][google.cloud.metastore.v1.DataprocMetastore.ListBackups]
    /// must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The filter to apply to list results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Specify the ordering of results as described in [Sorting
    /// Order](<https://cloud.google.com/apis/design/design_patterns#sorting_order>).
    /// If not specified, the results will be sorted in the default order.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for
/// [DataprocMetastore.ListBackups][google.cloud.metastore.v1.DataprocMetastore.ListBackups].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBackupsResponse {
    /// The backups of the specified service.
    #[prost(message, repeated, tag = "1")]
    pub backups: ::prost::alloc::vec::Vec<Backup>,
    /// A token that can be sent as `page_token` to retrieve the next page. If this
    /// field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for
/// [DataprocMetastore.GetBackup][google.cloud.metastore.v1.DataprocMetastore.GetBackup].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBackupRequest {
    /// Required. The relative resource name of the backup to retrieve, in the
    /// following form:
    ///
    /// `projects/{project_number}/locations/{location_id}/services/{service_id}/backups/{backup_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [DataprocMetastore.CreateBackup][google.cloud.metastore.v1.DataprocMetastore.CreateBackup].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBackupRequest {
    /// Required. The relative resource name of the service in which to create a
    /// backup of the following form:
    ///
    /// `projects/{project_number}/locations/{location_id}/services/{service_id}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID of the backup, which is used as the final component of the
    /// backup's name.
    ///
    /// This value must be between 1 and 64 characters long, begin with a letter,
    /// end with a letter or number, and consist of alpha-numeric ASCII characters
    /// or hyphens.
    #[prost(string, tag = "2")]
    pub backup_id: ::prost::alloc::string::String,
    /// Required. The backup to create. The `name` field is ignored. The ID of the
    /// created backup must be provided in the request's `backup_id` field.
    #[prost(message, optional, tag = "3")]
    pub backup: ::core::option::Option<Backup>,
    /// Optional. A request ID. Specify a unique request ID to allow the server to
    /// ignore the request if it has completed. The server will ignore subsequent
    /// requests that provide a duplicate request ID for at least 60 minutes after
    /// the first request.
    ///
    /// For example, if an initial request times out, followed by another request
    /// with the same request ID, the server ignores the second request to prevent
    /// the creation of duplicate commitments.
    ///
    /// The request ID must be a valid
    /// [UUID](<https://en.wikipedia.org/wiki/Universally_unique_identifier#Format>)
    /// A zero UUID (00000000-0000-0000-0000-000000000000) is not supported.
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [DataprocMetastore.DeleteBackup][google.cloud.metastore.v1.DataprocMetastore.DeleteBackup].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteBackupRequest {
    /// Required. The relative resource name of the backup to delete, in the
    /// following form:
    ///
    /// `projects/{project_number}/locations/{location_id}/services/{service_id}/backups/{backup_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A request ID. Specify a unique request ID to allow the server to
    /// ignore the request if it has completed. The server will ignore subsequent
    /// requests that provide a duplicate request ID for at least 60 minutes after
    /// the first request.
    ///
    /// For example, if an initial request times out, followed by another request
    /// with the same request ID, the server ignores the second request to prevent
    /// the creation of duplicate commitments.
    ///
    /// The request ID must be a valid
    /// [UUID](<https://en.wikipedia.org/wiki/Universally_unique_identifier#Format>)
    /// A zero UUID (00000000-0000-0000-0000-000000000000) is not supported.
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [DataprocMetastore.ExportMetadata][google.cloud.metastore.v1.DataprocMetastore.ExportMetadata].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportMetadataRequest {
    /// Required. The relative resource name of the metastore service to run
    /// export, in the following form:
    ///
    /// `projects/{project_id}/locations/{location_id}/services/{service_id}`.
    #[prost(string, tag = "1")]
    pub service: ::prost::alloc::string::String,
    /// Optional. A request ID. Specify a unique request ID to allow the server to
    /// ignore the request if it has completed. The server will ignore subsequent
    /// requests that provide a duplicate request ID for at least 60 minutes after
    /// the first request.
    ///
    /// For example, if an initial request times out, followed by another request
    /// with the same request ID, the server ignores the second request to prevent
    /// the creation of duplicate commitments.
    ///
    /// The request ID must be a valid
    /// [UUID](<https://en.wikipedia.org/wiki/Universally_unique_identifier#Format>).
    /// A zero UUID (00000000-0000-0000-0000-000000000000) is not supported.
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. The type of the database dump. If unspecified, defaults to
    /// `MYSQL`.
    #[prost(enumeration = "database_dump_spec::Type", tag = "4")]
    pub database_dump_type: i32,
    /// Required. Destination that metadata is exported to.
    #[prost(oneof = "export_metadata_request::Destination", tags = "2")]
    pub destination: ::core::option::Option<export_metadata_request::Destination>,
}
/// Nested message and enum types in `ExportMetadataRequest`.
pub mod export_metadata_request {
    /// Required. Destination that metadata is exported to.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// A Cloud Storage URI of a folder, in the format
        /// `gs://<bucket_name>/<path_inside_bucket>`. A sub-folder
        /// `<export_folder>` containing exported files will be created below it.
        #[prost(string, tag = "2")]
        DestinationGcsFolder(::prost::alloc::string::String),
    }
}
/// Request message for [DataprocMetastore.Restore][].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreServiceRequest {
    /// Required. The relative resource name of the metastore service to run
    /// restore, in the following form:
    ///
    /// `projects/{project_id}/locations/{location_id}/services/{service_id}`.
    #[prost(string, tag = "1")]
    pub service: ::prost::alloc::string::String,
    /// Required. The relative resource name of the metastore service backup to
    /// restore from, in the following form:
    ///
    /// `projects/{project_id}/locations/{location_id}/services/{service_id}/backups/{backup_id}`.
    #[prost(string, tag = "2")]
    pub backup: ::prost::alloc::string::String,
    /// Optional. The type of restore. If unspecified, defaults to `METADATA_ONLY`.
    #[prost(enumeration = "restore::RestoreType", tag = "3")]
    pub restore_type: i32,
    /// Optional. A request ID. Specify a unique request ID to allow the server to
    /// ignore the request if it has completed. The server will ignore subsequent
    /// requests that provide a duplicate request ID for at least 60 minutes after
    /// the first request.
    ///
    /// For example, if an initial request times out, followed by another request
    /// with the same request ID, the server ignores the second request to prevent
    /// the creation of duplicate commitments.
    ///
    /// The request ID must be a valid
    /// [UUID](<https://en.wikipedia.org/wiki/Universally_unique_identifier#Format>).
    /// A zero UUID (00000000-0000-0000-0000-000000000000) is not supported.
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Represents the metadata of a long-running operation.
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
    /// Output only. Identifies whether the caller has requested cancellation
    /// of the operation. Operations that have successfully been cancelled
    /// have [Operation.error][] value with a
    /// [google.rpc.Status.code][google.rpc.Status.code] of 1, corresponding to
    /// `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
/// Metadata about the service in a location.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationMetadata {
    /// The versions of Hive Metastore that can be used when creating a new
    /// metastore service in this location. The server guarantees that exactly one
    /// `HiveMetastoreVersion` in the list will set `is_default`.
    #[prost(message, repeated, tag = "1")]
    pub supported_hive_metastore_versions: ::prost::alloc::vec::Vec<
        location_metadata::HiveMetastoreVersion,
    >,
}
/// Nested message and enum types in `LocationMetadata`.
pub mod location_metadata {
    /// A specification of a supported version of the Hive Metastore software.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HiveMetastoreVersion {
        /// The semantic version of the Hive Metastore software.
        #[prost(string, tag = "1")]
        pub version: ::prost::alloc::string::String,
        /// Whether `version` will be chosen by the server if a metastore service is
        /// created with a `HiveMetastoreConfig` that omits the `version`.
        #[prost(bool, tag = "2")]
        pub is_default: bool,
    }
}
/// The specification of database dump to import from or export to.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatabaseDumpSpec {}
/// Nested message and enum types in `DatabaseDumpSpec`.
pub mod database_dump_spec {
    /// The type of the database dump.
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
        /// The type of the database dump is unknown.
        Unspecified = 0,
        /// Database dump is a MySQL dump file.
        Mysql = 1,
        /// Database dump contains Avro files.
        Avro = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::Mysql => "MYSQL",
                Type::Avro => "AVRO",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "MYSQL" => Some(Self::Mysql),
                "AVRO" => Some(Self::Avro),
                _ => None,
            }
        }
    }
}
/// Request message for
/// [DataprocMetastore.QueryMetadata][google.cloud.metastore.v1.DataprocMetastore.QueryMetadata].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMetadataRequest {
    /// Required. The relative resource name of the metastore service to query
    /// metadata, in the following format:
    ///
    /// `projects/{project_id}/locations/{location_id}/services/{service_id}`.
    #[prost(string, tag = "1")]
    pub service: ::prost::alloc::string::String,
    /// Required. A read-only SQL query to execute against the metadata database.
    /// The query cannot change or mutate the data.
    #[prost(string, tag = "2")]
    pub query: ::prost::alloc::string::String,
}
/// Response message for
/// [DataprocMetastore.QueryMetadata][google.cloud.metastore.v1.DataprocMetastore.QueryMetadata].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMetadataResponse {
    /// The manifest URI  is link to a JSON instance in Cloud Storage.
    /// This instance manifests immediately along with QueryMetadataResponse. The
    /// content of the URI is not retriable until the long-running operation query
    /// against the metadata finishes.
    #[prost(string, tag = "1")]
    pub result_manifest_uri: ::prost::alloc::string::String,
}
/// Error details in public error message for
/// [DataprocMetastore.QueryMetadata][google.cloud.metastore.v1.DataprocMetastore.QueryMetadata].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorDetails {
    /// Additional structured details about this error.
    ///
    /// Keys define the failure items.
    /// Value describes the exception or details of the item.
    #[prost(map = "string, string", tag = "1")]
    pub details: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Request message for
/// [DataprocMetastore.MoveTableToDatabase][google.cloud.metastore.v1.DataprocMetastore.MoveTableToDatabase].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveTableToDatabaseRequest {
    /// Required. The relative resource name of the metastore service to mutate
    /// metadata, in the following format:
    ///
    /// `projects/{project_id}/locations/{location_id}/services/{service_id}`.
    #[prost(string, tag = "1")]
    pub service: ::prost::alloc::string::String,
    /// Required. The name of the table to be moved.
    #[prost(string, tag = "2")]
    pub table_name: ::prost::alloc::string::String,
    /// Required. The name of the database where the table resides.
    #[prost(string, tag = "3")]
    pub db_name: ::prost::alloc::string::String,
    /// Required. The name of the database where the table should be moved.
    #[prost(string, tag = "4")]
    pub destination_db_name: ::prost::alloc::string::String,
}
/// Response message for
/// [DataprocMetastore.MoveTableToDatabase][google.cloud.metastore.v1.DataprocMetastore.MoveTableToDatabase].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveTableToDatabaseResponse {}
/// Request message for
/// [DataprocMetastore.AlterMetadataResourceLocation][google.cloud.metastore.v1.DataprocMetastore.AlterMetadataResourceLocation].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlterMetadataResourceLocationRequest {
    /// Required. The relative resource name of the metastore service to mutate
    /// metadata, in the following format:
    ///
    /// `projects/{project_id}/locations/{location_id}/services/{service_id}`.
    #[prost(string, tag = "1")]
    pub service: ::prost::alloc::string::String,
    /// Required. The relative metadata resource name in the following format.
    ///
    /// `databases/{database_id}`
    /// or
    /// `databases/{database_id}/tables/{table_id}`
    /// or
    /// `databases/{database_id}/tables/{table_id}/partitions/{partition_id}`
    #[prost(string, tag = "2")]
    pub resource_name: ::prost::alloc::string::String,
    /// Required. The new location URI for the metadata resource.
    #[prost(string, tag = "3")]
    pub location_uri: ::prost::alloc::string::String,
}
/// Response message for
/// [DataprocMetastore.AlterMetadataResourceLocation][google.cloud.metastore.v1.DataprocMetastore.AlterMetadataResourceLocation].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlterMetadataResourceLocationResponse {}
/// Generated client implementations.
pub mod dataproc_metastore_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Configures and manages metastore services.
    /// Metastore services are fully managed, highly available, autoscaled,
    /// autohealing, OSS-native deployments of technical metadata management
    /// software. Each metastore service exposes a network endpoint through which
    /// metadata queries are served. Metadata queries can originate from a variety
    /// of sources, including Apache Hive, Apache Presto, and Apache Spark.
    ///
    /// The Dataproc Metastore API defines the following resource model:
    ///
    /// * The service works with a collection of Google Cloud projects, named:
    /// `/projects/*`
    /// * Each project has a collection of available locations, named: `/locations/*`
    ///   (a location must refer to a Google Cloud `region`)
    /// * Each location has a collection of services, named: `/services/*`
    /// * Dataproc Metastore services are resources with names of the form:
    ///
    ///   `/projects/{project_number}/locations/{location_id}/services/{service_id}`.
    #[derive(Debug, Clone)]
    pub struct DataprocMetastoreClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DataprocMetastoreClient<tonic::transport::Channel> {
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
    impl<T> DataprocMetastoreClient<T>
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
        ) -> DataprocMetastoreClient<InterceptedService<T, F>>
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
            DataprocMetastoreClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists services in a project and location.
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
                "/google.cloud.metastore.v1.DataprocMetastore/ListServices",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.metastore.v1.DataprocMetastore",
                        "ListServices",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the details of a single service.
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
                "/google.cloud.metastore.v1.DataprocMetastore/GetService",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.metastore.v1.DataprocMetastore",
                        "GetService",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a metastore service in a project and location.
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
                "/google.cloud.metastore.v1.DataprocMetastore/CreateService",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.metastore.v1.DataprocMetastore",
                        "CreateService",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the parameters of a single service.
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
                "/google.cloud.metastore.v1.DataprocMetastore/UpdateService",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.metastore.v1.DataprocMetastore",
                        "UpdateService",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single service.
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
                "/google.cloud.metastore.v1.DataprocMetastore/DeleteService",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.metastore.v1.DataprocMetastore",
                        "DeleteService",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists imports in a service.
        pub async fn list_metadata_imports(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMetadataImportsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMetadataImportsResponse>,
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
                "/google.cloud.metastore.v1.DataprocMetastore/ListMetadataImports",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.metastore.v1.DataprocMetastore",
                        "ListMetadataImports",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single import.
        pub async fn get_metadata_import(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMetadataImportRequest>,
        ) -> std::result::Result<tonic::Response<super::MetadataImport>, tonic::Status> {
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
                "/google.cloud.metastore.v1.DataprocMetastore/GetMetadataImport",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.metastore.v1.DataprocMetastore",
                        "GetMetadataImport",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new MetadataImport in a given project and location.
        pub async fn create_metadata_import(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateMetadataImportRequest>,
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
                "/google.cloud.metastore.v1.DataprocMetastore/CreateMetadataImport",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.metastore.v1.DataprocMetastore",
                        "CreateMetadataImport",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a single import.
        /// Only the description field of MetadataImport is supported to be updated.
        pub async fn update_metadata_import(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateMetadataImportRequest>,
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
                "/google.cloud.metastore.v1.DataprocMetastore/UpdateMetadataImport",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.metastore.v1.DataprocMetastore",
                        "UpdateMetadataImport",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Exports metadata from a service.
        pub async fn export_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportMetadataRequest>,
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
                "/google.cloud.metastore.v1.DataprocMetastore/ExportMetadata",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.metastore.v1.DataprocMetastore",
                        "ExportMetadata",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Restores a service from a backup.
        pub async fn restore_service(
            &mut self,
            request: impl tonic::IntoRequest<super::RestoreServiceRequest>,
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
                "/google.cloud.metastore.v1.DataprocMetastore/RestoreService",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.metastore.v1.DataprocMetastore",
                        "RestoreService",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists backups in a service.
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
                "/google.cloud.metastore.v1.DataprocMetastore/ListBackups",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.metastore.v1.DataprocMetastore",
                        "ListBackups",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single backup.
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
                "/google.cloud.metastore.v1.DataprocMetastore/GetBackup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.metastore.v1.DataprocMetastore",
                        "GetBackup",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new backup in a given project and location.
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
                "/google.cloud.metastore.v1.DataprocMetastore/CreateBackup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.metastore.v1.DataprocMetastore",
                        "CreateBackup",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single backup.
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
                "/google.cloud.metastore.v1.DataprocMetastore/DeleteBackup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.metastore.v1.DataprocMetastore",
                        "DeleteBackup",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Query DPMS metadata.
        pub async fn query_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryMetadataRequest>,
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
                "/google.cloud.metastore.v1.DataprocMetastore/QueryMetadata",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.metastore.v1.DataprocMetastore",
                        "QueryMetadata",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Move a table to another database.
        pub async fn move_table_to_database(
            &mut self,
            request: impl tonic::IntoRequest<super::MoveTableToDatabaseRequest>,
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
                "/google.cloud.metastore.v1.DataprocMetastore/MoveTableToDatabase",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.metastore.v1.DataprocMetastore",
                        "MoveTableToDatabase",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Alter metadata resource location. The metadata resource can be a database,
        /// table, or partition. This functionality only updates the parent directory
        /// for the respective metadata resource and does not transfer any existing
        /// data to the new location.
        pub async fn alter_metadata_resource_location(
            &mut self,
            request: impl tonic::IntoRequest<super::AlterMetadataResourceLocationRequest>,
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
                "/google.cloud.metastore.v1.DataprocMetastore/AlterMetadataResourceLocation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.metastore.v1.DataprocMetastore",
                        "AlterMetadataResourceLocation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Represents a federation of multiple backend metastores.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Federation {
    /// Immutable. The relative resource name of the federation, of the
    /// form:
    /// projects/{project_number}/locations/{location_id}/federations/{federation_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The time when the metastore federation was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the metastore federation was last updated.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// User-defined labels for the metastore federation.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Immutable. The Apache Hive metastore version of the federation. All backend
    /// metastore versions must be compatible with the federation version.
    #[prost(string, tag = "5")]
    pub version: ::prost::alloc::string::String,
    /// A map from `BackendMetastore` rank to `BackendMetastore`s from which the
    /// federation service serves metadata at query time. The map key represents
    /// the order in which `BackendMetastore`s should be evaluated to resolve
    /// database names at query time and should be greater than or equal to zero. A
    /// `BackendMetastore` with a lower number will be evaluated before a
    /// `BackendMetastore` with a higher number.
    #[prost(map = "int32, message", tag = "6")]
    pub backend_metastores: ::std::collections::HashMap<i32, BackendMetastore>,
    /// Output only. The federation endpoint.
    #[prost(string, tag = "7")]
    pub endpoint_uri: ::prost::alloc::string::String,
    /// Output only. The current state of the federation.
    #[prost(enumeration = "federation::State", tag = "8")]
    pub state: i32,
    /// Output only. Additional information about the current state of the
    /// metastore federation, if available.
    #[prost(string, tag = "9")]
    pub state_message: ::prost::alloc::string::String,
    /// Output only. The globally unique resource identifier of the metastore
    /// federation.
    #[prost(string, tag = "10")]
    pub uid: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Federation`.
pub mod federation {
    /// The current state of the federation.
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
        /// The state of the metastore federation is unknown.
        Unspecified = 0,
        /// The metastore federation is in the process of being created.
        Creating = 1,
        /// The metastore federation is running and ready to serve queries.
        Active = 2,
        /// The metastore federation is being updated. It remains usable but cannot
        /// accept additional update requests or be deleted at this time.
        Updating = 3,
        /// The metastore federation is undergoing deletion. It cannot be used.
        Deleting = 4,
        /// The metastore federation has encountered an error and cannot be used. The
        /// metastore federation should be deleted.
        Error = 5,
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
                State::Updating => "UPDATING",
                State::Deleting => "DELETING",
                State::Error => "ERROR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "ACTIVE" => Some(Self::Active),
                "UPDATING" => Some(Self::Updating),
                "DELETING" => Some(Self::Deleting),
                "ERROR" => Some(Self::Error),
                _ => None,
            }
        }
    }
}
/// Represents a backend metastore for the federation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackendMetastore {
    /// The relative resource name of the metastore that is being federated.
    /// The formats of the relative resource names for the currently supported
    /// metastores are listed below:
    ///
    /// * BigQuery
    ///      * `projects/{project_id}`
    /// * Dataproc Metastore
    ///      * `projects/{project_id}/locations/{location}/services/{service_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The type of the backend metastore.
    #[prost(enumeration = "backend_metastore::MetastoreType", tag = "2")]
    pub metastore_type: i32,
}
/// Nested message and enum types in `BackendMetastore`.
pub mod backend_metastore {
    /// The type of the backend metastore.
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
    pub enum MetastoreType {
        /// The metastore type is not set.
        Unspecified = 0,
        /// The backend metastore is BigQuery.
        Bigquery = 2,
        /// The backend metastore is Dataproc Metastore.
        DataprocMetastore = 3,
    }
    impl MetastoreType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MetastoreType::Unspecified => "METASTORE_TYPE_UNSPECIFIED",
                MetastoreType::Bigquery => "BIGQUERY",
                MetastoreType::DataprocMetastore => "DATAPROC_METASTORE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "METASTORE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "BIGQUERY" => Some(Self::Bigquery),
                "DATAPROC_METASTORE" => Some(Self::DataprocMetastore),
                _ => None,
            }
        }
    }
}
/// Request message for ListFederations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFederationsRequest {
    /// Required. The relative resource name of the location of metastore
    /// federations to list, in the following form:
    /// `projects/{project_number}/locations/{location_id}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of federations to return. The response may
    /// contain less than the maximum number. If unspecified, no more than 500
    /// services are returned. The maximum value is 1000; values above 1000 are
    /// changed to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous ListFederationServices
    /// call. Provide this token to retrieve the subsequent page.
    ///
    /// To retrieve the first page, supply an empty page token.
    ///
    /// When paginating, other parameters provided to
    /// ListFederationServices must match the call that provided the
    /// page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The filter to apply to list results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Specify the ordering of results as described in [Sorting
    /// Order](<https://cloud.google.com/apis/design/design_patterns#sorting_order>).
    /// If not specified, the results will be sorted in the default order.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for ListFederations
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFederationsResponse {
    /// The services in the specified location.
    #[prost(message, repeated, tag = "1")]
    pub federations: ::prost::alloc::vec::Vec<Federation>,
    /// A token that can be sent as `page_token` to retrieve the next page. If this
    /// field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for GetFederation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFederationRequest {
    /// Required. The relative resource name of the metastore federation to
    /// retrieve, in the following form:
    ///
    /// `projects/{project_number}/locations/{location_id}/federations/{federation_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for CreateFederation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFederationRequest {
    /// Required. The relative resource name of the location in which to create a
    /// federation service, in the following form:
    ///
    /// `projects/{project_number}/locations/{location_id}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID of the metastore federation, which is used as the final
    /// component of the metastore federation's name.
    ///
    /// This value must be between 2 and 63 characters long inclusive, begin with a
    /// letter, end with a letter or number, and consist of alpha-numeric
    /// ASCII characters or hyphens.
    #[prost(string, tag = "2")]
    pub federation_id: ::prost::alloc::string::String,
    /// Required. The Metastore Federation to create. The `name` field is
    /// ignored. The ID of the created metastore federation must be
    /// provided in the request's `federation_id` field.
    #[prost(message, optional, tag = "3")]
    pub federation: ::core::option::Option<Federation>,
    /// Optional. A request ID. Specify a unique request ID to allow the server to
    /// ignore the request if it has completed. The server will ignore subsequent
    /// requests that provide a duplicate request ID for at least 60 minutes after
    /// the first request.
    ///
    /// For example, if an initial request times out, followed by another request
    /// with the same request ID, the server ignores the second request to prevent
    /// the creation of duplicate commitments.
    ///
    /// The request ID must be a valid
    /// [UUID](<https://en.wikipedia.org/wiki/Universally_unique_identifier#Format>)
    /// A zero UUID (00000000-0000-0000-0000-000000000000) is not supported.
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for UpdateFederation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFederationRequest {
    /// Required. A field mask used to specify the fields to be overwritten in the
    /// metastore federation resource by the update.
    /// Fields specified in the `update_mask` are relative to the resource (not
    /// to the full request). A field is overwritten if it is in the mask.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The metastore federation to update. The server only merges fields
    /// in the service if they are specified in `update_mask`.
    ///
    /// The metastore federation's `name` field is used to identify the
    /// metastore service to be updated.
    #[prost(message, optional, tag = "2")]
    pub federation: ::core::option::Option<Federation>,
    /// Optional. A request ID. Specify a unique request ID to allow the server to
    /// ignore the request if it has completed. The server will ignore subsequent
    /// requests that provide a duplicate request ID for at least 60 minutes after
    /// the first request.
    ///
    /// For example, if an initial request times out, followed by another request
    /// with the same request ID, the server ignores the second request to prevent
    /// the creation of duplicate commitments.
    ///
    /// The request ID must be a valid
    /// [UUID](<https://en.wikipedia.org/wiki/Universally_unique_identifier#Format>)
    /// A zero UUID (00000000-0000-0000-0000-000000000000) is not supported.
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for DeleteFederation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFederationRequest {
    /// Required. The relative resource name of the metastore federation to delete,
    /// in the following form:
    ///
    /// `projects/{project_number}/locations/{location_id}/federations/{federation_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A request ID. Specify a unique request ID to allow the server to
    /// ignore the request if it has completed. The server will ignore subsequent
    /// requests that provide a duplicate request ID for at least 60 minutes after
    /// the first request.
    ///
    /// For example, if an initial request times out, followed by another request
    /// with the same request ID, the server ignores the second request to prevent
    /// the creation of duplicate commitments.
    ///
    /// The request ID must be a valid
    /// [UUID](<https://en.wikipedia.org/wiki/Universally_unique_identifier#Format>)
    /// A zero UUID (00000000-0000-0000-0000-000000000000) is not supported.
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod dataproc_metastore_federation_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Configures and manages metastore federation services.
    /// Dataproc Metastore Federation Service allows federating a collection of
    /// backend metastores like BigQuery, Dataplex Lakes, and other Dataproc
    /// Metastores. The Federation Service exposes a gRPC URL through which metadata
    /// from the backend metastores are served at query time.
    ///
    /// The Dataproc Metastore Federation API defines the following resource model:
    /// * The service works with a collection of Google Cloud projects.
    /// * Each project has a collection of available locations.
    /// * Each location has a collection of federations.
    /// * Dataproc Metastore Federations are resources with names of the
    /// form:
    /// `projects/{project_number}/locations/{location_id}/federations/{federation_id}`.
    #[derive(Debug, Clone)]
    pub struct DataprocMetastoreFederationClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DataprocMetastoreFederationClient<tonic::transport::Channel> {
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
    impl<T> DataprocMetastoreFederationClient<T>
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
        ) -> DataprocMetastoreFederationClient<InterceptedService<T, F>>
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
            DataprocMetastoreFederationClient::new(
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
        /// Lists federations in a project and location.
        pub async fn list_federations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFederationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListFederationsResponse>,
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
                "/google.cloud.metastore.v1.DataprocMetastoreFederation/ListFederations",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.metastore.v1.DataprocMetastoreFederation",
                        "ListFederations",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the details of a single federation.
        pub async fn get_federation(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFederationRequest>,
        ) -> std::result::Result<tonic::Response<super::Federation>, tonic::Status> {
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
                "/google.cloud.metastore.v1.DataprocMetastoreFederation/GetFederation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.metastore.v1.DataprocMetastoreFederation",
                        "GetFederation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a metastore federation in a project and location.
        pub async fn create_federation(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateFederationRequest>,
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
                "/google.cloud.metastore.v1.DataprocMetastoreFederation/CreateFederation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.metastore.v1.DataprocMetastoreFederation",
                        "CreateFederation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the fields of a federation.
        pub async fn update_federation(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateFederationRequest>,
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
                "/google.cloud.metastore.v1.DataprocMetastoreFederation/UpdateFederation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.metastore.v1.DataprocMetastoreFederation",
                        "UpdateFederation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single federation.
        pub async fn delete_federation(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteFederationRequest>,
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
                "/google.cloud.metastore.v1.DataprocMetastoreFederation/DeleteFederation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.metastore.v1.DataprocMetastoreFederation",
                        "DeleteFederation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
