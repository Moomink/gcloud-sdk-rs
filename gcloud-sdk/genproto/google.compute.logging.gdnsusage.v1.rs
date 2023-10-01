/// Used for structured payload for reporting Platform Logs
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GdnsVmUsagePlatformLog {
    /// source vm's information
    #[prost(message, optional, tag = "1")]
    pub source_vm: ::core::option::Option<VmInfo>,
    /// destination vm's information
    #[prost(message, optional, tag = "2")]
    pub destination_vm: ::core::option::Option<VmInfo>,
    /// message that informs users on how to fix the global DNS query that is
    /// blocking the zonal DNS migration
    #[prost(string, optional, tag = "3")]
    pub debug_message: ::core::option::Option<::prost::alloc::string::String>,
    /// number of zDNS migration blocking queries sent from source_vm to
    /// destination_vm
    #[prost(int32, optional, tag = "5")]
    pub query_count: ::core::option::Option<i32>,
}
/// VM details
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VmInfo {
    /// project id of the vm
    #[prost(string, optional, tag = "1")]
    pub project_id: ::core::option::Option<::prost::alloc::string::String>,
    /// name of the vm
    #[prost(string, optional, tag = "2")]
    pub vm: ::core::option::Option<::prost::alloc::string::String>,
    /// zone of the vm
    #[prost(string, optional, tag = "3")]
    pub zone: ::core::option::Option<::prost::alloc::string::String>,
}
