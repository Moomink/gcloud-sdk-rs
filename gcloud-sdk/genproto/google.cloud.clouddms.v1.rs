/// SSL configuration information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SslConfig {
    /// Output only. The ssl config type according to 'client_key',
    /// 'client_certificate' and 'ca_certificate'.
    #[prost(enumeration = "ssl_config::SslType", tag = "1")]
    pub r#type: i32,
    /// Input only. The unencrypted PKCS#1 or PKCS#8 PEM-encoded private key
    /// associated with the Client Certificate. If this field is used then the
    /// 'client_certificate' field is mandatory.
    #[prost(string, tag = "2")]
    pub client_key: ::prost::alloc::string::String,
    /// Input only. The x509 PEM-encoded certificate that will be used by the
    /// replica to authenticate against the source database server.If this field is
    /// used then the 'client_key' field is mandatory.
    #[prost(string, tag = "3")]
    pub client_certificate: ::prost::alloc::string::String,
    /// Required. Input only. The x509 PEM-encoded certificate of the CA that
    /// signed the source database server's certificate. The replica will use this
    /// certificate to verify it's connecting to the right host.
    #[prost(string, tag = "4")]
    pub ca_certificate: ::prost::alloc::string::String,
}
/// Nested message and enum types in `SslConfig`.
pub mod ssl_config {
    /// Specifies The kind of ssl configuration used.
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
    pub enum SslType {
        /// Unspecified.
        Unspecified = 0,
        /// Only 'ca_certificate' specified.
        ServerOnly = 1,
        /// Both server ('ca_certificate'), and client ('client_key',
        /// 'client_certificate') specified.
        ServerClient = 2,
    }
    impl SslType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SslType::Unspecified => "SSL_TYPE_UNSPECIFIED",
                SslType::ServerOnly => "SERVER_ONLY",
                SslType::ServerClient => "SERVER_CLIENT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SSL_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "SERVER_ONLY" => Some(Self::ServerOnly),
                "SERVER_CLIENT" => Some(Self::ServerClient),
                _ => None,
            }
        }
    }
}
/// Specifies connection parameters required specifically for MySQL databases.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MySqlConnectionProfile {
    /// Required. The IP or hostname of the source MySQL database.
    #[prost(string, tag = "1")]
    pub host: ::prost::alloc::string::String,
    /// Required. The network port of the source MySQL database.
    #[prost(int32, tag = "2")]
    pub port: i32,
    /// Required. The username that Database Migration Service will use to connect
    /// to the database. The value is encrypted when stored in Database Migration
    /// Service.
    #[prost(string, tag = "3")]
    pub username: ::prost::alloc::string::String,
    /// Required. Input only. The password for the user that Database Migration
    /// Service will be using to connect to the database. This field is not
    /// returned on request, and the value is encrypted when stored in Database
    /// Migration Service.
    #[prost(string, tag = "4")]
    pub password: ::prost::alloc::string::String,
    /// Output only. Indicates If this connection profile password is stored.
    #[prost(bool, tag = "5")]
    pub password_set: bool,
    /// SSL configuration for the destination to connect to the source database.
    #[prost(message, optional, tag = "6")]
    pub ssl: ::core::option::Option<SslConfig>,
    /// If the source is a Cloud SQL database, use this field to
    /// provide the Cloud SQL instance ID of the source.
    #[prost(string, tag = "7")]
    pub cloud_sql_id: ::prost::alloc::string::String,
}
/// Specifies connection parameters required specifically for PostgreSQL
/// databases.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostgreSqlConnectionProfile {
    /// Required. The IP or hostname of the source PostgreSQL database.
    #[prost(string, tag = "1")]
    pub host: ::prost::alloc::string::String,
    /// Required. The network port of the source PostgreSQL database.
    #[prost(int32, tag = "2")]
    pub port: i32,
    /// Required. The username that Database Migration Service will use to connect
    /// to the database. The value is encrypted when stored in Database Migration
    /// Service.
    #[prost(string, tag = "3")]
    pub username: ::prost::alloc::string::String,
    /// Required. Input only. The password for the user that Database Migration
    /// Service will be using to connect to the database. This field is not
    /// returned on request, and the value is encrypted when stored in Database
    /// Migration Service.
    #[prost(string, tag = "4")]
    pub password: ::prost::alloc::string::String,
    /// Output only. Indicates If this connection profile password is stored.
    #[prost(bool, tag = "5")]
    pub password_set: bool,
    /// SSL configuration for the destination to connect to the source database.
    #[prost(message, optional, tag = "6")]
    pub ssl: ::core::option::Option<SslConfig>,
    /// If the source is a Cloud SQL database, use this field to
    /// provide the Cloud SQL instance ID of the source.
    #[prost(string, tag = "7")]
    pub cloud_sql_id: ::prost::alloc::string::String,
    /// Output only. If the source is a Cloud SQL database, this field indicates
    /// the network architecture it's associated with.
    #[prost(enumeration = "NetworkArchitecture", tag = "8")]
    pub network_architecture: i32,
    /// Connectivity options used to establish a connection to the database server.
    #[prost(oneof = "postgre_sql_connection_profile::Connectivity", tags = "100, 101")]
    pub connectivity: ::core::option::Option<
        postgre_sql_connection_profile::Connectivity,
    >,
}
/// Nested message and enum types in `PostgreSqlConnectionProfile`.
pub mod postgre_sql_connection_profile {
    /// Connectivity options used to establish a connection to the database server.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Connectivity {
        /// Static ip connectivity data (default, no additional details needed).
        #[prost(message, tag = "100")]
        StaticIpConnectivity(super::StaticIpConnectivity),
        /// Private service connect connectivity.
        #[prost(message, tag = "101")]
        PrivateServiceConnectConnectivity(super::PrivateServiceConnectConnectivity),
    }
}
/// Specifies connection parameters required specifically for Oracle
/// databases.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OracleConnectionProfile {
    /// Required. The IP or hostname of the source Oracle database.
    #[prost(string, tag = "1")]
    pub host: ::prost::alloc::string::String,
    /// Required. The network port of the source Oracle database.
    #[prost(int32, tag = "2")]
    pub port: i32,
    /// Required. The username that Database Migration Service will use to connect
    /// to the database. The value is encrypted when stored in Database Migration
    /// Service.
    #[prost(string, tag = "3")]
    pub username: ::prost::alloc::string::String,
    /// Required. Input only. The password for the user that Database Migration
    /// Service will be using to connect to the database. This field is not
    /// returned on request, and the value is encrypted when stored in Database
    /// Migration Service.
    #[prost(string, tag = "4")]
    pub password: ::prost::alloc::string::String,
    /// Output only. Indicates whether a new password is included in the request.
    #[prost(bool, tag = "5")]
    pub password_set: bool,
    /// Required. Database service for the Oracle connection.
    #[prost(string, tag = "6")]
    pub database_service: ::prost::alloc::string::String,
    /// SSL configuration for the connection to the source Oracle database.
    ///
    ///   * Only `SERVER_ONLY` configuration is supported for Oracle SSL.
    ///   * SSL is supported for Oracle versions 12 and above.
    #[prost(message, optional, tag = "7")]
    pub ssl: ::core::option::Option<SslConfig>,
    /// Connectivity options used to establish a connection to the database server.
    #[prost(oneof = "oracle_connection_profile::Connectivity", tags = "100, 101, 102")]
    pub connectivity: ::core::option::Option<oracle_connection_profile::Connectivity>,
}
/// Nested message and enum types in `OracleConnectionProfile`.
pub mod oracle_connection_profile {
    /// Connectivity options used to establish a connection to the database server.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Connectivity {
        /// Static Service IP connectivity.
        #[prost(message, tag = "100")]
        StaticServiceIpConnectivity(super::StaticServiceIpConnectivity),
        /// Forward SSH tunnel connectivity.
        #[prost(message, tag = "101")]
        ForwardSshConnectivity(super::ForwardSshTunnelConnectivity),
        /// Private connectivity.
        #[prost(message, tag = "102")]
        PrivateConnectivity(super::PrivateConnectivity),
    }
}
/// Specifies required connection parameters, and, optionally, the parameters
/// required to create a Cloud SQL destination database instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudSqlConnectionProfile {
    /// Output only. The Cloud SQL instance ID that this connection profile is
    /// associated with.
    #[prost(string, tag = "1")]
    pub cloud_sql_id: ::prost::alloc::string::String,
    /// Immutable. Metadata used to create the destination Cloud SQL database.
    #[prost(message, optional, tag = "2")]
    pub settings: ::core::option::Option<CloudSqlSettings>,
    /// Output only. The Cloud SQL database instance's private IP.
    #[prost(string, tag = "3")]
    pub private_ip: ::prost::alloc::string::String,
    /// Output only. The Cloud SQL database instance's public IP.
    #[prost(string, tag = "4")]
    pub public_ip: ::prost::alloc::string::String,
    /// Output only. The Cloud SQL database instance's additional (outgoing) public
    /// IP. Used when the Cloud SQL database availability type is REGIONAL (i.e.
    /// multiple zones / highly available).
    #[prost(string, tag = "5")]
    pub additional_public_ip: ::prost::alloc::string::String,
}
/// Specifies required connection parameters, and the parameters
/// required to create an AlloyDB destination cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlloyDbConnectionProfile {
    /// Required. The AlloyDB cluster ID that this connection profile is associated
    /// with.
    #[prost(string, tag = "1")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Immutable. Metadata used to create the destination AlloyDB cluster.
    #[prost(message, optional, tag = "2")]
    pub settings: ::core::option::Option<AlloyDbSettings>,
}
/// An entry for an Access Control list.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlAclEntry {
    /// The allowlisted value for the access control list.
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
    /// A label to identify this entry.
    #[prost(string, tag = "3")]
    pub label: ::prost::alloc::string::String,
    /// The access control entry entry expiration.
    #[prost(oneof = "sql_acl_entry::Expiration", tags = "10, 11")]
    pub expiration: ::core::option::Option<sql_acl_entry::Expiration>,
}
/// Nested message and enum types in `SqlAclEntry`.
pub mod sql_acl_entry {
    /// The access control entry entry expiration.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Expiration {
        /// The time when this access control entry expires in
        /// [RFC 3339](<https://tools.ietf.org/html/rfc3339>) format, for example:
        /// `2012-11-15T16:19:00.094Z`.
        #[prost(message, tag = "10")]
        ExpireTime(::prost_types::Timestamp),
        /// Input only. The time-to-leave of this access control entry.
        #[prost(message, tag = "11")]
        Ttl(::prost_types::Duration),
    }
}
/// IP Management configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlIpConfig {
    /// Whether the instance should be assigned an IPv4 address or not.
    #[prost(message, optional, tag = "1")]
    pub enable_ipv4: ::core::option::Option<bool>,
    /// The resource link for the VPC network from which the Cloud SQL instance is
    /// accessible for private IP. For example,
    /// `projects/myProject/global/networks/default`. This setting can
    /// be updated, but it cannot be removed after it is set.
    #[prost(string, tag = "2")]
    pub private_network: ::prost::alloc::string::String,
    /// Optional. The name of the allocated IP address range for the private IP
    /// Cloud SQL instance. This name refers to an already allocated IP range
    /// address. If set, the instance IP address will be created in the allocated
    /// range. Note that this IP address range can't be modified after the instance
    /// is created. If you change the VPC when configuring connectivity settings
    /// for the migration job, this field is not relevant.
    #[prost(string, tag = "5")]
    pub allocated_ip_range: ::prost::alloc::string::String,
    /// Whether SSL connections over IP should be enforced or not.
    #[prost(message, optional, tag = "3")]
    pub require_ssl: ::core::option::Option<bool>,
    /// The list of external networks that are allowed to connect to the instance
    /// using the IP. See
    /// <https://en.wikipedia.org/wiki/CIDR_notation#CIDR_notation,> also known as
    /// 'slash' notation (e.g. `192.168.100.0/24`).
    #[prost(message, repeated, tag = "4")]
    pub authorized_networks: ::prost::alloc::vec::Vec<SqlAclEntry>,
}
/// Settings for creating a Cloud SQL database instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudSqlSettings {
    /// The database engine type and version.
    #[prost(enumeration = "cloud_sql_settings::SqlDatabaseVersion", tag = "1")]
    pub database_version: i32,
    /// The resource labels for a Cloud SQL instance to use to annotate any related
    /// underlying resources such as Compute Engine VMs.
    /// An object containing a list of "key": "value" pairs.
    ///
    /// Example: `{ "name": "wrench", "mass": "18kg", "count": "3" }`.
    #[prost(map = "string, string", tag = "2")]
    pub user_labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The tier (or machine type) for this instance, for example:
    /// `db-n1-standard-1` (MySQL instances) or
    /// `db-custom-1-3840` (PostgreSQL instances).
    /// For more information, see
    /// [Cloud SQL Instance
    /// Settings](<https://cloud.google.com/sql/docs/mysql/instance-settings>).
    #[prost(string, tag = "3")]
    pub tier: ::prost::alloc::string::String,
    /// The maximum size to which storage capacity can be automatically increased.
    /// The default value is 0, which specifies that there is no limit.
    #[prost(message, optional, tag = "4")]
    pub storage_auto_resize_limit: ::core::option::Option<i64>,
    /// The activation policy specifies when the instance is activated; it is
    /// applicable only when the instance state is 'RUNNABLE'. Valid values:
    ///
    /// 'ALWAYS': The instance is on, and remains so even in
    /// the absence of connection requests.
    ///
    /// `NEVER`: The instance is off; it is not activated, even if a
    /// connection request arrives.
    #[prost(enumeration = "cloud_sql_settings::SqlActivationPolicy", tag = "5")]
    pub activation_policy: i32,
    /// The settings for IP Management. This allows to enable or disable the
    /// instance IP and manage which external networks can connect to the instance.
    /// The IPv4 address cannot be disabled.
    #[prost(message, optional, tag = "6")]
    pub ip_config: ::core::option::Option<SqlIpConfig>,
    /// \[default: ON\] If you enable this setting, Cloud SQL checks your available
    /// storage every 30 seconds. If the available storage falls below a threshold
    /// size, Cloud SQL automatically adds additional storage capacity. If the
    /// available storage repeatedly falls below the threshold size, Cloud SQL
    /// continues to add storage until it reaches the maximum of 30 TB.
    #[prost(message, optional, tag = "7")]
    pub auto_storage_increase: ::core::option::Option<bool>,
    /// The database flags passed to the Cloud SQL instance at startup.
    /// An object containing a list of "key": value pairs.
    /// Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
    #[prost(map = "string, string", tag = "8")]
    pub database_flags: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The type of storage: `PD_SSD` (default) or `PD_HDD`.
    #[prost(enumeration = "cloud_sql_settings::SqlDataDiskType", tag = "9")]
    pub data_disk_type: i32,
    /// The storage capacity available to the database, in GB.
    /// The minimum (and default) size is 10GB.
    #[prost(message, optional, tag = "10")]
    pub data_disk_size_gb: ::core::option::Option<i64>,
    /// The Google Cloud Platform zone where your Cloud SQL database instance is
    /// located.
    #[prost(string, tag = "11")]
    pub zone: ::prost::alloc::string::String,
    /// Optional. The Google Cloud Platform zone where the failover Cloud SQL
    /// database instance is located. Used when the Cloud SQL database availability
    /// type is REGIONAL (i.e. multiple zones / highly available).
    #[prost(string, tag = "18")]
    pub secondary_zone: ::prost::alloc::string::String,
    /// The Database Migration Service source connection profile ID,
    /// in the format:
    /// `projects/my_project_name/locations/us-central1/connectionProfiles/connection_profile_ID`
    #[prost(string, tag = "12")]
    pub source_id: ::prost::alloc::string::String,
    /// Input only. Initial root password.
    #[prost(string, tag = "13")]
    pub root_password: ::prost::alloc::string::String,
    /// Output only. Indicates If this connection profile root password is stored.
    #[prost(bool, tag = "14")]
    pub root_password_set: bool,
    /// The Cloud SQL default instance level collation.
    #[prost(string, tag = "15")]
    pub collation: ::prost::alloc::string::String,
    /// The KMS key name used for the csql instance.
    #[prost(string, tag = "16")]
    pub cmek_key_name: ::prost::alloc::string::String,
    /// Optional. Availability type. Potential values:
    /// *  `ZONAL`: The instance serves data from only one zone. Outages in that
    /// zone affect data availability.
    /// *  `REGIONAL`: The instance can serve data from more than one zone in a
    /// region (it is highly available).
    #[prost(enumeration = "cloud_sql_settings::SqlAvailabilityType", tag = "17")]
    pub availability_type: i32,
    /// Optional. The edition of the given Cloud SQL instance.
    #[prost(enumeration = "cloud_sql_settings::Edition", tag = "19")]
    pub edition: i32,
}
/// Nested message and enum types in `CloudSqlSettings`.
pub mod cloud_sql_settings {
    /// Specifies when the instance should be activated.
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
    pub enum SqlActivationPolicy {
        /// unspecified policy.
        Unspecified = 0,
        /// The instance is always up and running.
        Always = 1,
        /// The instance should never spin up.
        Never = 2,
    }
    impl SqlActivationPolicy {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SqlActivationPolicy::Unspecified => "SQL_ACTIVATION_POLICY_UNSPECIFIED",
                SqlActivationPolicy::Always => "ALWAYS",
                SqlActivationPolicy::Never => "NEVER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SQL_ACTIVATION_POLICY_UNSPECIFIED" => Some(Self::Unspecified),
                "ALWAYS" => Some(Self::Always),
                "NEVER" => Some(Self::Never),
                _ => None,
            }
        }
    }
    /// The storage options for Cloud SQL databases.
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
    pub enum SqlDataDiskType {
        /// Unspecified.
        Unspecified = 0,
        /// SSD disk.
        PdSsd = 1,
        /// HDD disk.
        PdHdd = 2,
    }
    impl SqlDataDiskType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SqlDataDiskType::Unspecified => "SQL_DATA_DISK_TYPE_UNSPECIFIED",
                SqlDataDiskType::PdSsd => "PD_SSD",
                SqlDataDiskType::PdHdd => "PD_HDD",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SQL_DATA_DISK_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "PD_SSD" => Some(Self::PdSsd),
                "PD_HDD" => Some(Self::PdHdd),
                _ => None,
            }
        }
    }
    /// The database engine type and version.
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
    pub enum SqlDatabaseVersion {
        /// Unspecified version.
        Unspecified = 0,
        /// MySQL 5.6.
        Mysql56 = 1,
        /// MySQL 5.7.
        Mysql57 = 2,
        /// PostgreSQL 9.6.
        Postgres96 = 3,
        /// PostgreSQL 11.
        Postgres11 = 4,
        /// PostgreSQL 10.
        Postgres10 = 5,
        /// MySQL 8.0.
        Mysql80 = 6,
        /// PostgreSQL 12.
        Postgres12 = 7,
        /// PostgreSQL 13.
        Postgres13 = 8,
        /// PostgreSQL 14.
        Postgres14 = 17,
        /// PostgreSQL 15.
        Postgres15 = 18,
    }
    impl SqlDatabaseVersion {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SqlDatabaseVersion::Unspecified => "SQL_DATABASE_VERSION_UNSPECIFIED",
                SqlDatabaseVersion::Mysql56 => "MYSQL_5_6",
                SqlDatabaseVersion::Mysql57 => "MYSQL_5_7",
                SqlDatabaseVersion::Postgres96 => "POSTGRES_9_6",
                SqlDatabaseVersion::Postgres11 => "POSTGRES_11",
                SqlDatabaseVersion::Postgres10 => "POSTGRES_10",
                SqlDatabaseVersion::Mysql80 => "MYSQL_8_0",
                SqlDatabaseVersion::Postgres12 => "POSTGRES_12",
                SqlDatabaseVersion::Postgres13 => "POSTGRES_13",
                SqlDatabaseVersion::Postgres14 => "POSTGRES_14",
                SqlDatabaseVersion::Postgres15 => "POSTGRES_15",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SQL_DATABASE_VERSION_UNSPECIFIED" => Some(Self::Unspecified),
                "MYSQL_5_6" => Some(Self::Mysql56),
                "MYSQL_5_7" => Some(Self::Mysql57),
                "POSTGRES_9_6" => Some(Self::Postgres96),
                "POSTGRES_11" => Some(Self::Postgres11),
                "POSTGRES_10" => Some(Self::Postgres10),
                "MYSQL_8_0" => Some(Self::Mysql80),
                "POSTGRES_12" => Some(Self::Postgres12),
                "POSTGRES_13" => Some(Self::Postgres13),
                "POSTGRES_14" => Some(Self::Postgres14),
                "POSTGRES_15" => Some(Self::Postgres15),
                _ => None,
            }
        }
    }
    /// The availability type of the given Cloud SQL instance.
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
    pub enum SqlAvailabilityType {
        /// This is an unknown Availability type.
        Unspecified = 0,
        /// Zonal availablility instance.
        Zonal = 1,
        /// Regional availability instance.
        Regional = 2,
    }
    impl SqlAvailabilityType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SqlAvailabilityType::Unspecified => "SQL_AVAILABILITY_TYPE_UNSPECIFIED",
                SqlAvailabilityType::Zonal => "ZONAL",
                SqlAvailabilityType::Regional => "REGIONAL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SQL_AVAILABILITY_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "ZONAL" => Some(Self::Zonal),
                "REGIONAL" => Some(Self::Regional),
                _ => None,
            }
        }
    }
    /// The edition of the given Cloud SQL instance.
    /// Can be ENTERPRISE or ENTERPRISE_PLUS.
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
    pub enum Edition {
        /// The instance did not specify the edition.
        Unspecified = 0,
        /// The instance is an enterprise edition.
        Enterprise = 2,
        /// The instance is an enterprise plus edition.
        EnterprisePlus = 3,
    }
    impl Edition {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Edition::Unspecified => "EDITION_UNSPECIFIED",
                Edition::Enterprise => "ENTERPRISE",
                Edition::EnterprisePlus => "ENTERPRISE_PLUS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "EDITION_UNSPECIFIED" => Some(Self::Unspecified),
                "ENTERPRISE" => Some(Self::Enterprise),
                "ENTERPRISE_PLUS" => Some(Self::EnterprisePlus),
                _ => None,
            }
        }
    }
}
/// Settings for creating an AlloyDB cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlloyDbSettings {
    /// Required. Input only. Initial user to setup during cluster creation.
    /// Required.
    #[prost(message, optional, tag = "1")]
    pub initial_user: ::core::option::Option<alloy_db_settings::UserPassword>,
    /// Required. The resource link for the VPC network in which cluster resources
    /// are created and from which they are accessible via Private IP. The network
    /// must belong to the same project as the cluster. It is specified in the
    /// form: "projects/{project_number}/global/networks/{network_id}". This is
    /// required to create a cluster.
    #[prost(string, tag = "2")]
    pub vpc_network: ::prost::alloc::string::String,
    /// Labels for the AlloyDB cluster created by DMS. An object containing a list
    /// of 'key', 'value' pairs.
    #[prost(map = "string, string", tag = "3")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(message, optional, tag = "4")]
    pub primary_instance_settings: ::core::option::Option<
        alloy_db_settings::PrimaryInstanceSettings,
    >,
    /// Optional. The encryption config can be specified to encrypt the data disks
    /// and other persistent data resources of a cluster with a
    /// customer-managed encryption key (CMEK). When this field is not
    /// specified, the cluster will then use default encryption scheme to
    /// protect the user data.
    #[prost(message, optional, tag = "5")]
    pub encryption_config: ::core::option::Option<alloy_db_settings::EncryptionConfig>,
}
/// Nested message and enum types in `AlloyDbSettings`.
pub mod alloy_db_settings {
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
        /// Output only. Indicates if the initial_user.password field has been set.
        #[prost(bool, tag = "3")]
        pub password_set: bool,
    }
    /// Settings for the cluster's primary instance
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PrimaryInstanceSettings {
        /// Required. The ID of the AlloyDB primary instance. The ID must satisfy the
        /// regex expression "\[a-z0-9-\]+".
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
        /// Configuration for the machines that host the underlying
        /// database engine.
        #[prost(message, optional, tag = "2")]
        pub machine_config: ::core::option::Option<
            primary_instance_settings::MachineConfig,
        >,
        /// Database flags to pass to AlloyDB when DMS is creating the AlloyDB
        /// cluster and instances. See the AlloyDB documentation for how these can be
        /// used.
        #[prost(map = "string, string", tag = "6")]
        pub database_flags: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
        /// Labels for the AlloyDB primary instance created by DMS. An object
        /// containing a list of 'key', 'value' pairs.
        #[prost(map = "string, string", tag = "7")]
        pub labels: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
        /// Output only. The private IP address for the Instance.
        /// This is the connection endpoint for an end-user application.
        #[prost(string, tag = "8")]
        pub private_ip: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `PrimaryInstanceSettings`.
    pub mod primary_instance_settings {
        /// MachineConfig describes the configuration of a machine.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MachineConfig {
            /// The number of CPU's in the VM instance.
            #[prost(int32, tag = "1")]
            pub cpu_count: i32,
        }
    }
    /// EncryptionConfig describes the encryption config of a cluster that is
    /// encrypted with a CMEK (customer-managed encryption key).
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EncryptionConfig {
        /// The fully-qualified resource name of the KMS key.
        /// Each Cloud KMS key is regionalized and has the following format:
        /// projects/\[PROJECT\]/locations/\[REGION\]/keyRings/\[RING\]/cryptoKeys/\[KEY_NAME\]
        #[prost(string, tag = "1")]
        pub kms_key_name: ::prost::alloc::string::String,
    }
}
/// The source database will allow incoming connections from the public IP of the
/// destination database. You can retrieve the public IP of the Cloud SQL
/// instance from the Cloud SQL console or using Cloud SQL APIs. No additional
/// configuration is required.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaticIpConnectivity {}
/// [Private Service Connect
/// connectivity](<https://cloud.google.com/vpc/docs/private-service-connect#service-attachments>)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateServiceConnectConnectivity {
    /// Required. A service attachment that exposes a database, and has the
    /// following format:
    /// projects/{project}/regions/{region}/serviceAttachments/{service_attachment_name}
    #[prost(string, tag = "1")]
    pub service_attachment: ::prost::alloc::string::String,
}
/// The details needed to configure a reverse SSH tunnel between the source and
/// destination databases. These details will be used when calling the
/// generateSshScript method (see
/// <https://cloud.google.com/database-migration/docs/reference/rest/v1/projects.locations.migrationJobs/generateSshScript>)
/// to produce the script that will help set up the reverse SSH tunnel, and to
/// set up the VPC peering between the Cloud SQL private network and the VPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReverseSshConnectivity {
    /// Required. The IP of the virtual machine (Compute Engine) used as the
    /// bastion server for the SSH tunnel.
    #[prost(string, tag = "1")]
    pub vm_ip: ::prost::alloc::string::String,
    /// Required. The forwarding port of the virtual machine (Compute Engine) used
    /// as the bastion server for the SSH tunnel.
    #[prost(int32, tag = "2")]
    pub vm_port: i32,
    /// The name of the virtual machine (Compute Engine) used as the bastion server
    /// for the SSH tunnel.
    #[prost(string, tag = "3")]
    pub vm: ::prost::alloc::string::String,
    /// The name of the VPC to peer with the Cloud SQL private network.
    #[prost(string, tag = "4")]
    pub vpc: ::prost::alloc::string::String,
}
/// The details of the VPC where the source database is located in Google Cloud.
/// We will use this information to set up the VPC peering connection between
/// Cloud SQL and this VPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VpcPeeringConnectivity {
    /// The name of the VPC network to peer with the Cloud SQL private network.
    #[prost(string, tag = "1")]
    pub vpc: ::prost::alloc::string::String,
}
/// Forward SSH Tunnel connectivity.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ForwardSshTunnelConnectivity {
    /// Required. Hostname for the SSH tunnel.
    #[prost(string, tag = "1")]
    pub hostname: ::prost::alloc::string::String,
    /// Required. Username for the SSH tunnel.
    #[prost(string, tag = "2")]
    pub username: ::prost::alloc::string::String,
    /// Port for the SSH tunnel, default value is 22.
    #[prost(int32, tag = "3")]
    pub port: i32,
    #[prost(
        oneof = "forward_ssh_tunnel_connectivity::AuthenticationMethod",
        tags = "100, 101"
    )]
    pub authentication_method: ::core::option::Option<
        forward_ssh_tunnel_connectivity::AuthenticationMethod,
    >,
}
/// Nested message and enum types in `ForwardSshTunnelConnectivity`.
pub mod forward_ssh_tunnel_connectivity {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AuthenticationMethod {
        /// Input only. SSH password.
        #[prost(string, tag = "100")]
        Password(::prost::alloc::string::String),
        /// Input only. SSH private key.
        #[prost(string, tag = "101")]
        PrivateKey(::prost::alloc::string::String),
    }
}
/// Static IP address connectivity configured on service project.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaticServiceIpConnectivity {}
/// Private Connectivity.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateConnectivity {
    /// Required. The resource name (URI) of the private connection.
    #[prost(string, tag = "1")]
    pub private_connection: ::prost::alloc::string::String,
}
/// A message defining the database engine and provider.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatabaseType {
    /// The database provider.
    #[prost(enumeration = "DatabaseProvider", tag = "1")]
    pub provider: i32,
    /// The database engine.
    #[prost(enumeration = "DatabaseEngine", tag = "2")]
    pub engine: i32,
}
/// Represents a Database Migration Service migration job object.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MigrationJob {
    /// The name (URI) of this migration job resource, in the form of:
    /// projects/{project}/locations/{location}/migrationJobs/{migrationJob}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The timestamp when the migration job resource was created.
    /// A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds.
    /// Example: "2014-10-02T15:01:23.045123456Z".
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when the migration job resource was last
    /// updated. A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds.
    /// Example: "2014-10-02T15:01:23.045123456Z".
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The resource labels for migration job to use to annotate any related
    /// underlying resources such as Compute Engine VMs. An object containing a
    /// list of "key": "value" pairs.
    ///
    /// Example: `{ "name": "wrench", "mass": "1.3kg", "count": "3" }`.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The migration job display name.
    #[prost(string, tag = "5")]
    pub display_name: ::prost::alloc::string::String,
    /// The current migration job state.
    #[prost(enumeration = "migration_job::State", tag = "6")]
    pub state: i32,
    /// Output only. The current migration job phase.
    #[prost(enumeration = "migration_job::Phase", tag = "7")]
    pub phase: i32,
    /// Required. The migration job type.
    #[prost(enumeration = "migration_job::Type", tag = "8")]
    pub r#type: i32,
    /// The path to the dump file in Google Cloud Storage,
    /// in the format: (gs://\[BUCKET_NAME\]/[OBJECT_NAME]).
    /// This field and the "dump_flags" field are mutually exclusive.
    #[prost(string, tag = "9")]
    pub dump_path: ::prost::alloc::string::String,
    /// The initial dump flags.
    /// This field and the "dump_path" field are mutually exclusive.
    #[prost(message, optional, tag = "17")]
    pub dump_flags: ::core::option::Option<migration_job::DumpFlags>,
    /// Required. The resource name (URI) of the source connection profile.
    #[prost(string, tag = "10")]
    pub source: ::prost::alloc::string::String,
    /// Required. The resource name (URI) of the destination connection profile.
    #[prost(string, tag = "11")]
    pub destination: ::prost::alloc::string::String,
    /// Output only. The duration of the migration job (in seconds). A duration in
    /// seconds with up to nine fractional digits, terminated by 's'. Example:
    /// "3.5s".
    #[prost(message, optional, tag = "12")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    /// Output only. The error details in case of state FAILED.
    #[prost(message, optional, tag = "13")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
    /// The database engine type and provider of the source.
    #[prost(message, optional, tag = "14")]
    pub source_database: ::core::option::Option<DatabaseType>,
    /// The database engine type and provider of the destination.
    #[prost(message, optional, tag = "15")]
    pub destination_database: ::core::option::Option<DatabaseType>,
    /// Output only. If the migration job is completed, the time when it was
    /// completed.
    #[prost(message, optional, tag = "16")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The conversion workspace used by the migration.
    #[prost(message, optional, tag = "18")]
    pub conversion_workspace: ::core::option::Option<ConversionWorkspaceInfo>,
    /// This field can be used to select the entities to migrate as part of
    /// the migration job. It uses AIP-160 notation to select a subset of the
    /// entities configured on the associated conversion-workspace. This field
    /// should not be set on migration-jobs that are not associated with a
    /// conversion workspace.
    #[prost(string, tag = "20")]
    pub filter: ::prost::alloc::string::String,
    /// The CMEK (customer-managed encryption key) fully qualified key name used
    /// for the migration job.
    /// This field supports all migration jobs types except for:
    /// * Mysql to Mysql (use the cmek field in the cloudsql connection profile
    /// instead).
    /// * PostrgeSQL to PostgreSQL (use the cmek field in the cloudsql
    /// connection profile instead).
    /// * PostgreSQL to AlloyDB (use the kms_key_name field in the alloydb
    /// connection profile instead).
    /// Each Cloud CMEK key has the following format:
    /// projects/\[PROJECT\]/locations/\[REGION\]/keyRings/\[RING\]/cryptoKeys/\[KEY_NAME\]
    #[prost(string, tag = "21")]
    pub cmek_key_name: ::prost::alloc::string::String,
    /// Optional. Data dump parallelism settings used by the migration.
    /// Currently applicable only for MySQL to Cloud SQL for MySQL migrations only.
    #[prost(message, optional, tag = "22")]
    pub performance_config: ::core::option::Option<migration_job::PerformanceConfig>,
    /// The connectivity method.
    #[prost(oneof = "migration_job::Connectivity", tags = "101, 102, 103")]
    pub connectivity: ::core::option::Option<migration_job::Connectivity>,
}
/// Nested message and enum types in `MigrationJob`.
pub mod migration_job {
    /// Dump flag definition.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DumpFlag {
        /// The name of the flag
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// The value of the flag.
        #[prost(string, tag = "2")]
        pub value: ::prost::alloc::string::String,
    }
    /// Dump flags definition.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DumpFlags {
        /// The flags for the initial dump.
        #[prost(message, repeated, tag = "1")]
        pub dump_flags: ::prost::alloc::vec::Vec<DumpFlag>,
    }
    /// Performance configuration definition.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PerformanceConfig {
        /// Initial dump parallelism level.
        #[prost(enumeration = "performance_config::DumpParallelLevel", tag = "1")]
        pub dump_parallel_level: i32,
    }
    /// Nested message and enum types in `PerformanceConfig`.
    pub mod performance_config {
        /// Describes the parallelism level during initial dump.
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
        pub enum DumpParallelLevel {
            /// Unknown dump parallel level. Will be defaulted to OPTIMAL.
            Unspecified = 0,
            /// Minimal parallel level.
            Min = 1,
            /// Optimal parallel level.
            Optimal = 2,
            /// Maximum parallel level.
            Max = 3,
        }
        impl DumpParallelLevel {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    DumpParallelLevel::Unspecified => "DUMP_PARALLEL_LEVEL_UNSPECIFIED",
                    DumpParallelLevel::Min => "MIN",
                    DumpParallelLevel::Optimal => "OPTIMAL",
                    DumpParallelLevel::Max => "MAX",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "DUMP_PARALLEL_LEVEL_UNSPECIFIED" => Some(Self::Unspecified),
                    "MIN" => Some(Self::Min),
                    "OPTIMAL" => Some(Self::Optimal),
                    "MAX" => Some(Self::Max),
                    _ => None,
                }
            }
        }
    }
    /// The current migration job states.
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
        /// The state of the migration job is unknown.
        Unspecified = 0,
        /// The migration job is down for maintenance.
        Maintenance = 1,
        /// The migration job is in draft mode and no resources are created.
        Draft = 2,
        /// The migration job is being created.
        Creating = 3,
        /// The migration job is created and not started.
        NotStarted = 4,
        /// The migration job is running.
        Running = 5,
        /// The migration job failed.
        Failed = 6,
        /// The migration job has been completed.
        Completed = 7,
        /// The migration job is being deleted.
        Deleting = 8,
        /// The migration job is being stopped.
        Stopping = 9,
        /// The migration job is currently stopped.
        Stopped = 10,
        /// The migration job has been deleted.
        Deleted = 11,
        /// The migration job is being updated.
        Updating = 12,
        /// The migration job is starting.
        Starting = 13,
        /// The migration job is restarting.
        Restarting = 14,
        /// The migration job is resuming.
        Resuming = 15,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Maintenance => "MAINTENANCE",
                State::Draft => "DRAFT",
                State::Creating => "CREATING",
                State::NotStarted => "NOT_STARTED",
                State::Running => "RUNNING",
                State::Failed => "FAILED",
                State::Completed => "COMPLETED",
                State::Deleting => "DELETING",
                State::Stopping => "STOPPING",
                State::Stopped => "STOPPED",
                State::Deleted => "DELETED",
                State::Updating => "UPDATING",
                State::Starting => "STARTING",
                State::Restarting => "RESTARTING",
                State::Resuming => "RESUMING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "MAINTENANCE" => Some(Self::Maintenance),
                "DRAFT" => Some(Self::Draft),
                "CREATING" => Some(Self::Creating),
                "NOT_STARTED" => Some(Self::NotStarted),
                "RUNNING" => Some(Self::Running),
                "FAILED" => Some(Self::Failed),
                "COMPLETED" => Some(Self::Completed),
                "DELETING" => Some(Self::Deleting),
                "STOPPING" => Some(Self::Stopping),
                "STOPPED" => Some(Self::Stopped),
                "DELETED" => Some(Self::Deleted),
                "UPDATING" => Some(Self::Updating),
                "STARTING" => Some(Self::Starting),
                "RESTARTING" => Some(Self::Restarting),
                "RESUMING" => Some(Self::Resuming),
                _ => None,
            }
        }
    }
    /// The current migration job phase.
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
    pub enum Phase {
        /// The phase of the migration job is unknown.
        Unspecified = 0,
        /// The migration job is in the full dump phase.
        FullDump = 1,
        /// The migration job is CDC phase.
        Cdc = 2,
        /// The migration job is running the promote phase.
        PromoteInProgress = 3,
        /// Only RDS flow - waiting for source writes to stop
        WaitingForSourceWritesToStop = 4,
        /// Only RDS flow - the sources writes stopped, waiting for dump to begin
        PreparingTheDump = 5,
    }
    impl Phase {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Phase::Unspecified => "PHASE_UNSPECIFIED",
                Phase::FullDump => "FULL_DUMP",
                Phase::Cdc => "CDC",
                Phase::PromoteInProgress => "PROMOTE_IN_PROGRESS",
                Phase::WaitingForSourceWritesToStop => {
                    "WAITING_FOR_SOURCE_WRITES_TO_STOP"
                }
                Phase::PreparingTheDump => "PREPARING_THE_DUMP",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PHASE_UNSPECIFIED" => Some(Self::Unspecified),
                "FULL_DUMP" => Some(Self::FullDump),
                "CDC" => Some(Self::Cdc),
                "PROMOTE_IN_PROGRESS" => Some(Self::PromoteInProgress),
                "WAITING_FOR_SOURCE_WRITES_TO_STOP" => {
                    Some(Self::WaitingForSourceWritesToStop)
                }
                "PREPARING_THE_DUMP" => Some(Self::PreparingTheDump),
                _ => None,
            }
        }
    }
    /// The type of migration job (one-time or continuous).
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
        /// The type of the migration job is unknown.
        Unspecified = 0,
        /// The migration job is a one time migration.
        OneTime = 1,
        /// The migration job is a continuous migration.
        Continuous = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::OneTime => "ONE_TIME",
                Type::Continuous => "CONTINUOUS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "ONE_TIME" => Some(Self::OneTime),
                "CONTINUOUS" => Some(Self::Continuous),
                _ => None,
            }
        }
    }
    /// The connectivity method.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Connectivity {
        /// The details needed to communicate to the source over Reverse SSH
        /// tunnel connectivity.
        #[prost(message, tag = "101")]
        ReverseSshConnectivity(super::ReverseSshConnectivity),
        /// The details of the VPC network that the source database is located in.
        #[prost(message, tag = "102")]
        VpcPeeringConnectivity(super::VpcPeeringConnectivity),
        /// static ip connectivity data (default, no additional details needed).
        #[prost(message, tag = "103")]
        StaticIpConnectivity(super::StaticIpConnectivity),
    }
}
/// A conversion workspace's version.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionWorkspaceInfo {
    /// The resource name (URI) of the conversion workspace.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The commit ID of the conversion workspace.
    #[prost(string, tag = "2")]
    pub commit_id: ::prost::alloc::string::String,
}
/// A connection profile definition.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionProfile {
    /// The name of this connection profile resource in the form of
    /// projects/{project}/locations/{location}/connectionProfiles/{connectionProfile}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The timestamp when the resource was created.
    /// A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds.
    /// Example: "2014-10-02T15:01:23.045123456Z".
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when the resource was last updated.
    /// A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds.
    /// Example: "2014-10-02T15:01:23.045123456Z".
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The resource labels for connection profile to use to annotate any related
    /// underlying resources such as Compute Engine VMs. An object containing a
    /// list of "key": "value" pairs.
    ///
    /// Example: `{ "name": "wrench", "mass": "1.3kg", "count": "3" }`.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The current connection profile state (e.g. DRAFT, READY, or FAILED).
    #[prost(enumeration = "connection_profile::State", tag = "5")]
    pub state: i32,
    /// The connection profile display name.
    #[prost(string, tag = "6")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. The error details in case of state FAILED.
    #[prost(message, optional, tag = "7")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
    /// The database provider.
    #[prost(enumeration = "DatabaseProvider", tag = "8")]
    pub provider: i32,
    /// The connection profile definition.
    #[prost(
        oneof = "connection_profile::ConnectionProfile",
        tags = "100, 101, 104, 102, 105"
    )]
    pub connection_profile: ::core::option::Option<
        connection_profile::ConnectionProfile,
    >,
}
/// Nested message and enum types in `ConnectionProfile`.
pub mod connection_profile {
    /// The current connection profile state (e.g. DRAFT, READY, or FAILED).
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
        /// The state of the connection profile is unknown.
        Unspecified = 0,
        /// The connection profile is in draft mode and fully editable.
        Draft = 1,
        /// The connection profile is being created.
        Creating = 2,
        /// The connection profile is ready.
        Ready = 3,
        /// The connection profile is being updated.
        Updating = 4,
        /// The connection profile is being deleted.
        Deleting = 5,
        /// The connection profile has been deleted.
        Deleted = 6,
        /// The last action on the connection profile failed.
        Failed = 7,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Draft => "DRAFT",
                State::Creating => "CREATING",
                State::Ready => "READY",
                State::Updating => "UPDATING",
                State::Deleting => "DELETING",
                State::Deleted => "DELETED",
                State::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "DRAFT" => Some(Self::Draft),
                "CREATING" => Some(Self::Creating),
                "READY" => Some(Self::Ready),
                "UPDATING" => Some(Self::Updating),
                "DELETING" => Some(Self::Deleting),
                "DELETED" => Some(Self::Deleted),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
    /// The connection profile definition.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConnectionProfile {
        /// A MySQL database connection profile.
        #[prost(message, tag = "100")]
        Mysql(super::MySqlConnectionProfile),
        /// A PostgreSQL database connection profile.
        #[prost(message, tag = "101")]
        Postgresql(super::PostgreSqlConnectionProfile),
        /// An Oracle database connection profile.
        #[prost(message, tag = "104")]
        Oracle(super::OracleConnectionProfile),
        /// A CloudSQL database connection profile.
        #[prost(message, tag = "102")]
        Cloudsql(super::CloudSqlConnectionProfile),
        /// An AlloyDB cluster connection profile.
        #[prost(message, tag = "105")]
        Alloydb(super::AlloyDbConnectionProfile),
    }
}
/// Error message of a verification Migration job.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MigrationJobVerificationError {
    /// Output only. An instance of ErrorCode specifying the error that occurred.
    #[prost(enumeration = "migration_job_verification_error::ErrorCode", tag = "1")]
    pub error_code: i32,
    /// Output only. A formatted message with further details about the error and a
    /// CTA.
    #[prost(string, tag = "2")]
    pub error_message: ::prost::alloc::string::String,
    /// Output only. A specific detailed error message, if supplied by the engine.
    #[prost(string, tag = "3")]
    pub error_detail_message: ::prost::alloc::string::String,
}
/// Nested message and enum types in `MigrationJobVerificationError`.
pub mod migration_job_verification_error {
    /// A general error code describing the type of error that occurred.
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
    pub enum ErrorCode {
        /// An unknown error occurred
        Unspecified = 0,
        /// We failed to connect to one of the connection profile.
        ConnectionFailure = 1,
        /// We failed to authenticate to one of the connection profile.
        AuthenticationFailure = 2,
        /// One of the involved connection profiles has an invalid configuration.
        InvalidConnectionProfileConfig = 3,
        /// The versions of the source and the destination are incompatible.
        VersionIncompatibility = 4,
        /// The types of the source and the destination are incompatible.
        ConnectionProfileTypesIncompatibility = 5,
        /// No pglogical extension installed on databases, applicable for postgres.
        NoPglogicalInstalled = 7,
        /// pglogical node already exists on databases, applicable for postgres.
        PglogicalNodeAlreadyExists = 8,
        /// The value of parameter wal_level is not set to logical.
        InvalidWalLevel = 9,
        /// The value of parameter shared_preload_libraries does not include
        /// pglogical.
        InvalidSharedPreloadLibrary = 10,
        /// The value of parameter max_replication_slots is not sufficient.
        InsufficientMaxReplicationSlots = 11,
        /// The value of parameter max_wal_senders is not sufficient.
        InsufficientMaxWalSenders = 12,
        /// The value of parameter max_worker_processes is not sufficient.
        InsufficientMaxWorkerProcesses = 13,
        /// Extensions installed are either not supported or having unsupported
        /// versions.
        UnsupportedExtensions = 14,
        /// Unsupported migration type.
        UnsupportedMigrationType = 15,
        /// Invalid RDS logical replication.
        InvalidRdsLogicalReplication = 16,
        /// The gtid_mode is not supported, applicable for MySQL.
        UnsupportedGtidMode = 17,
        /// The table definition is not support due to missing primary key or replica
        /// identity.
        UnsupportedTableDefinition = 18,
        /// The definer is not supported.
        UnsupportedDefiner = 19,
        /// Migration is already running at the time of restart request.
        CantRestartRunningMigration = 21,
        /// The source already has a replication setup.
        SourceAlreadySetup = 23,
        /// The source has tables with limited support.
        /// E.g. PostgreSQL tables without primary keys.
        TablesWithLimitedSupport = 24,
        /// The source uses an unsupported locale.
        UnsupportedDatabaseLocale = 25,
        /// The source uses an unsupported Foreign Data Wrapper configuration.
        UnsupportedDatabaseFdwConfig = 26,
        /// There was an underlying RDBMS error.
        ErrorRdbms = 27,
        /// The source DB size in Bytes exceeds a certain threshold. The migration
        /// might require an increase of quota, or might not be supported.
        SourceSizeExceedsThreshold = 28,
        /// The destination DB contains existing databases that are conflicting with
        /// those in the source DB.
        ExistingConflictingDatabases = 29,
        /// Insufficient privilege to enable the parallelism configuration.
        ParallelImportInsufficientPrivilege = 30,
    }
    impl ErrorCode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ErrorCode::Unspecified => "ERROR_CODE_UNSPECIFIED",
                ErrorCode::ConnectionFailure => "CONNECTION_FAILURE",
                ErrorCode::AuthenticationFailure => "AUTHENTICATION_FAILURE",
                ErrorCode::InvalidConnectionProfileConfig => {
                    "INVALID_CONNECTION_PROFILE_CONFIG"
                }
                ErrorCode::VersionIncompatibility => "VERSION_INCOMPATIBILITY",
                ErrorCode::ConnectionProfileTypesIncompatibility => {
                    "CONNECTION_PROFILE_TYPES_INCOMPATIBILITY"
                }
                ErrorCode::NoPglogicalInstalled => "NO_PGLOGICAL_INSTALLED",
                ErrorCode::PglogicalNodeAlreadyExists => "PGLOGICAL_NODE_ALREADY_EXISTS",
                ErrorCode::InvalidWalLevel => "INVALID_WAL_LEVEL",
                ErrorCode::InvalidSharedPreloadLibrary => {
                    "INVALID_SHARED_PRELOAD_LIBRARY"
                }
                ErrorCode::InsufficientMaxReplicationSlots => {
                    "INSUFFICIENT_MAX_REPLICATION_SLOTS"
                }
                ErrorCode::InsufficientMaxWalSenders => "INSUFFICIENT_MAX_WAL_SENDERS",
                ErrorCode::InsufficientMaxWorkerProcesses => {
                    "INSUFFICIENT_MAX_WORKER_PROCESSES"
                }
                ErrorCode::UnsupportedExtensions => "UNSUPPORTED_EXTENSIONS",
                ErrorCode::UnsupportedMigrationType => "UNSUPPORTED_MIGRATION_TYPE",
                ErrorCode::InvalidRdsLogicalReplication => {
                    "INVALID_RDS_LOGICAL_REPLICATION"
                }
                ErrorCode::UnsupportedGtidMode => "UNSUPPORTED_GTID_MODE",
                ErrorCode::UnsupportedTableDefinition => "UNSUPPORTED_TABLE_DEFINITION",
                ErrorCode::UnsupportedDefiner => "UNSUPPORTED_DEFINER",
                ErrorCode::CantRestartRunningMigration => {
                    "CANT_RESTART_RUNNING_MIGRATION"
                }
                ErrorCode::SourceAlreadySetup => "SOURCE_ALREADY_SETUP",
                ErrorCode::TablesWithLimitedSupport => "TABLES_WITH_LIMITED_SUPPORT",
                ErrorCode::UnsupportedDatabaseLocale => "UNSUPPORTED_DATABASE_LOCALE",
                ErrorCode::UnsupportedDatabaseFdwConfig => {
                    "UNSUPPORTED_DATABASE_FDW_CONFIG"
                }
                ErrorCode::ErrorRdbms => "ERROR_RDBMS",
                ErrorCode::SourceSizeExceedsThreshold => "SOURCE_SIZE_EXCEEDS_THRESHOLD",
                ErrorCode::ExistingConflictingDatabases => {
                    "EXISTING_CONFLICTING_DATABASES"
                }
                ErrorCode::ParallelImportInsufficientPrivilege => {
                    "PARALLEL_IMPORT_INSUFFICIENT_PRIVILEGE"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ERROR_CODE_UNSPECIFIED" => Some(Self::Unspecified),
                "CONNECTION_FAILURE" => Some(Self::ConnectionFailure),
                "AUTHENTICATION_FAILURE" => Some(Self::AuthenticationFailure),
                "INVALID_CONNECTION_PROFILE_CONFIG" => {
                    Some(Self::InvalidConnectionProfileConfig)
                }
                "VERSION_INCOMPATIBILITY" => Some(Self::VersionIncompatibility),
                "CONNECTION_PROFILE_TYPES_INCOMPATIBILITY" => {
                    Some(Self::ConnectionProfileTypesIncompatibility)
                }
                "NO_PGLOGICAL_INSTALLED" => Some(Self::NoPglogicalInstalled),
                "PGLOGICAL_NODE_ALREADY_EXISTS" => Some(Self::PglogicalNodeAlreadyExists),
                "INVALID_WAL_LEVEL" => Some(Self::InvalidWalLevel),
                "INVALID_SHARED_PRELOAD_LIBRARY" => {
                    Some(Self::InvalidSharedPreloadLibrary)
                }
                "INSUFFICIENT_MAX_REPLICATION_SLOTS" => {
                    Some(Self::InsufficientMaxReplicationSlots)
                }
                "INSUFFICIENT_MAX_WAL_SENDERS" => Some(Self::InsufficientMaxWalSenders),
                "INSUFFICIENT_MAX_WORKER_PROCESSES" => {
                    Some(Self::InsufficientMaxWorkerProcesses)
                }
                "UNSUPPORTED_EXTENSIONS" => Some(Self::UnsupportedExtensions),
                "UNSUPPORTED_MIGRATION_TYPE" => Some(Self::UnsupportedMigrationType),
                "INVALID_RDS_LOGICAL_REPLICATION" => {
                    Some(Self::InvalidRdsLogicalReplication)
                }
                "UNSUPPORTED_GTID_MODE" => Some(Self::UnsupportedGtidMode),
                "UNSUPPORTED_TABLE_DEFINITION" => Some(Self::UnsupportedTableDefinition),
                "UNSUPPORTED_DEFINER" => Some(Self::UnsupportedDefiner),
                "CANT_RESTART_RUNNING_MIGRATION" => {
                    Some(Self::CantRestartRunningMigration)
                }
                "SOURCE_ALREADY_SETUP" => Some(Self::SourceAlreadySetup),
                "TABLES_WITH_LIMITED_SUPPORT" => Some(Self::TablesWithLimitedSupport),
                "UNSUPPORTED_DATABASE_LOCALE" => Some(Self::UnsupportedDatabaseLocale),
                "UNSUPPORTED_DATABASE_FDW_CONFIG" => {
                    Some(Self::UnsupportedDatabaseFdwConfig)
                }
                "ERROR_RDBMS" => Some(Self::ErrorRdbms),
                "SOURCE_SIZE_EXCEEDS_THRESHOLD" => Some(Self::SourceSizeExceedsThreshold),
                "EXISTING_CONFLICTING_DATABASES" => {
                    Some(Self::ExistingConflictingDatabases)
                }
                "PARALLEL_IMPORT_INSUFFICIENT_PRIVILEGE" => {
                    Some(Self::ParallelImportInsufficientPrivilege)
                }
                _ => None,
            }
        }
    }
}
/// The PrivateConnection resource is used to establish private connectivity
/// with the customer's network.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateConnection {
    /// The name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The create time of the resource.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last update time of the resource.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The resource labels for private connections to use to annotate any related
    /// underlying resources such as Compute Engine VMs. An object containing a
    /// list of "key": "value" pairs.
    ///
    /// Example: `{ "name": "wrench", "mass": "1.3kg", "count": "3" }`.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The private connection display name.
    #[prost(string, tag = "5")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. The state of the private connection.
    #[prost(enumeration = "private_connection::State", tag = "6")]
    pub state: i32,
    /// Output only. The error details in case of state FAILED.
    #[prost(message, optional, tag = "7")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
    #[prost(oneof = "private_connection::Connectivity", tags = "100")]
    pub connectivity: ::core::option::Option<private_connection::Connectivity>,
}
/// Nested message and enum types in `PrivateConnection`.
pub mod private_connection {
    /// Private Connection state.
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
        Unspecified = 0,
        /// The private connection is in creation state - creating resources.
        Creating = 1,
        /// The private connection has been created with all of its resources.
        Created = 2,
        /// The private connection creation has failed.
        Failed = 3,
        /// The private connection is being deleted.
        Deleting = 4,
        /// Delete request has failed, resource is in invalid state.
        FailedToDelete = 5,
        /// The private connection has been deleted.
        Deleted = 6,
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
                State::Created => "CREATED",
                State::Failed => "FAILED",
                State::Deleting => "DELETING",
                State::FailedToDelete => "FAILED_TO_DELETE",
                State::Deleted => "DELETED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "CREATED" => Some(Self::Created),
                "FAILED" => Some(Self::Failed),
                "DELETING" => Some(Self::Deleting),
                "FAILED_TO_DELETE" => Some(Self::FailedToDelete),
                "DELETED" => Some(Self::Deleted),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Connectivity {
        /// VPC peering configuration.
        #[prost(message, tag = "100")]
        VpcPeeringConfig(super::VpcPeeringConfig),
    }
}
/// The VPC peering configuration is used to create VPC peering with the
/// consumer's VPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VpcPeeringConfig {
    /// Required. Fully qualified name of the VPC that Database Migration Service
    /// will peer to.
    #[prost(string, tag = "1")]
    pub vpc_name: ::prost::alloc::string::String,
    /// Required. A free subnet for peering. (CIDR of /29)
    #[prost(string, tag = "2")]
    pub subnet: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NetworkArchitecture {
    Unspecified = 0,
    /// Instance is in Cloud SQL's old producer network architecture.
    OldCsqlProducer = 1,
    /// Instance is in Cloud SQL's new producer network architecture.
    NewCsqlProducer = 2,
}
impl NetworkArchitecture {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            NetworkArchitecture::Unspecified => "NETWORK_ARCHITECTURE_UNSPECIFIED",
            NetworkArchitecture::OldCsqlProducer => {
                "NETWORK_ARCHITECTURE_OLD_CSQL_PRODUCER"
            }
            NetworkArchitecture::NewCsqlProducer => {
                "NETWORK_ARCHITECTURE_NEW_CSQL_PRODUCER"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NETWORK_ARCHITECTURE_UNSPECIFIED" => Some(Self::Unspecified),
            "NETWORK_ARCHITECTURE_OLD_CSQL_PRODUCER" => Some(Self::OldCsqlProducer),
            "NETWORK_ARCHITECTURE_NEW_CSQL_PRODUCER" => Some(Self::NewCsqlProducer),
            _ => None,
        }
    }
}
/// The database engine types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DatabaseEngine {
    /// The source database engine of the migration job is unknown.
    Unspecified = 0,
    /// The source engine is MySQL.
    Mysql = 1,
    /// The source engine is PostgreSQL.
    Postgresql = 2,
    /// The source engine is Oracle.
    Oracle = 4,
}
impl DatabaseEngine {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DatabaseEngine::Unspecified => "DATABASE_ENGINE_UNSPECIFIED",
            DatabaseEngine::Mysql => "MYSQL",
            DatabaseEngine::Postgresql => "POSTGRESQL",
            DatabaseEngine::Oracle => "ORACLE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DATABASE_ENGINE_UNSPECIFIED" => Some(Self::Unspecified),
            "MYSQL" => Some(Self::Mysql),
            "POSTGRESQL" => Some(Self::Postgresql),
            "ORACLE" => Some(Self::Oracle),
            _ => None,
        }
    }
}
/// The database providers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DatabaseProvider {
    /// The database provider is unknown.
    Unspecified = 0,
    /// CloudSQL runs the database.
    Cloudsql = 1,
    /// RDS runs the database.
    Rds = 2,
    /// Amazon Aurora.
    Aurora = 3,
    /// AlloyDB.
    Alloydb = 4,
}
impl DatabaseProvider {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DatabaseProvider::Unspecified => "DATABASE_PROVIDER_UNSPECIFIED",
            DatabaseProvider::Cloudsql => "CLOUDSQL",
            DatabaseProvider::Rds => "RDS",
            DatabaseProvider::Aurora => "AURORA",
            DatabaseProvider::Alloydb => "ALLOYDB",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DATABASE_PROVIDER_UNSPECIFIED" => Some(Self::Unspecified),
            "CLOUDSQL" => Some(Self::Cloudsql),
            "RDS" => Some(Self::Rds),
            "AURORA" => Some(Self::Aurora),
            "ALLOYDB" => Some(Self::Alloydb),
            _ => None,
        }
    }
}
/// The type and version of a source or destination database.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatabaseEngineInfo {
    /// Required. Engine type.
    #[prost(enumeration = "DatabaseEngine", tag = "1")]
    pub engine: i32,
    /// Required. Engine named version, for example 12.c.1.
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
}
/// The main conversion workspace resource entity.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionWorkspace {
    /// Full name of the workspace resource, in the form of:
    /// projects/{project}/locations/{location}/conversionWorkspaces/{conversion_workspace}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The source engine details.
    #[prost(message, optional, tag = "2")]
    pub source: ::core::option::Option<DatabaseEngineInfo>,
    /// Required. The destination engine details.
    #[prost(message, optional, tag = "3")]
    pub destination: ::core::option::Option<DatabaseEngineInfo>,
    /// Optional. A generic list of settings for the workspace.
    /// The settings are database pair dependant and can indicate default behavior
    /// for the mapping rules engine or turn on or off specific features.
    /// Such examples can be: convert_foreign_key_to_interleave=true,
    /// skip_triggers=false, ignore_non_table_synonyms=true
    #[prost(map = "string, string", tag = "4")]
    pub global_settings: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. Whether the workspace has uncommitted changes (changes which
    /// were made after the workspace was committed).
    #[prost(bool, tag = "5")]
    pub has_uncommitted_changes: bool,
    /// Output only. The latest commit ID.
    #[prost(string, tag = "6")]
    pub latest_commit_id: ::prost::alloc::string::String,
    /// Output only. The timestamp when the workspace was committed.
    #[prost(message, optional, tag = "7")]
    pub latest_commit_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when the workspace resource was created.
    #[prost(message, optional, tag = "9")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when the workspace resource was last updated.
    #[prost(message, optional, tag = "10")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. The display name for the workspace.
    #[prost(string, tag = "11")]
    pub display_name: ::prost::alloc::string::String,
}
/// Execution log of a background job.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackgroundJobLogEntry {
    /// The background job log entry ID.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The type of job that was executed.
    #[prost(enumeration = "BackgroundJobType", tag = "2")]
    pub job_type: i32,
    /// The timestamp when the background job was started.
    #[prost(message, optional, tag = "3")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The timestamp when the background job was finished.
    #[prost(message, optional, tag = "4")]
    pub finish_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Job completion state, i.e. the final state after the job
    /// completed.
    #[prost(enumeration = "background_job_log_entry::JobCompletionState", tag = "5")]
    pub completion_state: i32,
    /// Output only. Job completion comment, such as how many entities were seeded,
    /// how many warnings were found during conversion, and similar information.
    #[prost(string, tag = "6")]
    pub completion_comment: ::prost::alloc::string::String,
    /// Output only. Whether the client requested the conversion workspace to be
    /// committed after a successful completion of the job.
    #[prost(bool, tag = "7")]
    pub request_autocommit: bool,
    #[prost(oneof = "background_job_log_entry::JobDetails", tags = "100, 101, 102, 103")]
    pub job_details: ::core::option::Option<background_job_log_entry::JobDetails>,
}
/// Nested message and enum types in `BackgroundJobLogEntry`.
pub mod background_job_log_entry {
    /// Details regarding a Seed background job.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SeedJobDetails {
        /// Output only. The connection profile which was used for the seed job.
        #[prost(string, tag = "1")]
        pub connection_profile: ::prost::alloc::string::String,
    }
    /// Details regarding an Import Rules background job.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ImportRulesJobDetails {
        /// Output only. File names used for the import rules job.
        #[prost(string, repeated, tag = "1")]
        pub files: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Output only. The requested file format.
        #[prost(enumeration = "super::ImportRulesFileFormat", tag = "2")]
        pub file_format: i32,
    }
    /// Details regarding a Convert background job.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConvertJobDetails {
        /// Output only. AIP-160 based filter used to specify the entities to convert
        #[prost(string, tag = "1")]
        pub filter: ::prost::alloc::string::String,
    }
    /// Details regarding an Apply background job.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ApplyJobDetails {
        /// Output only. The connection profile which was used for the apply job.
        #[prost(string, tag = "1")]
        pub connection_profile: ::prost::alloc::string::String,
        /// Output only. AIP-160 based filter used to specify the entities to apply
        #[prost(string, tag = "2")]
        pub filter: ::prost::alloc::string::String,
    }
    /// Final state after a job completes.
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
    pub enum JobCompletionState {
        /// The status is not specified. This state is used when job is not yet
        /// finished.
        Unspecified = 0,
        /// Success.
        Succeeded = 1,
        /// Error.
        Failed = 2,
    }
    impl JobCompletionState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                JobCompletionState::Unspecified => "JOB_COMPLETION_STATE_UNSPECIFIED",
                JobCompletionState::Succeeded => "SUCCEEDED",
                JobCompletionState::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "JOB_COMPLETION_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "SUCCEEDED" => Some(Self::Succeeded),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum JobDetails {
        /// Output only. Seed job details.
        #[prost(message, tag = "100")]
        SeedJobDetails(SeedJobDetails),
        /// Output only. Import rules job details.
        #[prost(message, tag = "101")]
        ImportRulesJobDetails(ImportRulesJobDetails),
        /// Output only. Convert job details.
        #[prost(message, tag = "102")]
        ConvertJobDetails(ConvertJobDetails),
        /// Output only. Apply job details.
        #[prost(message, tag = "103")]
        ApplyJobDetails(ApplyJobDetails),
    }
}
/// A filter defining the entities that a mapping rule should be applied to.
/// When more than one field is specified, the rule is applied only to
/// entities which match all the fields.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MappingRuleFilter {
    /// Optional. The rule should be applied to entities whose parent entity
    /// (fully qualified name) matches the given value.
    /// For example, if the rule applies to a table entity, the expected value
    /// should be a schema (schema). If the rule applies to a column or index
    /// entity, the expected value can be either a schema (schema) or a table
    /// (schema.table)
    #[prost(string, tag = "1")]
    pub parent_entity: ::prost::alloc::string::String,
    /// Optional. The rule should be applied to entities whose non-qualified name
    /// starts with the given prefix.
    #[prost(string, tag = "2")]
    pub entity_name_prefix: ::prost::alloc::string::String,
    /// Optional. The rule should be applied to entities whose non-qualified name
    /// ends with the given suffix.
    #[prost(string, tag = "3")]
    pub entity_name_suffix: ::prost::alloc::string::String,
    /// Optional. The rule should be applied to entities whose non-qualified name
    /// contains the given string.
    #[prost(string, tag = "4")]
    pub entity_name_contains: ::prost::alloc::string::String,
    /// Optional. The rule should be applied to specific entities defined by their
    /// fully qualified names.
    #[prost(string, repeated, tag = "5")]
    pub entities: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Definition of a transformation that is to be applied to a group of entities
