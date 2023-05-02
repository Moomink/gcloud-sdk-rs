/// A process is the definition of a data transformation operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Process {
    /// Immutable. The resource name of the lineage process. Format:
    /// `projects/{project}/locations/{location}/processes/{process}`.
    /// Can be specified or auto-assigned.
    /// {process} must be not longer than 200 characters and only
    /// contain characters in a set: `a-zA-Z0-9_-:.`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A human-readable name you can set to display in a user interface.
    /// Must be not longer than 200 characters and only contain UTF-8 letters
    /// or numbers, spaces or characters like `_-:&.`
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. The attributes of the process. Can be anything, for example,
    /// "author". Up to 100 attributes are allowed.
    #[prost(map = "string, message", tag = "3")]
    pub attributes: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost_types::Value,
    >,
    /// Optional. The origin of this process and its runs and lineage events.
    #[prost(message, optional, tag = "4")]
    pub origin: ::core::option::Option<Origin>,
}
/// A lineage run represents an execution of a process that creates
/// lineage events.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Run {
    /// Immutable. The resource name of the run. Format:
    /// `projects/{project}/locations/{location}/processes/{process}/runs/{run}`.
    /// Can be specified or auto-assigned.
    /// {run} must be not longer than 200 characters and only
    /// contain characters in a set: `a-zA-Z0-9_-:.`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A human-readable name you can set to display in a user interface.
    /// Must be not longer than 1024 characters and only contain UTF-8 letters
    /// or numbers, spaces or characters like `_-:&.`
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. The attributes of the run. Can be anything, for example, a string
    /// with an SQL request. Up to 100 attributes are allowed.
    #[prost(map = "string, message", tag = "3")]
    pub attributes: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost_types::Value,
    >,
    /// Required. The timestamp of the start of the run.
    #[prost(message, optional, tag = "4")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. The timestamp of the end of the run.
    #[prost(message, optional, tag = "5")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. The state of the run.
    #[prost(enumeration = "run::State", tag = "6")]
    pub state: i32,
}
/// Nested message and enum types in `Run`.
pub mod run {
    /// The current state of the run.
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
        /// The state is unknown. The true state may be any of the below or a
        /// different state that is not supported here explicitly.
        Unknown = 0,
        /// The run is still executing.
        Started = 1,
        /// The run completed.
        Completed = 2,
        /// The run failed.
        Failed = 3,
        /// The run aborted.
        Aborted = 4,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unknown => "UNKNOWN",
                State::Started => "STARTED",
                State::Completed => "COMPLETED",
                State::Failed => "FAILED",
                State::Aborted => "ABORTED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN" => Some(Self::Unknown),
                "STARTED" => Some(Self::Started),
                "COMPLETED" => Some(Self::Completed),
                "FAILED" => Some(Self::Failed),
                "ABORTED" => Some(Self::Aborted),
                _ => None,
            }
        }
    }
}
/// A lineage event represents an operation on assets. Within the operation, the
/// data flows from the source to the target defined in the links field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LineageEvent {
    /// Immutable. The resource name of the lineage event.
    /// Format:
    /// `projects/{project}/locations/{location}/processes/{process}/runs/{run}/lineageEvents/{lineage_event}`.
    /// Can be specified or auto-assigned.
    /// {lineage_event} must be not longer than 200 characters and only
    /// contain characters in a set: `a-zA-Z0-9_-:.`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. List of source-target pairs. Can't contain more than 100 tuples.
    #[prost(message, repeated, tag = "8")]
    pub links: ::prost::alloc::vec::Vec<EventLink>,
    /// Optional. The beginning of the transformation which resulted in this
    /// lineage event. For streaming scenarios, it should be the beginning of the
    /// period from which the lineage is being reported.
    #[prost(message, optional, tag = "6")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. The end of the transformation which resulted in this lineage
    /// event.  For streaming scenarios, it should be the end of the period from
    /// which the lineage is being reported.
    #[prost(message, optional, tag = "7")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// A lineage between source and target entities.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventLink {
    /// Required. Reference to the source entity
    #[prost(message, optional, tag = "1")]
    pub source: ::core::option::Option<EntityReference>,
    /// Required. Reference to the target entity
    #[prost(message, optional, tag = "2")]
    pub target: ::core::option::Option<EntityReference>,
}
/// The soft reference to everything you can attach a lineage event to.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityReference {
    /// Required. Fully Qualified Name of the entity. Useful for referencing
    /// entities that aren't represented as GCP resources, for example, tables in
    /// Dataproc Metastore API.
    ///
    /// Examples:
    ///
    ///    * `bigquery:dataset.project_id.dataset_id`
    ///    * `bigquery:table.project_id.dataset_id.table_id`
    ///    * `pubsub:project_id.topic_id`
    ///    * `dataproc_metastore:projectId.locationId.instanceId.databaseId.tableId`
    #[prost(string, tag = "1")]
    pub fully_qualified_name: ::prost::alloc::string::String,
}
/// Metadata describing the operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// Output only. The current operation state.
    #[prost(enumeration = "operation_metadata::State", tag = "1")]
    pub state: i32,
    /// Output only. The type of the operation being performed.
    #[prost(enumeration = "operation_metadata::Type", tag = "2")]
    pub operation_type: i32,
    /// Output only. The [relative name]
    /// (<https://cloud.google.com//apis/design/resource_names#relative_resource_name>)
    /// of the resource being operated on.
    #[prost(string, tag = "3")]
    pub resource: ::prost::alloc::string::String,
    /// Output only. The UUID of the resource being operated on.
    #[prost(string, tag = "4")]
    pub resource_uuid: ::prost::alloc::string::String,
    /// Output only. The timestamp of the operation submission to the server.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp of the operation termination, regardless of its
    /// success. This field is unset if the operation is still ongoing.
    #[prost(message, optional, tag = "6")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `OperationMetadata`.
