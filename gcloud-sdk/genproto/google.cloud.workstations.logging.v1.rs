/// JSON payload for the Cloud Logging event:
/// `workstations.googleapis.com%2Fworkstation_events`
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkstationEvent {
    #[prost(oneof = "workstation_event::EventType", tags = "1, 2")]
    pub event_type: ::core::option::Option<workstation_event::EventType>,
}
/// Nested message and enum types in `WorkstationEvent`.
pub mod workstation_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum EventType {
        /// Vm assignment event.
        #[prost(message, tag = "1")]
        VmAssignmentEvent(super::VmAssignmentEvent),
        /// Disk assignment event.
        #[prost(message, tag = "2")]
        DiskAssignmentEvent(super::DiskAssignmentEvent),
    }
}
/// Vm assignment event.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VmAssignmentEvent {
    /// Name of the VM assigned to this workstation.
    #[prost(string, tag = "1")]
    pub vm: ::prost::alloc::string::String,
}
/// Disk assignment event.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiskAssignmentEvent {
    /// Name of the disk assigned to this workstation.
    #[prost(string, tag = "1")]
    pub disk: ::prost::alloc::string::String,
}