/// in the source schema. Several such transformations can be applied to an
/// entity sequentially to define the corresponding entity in the target schema.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MappingRule {
    /// Full name of the mapping rule resource, in the form of:
    /// projects/{project}/locations/{location}/conversionWorkspaces/{set}/mappingRule/{rule}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A human readable name
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. The mapping rule state
    #[prost(enumeration = "mapping_rule::State", tag = "3")]
    pub state: i32,
    /// Required. The rule scope
    #[prost(enumeration = "DatabaseEntityType", tag = "4")]
    pub rule_scope: i32,
    /// Required. The rule filter
    #[prost(message, optional, tag = "5")]
    pub filter: ::core::option::Option<MappingRuleFilter>,
    /// Required. The order in which the rule is applied. Lower order rules are
    /// applied before higher value rules so they may end up being overridden.
    #[prost(int64, tag = "6")]
    pub rule_order: i64,
    /// Output only. The revision ID of the mapping rule.
    /// A new revision is committed whenever the mapping rule is changed in any
    /// way. The format is an 8-character hexadecimal string.
    #[prost(string, tag = "7")]
    pub revision_id: ::prost::alloc::string::String,
    /// Output only. The timestamp that the revision was created.
    #[prost(message, optional, tag = "8")]
    pub revision_create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The rule specific details.
    #[prost(
        oneof = "mapping_rule::Details",
        tags = "102, 103, 105, 106, 107, 108, 114, 115, 116, 117, 118"
    )]
    pub details: ::core::option::Option<mapping_rule::Details>,
}
/// Nested message and enum types in `MappingRule`.
pub mod mapping_rule {
    /// The current mapping rule state such as enabled, disabled or deleted.
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
        /// The state of the mapping rule is unknown.
        Unspecified = 0,
        /// The rule is enabled.
        Enabled = 1,
        /// The rule is disabled.
        Disabled = 2,
        /// The rule is logically deleted.
        Deleted = 3,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Enabled => "ENABLED",
                State::Disabled => "DISABLED",
                State::Deleted => "DELETED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ENABLED" => Some(Self::Enabled),
                "DISABLED" => Some(Self::Disabled),
                "DELETED" => Some(Self::Deleted),
                _ => None,
            }
        }
    }
    /// The rule specific details.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Details {
        /// Optional. Rule to specify how a single entity should be renamed.
        #[prost(message, tag = "102")]
        SingleEntityRename(super::SingleEntityRename),
        /// Optional. Rule to specify how multiple entities should be renamed.
        #[prost(message, tag = "103")]
        MultiEntityRename(super::MultiEntityRename),
        /// Optional. Rule to specify how multiple entities should be relocated into
        /// a different schema.
        #[prost(message, tag = "105")]
        EntityMove(super::EntityMove),
        /// Optional. Rule to specify how a single column is converted.
        #[prost(message, tag = "106")]
        SingleColumnChange(super::SingleColumnChange),
        /// Optional. Rule to specify how multiple columns should be converted to a
        /// different data type.
        #[prost(message, tag = "107")]
        MultiColumnDataTypeChange(super::MultiColumnDatatypeChange),
        /// Optional. Rule to specify how the data contained in a column should be
        /// transformed (such as trimmed, rounded, etc) provided that the data meets
        /// certain criteria.
        #[prost(message, tag = "108")]
        ConditionalColumnSetValue(super::ConditionalColumnSetValue),
        /// Optional. Rule to specify how multiple tables should be converted with an
        /// additional rowid column.
        #[prost(message, tag = "114")]
        ConvertRowidColumn(super::ConvertRowIdToColumn),
        /// Optional. Rule to specify the primary key for a table
        #[prost(message, tag = "115")]
        SetTablePrimaryKey(super::SetTablePrimaryKey),
        /// Optional. Rule to specify how a single package is converted.
        #[prost(message, tag = "116")]
        SinglePackageChange(super::SinglePackageChange),
        /// Optional. Rule to change the sql code for an entity, for example,
        /// function, procedure.
        #[prost(message, tag = "117")]
        SourceSqlChange(super::SourceSqlChange),
        /// Optional. Rule to specify the list of columns to include or exclude from
        /// a table.
        #[prost(message, tag = "118")]
        FilterTableColumns(super::FilterTableColumns),
    }
}
/// Options to configure rule type SingleEntityRename.
/// The rule is used to rename an entity.
///
/// The rule filter field can refer to only one entity.
///
/// The rule scope can be one of: Database, Schema, Table, Column, Constraint,
/// Index, View, Function, Stored Procedure, Materialized View, Sequence, UDT,
/// Synonym
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SingleEntityRename {
    /// Required. The new name of the destination entity
    #[prost(string, tag = "1")]
    pub new_name: ::prost::alloc::string::String,
}
/// Options to configure rule type MultiEntityRename.
/// The rule is used to rename multiple entities.
///
/// The rule filter field can refer to one or more entities.
///
/// The rule scope can be one of: Database, Schema, Table, Column, Constraint,
/// Index, View, Function, Stored Procedure, Materialized View, Sequence, UDT
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiEntityRename {
    /// Optional. The pattern used to generate the new entity's name. This pattern
    /// must include the characters '{name}', which will be replaced with the name
    /// of the original entity. For example, the pattern 't_{name}' for an entity
    /// name jobs would be converted to 't_jobs'.
    ///
    /// If unspecified, the default value for this field is '{name}'
    #[prost(string, tag = "1")]
    pub new_name_pattern: ::prost::alloc::string::String,
    /// Optional. Additional transformation that can be done on the source entity
    /// name before it is being used by the new_name_pattern, for example lower
    /// case. If no transformation is desired, use NO_TRANSFORMATION
    #[prost(enumeration = "EntityNameTransformation", tag = "2")]
    pub source_name_transformation: i32,
}
/// Options to configure rule type EntityMove.
/// The rule is used to move an entity to a new schema.
///
/// The rule filter field can refer to one or more entities.
///
/// The rule scope can be one of: Table, Column, Constraint, Index, View,
/// Function, Stored Procedure, Materialized View, Sequence, UDT
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityMove {
    /// Required. The new schema
    #[prost(string, tag = "1")]
    pub new_schema: ::prost::alloc::string::String,
}
/// Options to configure rule type SingleColumnChange.
/// The rule is used to change the properties of a column.
///
/// The rule filter field can refer to one entity.
///
/// The rule scope can be one of: Column.
///
/// When using this rule, if a field is not specified than the destination
/// column's configuration will be the same as the one in the source column..
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SingleColumnChange {
    /// Optional. Column data type name.
    #[prost(string, tag = "1")]
    pub data_type: ::prost::alloc::string::String,
    /// Optional. Charset override - instead of table level charset.
    #[prost(string, tag = "2")]
    pub charset: ::prost::alloc::string::String,
    /// Optional. Collation override - instead of table level collation.
    #[prost(string, tag = "3")]
    pub collation: ::prost::alloc::string::String,
    /// Optional. Column length - e.g. 50 as in varchar (50) - when relevant.
    #[prost(int64, tag = "4")]
    pub length: i64,
    /// Optional. Column precision - e.g. 8 as in double (8,2) - when relevant.
    #[prost(int32, tag = "5")]
    pub precision: i32,
    /// Optional. Column scale - e.g. 2 as in double (8,2) - when relevant.
    #[prost(int32, tag = "6")]
    pub scale: i32,
    /// Optional. Column fractional seconds precision - e.g. 2 as in timestamp (2)
    /// - when relevant.
    #[prost(int32, tag = "7")]
    pub fractional_seconds_precision: i32,
    /// Optional. Is the column of array type.
    #[prost(bool, tag = "8")]
    pub array: bool,
    /// Optional. The length of the array, only relevant if the column type is an
    /// array.
    #[prost(int32, tag = "9")]
    pub array_length: i32,
    /// Optional. Is the column nullable.
    #[prost(bool, tag = "10")]
    pub nullable: bool,
    /// Optional. Is the column auto-generated/identity.
    #[prost(bool, tag = "11")]
    pub auto_generated: bool,
    /// Optional. Is the column a UDT (User-defined Type).
    #[prost(bool, tag = "12")]
    pub udt: bool,
    /// Optional. Custom engine specific features.
    #[prost(message, optional, tag = "13")]
    pub custom_features: ::core::option::Option<::prost_types::Struct>,
    /// Optional. Specifies the list of values allowed in the column.
    #[prost(string, repeated, tag = "14")]
    pub set_values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. Comment associated with the column.
    #[prost(string, tag = "15")]
    pub comment: ::prost::alloc::string::String,
}
/// Options to configure rule type MultiColumnDatatypeChange.
/// The rule is used to change the data type and associated properties of
/// multiple columns at once.
///
/// The rule filter field can refer to one or more entities.
///
/// The rule scope can be one of:Column.
///
/// This rule requires additional filters to be specified beyond the basic rule
/// filter field, which is the source data type, but the rule supports additional
/// filtering capabilities such as the minimum and maximum field length. All
/// additional filters which are specified are required to be met in order for
/// the rule to be applied (logical AND between the fields).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiColumnDatatypeChange {
    /// Required. Filter on source data type.
    #[prost(string, tag = "1")]
    pub source_data_type_filter: ::prost::alloc::string::String,
    /// Required. New data type.
    #[prost(string, tag = "2")]
    pub new_data_type: ::prost::alloc::string::String,
    /// Optional. Column length - e.g. varchar (50) - if not specified and relevant
    /// uses the source column length.
    #[prost(int64, tag = "3")]
    pub override_length: i64,
    /// Optional. Column scale - when relevant - if not specified and relevant
    /// uses the source column scale.
    #[prost(int32, tag = "4")]
    pub override_scale: i32,
    /// Optional. Column precision - when relevant - if not specified and relevant
    /// uses the source column precision.
    #[prost(int32, tag = "5")]
    pub override_precision: i32,
    /// Optional. Column fractional seconds precision - used only for timestamp
    /// based datatypes - if not specified and relevant uses the source column
    /// fractional seconds precision.
    #[prost(int32, tag = "6")]
    pub override_fractional_seconds_precision: i32,
    /// Optional. Custom engine specific features.
    #[prost(message, optional, tag = "7")]
    pub custom_features: ::core::option::Option<::prost_types::Struct>,
    /// Filter on source column parameters.
    #[prost(oneof = "multi_column_datatype_change::SourceFilter", tags = "100, 101")]
    pub source_filter: ::core::option::Option<
        multi_column_datatype_change::SourceFilter,
    >,
}
/// Nested message and enum types in `MultiColumnDatatypeChange`.
pub mod multi_column_datatype_change {
    /// Filter on source column parameters.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SourceFilter {
        /// Optional. Filter for text-based data types like varchar.
        #[prost(message, tag = "100")]
        SourceTextFilter(super::SourceTextFilter),
        /// Optional. Filter for fixed point number data types such as
        /// NUMERIC/NUMBER.
        #[prost(message, tag = "101")]
        SourceNumericFilter(super::SourceNumericFilter),
    }
}
/// Filter for text-based data types like varchar.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceTextFilter {
    /// Optional. The filter will match columns with length greater than or equal
    /// to this number.
    #[prost(int64, tag = "1")]
    pub source_min_length_filter: i64,
    /// Optional. The filter will match columns with length smaller than or equal
    /// to this number.
    #[prost(int64, tag = "2")]
    pub source_max_length_filter: i64,
}
/// Filter for fixed point number data types such as NUMERIC/NUMBER
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceNumericFilter {
    /// Optional. The filter will match columns with scale greater than or equal to
    /// this number.
    #[prost(int32, tag = "1")]
    pub source_min_scale_filter: i32,
    /// Optional. The filter will match columns with scale smaller than or equal to
    /// this number.
    #[prost(int32, tag = "2")]
    pub source_max_scale_filter: i32,
    /// Optional. The filter will match columns with precision greater than or
    /// equal to this number.
    #[prost(int32, tag = "3")]
    pub source_min_precision_filter: i32,
    /// Optional. The filter will match columns with precision smaller than or
    /// equal to this number.
    #[prost(int32, tag = "4")]
    pub source_max_precision_filter: i32,
    /// Required. Enum to set the option defining the datatypes numeric filter has
    /// to be applied to
    #[prost(enumeration = "NumericFilterOption", tag = "5")]
    pub numeric_filter_option: i32,
}
/// Options to configure rule type ConditionalColumnSetValue.
/// The rule is used to transform the data which is being replicated/migrated.
///
/// The rule filter field can refer to one or more entities.
///
/// The rule scope can be one of: Column.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConditionalColumnSetValue {
    /// Required. Description of data transformation during migration.
    #[prost(message, optional, tag = "1")]
    pub value_transformation: ::core::option::Option<ValueTransformation>,
    /// Optional. Custom engine specific features.
    #[prost(message, optional, tag = "2")]
    pub custom_features: ::core::option::Option<::prost_types::Struct>,
    #[prost(oneof = "conditional_column_set_value::SourceFilter", tags = "100, 101")]
    pub source_filter: ::core::option::Option<
        conditional_column_set_value::SourceFilter,
    >,
}
/// Nested message and enum types in `ConditionalColumnSetValue`.
pub mod conditional_column_set_value {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SourceFilter {
        /// Optional. Optional filter on source column length. Used for text based
        /// data types like varchar.
        #[prost(message, tag = "100")]
        SourceTextFilter(super::SourceTextFilter),
        /// Optional. Optional filter on source column precision and scale. Used for
        /// fixed point numbers such as NUMERIC/NUMBER data types.
        #[prost(message, tag = "101")]
        SourceNumericFilter(super::SourceNumericFilter),
    }
}
/// Description of data transformation during migration as part of the
/// ConditionalColumnSetValue.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValueTransformation {
    #[prost(oneof = "value_transformation::Filter", tags = "100, 101, 102, 103")]
    pub filter: ::core::option::Option<value_transformation::Filter>,
    #[prost(
        oneof = "value_transformation::Action",
        tags = "200, 201, 202, 203, 204, 205"
    )]
    pub action: ::core::option::Option<value_transformation::Action>,
}
/// Nested message and enum types in `ValueTransformation`.
pub mod value_transformation {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Filter {
        /// Optional. Value is null
        #[prost(message, tag = "100")]
        IsNull(()),
        /// Optional. Value is found in the specified list.
        #[prost(message, tag = "101")]
        ValueList(super::ValueListFilter),
        /// Optional. Filter on relation between source value and compare value of
        /// type integer.
        #[prost(message, tag = "102")]
        IntComparison(super::IntComparisonFilter),
        /// Optional. Filter on relation between source value and compare value of
        /// type double.
        #[prost(message, tag = "103")]
        DoubleComparison(super::DoubleComparisonFilter),
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Action {
        /// Optional. Set to null
        #[prost(message, tag = "200")]
        AssignNull(()),
        /// Optional. Set to a specific value (value is converted to fit the target
        /// data type)
        #[prost(message, tag = "201")]
        AssignSpecificValue(super::AssignSpecificValue),
        /// Optional. Set to min_value - if integer or numeric, will use
        /// int.minvalue, etc
        #[prost(message, tag = "202")]
        AssignMinValue(()),
        /// Optional. Set to max_value - if integer or numeric, will use
        /// int.maxvalue, etc
        #[prost(message, tag = "203")]
        AssignMaxValue(()),
        /// Optional. Allows the data to change scale
        #[prost(message, tag = "204")]
        RoundScale(super::RoundToScale),
        /// Optional. Applies a hash function on the data
        #[prost(message, tag = "205")]
        ApplyHash(super::ApplyHash),
    }
}
/// Options to configure rule type ConvertROWIDToColumn.
/// The rule is used to add column rowid to destination tables based on an Oracle
/// rowid function/property.
///
/// The rule filter field can refer to one or more entities.
///
/// The rule scope can be one of: Table.
///
/// This rule requires additional filter to be specified beyond the basic rule
/// filter field, which is whether or not to work on tables which already have a
/// primary key defined.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConvertRowIdToColumn {
    /// Required. Only work on tables without primary key defined
    #[prost(bool, tag = "1")]
    pub only_if_no_primary_key: bool,
}
/// Options to configure rule type SetTablePrimaryKey.
/// The rule is used to specify the columns and name to configure/alter the
/// primary key of a table.
///
/// The rule filter field can refer to one entity.
///
/// The rule scope can be one of: Table.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetTablePrimaryKey {
    /// Required. List of column names for the primary key
    #[prost(string, repeated, tag = "1")]
    pub primary_key_columns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. Name for the primary key
    #[prost(string, tag = "2")]
    pub primary_key: ::prost::alloc::string::String,
}
/// Options to configure rule type SinglePackageChange.
/// The rule is used to alter the sql code for a package entities.
///
/// The rule filter field can refer to one entity.
///
/// The rule scope can be: Package
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SinglePackageChange {
    /// Optional. Sql code for package description
    #[prost(string, tag = "1")]
    pub package_description: ::prost::alloc::string::String,
    /// Optional. Sql code for package body
    #[prost(string, tag = "2")]
    pub package_body: ::prost::alloc::string::String,
}
/// Options to configure rule type SourceSqlChange.
/// The rule is used to alter the sql code for database entities.
///
/// The rule filter field can refer to one entity.
///
/// The rule scope can be: StoredProcedure, Function, Trigger, View
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceSqlChange {
    /// Required. Sql code for source (stored procedure, function, trigger or view)
    #[prost(string, tag = "1")]
    pub sql_code: ::prost::alloc::string::String,
}
/// Options to configure rule type FilterTableColumns.
/// The rule is used to filter the list of columns to include or exclude from a
/// table.
///
/// The rule filter field can refer to one entity.
///
/// The rule scope can be: Table
///
/// Only one of the two lists can be specified for the rule.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterTableColumns {
    /// Optional. List of columns to be included for a particular table.
    #[prost(string, repeated, tag = "1")]
    pub include_columns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. List of columns to be excluded for a particular table.
    #[prost(string, repeated, tag = "2")]
    pub exclude_columns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A list of values to filter by in ConditionalColumnSetValue
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValueListFilter {
    /// Required. Indicates whether the filter matches rows with values that are
    /// present in the list or those with values not present in it.
    #[prost(enumeration = "ValuePresentInList", tag = "1")]
    pub value_present_list: i32,
    /// Required. The list to be used to filter by
    #[prost(string, repeated, tag = "2")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. Whether to ignore case when filtering by values. Defaults to
    /// false
    #[prost(bool, tag = "3")]
    pub ignore_case: bool,
}
/// Filter based on relation between source value and compare value of type
/// integer in ConditionalColumnSetValue
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntComparisonFilter {
    /// Required. Relation between source value and compare value
    #[prost(enumeration = "ValueComparison", tag = "1")]
    pub value_comparison: i32,
    /// Required. Integer compare value to be used
    #[prost(int64, tag = "2")]
    pub value: i64,
}
/// Filter based on relation between source
/// value and compare value of type double in ConditionalColumnSetValue
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DoubleComparisonFilter {
    /// Required. Relation between source value and compare value
    #[prost(enumeration = "ValueComparison", tag = "1")]
    pub value_comparison: i32,
    /// Required. Double compare value to be used
    #[prost(double, tag = "2")]
    pub value: f64,
}
/// Set to a specific value (value is converted to fit the target data type)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssignSpecificValue {
    /// Required. Specific value to be assigned
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
/// Apply a hash function on the value.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyHash {
    #[prost(oneof = "apply_hash::HashFunction", tags = "100")]
    pub hash_function: ::core::option::Option<apply_hash::HashFunction>,
}
/// Nested message and enum types in `ApplyHash`.
pub mod apply_hash {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum HashFunction {
        /// Optional. Generate UUID from the data's byte array
        #[prost(message, tag = "100")]
        UuidFromBytes(()),
    }
}
/// This allows the data to change scale, for example if the source is 2 digits
/// after the decimal point, specify round to scale value = 2. If for example the
/// value needs to be converted to an integer, use round to scale value = 0.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoundToScale {
    /// Required. Scale value to be used
    #[prost(int32, tag = "1")]
    pub scale: i32,
}
/// The base entity type for all the database related entities.
/// The message contains the entity name, the name of its parent, the entity
/// type, and the specific details per entity type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatabaseEntity {
    /// The short name (e.g. table name) of the entity.
    #[prost(string, tag = "1")]
    pub short_name: ::prost::alloc::string::String,
    /// The full name of the parent entity (e.g. schema name).
    #[prost(string, tag = "2")]
    pub parent_entity: ::prost::alloc::string::String,
    /// The type of tree the entity belongs to.
    #[prost(enumeration = "database_entity::TreeType", tag = "3")]
    pub tree: i32,
    /// The type of the database entity (table, view, index, ...).
    #[prost(enumeration = "DatabaseEntityType", tag = "4")]
    pub entity_type: i32,
    /// Details about entity mappings.
    /// For source tree entities, this holds the draft entities which were
    /// generated by the mapping rules.
    /// For draft tree entities, this holds the source entities which were
    /// converted to form the draft entity.
    /// Destination entities will have no mapping details.
    #[prost(message, repeated, tag = "5")]
    pub mappings: ::prost::alloc::vec::Vec<EntityMapping>,
    /// Details about the entity DDL script. Multiple DDL scripts are provided for
    /// child entities such as a table entity will have one DDL for the table with
    /// additional DDLs for each index, constraint and such.
    #[prost(message, repeated, tag = "6")]
    pub entity_ddl: ::prost::alloc::vec::Vec<EntityDdl>,
    /// Details about the various issues found for the entity.
    #[prost(message, repeated, tag = "7")]
    pub issues: ::prost::alloc::vec::Vec<EntityIssue>,
    /// The specific body for each entity type.
    #[prost(
        oneof = "database_entity::EntityBody",
        tags = "101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111"
    )]
    pub entity_body: ::core::option::Option<database_entity::EntityBody>,
}
/// Nested message and enum types in `DatabaseEntity`.
pub mod database_entity {
    /// The type of database entities tree.
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
    pub enum TreeType {
        /// Tree type unspecified.
        Unspecified = 0,
        /// Tree of entities loaded from a source database.
        Source = 1,
        /// Tree of entities converted from the source tree using the mapping rules.
        Draft = 2,
        /// Tree of entities observed on the destination database.
        Destination = 3,
    }
    impl TreeType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TreeType::Unspecified => "TREE_TYPE_UNSPECIFIED",
                TreeType::Source => "SOURCE",
                TreeType::Draft => "DRAFT",
                TreeType::Destination => "DESTINATION",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TREE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "SOURCE" => Some(Self::Source),
                "DRAFT" => Some(Self::Draft),
                "DESTINATION" => Some(Self::Destination),
                _ => None,
            }
        }
    }
    /// The specific body for each entity type.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum EntityBody {
        /// Database.
        #[prost(message, tag = "101")]
        Database(super::DatabaseInstanceEntity),
        /// Schema.
        #[prost(message, tag = "102")]
        Schema(super::SchemaEntity),
        /// Table.
        #[prost(message, tag = "103")]
        Table(super::TableEntity),
        /// View.
        #[prost(message, tag = "104")]
        View(super::ViewEntity),
        /// Sequence.
        #[prost(message, tag = "105")]
        Sequence(super::SequenceEntity),
        /// Stored procedure.
        #[prost(message, tag = "106")]
        StoredProcedure(super::StoredProcedureEntity),
        /// Function.
        #[prost(message, tag = "107")]
        DatabaseFunction(super::FunctionEntity),
        /// Synonym.
        #[prost(message, tag = "108")]
        Synonym(super::SynonymEntity),
        /// Package.
        #[prost(message, tag = "109")]
        DatabasePackage(super::PackageEntity),
        /// UDT.
        #[prost(message, tag = "110")]
        Udt(super::UdtEntity),
        /// Materialized view.
        #[prost(message, tag = "111")]
        MaterializedView(super::MaterializedViewEntity),
    }
}
/// DatabaseInstance acts as a parent entity to other database entities.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatabaseInstanceEntity {
    /// Custom engine specific features.
    #[prost(message, optional, tag = "1")]
    pub custom_features: ::core::option::Option<::prost_types::Struct>,
}
/// Schema typically has no parent entity, but can have a parent entity
/// DatabaseInstance (for database engines which support it).  For some database
/// engines, the terms  schema and user can be used interchangeably when they
/// refer to a namespace or a collection of other database entities. Can store
/// additional information which is schema specific.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SchemaEntity {
    /// Custom engine specific features.
    #[prost(message, optional, tag = "1")]
    pub custom_features: ::core::option::Option<::prost_types::Struct>,
}
/// Table's parent is a schema.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableEntity {
    /// Table columns.
    #[prost(message, repeated, tag = "1")]
    pub columns: ::prost::alloc::vec::Vec<ColumnEntity>,
    /// Table constraints.
    #[prost(message, repeated, tag = "2")]
    pub constraints: ::prost::alloc::vec::Vec<ConstraintEntity>,
    /// Table indices.
    #[prost(message, repeated, tag = "3")]
    pub indices: ::prost::alloc::vec::Vec<IndexEntity>,
    /// Table triggers.
    #[prost(message, repeated, tag = "4")]
    pub triggers: ::prost::alloc::vec::Vec<TriggerEntity>,
    /// Custom engine specific features.
    #[prost(message, optional, tag = "5")]
    pub custom_features: ::core::option::Option<::prost_types::Struct>,
    /// Comment associated with the table.
    #[prost(string, tag = "6")]
    pub comment: ::prost::alloc::string::String,
}
/// Column is not used as an independent entity, it is retrieved as part of a
/// Table entity.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ColumnEntity {
    /// Column name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Column data type.
    #[prost(string, tag = "2")]
    pub data_type: ::prost::alloc::string::String,
    /// Charset override - instead of table level charset.
    #[prost(string, tag = "3")]
    pub charset: ::prost::alloc::string::String,
    /// Collation override - instead of table level collation.
    #[prost(string, tag = "4")]
    pub collation: ::prost::alloc::string::String,
    /// Column length - e.g. varchar (50).
    #[prost(int64, tag = "5")]
    pub length: i64,
    /// Column precision - when relevant.
    #[prost(int32, tag = "6")]
    pub precision: i32,
    /// Column scale - when relevant.
    #[prost(int32, tag = "7")]
    pub scale: i32,
    /// Column fractional second precision - used for timestamp based datatypes.
    #[prost(int32, tag = "8")]
    pub fractional_seconds_precision: i32,
    /// Is the column of array type.
    #[prost(bool, tag = "9")]
    pub array: bool,
    /// If the column is array, of which length.
    #[prost(int32, tag = "10")]
    pub array_length: i32,
    /// Is the column nullable.
    #[prost(bool, tag = "11")]
    pub nullable: bool,
    /// Is the column auto-generated/identity.
    #[prost(bool, tag = "12")]
    pub auto_generated: bool,
    /// Is the column a UDT.
    #[prost(bool, tag = "13")]
    pub udt: bool,
    /// Custom engine specific features.
    #[prost(message, optional, tag = "14")]
    pub custom_features: ::core::option::Option<::prost_types::Struct>,
    /// Specifies the list of values allowed in the column.
    /// Only used for set data type.
    #[prost(string, repeated, tag = "15")]
    pub set_values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Comment associated with the column.
    #[prost(string, tag = "16")]
    pub comment: ::prost::alloc::string::String,
    /// Column order in the table.
    #[prost(int32, tag = "17")]
    pub ordinal_position: i32,
    /// Default value of the column.
    #[prost(string, tag = "18")]
    pub default_value: ::prost::alloc::string::String,
}
/// Constraint is not used as an independent entity, it is retrieved
/// as part of another entity such as Table or View.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConstraintEntity {
    /// The name of the table constraint.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Type of constraint, for example unique, primary key, foreign key (currently
    /// only primary key is supported).
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
    /// Table columns used as part of the Constraint, for example primary key
    /// constraint should list the columns which constitutes the key.
    #[prost(string, repeated, tag = "3")]
    pub table_columns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Custom engine specific features.
    #[prost(message, optional, tag = "4")]
    pub custom_features: ::core::option::Option<::prost_types::Struct>,
    /// Reference columns which may be associated with the constraint. For example,
    /// if the constraint is a FOREIGN_KEY, this represents the list of full names
    /// of referenced columns by the foreign key.
    #[prost(string, repeated, tag = "5")]
    pub reference_columns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Reference table which may be associated with the constraint. For example,
    /// if the constraint is a FOREIGN_KEY, this represents the list of full name
    /// of the referenced table by the foreign key.
    #[prost(string, tag = "6")]
    pub reference_table: ::prost::alloc::string::String,
    /// Table which is associated with the constraint. In case the constraint
    /// is defined on a table, this field is left empty as this information is
    /// stored in parent_name. However, if constraint is defined on a view, this
    /// field stores the table name on which the view is defined.
    #[prost(string, tag = "7")]
    pub table_name: ::prost::alloc::string::String,
}
/// Index is not used as an independent entity, it is retrieved as part of a
/// Table entity.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexEntity {
    /// The name of the index.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Type of index, for example B-TREE.
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
    /// Table columns used as part of the Index, for example B-TREE index should
    /// list the columns which constitutes the index.
    #[prost(string, repeated, tag = "3")]
    pub table_columns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Boolean value indicating whether the index is unique.
    #[prost(bool, tag = "4")]
    pub unique: bool,
    /// Custom engine specific features.
    #[prost(message, optional, tag = "5")]
    pub custom_features: ::core::option::Option<::prost_types::Struct>,
}
/// Trigger is not used as an independent entity, it is retrieved as part of a
/// Table entity.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TriggerEntity {
    /// The name of the trigger.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The DML, DDL, or database events that fire the trigger, for example
    /// INSERT, UPDATE.
    #[prost(string, repeated, tag = "2")]
    pub triggering_events: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Indicates when the trigger fires, for example BEFORE STATEMENT, AFTER EACH
    /// ROW.
    #[prost(string, tag = "3")]
    pub trigger_type: ::prost::alloc::string::String,
    /// The SQL code which creates the trigger.
    #[prost(string, tag = "4")]
    pub sql_code: ::prost::alloc::string::String,
    /// Custom engine specific features.
    #[prost(message, optional, tag = "5")]
    pub custom_features: ::core::option::Option<::prost_types::Struct>,
}
/// View's parent is a schema.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewEntity {
    /// The SQL code which creates the view.
    #[prost(string, tag = "1")]
    pub sql_code: ::prost::alloc::string::String,
    /// Custom engine specific features.
    #[prost(message, optional, tag = "2")]
    pub custom_features: ::core::option::Option<::prost_types::Struct>,
    /// View constraints.
    #[prost(message, repeated, tag = "3")]
    pub constraints: ::prost::alloc::vec::Vec<ConstraintEntity>,
}
/// Sequence's parent is a schema.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SequenceEntity {
    /// Increment value for the sequence.
    #[prost(int64, tag = "1")]
    pub increment: i64,
    /// Start number for the sequence represented as bytes to accommodate large.
    /// numbers
    #[prost(bytes = "vec", tag = "2")]
    pub start_value: ::prost::alloc::vec::Vec<u8>,
    /// Maximum number for the sequence represented as bytes to accommodate large.
    /// numbers
    #[prost(bytes = "vec", tag = "3")]
    pub max_value: ::prost::alloc::vec::Vec<u8>,
    /// Minimum number for the sequence represented as bytes to accommodate large.
    /// numbers
    #[prost(bytes = "vec", tag = "4")]
    pub min_value: ::prost::alloc::vec::Vec<u8>,
    /// Indicates whether the sequence value should cycle through.
    #[prost(bool, tag = "5")]
    pub cycle: bool,
    /// Indicates number of entries to cache / precreate.
    #[prost(int64, tag = "6")]
    pub cache: i64,
    /// Custom engine specific features.
    #[prost(message, optional, tag = "7")]
    pub custom_features: ::core::option::Option<::prost_types::Struct>,
}
/// Stored procedure's parent is a schema.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoredProcedureEntity {
    /// The SQL code which creates the stored procedure.
    #[prost(string, tag = "1")]
    pub sql_code: ::prost::alloc::string::String,
    /// Custom engine specific features.
    #[prost(message, optional, tag = "2")]
    pub custom_features: ::core::option::Option<::prost_types::Struct>,
}
/// Function's parent is a schema.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunctionEntity {
    /// The SQL code which creates the function.
    #[prost(string, tag = "1")]
    pub sql_code: ::prost::alloc::string::String,
    /// Custom engine specific features.
    #[prost(message, optional, tag = "2")]
    pub custom_features: ::core::option::Option<::prost_types::Struct>,
}
/// MaterializedView's parent is a schema.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaterializedViewEntity {
    /// The SQL code which creates the view.
    #[prost(string, tag = "1")]
    pub sql_code: ::prost::alloc::string::String,
    /// Custom engine specific features.
    #[prost(message, optional, tag = "2")]
    pub custom_features: ::core::option::Option<::prost_types::Struct>,
}
/// Synonym's parent is a schema.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SynonymEntity {
    /// The name of the entity for which the synonym is being created (the source).
    #[prost(string, tag = "1")]
    pub source_entity: ::prost::alloc::string::String,
    /// The type of the entity for which the synonym is being created
    /// (usually a table or a sequence).
    #[prost(enumeration = "DatabaseEntityType", tag = "2")]
    pub source_type: i32,
    /// Custom engine specific features.
    #[prost(message, optional, tag = "3")]
    pub custom_features: ::core::option::Option<::prost_types::Struct>,
}
/// Package's parent is a schema.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PackageEntity {
    /// The SQL code which creates the package.
    #[prost(string, tag = "1")]
    pub package_sql_code: ::prost::alloc::string::String,
    /// The SQL code which creates the package body. If the package specification
    /// has cursors or subprograms, then the package body is mandatory.
    #[prost(string, tag = "2")]
    pub package_body: ::prost::alloc::string::String,
    /// Custom engine specific features.
    #[prost(message, optional, tag = "3")]
    pub custom_features: ::core::option::Option<::prost_types::Struct>,
}
/// UDT's parent is a schema.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UdtEntity {
    /// The SQL code which creates the udt.
    #[prost(string, tag = "1")]
    pub udt_sql_code: ::prost::alloc::string::String,
    /// The SQL code which creates the udt body.
    #[prost(string, tag = "2")]
    pub udt_body: ::prost::alloc::string::String,
    /// Custom engine specific features.
    #[prost(message, optional, tag = "3")]
    pub custom_features: ::core::option::Option<::prost_types::Struct>,
}
/// Details of the mappings of a database entity.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityMapping {
    /// Source entity full name.
    /// The source entity can also be a column, index or constraint using the
    /// same naming notation schema.table.column.
    #[prost(string, tag = "1")]
    pub source_entity: ::prost::alloc::string::String,
    /// Target entity full name.
    /// The draft entity can also include a column, index or constraint using the
    /// same naming notation schema.table.column.
    #[prost(string, tag = "2")]
    pub draft_entity: ::prost::alloc::string::String,
    /// Type of source entity.
    #[prost(enumeration = "DatabaseEntityType", tag = "4")]
    pub source_type: i32,
    /// Type of draft entity.
    #[prost(enumeration = "DatabaseEntityType", tag = "5")]
    pub draft_type: i32,
    /// Entity mapping log entries.
    /// Multiple rules can be effective and contribute changes to a converted
    /// entity, such as a rule can handle the entity name, another rule can handle
    /// an entity type. In addition, rules which did not change the entity are also
    /// logged along with the reason preventing them to do so.
    #[prost(message, repeated, tag = "3")]
    pub mapping_log: ::prost::alloc::vec::Vec<EntityMappingLogEntry>,
}
/// A single record of a rule which was used for a mapping.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityMappingLogEntry {
    /// Which rule caused this log entry.
    #[prost(string, tag = "1")]
    pub rule_id: ::prost::alloc::string::String,
    /// Rule revision ID.
    #[prost(string, tag = "2")]
    pub rule_revision_id: ::prost::alloc::string::String,
    /// Comment.
    #[prost(string, tag = "3")]
    pub mapping_comment: ::prost::alloc::string::String,
}
/// A single DDL statement for a specific entity
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityDdl {
    /// Type of DDL (Create, Alter).
    #[prost(string, tag = "1")]
    pub ddl_type: ::prost::alloc::string::String,
    /// The name of the database entity the ddl refers to.
    #[prost(string, tag = "2")]
    pub entity: ::prost::alloc::string::String,
    /// The actual ddl code.
    #[prost(string, tag = "3")]
    pub ddl: ::prost::alloc::string::String,
    /// The entity type (if the DDL is for a sub entity).
    #[prost(enumeration = "DatabaseEntityType", tag = "4")]
    pub entity_type: i32,
    /// EntityIssues found for this ddl.
    #[prost(string, repeated, tag = "100")]
    pub issue_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Issue related to the entity.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityIssue {
    /// Unique Issue ID.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The type of the issue.
    #[prost(enumeration = "entity_issue::IssueType", tag = "2")]
    pub r#type: i32,
    /// Severity of the issue
    #[prost(enumeration = "entity_issue::IssueSeverity", tag = "3")]
    pub severity: i32,
    /// Issue detailed message
    #[prost(string, tag = "4")]
    pub message: ::prost::alloc::string::String,
    /// Error/Warning code
    #[prost(string, tag = "5")]
    pub code: ::prost::alloc::string::String,
    /// The ddl which caused the issue, if relevant.
    #[prost(string, optional, tag = "6")]
    pub ddl: ::core::option::Option<::prost::alloc::string::String>,
    /// The position of the issue found, if relevant.
    #[prost(message, optional, tag = "7")]
    pub position: ::core::option::Option<entity_issue::Position>,
    /// The entity type (if the DDL is for a sub entity).
    #[prost(enumeration = "DatabaseEntityType", tag = "8")]
    pub entity_type: i32,
}
/// Nested message and enum types in `EntityIssue`.
pub mod entity_issue {
    /// Issue position.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Position {
        /// Issue line number
        #[prost(int32, tag = "1")]
        pub line: i32,
        /// Issue column number
        #[prost(int32, tag = "2")]
        pub column: i32,
        /// Issue offset
        #[prost(int32, tag = "3")]
        pub offset: i32,
        /// Issue length
        #[prost(int32, tag = "4")]
        pub length: i32,
    }
    /// Type of issue.
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
    pub enum IssueType {
        /// Unspecified issue type.
        Unspecified = 0,
        /// Issue originated from the DDL
        Ddl = 1,
        /// Issue originated during the apply process
        Apply = 2,
        /// Issue originated during the convert process
        Convert = 3,
    }
    impl IssueType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                IssueType::Unspecified => "ISSUE_TYPE_UNSPECIFIED",
                IssueType::Ddl => "ISSUE_TYPE_DDL",
                IssueType::Apply => "ISSUE_TYPE_APPLY",
                IssueType::Convert => "ISSUE_TYPE_CONVERT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ISSUE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "ISSUE_TYPE_DDL" => Some(Self::Ddl),
                "ISSUE_TYPE_APPLY" => Some(Self::Apply),
                "ISSUE_TYPE_CONVERT" => Some(Self::Convert),
                _ => None,
            }
        }
    }
    /// Severity of issue.
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
    pub enum IssueSeverity {
        /// Unspecified issue severity
        Unspecified = 0,
        /// Info
        Info = 1,
        /// Warning
        Warning = 2,
        /// Error
        Error = 3,
    }
    impl IssueSeverity {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                IssueSeverity::Unspecified => "ISSUE_SEVERITY_UNSPECIFIED",
                IssueSeverity::Info => "ISSUE_SEVERITY_INFO",
                IssueSeverity::Warning => "ISSUE_SEVERITY_WARNING",
                IssueSeverity::Error => "ISSUE_SEVERITY_ERROR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ISSUE_SEVERITY_UNSPECIFIED" => Some(Self::Unspecified),
                "ISSUE_SEVERITY_INFO" => Some(Self::Info),
                "ISSUE_SEVERITY_WARNING" => Some(Self::Warning),
                "ISSUE_SEVERITY_ERROR" => Some(Self::Error),
                _ => None,
            }
        }
    }
}
/// Enum used by ValueListFilter to indicate whether the source value is in the
/// supplied list
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ValuePresentInList {
    /// Value present in list unspecified
    Unspecified = 0,
    /// If the source value is in the supplied list at value_list
    IfValueList = 1,
    /// If the source value is not in the supplied list at value_list
    IfValueNotList = 2,
}
impl ValuePresentInList {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ValuePresentInList::Unspecified => "VALUE_PRESENT_IN_LIST_UNSPECIFIED",
            ValuePresentInList::IfValueList => "VALUE_PRESENT_IN_LIST_IF_VALUE_LIST",
            ValuePresentInList::IfValueNotList => {
                "VALUE_PRESENT_IN_LIST_IF_VALUE_NOT_LIST"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VALUE_PRESENT_IN_LIST_UNSPECIFIED" => Some(Self::Unspecified),
            "VALUE_PRESENT_IN_LIST_IF_VALUE_LIST" => Some(Self::IfValueList),
            "VALUE_PRESENT_IN_LIST_IF_VALUE_NOT_LIST" => Some(Self::IfValueNotList),
            _ => None,
        }
    }
}
/// The type of database entities supported,
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DatabaseEntityType {
    /// Unspecified database entity type.
    Unspecified = 0,
    /// Schema.
    Schema = 1,
    /// Table.
    Table = 2,
    /// Column.
    Column = 3,
    /// Constraint.
    Constraint = 4,
    /// Index.
    Index = 5,
    /// Trigger.
    Trigger = 6,
    /// View.
    View = 7,
    /// Sequence.
    Sequence = 8,
    /// Stored Procedure.
    StoredProcedure = 9,
    /// Function.
    Function = 10,
    /// Synonym.
    Synonym = 11,
    /// Package.
    DatabasePackage = 12,
    /// UDT.
    Udt = 13,
    /// Materialized View.
    MaterializedView = 14,
    /// Database.
    Database = 15,
}
impl DatabaseEntityType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DatabaseEntityType::Unspecified => "DATABASE_ENTITY_TYPE_UNSPECIFIED",
            DatabaseEntityType::Schema => "DATABASE_ENTITY_TYPE_SCHEMA",
            DatabaseEntityType::Table => "DATABASE_ENTITY_TYPE_TABLE",
            DatabaseEntityType::Column => "DATABASE_ENTITY_TYPE_COLUMN",
            DatabaseEntityType::Constraint => "DATABASE_ENTITY_TYPE_CONSTRAINT",
            DatabaseEntityType::Index => "DATABASE_ENTITY_TYPE_INDEX",
            DatabaseEntityType::Trigger => "DATABASE_ENTITY_TYPE_TRIGGER",
            DatabaseEntityType::View => "DATABASE_ENTITY_TYPE_VIEW",
            DatabaseEntityType::Sequence => "DATABASE_ENTITY_TYPE_SEQUENCE",
            DatabaseEntityType::StoredProcedure => {
                "DATABASE_ENTITY_TYPE_STORED_PROCEDURE"
            }
            DatabaseEntityType::Function => "DATABASE_ENTITY_TYPE_FUNCTION",
            DatabaseEntityType::Synonym => "DATABASE_ENTITY_TYPE_SYNONYM",
            DatabaseEntityType::DatabasePackage => {
                "DATABASE_ENTITY_TYPE_DATABASE_PACKAGE"
            }
            DatabaseEntityType::Udt => "DATABASE_ENTITY_TYPE_UDT",
            DatabaseEntityType::MaterializedView => {
                "DATABASE_ENTITY_TYPE_MATERIALIZED_VIEW"
            }
            DatabaseEntityType::Database => "DATABASE_ENTITY_TYPE_DATABASE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DATABASE_ENTITY_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "DATABASE_ENTITY_TYPE_SCHEMA" => Some(Self::Schema),
            "DATABASE_ENTITY_TYPE_TABLE" => Some(Self::Table),
            "DATABASE_ENTITY_TYPE_COLUMN" => Some(Self::Column),
            "DATABASE_ENTITY_TYPE_CONSTRAINT" => Some(Self::Constraint),
            "DATABASE_ENTITY_TYPE_INDEX" => Some(Self::Index),
            "DATABASE_ENTITY_TYPE_TRIGGER" => Some(Self::Trigger),
            "DATABASE_ENTITY_TYPE_VIEW" => Some(Self::View),
            "DATABASE_ENTITY_TYPE_SEQUENCE" => Some(Self::Sequence),
            "DATABASE_ENTITY_TYPE_STORED_PROCEDURE" => Some(Self::StoredProcedure),
            "DATABASE_ENTITY_TYPE_FUNCTION" => Some(Self::Function),
            "DATABASE_ENTITY_TYPE_SYNONYM" => Some(Self::Synonym),
            "DATABASE_ENTITY_TYPE_DATABASE_PACKAGE" => Some(Self::DatabasePackage),
            "DATABASE_ENTITY_TYPE_UDT" => Some(Self::Udt),
            "DATABASE_ENTITY_TYPE_MATERIALIZED_VIEW" => Some(Self::MaterializedView),
            "DATABASE_ENTITY_TYPE_DATABASE" => Some(Self::Database),
            _ => None,
        }
    }
}
/// Entity Name Transformation Types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EntityNameTransformation {
    /// Entity name transformation unspecified.
    Unspecified = 0,
    /// No transformation.
    NoTransformation = 1,
    /// Transform to lower case.
    LowerCase = 2,
    /// Transform to upper case.
    UpperCase = 3,
    /// Transform to capitalized case.
    CapitalizedCase = 4,
}
impl EntityNameTransformation {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EntityNameTransformation::Unspecified => {
                "ENTITY_NAME_TRANSFORMATION_UNSPECIFIED"
            }
            EntityNameTransformation::NoTransformation => {
                "ENTITY_NAME_TRANSFORMATION_NO_TRANSFORMATION"
            }
            EntityNameTransformation::LowerCase => {
                "ENTITY_NAME_TRANSFORMATION_LOWER_CASE"
            }
            EntityNameTransformation::UpperCase => {
                "ENTITY_NAME_TRANSFORMATION_UPPER_CASE"
            }
            EntityNameTransformation::CapitalizedCase => {
                "ENTITY_NAME_TRANSFORMATION_CAPITALIZED_CASE"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ENTITY_NAME_TRANSFORMATION_UNSPECIFIED" => Some(Self::Unspecified),
            "ENTITY_NAME_TRANSFORMATION_NO_TRANSFORMATION" => {
                Some(Self::NoTransformation)
            }
            "ENTITY_NAME_TRANSFORMATION_LOWER_CASE" => Some(Self::LowerCase),
            "ENTITY_NAME_TRANSFORMATION_UPPER_CASE" => Some(Self::UpperCase),
            "ENTITY_NAME_TRANSFORMATION_CAPITALIZED_CASE" => Some(Self::CapitalizedCase),
            _ => None,
        }
    }
}
/// The types of jobs that can be executed in the background.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BackgroundJobType {
    /// Unspecified background job type.
    Unspecified = 0,
    /// Job to seed from the source database.
    SourceSeed = 1,
    /// Job to convert the source database into a draft of the destination
    /// database.
    Convert = 2,
    /// Job to apply the draft tree onto the destination.
    ApplyDestination = 3,
    /// Job to import and convert mapping rules from an external source such as an
    /// ora2pg config file.
    ImportRulesFile = 5,
}
impl BackgroundJobType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BackgroundJobType::Unspecified => "BACKGROUND_JOB_TYPE_UNSPECIFIED",
            BackgroundJobType::SourceSeed => "BACKGROUND_JOB_TYPE_SOURCE_SEED",
            BackgroundJobType::Convert => "BACKGROUND_JOB_TYPE_CONVERT",
            BackgroundJobType::ApplyDestination => {
                "BACKGROUND_JOB_TYPE_APPLY_DESTINATION"
            }
            BackgroundJobType::ImportRulesFile => "BACKGROUND_JOB_TYPE_IMPORT_RULES_FILE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BACKGROUND_JOB_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "BACKGROUND_JOB_TYPE_SOURCE_SEED" => Some(Self::SourceSeed),
            "BACKGROUND_JOB_TYPE_CONVERT" => Some(Self::Convert),
            "BACKGROUND_JOB_TYPE_APPLY_DESTINATION" => Some(Self::ApplyDestination),
            "BACKGROUND_JOB_TYPE_IMPORT_RULES_FILE" => Some(Self::ImportRulesFile),
            _ => None,
        }
    }
}
/// The format for the import rules file.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ImportRulesFileFormat {
    /// Unspecified rules format.
    Unspecified = 0,
    /// HarbourBridge session file.
    HarbourBridgeSessionFile = 1,
    /// Ora2Pg configuration file.
    OratopgConfigFile = 2,
}
impl ImportRulesFileFormat {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ImportRulesFileFormat::Unspecified => "IMPORT_RULES_FILE_FORMAT_UNSPECIFIED",
            ImportRulesFileFormat::HarbourBridgeSessionFile => {
                "IMPORT_RULES_FILE_FORMAT_HARBOUR_BRIDGE_SESSION_FILE"
            }
            ImportRulesFileFormat::OratopgConfigFile => {
                "IMPORT_RULES_FILE_FORMAT_ORATOPG_CONFIG_FILE"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "IMPORT_RULES_FILE_FORMAT_UNSPECIFIED" => Some(Self::Unspecified),
            "IMPORT_RULES_FILE_FORMAT_HARBOUR_BRIDGE_SESSION_FILE" => {
                Some(Self::HarbourBridgeSessionFile)
            }
            "IMPORT_RULES_FILE_FORMAT_ORATOPG_CONFIG_FILE" => {
                Some(Self::OratopgConfigFile)
            }
            _ => None,
        }
    }
}
/// Enum used by IntComparisonFilter and DoubleComparisonFilter to indicate the
/// relation between source value and compare value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ValueComparison {
    /// Value comparison unspecified.
    Unspecified = 0,
    /// Value is smaller than the Compare value.
    IfValueSmallerThan = 1,
    /// Value is smaller or equal than the Compare value.
    IfValueSmallerEqualThan = 2,
    /// Value is larger than the Compare value.
    IfValueLargerThan = 3,
    /// Value is larger or equal than the Compare value.
    IfValueLargerEqualThan = 4,
}
impl ValueComparison {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ValueComparison::Unspecified => "VALUE_COMPARISON_UNSPECIFIED",
            ValueComparison::IfValueSmallerThan => {
                "VALUE_COMPARISON_IF_VALUE_SMALLER_THAN"
            }
            ValueComparison::IfValueSmallerEqualThan => {
                "VALUE_COMPARISON_IF_VALUE_SMALLER_EQUAL_THAN"
            }
            ValueComparison::IfValueLargerThan => "VALUE_COMPARISON_IF_VALUE_LARGER_THAN",
            ValueComparison::IfValueLargerEqualThan => {
                "VALUE_COMPARISON_IF_VALUE_LARGER_EQUAL_THAN"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VALUE_COMPARISON_UNSPECIFIED" => Some(Self::Unspecified),
            "VALUE_COMPARISON_IF_VALUE_SMALLER_THAN" => Some(Self::IfValueSmallerThan),
            "VALUE_COMPARISON_IF_VALUE_SMALLER_EQUAL_THAN" => {
                Some(Self::IfValueSmallerEqualThan)
            }
            "VALUE_COMPARISON_IF_VALUE_LARGER_THAN" => Some(Self::IfValueLargerThan),
            "VALUE_COMPARISON_IF_VALUE_LARGER_EQUAL_THAN" => {
                Some(Self::IfValueLargerEqualThan)
            }
            _ => None,
        }
    }
}
/// Specifies the columns on which numeric filter needs to be applied.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NumericFilterOption {
    /// Numeric filter option unspecified
    Unspecified = 0,
    /// Numeric filter option that matches all numeric columns.
    All = 1,
    /// Numeric filter option that matches columns having numeric datatypes with
    /// specified precision and scale within the limited range of filter.
    Limit = 2,
    /// Numeric filter option that matches only the numeric columns with no
    /// precision and scale specified.
    Limitless = 3,
}
impl NumericFilterOption {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            NumericFilterOption::Unspecified => "NUMERIC_FILTER_OPTION_UNSPECIFIED",
            NumericFilterOption::All => "NUMERIC_FILTER_OPTION_ALL",
            NumericFilterOption::Limit => "NUMERIC_FILTER_OPTION_LIMIT",
            NumericFilterOption::Limitless => "NUMERIC_FILTER_OPTION_LIMITLESS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NUMERIC_FILTER_OPTION_UNSPECIFIED" => Some(Self::Unspecified),
            "NUMERIC_FILTER_OPTION_ALL" => Some(Self::All),
            "NUMERIC_FILTER_OPTION_LIMIT" => Some(Self::Limit),
            "NUMERIC_FILTER_OPTION_LIMITLESS" => Some(Self::Limitless),
            _ => None,
        }
    }
}
/// Retrieves a list of all migration jobs in a given project and location.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMigrationJobsRequest {
    /// Required. The parent which owns this collection of migrationJobs.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of migration jobs to return. The service may return
    /// fewer than this value. If unspecified, at most 50 migration jobs will be
    /// returned. The maximum value is 1000; values above 1000 are coerced to
    /// 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The nextPageToken value received in the previous call to
    /// migrationJobs.list, used in the subsequent request to retrieve the next
    /// page of results. On first call this should be left blank. When paginating,
    /// all other parameters provided to migrationJobs.list must match the call
    /// that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// A filter expression that filters migration jobs listed in the response.
    /// The expression must specify the field name, a comparison operator, and the
    /// value that you want to use for filtering. The value must be a string,
    /// a number, or a boolean. The comparison operator must be
    /// either =, !=, >, or <. For example, list migration jobs created this year
    /// by specifying **createTime %gt; 2020-01-01T00:00:00.000000000Z.**
    /// You can also filter nested fields. For example, you could specify
    /// **reverseSshConnectivity.vmIp = "1.2.3.4"** to select all migration
    /// jobs connecting through the specific SSH tunnel bastion.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Sort the results based on the migration job name.
    /// Valid values are: "name", "name asc", and "name desc".
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for 'ListMigrationJobs' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMigrationJobsResponse {
    /// The list of migration jobs objects.
    #[prost(message, repeated, tag = "1")]
    pub migration_jobs: ::prost::alloc::vec::Vec<MigrationJob>,
    /// A token which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for 'GetMigrationJob' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMigrationJobRequest {
    /// Required. Name of the migration job resource to get.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message to create a new Database Migration Service migration job
/// in the specified project and region.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMigrationJobRequest {
    /// Required. The parent which owns this collection of migration jobs.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID of the instance to create.
    #[prost(string, tag = "2")]
    pub migration_job_id: ::prost::alloc::string::String,
    /// Required. Represents a [migration
    /// job](<https://cloud.google.com/database-migration/docs/reference/rest/v1/projects.locations.migrationJobs>)
    /// object.
    #[prost(message, optional, tag = "3")]
    pub migration_job: ::core::option::Option<MigrationJob>,
    /// Optional. A unique ID used to identify the request. If the server receives
    /// two requests with the same ID, then the second request is ignored.
    ///
    /// It is recommended to always set this value to a UUID.
    ///
    /// The ID must contain only letters (a-z, A-Z), numbers (0-9), underscores
    /// (_), and hyphens (-). The maximum length is 40 characters.
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for 'UpdateMigrationJob' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMigrationJobRequest {
    /// Required. Field mask is used to specify the fields to be overwritten by the
    /// update in the conversion workspace resource.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The migration job parameters to update.
    #[prost(message, optional, tag = "2")]
    pub migration_job: ::core::option::Option<MigrationJob>,
    /// A unique ID used to identify the request. If the server receives two
    /// requests with the same ID, then the second request is ignored.
    ///
    /// It is recommended to always set this value to a UUID.
    ///
    /// The ID must contain only letters (a-z, A-Z), numbers (0-9), underscores
    /// (_), and hyphens (-). The maximum length is 40 characters.
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for 'DeleteMigrationJob' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteMigrationJobRequest {
    /// Required. Name of the migration job resource to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A unique ID used to identify the request. If the server receives two
    /// requests with the same ID, then the second request is ignored.
    ///
    /// It is recommended to always set this value to a UUID.
    ///
    /// The ID must contain only letters (a-z, A-Z), numbers (0-9), underscores
    /// (_), and hyphens (-). The maximum length is 40 characters.
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// The destination CloudSQL connection profile is always deleted with the
    /// migration job. In case of force delete, the destination CloudSQL replica
    /// database is also deleted.
    #[prost(bool, tag = "3")]
    pub force: bool,
}
/// Request message for 'StartMigrationJob' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartMigrationJobRequest {
    /// Name of the migration job resource to start.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Start the migration job without running prior configuration
    /// verification. Defaults to `false`.
    #[prost(bool, tag = "2")]
    pub skip_validation: bool,
}
/// Request message for 'StopMigrationJob' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopMigrationJobRequest {
    /// Name of the migration job resource to stop.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for 'ResumeMigrationJob' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResumeMigrationJobRequest {
    /// Name of the migration job resource to resume.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for 'PromoteMigrationJob' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PromoteMigrationJobRequest {
    /// Name of the migration job resource to promote.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for 'VerifyMigrationJob' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyMigrationJobRequest {
    /// Name of the migration job resource to verify.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Field mask is used to specify the changed fields to be verified.
    /// It will not update the migration job.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Optional. The changed migration job parameters to verify.
    /// It will not update the migration job.
    #[prost(message, optional, tag = "3")]
    pub migration_job: ::core::option::Option<MigrationJob>,
}
/// Request message for 'RestartMigrationJob' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestartMigrationJobRequest {
    /// Name of the migration job resource to restart.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Restart the migration job without running prior configuration
    /// verification. Defaults to `false`.
    #[prost(bool, tag = "2")]
    pub skip_validation: bool,
}
/// Request message for 'GenerateSshScript' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateSshScriptRequest {
    /// Name of the migration job resource to generate the SSH script.
    #[prost(string, tag = "1")]
    pub migration_job: ::prost::alloc::string::String,
    /// Required. Bastion VM Instance name to use or to create.
    #[prost(string, tag = "2")]
    pub vm: ::prost::alloc::string::String,
    /// The port that will be open on the bastion host.
    #[prost(int32, tag = "3")]
    pub vm_port: i32,
    /// The VM configuration
    #[prost(oneof = "generate_ssh_script_request::VmConfig", tags = "100, 101")]
    pub vm_config: ::core::option::Option<generate_ssh_script_request::VmConfig>,
}
/// Nested message and enum types in `GenerateSshScriptRequest`.
pub mod generate_ssh_script_request {
    /// The VM configuration
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum VmConfig {
        /// The VM creation configuration
        #[prost(message, tag = "100")]
        VmCreationConfig(super::VmCreationConfig),
        /// The VM selection configuration
        #[prost(message, tag = "101")]
        VmSelectionConfig(super::VmSelectionConfig),
    }
}
/// VM creation configuration message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VmCreationConfig {
    /// Required. VM instance machine type to create.
    #[prost(string, tag = "1")]
    pub vm_machine_type: ::prost::alloc::string::String,
    /// The Google Cloud Platform zone to create the VM in.
    #[prost(string, tag = "2")]
    pub vm_zone: ::prost::alloc::string::String,
    /// The subnet name the vm needs to be created in.
    #[prost(string, tag = "3")]
    pub subnet: ::prost::alloc::string::String,
}
/// VM selection configuration message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VmSelectionConfig {
    /// Required. The Google Cloud Platform zone the VM is located.
    #[prost(string, tag = "1")]
    pub vm_zone: ::prost::alloc::string::String,
}
/// Response message for 'GenerateSshScript' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SshScript {
    /// The ssh configuration script.
    #[prost(string, tag = "1")]
    pub script: ::prost::alloc::string::String,
}
/// Request message for 'GenerateTcpProxyScript' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateTcpProxyScriptRequest {
    /// Name of the migration job resource to generate the TCP Proxy script.
    #[prost(string, tag = "1")]
    pub migration_job: ::prost::alloc::string::String,
    /// Required. The name of the Compute instance that will host the proxy.
    #[prost(string, tag = "2")]
    pub vm_name: ::prost::alloc::string::String,
    /// Required. The type of the Compute instance that will host the proxy.
    #[prost(string, tag = "3")]
    pub vm_machine_type: ::prost::alloc::string::String,
    /// Optional. The Google Cloud Platform zone to create the VM in. The fully
    /// qualified name of the zone must be specified, including the region name,
    /// for example "us-central1-b". If not specified, uses the "-b" zone of the
    /// destination Connection Profile's region.
    #[prost(string, tag = "4")]
    pub vm_zone: ::prost::alloc::string::String,
    /// Required. The name of the subnet the Compute instance will use for private
    /// connectivity. Must be supplied in the form of
    /// projects/{project}/regions/{region}/subnetworks/{subnetwork}.
    /// Note: the region for the subnet must match the Compute instance region.
    #[prost(string, tag = "5")]
    pub vm_subnet: ::prost::alloc::string::String,
}
/// Response message for 'GenerateTcpProxyScript' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TcpProxyScript {
    /// The TCP Proxy configuration script.
    #[prost(string, tag = "1")]
    pub script: ::prost::alloc::string::String,
}
/// Request message for 'ListConnectionProfiles' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConnectionProfilesRequest {
    /// Required. The parent which owns this collection of connection profiles.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of connection profiles to return. The service may return
    /// fewer than this value. If unspecified, at most 50 connection profiles will
    /// be returned. The maximum value is 1000; values above 1000 are coerced
    /// to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListConnectionProfiles` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListConnectionProfiles`
    /// must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// A filter expression that filters connection profiles listed in the
    /// response. The expression must specify the field name, a comparison
    /// operator, and the value that you want to use for filtering. The value must
    /// be a string, a number, or a boolean. The comparison operator must be either
    /// =, !=, >, or <. For example, list connection profiles created this year by
    /// specifying **createTime %gt; 2020-01-01T00:00:00.000000000Z**. You can
    /// also filter nested fields. For example, you could specify **mySql.username
    /// = %lt;my_username%gt;** to list all connection profiles configured to
    /// connect with a specific username.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// A comma-separated list of fields to order results according to.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for 'ListConnectionProfiles' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConnectionProfilesResponse {
    /// The response list of connection profiles.
    #[prost(message, repeated, tag = "1")]
    pub connection_profiles: ::prost::alloc::vec::Vec<ConnectionProfile>,
    /// A token which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for 'GetConnectionProfile' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConnectionProfileRequest {
    /// Required. Name of the connection profile resource to get.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for 'CreateConnectionProfile' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateConnectionProfileRequest {
    /// Required. The parent which owns this collection of connection profiles.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The connection profile identifier.
    #[prost(string, tag = "2")]
    pub connection_profile_id: ::prost::alloc::string::String,
    /// Required. The create request body including the connection profile data
    #[prost(message, optional, tag = "3")]
    pub connection_profile: ::core::option::Option<ConnectionProfile>,
    /// Optional. A unique ID used to identify the request. If the server receives
    /// two requests with the same ID, then the second request is ignored.
    ///
    /// It is recommended to always set this value to a UUID.
    ///
    /// The ID must contain only letters (a-z, A-Z), numbers (0-9), underscores
    /// (_), and hyphens (-). The maximum length is 40 characters.
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. Only validate the connection profile, but don't create any
    /// resources. The default is false. Only supported for Oracle connection
    /// profiles.
    #[prost(bool, tag = "5")]
    pub validate_only: bool,
    /// Optional. Create the connection profile without validating it.
    /// The default is false.
    /// Only supported for Oracle connection profiles.
    #[prost(bool, tag = "6")]
    pub skip_validation: bool,
}
/// Request message for 'UpdateConnectionProfile' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateConnectionProfileRequest {
    /// Required. Field mask is used to specify the fields to be overwritten by the
    /// update in the conversion workspace resource.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The connection profile parameters to update.
    #[prost(message, optional, tag = "2")]
    pub connection_profile: ::core::option::Option<ConnectionProfile>,
    /// Optional. A unique ID used to identify the request. If the server receives
    /// two requests with the same ID, then the second request is ignored.
    ///
    /// It is recommended to always set this value to a UUID.
    ///
    /// The ID must contain only letters (a-z, A-Z), numbers (0-9), underscores
    /// (_), and hyphens (-). The maximum length is 40 characters.
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. Only validate the connection profile, but don't update any
    /// resources. The default is false. Only supported for Oracle connection
    /// profiles.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
    /// Optional. Update the connection profile without validating it.
    /// The default is false.
    /// Only supported for Oracle connection profiles.
    #[prost(bool, tag = "5")]
    pub skip_validation: bool,
}
/// Request message for 'DeleteConnectionProfile' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteConnectionProfileRequest {
    /// Required. Name of the connection profile resource to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A unique ID used to identify the request. If the server receives two
    /// requests with the same ID, then the second request is ignored.
    ///
    /// It is recommended to always set this value to a UUID.
    ///
    /// The ID must contain only letters (a-z, A-Z), numbers (0-9), underscores
    /// (_), and hyphens (-). The maximum length is 40 characters.
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// In case of force delete, the CloudSQL replica database is also deleted
    /// (only for CloudSQL connection profile).
    #[prost(bool, tag = "3")]
    pub force: bool,
}
/// Request message to create a new private connection in the specified project
/// and region.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePrivateConnectionRequest {
    /// Required. The parent that owns the collection of PrivateConnections.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The private connection identifier.
    #[prost(string, tag = "2")]
    pub private_connection_id: ::prost::alloc::string::String,
    /// Required. The private connection resource to create.
    #[prost(message, optional, tag = "3")]
    pub private_connection: ::core::option::Option<PrivateConnection>,
    /// Optional. A unique ID used to identify the request. If the server receives
    /// two requests with the same ID, then the second request is ignored.
    ///
    /// It is recommended to always set this value to a UUID.
    ///
    /// The ID must contain only letters (a-z, A-Z), numbers (0-9), underscores
    /// (_), and hyphens (-). The maximum length is 40 characters.
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set to true, will skip validations.
    #[prost(bool, tag = "5")]
    pub skip_validation: bool,
}
/// Request message to retrieve a list of private connections in a given project
/// and location.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPrivateConnectionsRequest {
    /// Required. The parent that owns the collection of private connections.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of private connections to return.
    /// If unspecified, at most 50 private connections that are returned.
    /// The maximum value is 1000; values above 1000 are coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Page token received from a previous `ListPrivateConnections` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// `ListPrivateConnections` must match the call that provided the page
    /// token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// A filter expression that filters private connections listed in the
    /// response. The expression must specify the field name, a comparison
    /// operator, and the value that you want to use for filtering. The value must
    /// be a string, a number, or a boolean. The comparison operator must be either
    /// =, !=, >, or <. For example, list private connections created this year by
    /// specifying **createTime %gt; 2021-01-01T00:00:00.000000000Z**.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Order by fields for the result.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for 'ListPrivateConnections' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPrivateConnectionsResponse {
    /// List of private connections.
    #[prost(message, repeated, tag = "1")]
    pub private_connections: ::prost::alloc::vec::Vec<PrivateConnection>,
    /// A token which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message to delete a private connection.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePrivateConnectionRequest {
    /// Required. The name of the private connection to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A unique ID used to identify the request. If the server receives
    /// two requests with the same ID, then the second request is ignored.
    ///
    /// It is recommended to always set this value to a UUID.
    ///
    /// The ID must contain only letters (a-z, A-Z), numbers (0-9), underscores
    /// (_), and hyphens (-). The maximum length is 40 characters.
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message to get a private connection resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPrivateConnectionRequest {
    /// Required. The name of the private connection to get.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
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
}
/// Retrieve a list of all conversion workspaces in a given project and location.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConversionWorkspacesRequest {
    /// Required. The parent which owns this collection of conversion workspaces.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of conversion workspaces to return. The service may
    /// return fewer than this value. If unspecified, at most 50 sets are returned.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The nextPageToken value received in the previous call to
    /// conversionWorkspaces.list, used in the subsequent request to retrieve the
    /// next page of results. On first call this should be left blank. When
    /// paginating, all other parameters provided to conversionWorkspaces.list must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// A filter expression that filters conversion workspaces listed in the
    /// response. The expression must specify the field name, a comparison
    /// operator, and the value that you want to use for filtering. The value must
    /// be a string, a number, or a boolean. The comparison operator must be either
    /// =, !=, >, or <. For example, list conversion workspaces created this year
    /// by specifying **createTime %gt; 2020-01-01T00:00:00.000000000Z.** You can
    /// also filter nested fields. For example, you could specify
    /// **source.version = "12.c.1"** to select all conversion workspaces with
    /// source database version equal to 12.c.1.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// Response message for 'ListConversionWorkspaces' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConversionWorkspacesResponse {
    /// The list of conversion workspace objects.
    #[prost(message, repeated, tag = "1")]
    pub conversion_workspaces: ::prost::alloc::vec::Vec<ConversionWorkspace>,
    /// A token which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for 'GetConversionWorkspace' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConversionWorkspaceRequest {
    /// Required. Name of the conversion workspace resource to get.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message to create a new Conversion Workspace