pub mod operation_metadata {
    /// An enum with the state of the operation.
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
        /// Unused.
        Unspecified = 0,
        /// The operation has been created but is not yet started.
        Pending = 1,
        /// The operation is underway.
        Running = 2,
        /// The operation completed successfully.
        Succeeded = 3,
        /// The operation is no longer running and did not succeed.
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
                State::Pending => "PENDING",
                State::Running => "RUNNING",
                State::Succeeded => "SUCCEEDED",
                State::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "PENDING" => Some(Self::Pending),
                "RUNNING" => Some(Self::Running),
                "SUCCEEDED" => Some(Self::Succeeded),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
    /// Type of the long running operation.
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
        /// Unused.
        Unspecified = 0,
        /// The resource deletion operation.
        Delete = 1,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::Delete => "DELETE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "DELETE" => Some(Self::Delete),
                _ => None,
            }
        }
    }
}
/// Request message for
/// \[CreateProcess][google.cloud.datacatalog.lineage.v1.CreateProcess\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateProcessRequest {
    /// Required. The name of the project and its location that should own the
    /// process.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The process to create.
    #[prost(message, optional, tag = "2")]
    pub process: ::core::option::Option<Process>,
    /// A unique identifier for this request. Restricted to 36 ASCII characters.
    /// A random UUID is recommended. This request is idempotent only if a
    /// `request_id` is provided.
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// \[UpdateProcess][google.cloud.datacatalog.lineage.v1.UpdateProcess\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateProcessRequest {
    /// Required. The lineage process to update.
    ///
    /// The process's `name` field is used to identify the process to update.
    #[prost(message, optional, tag = "1")]
    pub process: ::core::option::Option<Process>,
    /// The list of fields to update. Currently not used. The whole message is
    /// updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// If set to true and the process is not found, the request inserts it.
    #[prost(bool, tag = "3")]
    pub allow_missing: bool,
}
/// Request message for
/// \[GetProcess][google.cloud.datacatalog.lineage.v1.GetProcess\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProcessRequest {
    /// Required. The name of the process to get.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// \[ListProcesses][google.cloud.datacatalog.lineage.v1.ListProcesses\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProcessesRequest {
    /// Required. The name of the project and its location that owns this
    /// collection of processes.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of processes to return. The service may return
    /// fewer than this value. If unspecified, at most 50 processes are
    /// returned. The maximum value is 100; values greater than 100 are cut to
    /// 100.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The page token received from a previous `ListProcesses` call. Specify
    /// it to get the next page.
    ///
    /// When paginating, all other parameters specified in this call must
    /// match the parameters of the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for
/// \[ListProcesses][google.cloud.datacatalog.lineage.v1.ListProcesses\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProcessesResponse {
    /// The processes from the specified project and location.
    #[prost(message, repeated, tag = "1")]
    pub processes: ::prost::alloc::vec::Vec<Process>,
    /// The token to specify as `page_token` in the next call to get the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for
/// \[DeleteProcess][google.cloud.datacatalog.lineage.v1.DeleteProcess\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteProcessRequest {
    /// Required. The name of the process to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If set to true and the process is not found, the request
    /// succeeds but the server doesn't perform any actions.
    #[prost(bool, tag = "2")]
    pub allow_missing: bool,
}
/// Request message for
/// \[CreateRun][google.cloud.datacatalog.lineage.v1.CreateRun\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRunRequest {
    /// Required. The name of the process that should own the run.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The run to create.
    #[prost(message, optional, tag = "2")]
    pub run: ::core::option::Option<Run>,
    /// A unique identifier for this request. Restricted to 36 ASCII characters.
    /// A random UUID is recommended. This request is idempotent only if a
    /// `request_id` is provided.
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// \[UpdateRun][google.cloud.datacatalog.lineage.v1.UpdateRun\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRunRequest {
    /// Required. The lineage run to update.
    ///
    /// The run's `name` field is used to identify the run to update.
    ///
    /// Format:
    /// `projects/{project}/locations/{location}/processes/{process}/runs/{run}`.
    #[prost(message, optional, tag = "1")]
    pub run: ::core::option::Option<Run>,
    /// The list of fields to update. Currently not used. The whole message is
    /// updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for
/// \[GetRun][google.cloud.datacatalog.lineage.v1.GetRun\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRunRequest {
    /// Required. The name of the run to get.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// \[ListRuns][google.cloud.datacatalog.lineage.v1.ListRuns\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRunsRequest {
    /// Required. The name of process that owns this collection of runs.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of runs to return. The service may return
    /// fewer than this value. If unspecified, at most 50 runs are
    /// returned. The maximum value is 100; values greater than 100 are cut to
    /// 100.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The page token received from a previous `ListRuns` call. Specify
    /// it to get the next page.
    ///
    /// When paginating, all other parameters specified in this call must
    /// match the parameters of the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for
/// \[ListRuns][google.cloud.datacatalog.lineage.v1.ListRuns\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRunsResponse {
    /// The runs from the specified project and location.
    #[prost(message, repeated, tag = "1")]
    pub runs: ::prost::alloc::vec::Vec<Run>,
    /// The token to specify as `page_token` in the next call to get the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for
/// \[DeleteRun][google.cloud.datacatalog.lineage.v1.DeleteRun\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRunRequest {
    /// Required. The name of the run to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If set to true and the run is not found, the request
    /// succeeds but the server doesn't perform any actions.
    #[prost(bool, tag = "2")]
    pub allow_missing: bool,
}
/// Request message for
/// \[CreateLineageEvent][google.cloud.datacatalog.lineage.v1.CreateLineageEvent\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateLineageEventRequest {
    /// Required. The name of the run that should own the lineage event.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The lineage event to create.
    #[prost(message, optional, tag = "2")]
    pub lineage_event: ::core::option::Option<LineageEvent>,
    /// A unique identifier for this request. Restricted to 36 ASCII characters.
    /// A random UUID is recommended. This request is idempotent only if a
    /// `request_id` is provided.
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// \[GetLineageEvent][google.cloud.datacatalog.lineage.v1.GetLineageEvent\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLineageEventRequest {
    /// Required. The name of the lineage event to get.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// \[ListLineageEvents][google.cloud.datacatalog.lineage.v1.ListLineageEvents\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLineageEventsRequest {
    /// Required. The name of the run that owns the collection of lineage events to
    /// get.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of lineage events to return.
    ///
    /// The service may return fewer events than this value.
    /// If unspecified, at most 50 events are returned. The maximum value is 100;
    /// values greater than 100 are cut to 100.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The page token received from a previous `ListLineageEvents` call. Specify
    /// it to get the next page.
    ///
    /// When paginating, all other parameters specified in this call must
    /// match the parameters of the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for
/// \[ListLineageEvents][google.cloud.datacatalog.lineage.v1.ListLineageEvents\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLineageEventsResponse {
    /// Lineage events from the specified project and location.
    #[prost(message, repeated, tag = "1")]
    pub lineage_events: ::prost::alloc::vec::Vec<LineageEvent>,
    /// The token to specify as `page_token` in the next call to get the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for