/// in the specified project and region.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateConversionWorkspaceRequest {
    /// Required. The parent which owns this collection of conversion workspaces.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID of the conversion workspace to create.
    #[prost(string, tag = "2")]
    pub conversion_workspace_id: ::prost::alloc::string::String,
    /// Required. Represents a conversion workspace object.
    #[prost(message, optional, tag = "3")]
    pub conversion_workspace: ::core::option::Option<ConversionWorkspace>,
    /// A unique ID used to identify the request. If the server receives two
    /// requests with the same ID, then the second request is ignored.
    ///
    /// It is recommended to always set this value to a UUID.
    ///
    /// The ID must contain only letters (a-z, A-Z), numbers (0-9), underscores
    /// (_), and hyphens (-). The maximum length is 40 characters.
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for 'UpdateConversionWorkspace' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateConversionWorkspaceRequest {
    /// Required. Field mask is used to specify the fields to be overwritten by the
    /// update in the conversion workspace resource.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The conversion workspace parameters to update.
    #[prost(message, optional, tag = "2")]
    pub conversion_workspace: ::core::option::Option<ConversionWorkspace>,
    /// A unique ID used to identify the request. If the server receives two
    /// requests with the same ID, then the second request is ignored.
    ///
    /// It is recommended to always set this value to a UUID.
    ///
    /// The ID must contain only letters (a-z, A-Z), numbers (0-9), underscores
    /// (_), and hyphens (-). The maximum length is 40 characters.
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for 'DeleteConversionWorkspace' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteConversionWorkspaceRequest {
    /// Required. Name of the conversion workspace resource to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A unique ID used to identify the request. If the server receives two
    /// requests with the same ID, then the second request is ignored.
    ///
    /// It is recommended to always set this value to a UUID.
    ///
    /// The ID must contain only letters (a-z, A-Z), numbers (0-9), underscores
    /// (_), and hyphens (-). The maximum length is 40 characters.
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// Force delete the conversion workspace, even if there's a running migration
    /// that is using the workspace.
    #[prost(bool, tag = "3")]
    pub force: bool,
}
/// Request message for 'CommitConversionWorkspace' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitConversionWorkspaceRequest {
    /// Required. Name of the conversion workspace resource to commit.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Optional name of the commit.
    #[prost(string, tag = "2")]
    pub commit_name: ::prost::alloc::string::String,
}
/// Request message for 'RollbackConversionWorkspace' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollbackConversionWorkspaceRequest {
    /// Required. Name of the conversion workspace resource to roll back to.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for 'ApplyConversionWorkspace' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyConversionWorkspaceRequest {
    /// Required. The name of the conversion workspace resource for which to apply
    /// the draft tree. Must be in the form of:
    ///   projects/{project}/locations/{location}/conversionWorkspaces/{conversion_workspace}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Filter which entities to apply. Leaving this field empty will apply all of
    /// the entities. Supports Google AIP 160 based filtering.
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Only validates the apply process, but doesn't change the
    /// destination database. Only works for PostgreSQL destination connection
    /// profile.
    #[prost(bool, tag = "3")]
    pub dry_run: bool,
    /// Optional. Specifies whether the conversion workspace is to be committed
    /// automatically after the apply.
    #[prost(bool, tag = "4")]
    pub auto_commit: bool,
    /// Which destination to use when applying the conversion workspace.
    #[prost(oneof = "apply_conversion_workspace_request::Destination", tags = "100")]
    pub destination: ::core::option::Option<
        apply_conversion_workspace_request::Destination,
    >,
}
/// Nested message and enum types in `ApplyConversionWorkspaceRequest`.
pub mod apply_conversion_workspace_request {
    /// Which destination to use when applying the conversion workspace.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// Optional. Fully qualified (Uri) name of the destination connection
        /// profile.
        #[prost(string, tag = "100")]
        ConnectionProfile(::prost::alloc::string::String),
    }
}
/// Retrieve a list of all mapping rules in a given conversion workspace.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMappingRulesRequest {
    /// Required. Name of the conversion workspace resource whose mapping rules are
    /// listed in the form of:
    /// projects/{project}/locations/{location}/conversionWorkspaces/{conversion_workspace}.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of rules to return. The service may return
    /// fewer than this value.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The nextPageToken value received in the previous call to
    /// mappingRules.list, used in the subsequent request to retrieve the next
    /// page of results. On first call this should be left blank. When paginating,
    /// all other parameters provided to mappingRules.list must match the call
    /// that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for 'ListMappingRulesRequest' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMappingRulesResponse {
    /// The list of conversion workspace mapping rules.
    #[prost(message, repeated, tag = "1")]
    pub mapping_rules: ::prost::alloc::vec::Vec<MappingRule>,
    /// A token which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for 'GetMappingRule' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMappingRuleRequest {
    /// Required. Name of the mapping rule resource to get.
    /// Example: conversionWorkspaces/123/mappingRules/rule123
    ///
    /// In order to retrieve a previous revision of the mapping rule, also provide
    /// the revision ID.
    /// Example:
    /// conversionWorkspace/123/mappingRules/rule123@c7cfa2a8c7cfa2a8c7cfa2a8c7cfa2a8
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for 'SeedConversionWorkspace' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeedConversionWorkspaceRequest {
    /// Name of the conversion workspace resource to seed with new database
    /// structure, in the form of:
    /// projects/{project}/locations/{location}/conversionWorkspaces/{conversion_workspace}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Should the conversion workspace be committed automatically after the
    /// seed operation.
    #[prost(bool, tag = "2")]
    pub auto_commit: bool,
    /// The input to be used for seeding the conversion workspace. The input can
    /// either be from the source or destination databases and it can be provided
    /// through a connection profile or a DDL file.
    #[prost(oneof = "seed_conversion_workspace_request::SeedFrom", tags = "100, 101")]
    pub seed_from: ::core::option::Option<seed_conversion_workspace_request::SeedFrom>,
}
/// Nested message and enum types in `SeedConversionWorkspaceRequest`.
pub mod seed_conversion_workspace_request {
    /// The input to be used for seeding the conversion workspace. The input can
    /// either be from the source or destination databases and it can be provided
    /// through a connection profile or a DDL file.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SeedFrom {
        /// Optional. Fully qualified (Uri) name of the source connection profile.
        #[prost(string, tag = "100")]
        SourceConnectionProfile(::prost::alloc::string::String),
        /// Optional. Fully qualified (Uri) name of the destination connection
        /// profile.
        #[prost(string, tag = "101")]
        DestinationConnectionProfile(::prost::alloc::string::String),
    }
}
/// Request message for 'ConvertConversionWorkspace' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConvertConversionWorkspaceRequest {
    /// Name of the conversion workspace resource to convert in the form of:
    /// projects/{project}/locations/{location}/conversionWorkspaces/{conversion_workspace}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Specifies whether the conversion workspace is to be committed
    /// automatically after the conversion.
    #[prost(bool, tag = "4")]
    pub auto_commit: bool,
    /// Optional. Filter the entities to convert. Leaving this field empty will
    /// convert all of the entities. Supports Google AIP-160 style filtering.
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Automatically convert the full entity path for each entity
    /// specified by the filter. For example, if the filter specifies a table, that
    /// table schema (and database if there is one) will also be converted.
    #[prost(bool, tag = "6")]
    pub convert_full_path: bool,
}
/// Request message for 'ImportMappingRules' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportMappingRulesRequest {
    /// Required. Name of the conversion workspace resource to import the rules to
    /// in the form of:
    /// projects/{project}/locations/{location}/conversionWorkspaces/{conversion_workspace}.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The format of the rules content file.
    #[prost(enumeration = "ImportRulesFileFormat", tag = "2")]
    pub rules_format: i32,
    /// Required. One or more rules files.
    #[prost(message, repeated, tag = "3")]
    pub rules_files: ::prost::alloc::vec::Vec<import_mapping_rules_request::RulesFile>,
    /// Required. Should the conversion workspace be committed automatically after
    /// the import operation.
    #[prost(bool, tag = "6")]
    pub auto_commit: bool,
}
/// Nested message and enum types in `ImportMappingRulesRequest`.
pub mod import_mapping_rules_request {
    /// Details of a single rules file.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RulesFile {
        /// Required. The filename of the rules that needs to be converted. The
        /// filename is used mainly so that future logs of the import rules job
        /// contain it, and can therefore be searched by it.
        #[prost(string, tag = "1")]
        pub rules_source_filename: ::prost::alloc::string::String,
        /// Required. The text content of the rules that needs to be converted.
        #[prost(string, tag = "2")]
        pub rules_content: ::prost::alloc::string::String,
    }
}
/// Request message for 'DescribeDatabaseEntities' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeDatabaseEntitiesRequest {
    /// Required. Name of the conversion workspace resource whose database entities
    /// are described. Must be in the form of:
    /// projects/{project}/locations/{location}/conversionWorkspaces/{conversion_workspace}.
    #[prost(string, tag = "1")]
    pub conversion_workspace: ::prost::alloc::string::String,
    /// Optional. The maximum number of entities to return. The service may return
    /// fewer entities than the value specifies.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Optional. The nextPageToken value received in the previous call to
    /// conversionWorkspace.describeDatabaseEntities, used in the subsequent
    /// request to retrieve the next page of results. On first call this should be
    /// left blank. When paginating, all other parameters provided to
    /// conversionWorkspace.describeDatabaseEntities must match the call that
    /// provided the page token.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
    /// Required. The tree to fetch.
    #[prost(enumeration = "describe_database_entities_request::DbTreeType", tag = "6")]
    pub tree: i32,
    /// Optional. Whether to retrieve the latest committed version of the entities
    /// or the latest version. This field is ignored if a specific commit_id is
    /// specified.
    #[prost(bool, tag = "11")]
    pub uncommitted: bool,
    /// Optional. Request a specific commit ID. If not specified, the entities from
    /// the latest commit are returned.
    #[prost(string, tag = "12")]
    pub commit_id: ::prost::alloc::string::String,
    /// Optional. Filter the returned entities based on AIP-160 standard.
    #[prost(string, tag = "13")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Results view based on AIP-157
    #[prost(enumeration = "DatabaseEntityView", tag = "14")]
    pub view: i32,
}
/// Nested message and enum types in `DescribeDatabaseEntitiesRequest`.
pub mod describe_database_entities_request {
    /// The type of a tree to return
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
    pub enum DbTreeType {
        /// Unspecified tree type.
        Unspecified = 0,
        /// The source database tree.
        SourceTree = 1,
        /// The draft database tree.
        DraftTree = 2,
        /// The destination database tree.
        DestinationTree = 3,
    }
    impl DbTreeType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DbTreeType::Unspecified => "DB_TREE_TYPE_UNSPECIFIED",
                DbTreeType::SourceTree => "SOURCE_TREE",
                DbTreeType::DraftTree => "DRAFT_TREE",
                DbTreeType::DestinationTree => "DESTINATION_TREE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DB_TREE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "SOURCE_TREE" => Some(Self::SourceTree),
                "DRAFT_TREE" => Some(Self::DraftTree),
                "DESTINATION_TREE" => Some(Self::DestinationTree),
                _ => None,
            }
        }
    }
}
/// Response message for 'DescribeDatabaseEntities' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeDatabaseEntitiesResponse {
    /// The list of database entities for the conversion workspace.
    #[prost(message, repeated, tag = "1")]
    pub database_entities: ::prost::alloc::vec::Vec<DatabaseEntity>,
    /// A token which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for 'SearchBackgroundJobs' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchBackgroundJobsRequest {
    /// Required. Name of the conversion workspace resource whose jobs are listed,
    /// in the form of:
    /// projects/{project}/locations/{location}/conversionWorkspaces/{conversion_workspace}.
    #[prost(string, tag = "1")]
    pub conversion_workspace: ::prost::alloc::string::String,
    /// Optional. Whether or not to return just the most recent job per job type,
    #[prost(bool, tag = "2")]
    pub return_most_recent_per_job_type: bool,
    /// Optional. The maximum number of jobs to return. The service may return
    /// fewer than this value. If unspecified, at most 100 jobs are
    /// returned. The maximum value is 100; values above 100 are coerced to
    /// 100.
    #[prost(int32, tag = "3")]
    pub max_size: i32,
    /// Optional. If provided, only returns jobs that completed until (not
    /// including) the given timestamp.
    #[prost(message, optional, tag = "4")]
    pub completed_until_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Response message for 'SearchBackgroundJobs' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchBackgroundJobsResponse {
    /// The list of conversion workspace mapping rules.
    #[prost(message, repeated, tag = "1")]
    pub jobs: ::prost::alloc::vec::Vec<BackgroundJobLogEntry>,
}
/// Request message for 'DescribeConversionWorkspaceRevisions' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeConversionWorkspaceRevisionsRequest {
    /// Required. Name of the conversion workspace resource whose revisions are
    /// listed. Must be in the form of:
    /// projects/{project}/locations/{location}/conversionWorkspaces/{conversion_workspace}.
    #[prost(string, tag = "1")]
    pub conversion_workspace: ::prost::alloc::string::String,
    /// Optional. Optional filter to request a specific commit ID.
    #[prost(string, tag = "2")]
    pub commit_id: ::prost::alloc::string::String,
}
/// Response message for 'DescribeConversionWorkspaceRevisions' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeConversionWorkspaceRevisionsResponse {
    /// The list of conversion workspace revisions.
    #[prost(message, repeated, tag = "1")]
    pub revisions: ::prost::alloc::vec::Vec<ConversionWorkspace>,
}
/// Request message for 'CreateMappingRule' command.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMappingRuleRequest {
    /// Required. The parent which owns this collection of mapping rules.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID of the rule to create.
    #[prost(string, tag = "2")]
    pub mapping_rule_id: ::prost::alloc::string::String,
    /// Required. Represents a \[mapping rule\]
    /// (<https://cloud.google.com/database-migration/reference/rest/v1/projects.locations.mappingRules>)
    /// object.
    #[prost(message, optional, tag = "3")]
    pub mapping_rule: ::core::option::Option<MappingRule>,
    /// A unique ID used to identify the request. If the server receives two
    /// requests with the same ID, then the second request is ignored.
    ///
    /// It is recommended to always set this value to a UUID.
    ///
    /// The ID must contain only letters (a-z, A-Z), numbers (0-9), underscores
    /// (_), and hyphens (-). The maximum length is 40 characters.
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for 'DeleteMappingRule' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteMappingRuleRequest {
    /// Required. Name of the mapping rule resource to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A unique ID used to identify the request. If the server receives
    /// two requests with the same ID, then the second request is ignored.
    ///
    /// It is recommended to always set this value to a UUID.
    ///
    /// The ID must contain only letters (a-z, A-Z), numbers (0-9), underscores
    /// (_), and hyphens (-). The maximum length is 40 characters.
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for 'FetchStaticIps' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchStaticIpsRequest {
    /// Required. The resource name for the location for which static IPs should be
    /// returned. Must be in the format `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Maximum number of IPs to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `FetchStaticIps` call.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for a 'FetchStaticIps' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchStaticIpsResponse {
    /// List of static IPs.
    #[prost(string, repeated, tag = "1")]
    pub static_ips: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A token that can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// AIP-157 Partial Response view for Database Entity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DatabaseEntityView {
    /// Unspecified view. Defaults to basic view.
    Unspecified = 0,
    /// Default view. Does not return DDLs or Issues.
    Basic = 1,
    /// Return full entity details including mappings, ddl and issues.
    Full = 2,
    /// Top-most (Database, Schema) nodes which are returned contains summary
    /// details for their decendents such as the number of entities per type and
    /// issues rollups. When this view is used, only a single page of result is
    /// returned and the page_size property of the request is ignored. The
    /// returned page will only include the top-most node types.
    RootSummary = 3,
}
impl DatabaseEntityView {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DatabaseEntityView::Unspecified => "DATABASE_ENTITY_VIEW_UNSPECIFIED",
            DatabaseEntityView::Basic => "DATABASE_ENTITY_VIEW_BASIC",
            DatabaseEntityView::Full => "DATABASE_ENTITY_VIEW_FULL",
            DatabaseEntityView::RootSummary => "DATABASE_ENTITY_VIEW_ROOT_SUMMARY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DATABASE_ENTITY_VIEW_UNSPECIFIED" => Some(Self::Unspecified),
            "DATABASE_ENTITY_VIEW_BASIC" => Some(Self::Basic),
            "DATABASE_ENTITY_VIEW_FULL" => Some(Self::Full),
            "DATABASE_ENTITY_VIEW_ROOT_SUMMARY" => Some(Self::RootSummary),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod data_migration_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Database Migration service
    #[derive(Debug, Clone)]
    pub struct DataMigrationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DataMigrationServiceClient<tonic::transport::Channel> {
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
    impl<T> DataMigrationServiceClient<T>
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
        ) -> DataMigrationServiceClient<InterceptedService<T, F>>
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
            DataMigrationServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists migration jobs in a given project and location.
        pub async fn list_migration_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMigrationJobsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMigrationJobsResponse>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/ListMigrationJobs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.clouddms.v1.DataMigrationService",
                        "ListMigrationJobs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single migration job.
        pub async fn get_migration_job(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMigrationJobRequest>,
        ) -> std::result::Result<tonic::Response<super::MigrationJob>, tonic::Status> {
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
                "/google.cloud.clouddms.v1.DataMigrationService/GetMigrationJob",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.clouddms.v1.DataMigrationService",
                        "GetMigrationJob",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new migration job in a given project and location.
        pub async fn create_migration_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateMigrationJobRequest>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/CreateMigrationJob",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.clouddms.v1.DataMigrationService",
                        "CreateMigrationJob",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the parameters of a single migration job.
        pub async fn update_migration_job(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateMigrationJobRequest>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/UpdateMigrationJob",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.clouddms.v1.DataMigrationService",
                        "UpdateMigrationJob",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single migration job.
        pub async fn delete_migration_job(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteMigrationJobRequest>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/DeleteMigrationJob",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.clouddms.v1.DataMigrationService",
                        "DeleteMigrationJob",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Start an already created migration job.
        pub async fn start_migration_job(
            &mut self,
            request: impl tonic::IntoRequest<super::StartMigrationJobRequest>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/StartMigrationJob",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.clouddms.v1.DataMigrationService",
                        "StartMigrationJob",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Stops a running migration job.
        pub async fn stop_migration_job(
            &mut self,
            request: impl tonic::IntoRequest<super::StopMigrationJobRequest>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/StopMigrationJob",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.clouddms.v1.DataMigrationService",
                        "StopMigrationJob",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Resume a migration job that is currently stopped and is resumable (was
        /// stopped during CDC phase).
        pub async fn resume_migration_job(
            &mut self,
            request: impl tonic::IntoRequest<super::ResumeMigrationJobRequest>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/ResumeMigrationJob",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.clouddms.v1.DataMigrationService",
                        "ResumeMigrationJob",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Promote a migration job, stopping replication to the destination and
        /// promoting the destination to be a standalone database.
        pub async fn promote_migration_job(
            &mut self,
            request: impl tonic::IntoRequest<super::PromoteMigrationJobRequest>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/PromoteMigrationJob",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.clouddms.v1.DataMigrationService",
                        "PromoteMigrationJob",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Verify a migration job, making sure the destination can reach the source
        /// and that all configuration and prerequisites are met.
        pub async fn verify_migration_job(
            &mut self,
            request: impl tonic::IntoRequest<super::VerifyMigrationJobRequest>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/VerifyMigrationJob",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.clouddms.v1.DataMigrationService",
                        "VerifyMigrationJob",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Restart a stopped or failed migration job, resetting the destination
        /// instance to its original state and starting the migration process from
        /// scratch.
        pub async fn restart_migration_job(
            &mut self,
            request: impl tonic::IntoRequest<super::RestartMigrationJobRequest>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/RestartMigrationJob",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.clouddms.v1.DataMigrationService",
                        "RestartMigrationJob",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Generate a SSH configuration script to configure the reverse SSH
        /// connectivity.
        pub async fn generate_ssh_script(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateSshScriptRequest>,
        ) -> std::result::Result<tonic::Response<super::SshScript>, tonic::Status> {
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
                "/google.cloud.clouddms.v1.DataMigrationService/GenerateSshScript",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.clouddms.v1.DataMigrationService",
                        "GenerateSshScript",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Generate a TCP Proxy configuration script to configure a cloud-hosted VM
        /// running a TCP Proxy.
        pub async fn generate_tcp_proxy_script(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateTcpProxyScriptRequest>,
        ) -> std::result::Result<tonic::Response<super::TcpProxyScript>, tonic::Status> {
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
                "/google.cloud.clouddms.v1.DataMigrationService/GenerateTcpProxyScript",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.clouddms.v1.DataMigrationService",
                        "GenerateTcpProxyScript",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves a list of all connection profiles in a given project and
        /// location.
        pub async fn list_connection_profiles(
            &mut self,
            request: impl tonic::IntoRequest<super::ListConnectionProfilesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListConnectionProfilesResponse>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/ListConnectionProfiles",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.clouddms.v1.DataMigrationService",
                        "ListConnectionProfiles",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single connection profile.
        pub async fn get_connection_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConnectionProfileRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ConnectionProfile>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/GetConnectionProfile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.clouddms.v1.DataMigrationService",
                        "GetConnectionProfile",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new connection profile in a given project and location.
        pub async fn create_connection_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateConnectionProfileRequest>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/CreateConnectionProfile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.clouddms.v1.DataMigrationService",
                        "CreateConnectionProfile",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Update the configuration of a single connection profile.
        pub async fn update_connection_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateConnectionProfileRequest>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/UpdateConnectionProfile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.clouddms.v1.DataMigrationService",
                        "UpdateConnectionProfile",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single Database Migration Service connection profile.
        /// A connection profile can only be deleted if it is not in use by any
        /// active migration jobs.
        pub async fn delete_connection_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteConnectionProfileRequest>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/DeleteConnectionProfile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.clouddms.v1.DataMigrationService",
                        "DeleteConnectionProfile",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new private connection in a given project and location.
        pub async fn create_private_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePrivateConnectionRequest>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/CreatePrivateConnection",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.clouddms.v1.DataMigrationService",
                        "CreatePrivateConnection",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single private connection.
        pub async fn get_private_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPrivateConnectionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PrivateConnection>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/GetPrivateConnection",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.clouddms.v1.DataMigrationService",
                        "GetPrivateConnection",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves a list of private connections in a given project and location.
        pub async fn list_private_connections(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPrivateConnectionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPrivateConnectionsResponse>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/ListPrivateConnections",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.clouddms.v1.DataMigrationService",
                        "ListPrivateConnections",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single Database Migration Service private connection.
        pub async fn delete_private_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePrivateConnectionRequest>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/DeletePrivateConnection",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.clouddms.v1.DataMigrationService",
                        "DeletePrivateConnection",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single conversion workspace.
        pub async fn get_conversion_workspace(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConversionWorkspaceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ConversionWorkspace>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/GetConversionWorkspace",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.clouddms.v1.DataMigrationService",
                        "GetConversionWorkspace",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists conversion workspaces in a given project and location.
        pub async fn list_conversion_workspaces(
            &mut self,
            request: impl tonic::IntoRequest<super::ListConversionWorkspacesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListConversionWorkspacesResponse>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/ListConversionWorkspaces",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.clouddms.v1.DataMigrationService",
                        "ListConversionWorkspaces",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new conversion workspace in a given project and location.
        pub async fn create_conversion_workspace(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateConversionWorkspaceRequest>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/CreateConversionWorkspace",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.clouddms.v1.DataMigrationService",
                        "CreateConversionWorkspace",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the parameters of a single conversion workspace.
        pub async fn update_conversion_workspace(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateConversionWorkspaceRequest>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/UpdateConversionWorkspace",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.clouddms.v1.DataMigrationService",
                        "UpdateConversionWorkspace",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single conversion workspace.
        pub async fn delete_conversion_workspace(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteConversionWorkspaceRequest>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/DeleteConversionWorkspace",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.clouddms.v1.DataMigrationService",
                        "DeleteConversionWorkspace",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new mapping rule for a given conversion workspace.
        pub async fn create_mapping_rule(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateMappingRuleRequest>,
        ) -> std::result::Result<tonic::Response<super::MappingRule>, tonic::Status> {
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
                "/google.cloud.clouddms.v1.DataMigrationService/CreateMappingRule",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.clouddms.v1.DataMigrationService",
                        "CreateMappingRule",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single mapping rule.
        pub async fn delete_mapping_rule(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteMappingRuleRequest>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/DeleteMappingRule",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.clouddms.v1.DataMigrationService",
                        "DeleteMappingRule",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists the mapping rules for a specific conversion workspace.
        pub async fn list_mapping_rules(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMappingRulesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMappingRulesResponse>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/ListMappingRules",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.clouddms.v1.DataMigrationService",
                        "ListMappingRules",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the details of a mapping rule.
        pub async fn get_mapping_rule(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMappingRuleRequest>,
        ) -> std::result::Result<tonic::Response<super::MappingRule>, tonic::Status> {
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
                "/google.cloud.clouddms.v1.DataMigrationService/GetMappingRule",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.clouddms.v1.DataMigrationService",
                        "GetMappingRule",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Imports a snapshot of the source database into the
        /// conversion workspace.
        pub async fn seed_conversion_workspace(
            &mut self,
            request: impl tonic::IntoRequest<super::SeedConversionWorkspaceRequest>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/SeedConversionWorkspace",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.clouddms.v1.DataMigrationService",
                        "SeedConversionWorkspace",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Imports the mapping rules for a given conversion workspace.
        /// Supports various formats of external rules files.
        pub async fn import_mapping_rules(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportMappingRulesRequest>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/ImportMappingRules",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.clouddms.v1.DataMigrationService",
                        "ImportMappingRules",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a draft tree schema for the destination database.
        pub async fn convert_conversion_workspace(
            &mut self,
            request: impl tonic::IntoRequest<super::ConvertConversionWorkspaceRequest>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/ConvertConversionWorkspace",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.clouddms.v1.DataMigrationService",
                        "ConvertConversionWorkspace",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Marks all the data in the conversion workspace as committed.
        pub async fn commit_conversion_workspace(
            &mut self,
            request: impl tonic::IntoRequest<super::CommitConversionWorkspaceRequest>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/CommitConversionWorkspace",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.clouddms.v1.DataMigrationService",
                        "CommitConversionWorkspace",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Rolls back a conversion workspace to the last committed snapshot.
        pub async fn rollback_conversion_workspace(
            &mut self,
            request: impl tonic::IntoRequest<super::RollbackConversionWorkspaceRequest>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/RollbackConversionWorkspace",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.clouddms.v1.DataMigrationService",
                        "RollbackConversionWorkspace",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Applies draft tree onto a specific destination database.
        pub async fn apply_conversion_workspace(
            &mut self,
            request: impl tonic::IntoRequest<super::ApplyConversionWorkspaceRequest>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/ApplyConversionWorkspace",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.clouddms.v1.DataMigrationService",
                        "ApplyConversionWorkspace",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Describes the database entities tree for a specific conversion workspace
        /// and a specific tree type.
        ///
        /// Database entities are not resources like conversion workspaces or mapping
        /// rules, and they can't be created, updated or deleted. Instead, they are
        /// simple data objects describing the structure of the client database.
        pub async fn describe_database_entities(
            &mut self,
            request: impl tonic::IntoRequest<super::DescribeDatabaseEntitiesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DescribeDatabaseEntitiesResponse>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/DescribeDatabaseEntities",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.clouddms.v1.DataMigrationService",
                        "DescribeDatabaseEntities",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Searches/lists the background jobs for a specific
        /// conversion workspace.
        ///
        /// The background jobs are not resources like conversion workspaces or
        /// mapping rules, and they can't be created, updated or deleted.
        /// Instead, they are a way to expose the data plane jobs log.
        pub async fn search_background_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchBackgroundJobsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SearchBackgroundJobsResponse>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/SearchBackgroundJobs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.clouddms.v1.DataMigrationService",
                        "SearchBackgroundJobs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves a list of committed revisions of a specific conversion
        /// workspace.
        pub async fn describe_conversion_workspace_revisions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::DescribeConversionWorkspaceRevisionsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::DescribeConversionWorkspaceRevisionsResponse>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/DescribeConversionWorkspaceRevisions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.clouddms.v1.DataMigrationService",
                        "DescribeConversionWorkspaceRevisions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Fetches a set of static IP addresses that need to be allowlisted by the
        /// customer when using the static-IP connectivity method.
        pub async fn fetch_static_ips(
            &mut self,
            request: impl tonic::IntoRequest<super::FetchStaticIpsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FetchStaticIpsResponse>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/FetchStaticIps",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.clouddms.v1.DataMigrationService",
                        "FetchStaticIps",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