/// \[DeleteLineageEvent][google.cloud.datacatalog.lineage.v1.DeleteLineageEvent\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteLineageEventRequest {
    /// Required. The name of the lineage event to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If set to true and the lineage event is not found, the request
    /// succeeds but the server doesn't perform any actions.
    #[prost(bool, tag = "2")]
    pub allow_missing: bool,
}
/// Request message for
/// \[SearchLinks][google.cloud.datacatalog.lineage.v1.Lineage.SearchLinks\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchLinksRequest {
    /// Required. The project and location you want search in the format `projects/*/locations/*`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of links to return in a single page of the
    /// response. A page may contain fewer links than this value. If unspecified,
    /// at most 10 links are returned.
    ///
    /// Maximum value is 100; values greater than 100 are reduced to 100.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The page token received from a previous `SearchLinksRequest`
    /// call. Use it to get the next page.
    ///
    /// When requesting subsequent pages of a response, remember that
    /// all parameters must match the values you provided
    /// in the original request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// The asset for which you want to retrieve links.
    #[prost(oneof = "search_links_request::Criteria", tags = "4, 5")]
    pub criteria: ::core::option::Option<search_links_request::Criteria>,
}
/// Nested message and enum types in `SearchLinksRequest`.
pub mod search_links_request {
    /// The asset for which you want to retrieve links.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Criteria {
        /// Optional. Send asset information in the **source** field to retrieve all
        /// links that lead from the specified asset to downstream assets.
        #[prost(message, tag = "4")]
        Source(super::EntityReference),
        /// Optional. Send asset information in the **target** field to retrieve all
        /// links that lead from upstream assets to the specified asset.
        #[prost(message, tag = "5")]
        Target(super::EntityReference),
    }
}
/// Response message for
/// \[SearchLinks][google.cloud.datacatalog.lineage.v1.Lineage.SearchLinks\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchLinksResponse {
    /// The list of links for a given asset. Can be empty if the asset has no
    /// relations of requested type (source or target).
    #[prost(message, repeated, tag = "1")]
    pub links: ::prost::alloc::vec::Vec<Link>,
    /// The token to specify as `page_token` in the subsequent call to get the next
    /// page. Omitted if there are no more pages in the response.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Links represent the data flow between **source** (upstream)
/// and **target** (downstream) assets in transformation pipelines.
///
/// Links are created when LineageEvents record data transformation between
/// related assets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Link {
    /// Output only. Immutable. The name of the link. Format:
    /// `projects/{project}/locations/{location}/links/{link}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The pointer to the entity that is the **source** of this link.
    #[prost(message, optional, tag = "2")]
    pub source: ::core::option::Option<EntityReference>,
    /// The pointer to the entity that is the **target** of this link.
    #[prost(message, optional, tag = "3")]
    pub target: ::core::option::Option<EntityReference>,
    /// The start of the first event establishing this link.
    #[prost(message, optional, tag = "4")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The end of the last event establishing this link.
    #[prost(message, optional, tag = "5")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request message for
/// \[BatchSearchLinkProcesses][google.cloud.datacatalog.lineage.v1.Lineage.BatchSearchLinkProcesses\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchSearchLinkProcessesRequest {
    /// Required. The project and location you want search in the format `projects/*/locations/*`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. An array of links to check for their associated LineageProcesses.
    ///
    /// The maximum number of items in this array is 100.
    /// If the request contains more than 100 links, it returns the
    /// `INVALID_ARGUMENT` error.
    ///
    /// Format: `projects/{project}/locations/{location}/links/{link}`.
    #[prost(string, repeated, tag = "2")]
    pub links: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The maximum number of processes to return in a single page of the response.
    /// A page may contain fewer results than this value.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// The page token received from a previous `BatchSearchLinkProcesses` call.
    /// Use it to get the next page.
    ///
    /// When requesting subsequent pages of a response, remember that
    /// all parameters must match the values you provided
    /// in the original request.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for
/// \[BatchSearchLinkProcesses][google.cloud.datacatalog.lineage.v1.Lineage.BatchSearchLinkProcesses\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchSearchLinkProcessesResponse {
    /// An array of processes associated with the specified links.
    #[prost(message, repeated, tag = "1")]
    pub process_links: ::prost::alloc::vec::Vec<ProcessLinks>,
    /// The token to specify as `page_token` in the subsequent call to get the next
    /// page. Omitted if there are no more pages in the response.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Links associated with a specific process.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessLinks {
    /// The process name in the format of
    /// `projects/{project}/locations/{location}/processes/{process}`.
    #[prost(string, tag = "1")]
    pub process: ::prost::alloc::string::String,
    /// An array containing link details objects of the links provided in
    /// the original request.
    ///
    /// A single process can result in creating multiple links.
    /// If any of the links you provide in the request are created by
    /// the same process, they all are included in this array.
    #[prost(message, repeated, tag = "2")]
    pub links: ::prost::alloc::vec::Vec<ProcessLinkInfo>,
}
/// Link details.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessLinkInfo {
    /// The name of the link in the format of
    /// `projects/{project}/locations/{location}/links/{link}`.
    #[prost(string, tag = "1")]
    pub link: ::prost::alloc::string::String,
    /// The start of the first event establishing this link-process tuple.
    #[prost(message, optional, tag = "2")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The end of the last event establishing this link-process tuple.
    #[prost(message, optional, tag = "3")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Origin of a process.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Origin {
    /// Type of the source.
    #[prost(enumeration = "origin::SourceType", tag = "1")]
    pub source_type: i32,
    /// If the source_type isn't CUSTOM, the value of this field should be a GCP
    /// resource name of the system, which reports lineage. The project and
    /// location parts of the resource name must match the project and location of
    /// the lineage resource being created. Examples:
    ///
    /// - `{source_type: COMPOSER, name:
    ///    "projects/foo/locations/us/environments/bar"}`
    /// - `{source_type: BIGQUERY, name: "projects/foo/locations/eu"}`
    /// - `{source_type: CUSTOM,   name: "myCustomIntegration"}`
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Origin`.
pub mod origin {
    /// Type of the source of a process.
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
    pub enum SourceType {
        /// Source is Unspecified
        Unspecified = 0,
        /// A custom source
        Custom = 1,
        /// BigQuery
        Bigquery = 2,
        /// Data Fusion
        DataFusion = 3,
        /// Composer
        Composer = 4,
        /// Looker Studio
        LookerStudio = 5,
    }
    impl SourceType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SourceType::Unspecified => "SOURCE_TYPE_UNSPECIFIED",
                SourceType::Custom => "CUSTOM",
                SourceType::Bigquery => "BIGQUERY",
                SourceType::DataFusion => "DATA_FUSION",
                SourceType::Composer => "COMPOSER",
                SourceType::LookerStudio => "LOOKER_STUDIO",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SOURCE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "CUSTOM" => Some(Self::Custom),
                "BIGQUERY" => Some(Self::Bigquery),
                "DATA_FUSION" => Some(Self::DataFusion),
                "COMPOSER" => Some(Self::Composer),
                "LOOKER_STUDIO" => Some(Self::LookerStudio),
                _ => None,
            }
        }
    }
}
/// Generated client implementations.
pub mod lineage_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Lineage is used to track data flows between assets over time. You can
    /// create [LineageEvents][google.cloud.datacatalog.lineage.v1.LineageEvent]
    /// to record lineage between multiple sources and a single target, for
    /// example, when table data is based on data from multiple tables.
    #[derive(Debug, Clone)]
    pub struct LineageClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl LineageClient<tonic::transport::Channel> {
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
    impl<T> LineageClient<T>
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
        ) -> LineageClient<InterceptedService<T, F>>
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
            LineageClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a new process.
        pub async fn create_process(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateProcessRequest>,
        ) -> std::result::Result<tonic::Response<super::Process>, tonic::Status> {
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
                "/google.cloud.datacatalog.lineage.v1.Lineage/CreateProcess",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.lineage.v1.Lineage",
                        "CreateProcess",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a process.
        pub async fn update_process(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateProcessRequest>,
        ) -> std::result::Result<tonic::Response<super::Process>, tonic::Status> {
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
                "/google.cloud.datacatalog.lineage.v1.Lineage/UpdateProcess",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.lineage.v1.Lineage",
                        "UpdateProcess",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the details of the specified process.
        pub async fn get_process(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProcessRequest>,
        ) -> std::result::Result<tonic::Response<super::Process>, tonic::Status> {
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
                "/google.cloud.datacatalog.lineage.v1.Lineage/GetProcess",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.lineage.v1.Lineage",
                        "GetProcess",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List processes in the given project and location. List order is descending
        /// by insertion time.
        pub async fn list_processes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListProcessesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListProcessesResponse>,
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
                "/google.cloud.datacatalog.lineage.v1.Lineage/ListProcesses",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.lineage.v1.Lineage",
                        "ListProcesses",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the process with the specified name.
        pub async fn delete_process(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteProcessRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.datacatalog.lineage.v1.Lineage/DeleteProcess",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.lineage.v1.Lineage",
                        "DeleteProcess",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new run.
        pub async fn create_run(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRunRequest>,
        ) -> std::result::Result<tonic::Response<super::Run>, tonic::Status> {
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
                "/google.cloud.datacatalog.lineage.v1.Lineage/CreateRun",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.lineage.v1.Lineage",
                        "CreateRun",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a run.
        pub async fn update_run(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateRunRequest>,
        ) -> std::result::Result<tonic::Response<super::Run>, tonic::Status> {
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
                "/google.cloud.datacatalog.lineage.v1.Lineage/UpdateRun",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.lineage.v1.Lineage",
                        "UpdateRun",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the details of the specified run.
        pub async fn get_run(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRunRequest>,
        ) -> std::result::Result<tonic::Response<super::Run>, tonic::Status> {
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
                "/google.cloud.datacatalog.lineage.v1.Lineage/GetRun",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.lineage.v1.Lineage",
                        "GetRun",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists runs in the given project and location. List order is descending by
        /// `start_time`.
        pub async fn list_runs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRunsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRunsResponse>,
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
                "/google.cloud.datacatalog.lineage.v1.Lineage/ListRuns",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.lineage.v1.Lineage",
                        "ListRuns",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the run with the specified name.
        pub async fn delete_run(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRunRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.datacatalog.lineage.v1.Lineage/DeleteRun",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.lineage.v1.Lineage",
                        "DeleteRun",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new lineage event.
        pub async fn create_lineage_event(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateLineageEventRequest>,
        ) -> std::result::Result<tonic::Response<super::LineageEvent>, tonic::Status> {
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
                "/google.cloud.datacatalog.lineage.v1.Lineage/CreateLineageEvent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.lineage.v1.Lineage",
                        "CreateLineageEvent",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a specified lineage event.
        pub async fn get_lineage_event(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLineageEventRequest>,
        ) -> std::result::Result<tonic::Response<super::LineageEvent>, tonic::Status> {
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
                "/google.cloud.datacatalog.lineage.v1.Lineage/GetLineageEvent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.lineage.v1.Lineage",
                        "GetLineageEvent",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists lineage events in the given project and location. The list order is
        /// not defined.
        pub async fn list_lineage_events(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLineageEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListLineageEventsResponse>,
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
                "/google.cloud.datacatalog.lineage.v1.Lineage/ListLineageEvents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.lineage.v1.Lineage",
                        "ListLineageEvents",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the lineage event with the specified name.
        pub async fn delete_lineage_event(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteLineageEventRequest>,
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
                "/google.cloud.datacatalog.lineage.v1.Lineage/DeleteLineageEvent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.lineage.v1.Lineage",
                        "DeleteLineageEvent",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieve a list of links connected to a specific asset.
        /// Links represent the data flow between **source** (upstream)
        /// and **target** (downstream) assets in transformation pipelines.
        /// Links are stored in the same project as the Lineage Events that create
        /// them.
        ///
        /// You can retrieve links in every project where you have the
        /// `datalineage.events.get` permission. The project provided in the URL
        /// is used for Billing and Quota.
        pub async fn search_links(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchLinksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SearchLinksResponse>,
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
                "/google.cloud.datacatalog.lineage.v1.Lineage/SearchLinks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.lineage.v1.Lineage",
                        "SearchLinks",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieve information about LineageProcesses associated with specific
        /// links. LineageProcesses are transformation pipelines that result in data
        /// flowing from **source** to **target** assets. Links between assets
        /// represent this operation.
        ///
        /// If you have specific link names, you can use this method to
        /// verify which LineageProcesses contribute to creating those links.
        /// See the
        /// [SearchLinks][google.cloud.datacatalog.lineage.v1.Lineage.SearchLinks]
        /// method for more information on how to retrieve link name.
        ///
        /// You can retrieve the LineageProcess information in every project where you
        /// have the `datalineage.events.get` permission. The project provided in the
        /// URL is used for Billing and Quota.
        pub async fn batch_search_link_processes(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchSearchLinkProcessesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchSearchLinkProcessesResponse>,
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
                "/google.cloud.datacatalog.lineage.v1.Lineage/BatchSearchLinkProcesses",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.datacatalog.lineage.v1.Lineage",
                        "BatchSearchLinkProcesses",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
