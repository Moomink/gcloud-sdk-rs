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
    /// of the operation. Operations that have been cancelled successfully
    /// have [Operation.error][] value with a
    /// [google.rpc.Status.code][google.rpc.Status.code] of 1, corresponding to
    /// `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
/// A Network Connectivity Center hub is a global management resource to which
/// you attach spokes. A single hub can contain spokes from multiple regions.
/// However, if any of a hub's spokes use the site-to-site data transfer feature,
/// the resources associated with those spokes must all be in the same VPC
/// network. Spokes that do not use site-to-site data transfer can be associated
/// with any VPC network in your project.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hub {
    /// Immutable. The name of the hub. Hub names must be unique. They use the
    /// following form:
    ///      `projects/{project_number}/locations/global/hubs/{hub_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The time the hub was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the hub was last updated.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional labels in key-value pair format. For more information about
    /// labels, see [Requirements for
    /// labels](<https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements>).
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// An optional description of the hub.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The Google-generated UUID for the hub. This value is unique
    /// across all hub resources. If a hub is deleted and another with the same
    /// name is created, the new hub is assigned a different unique_id.
    #[prost(string, tag = "8")]
    pub unique_id: ::prost::alloc::string::String,
    /// Output only. The current lifecycle state of this hub.
    #[prost(enumeration = "State", tag = "9")]
    pub state: i32,
    /// The VPC networks associated with this hub's spokes.
    ///
    /// This field is read-only. Network Connectivity Center automatically
    /// populates it based on the set of spokes attached to the hub.
    #[prost(message, repeated, tag = "10")]
    pub routing_vpcs: ::prost::alloc::vec::Vec<RoutingVpc>,
    /// Output only. The route tables that belong to this hub. They use the
    /// following form:
    ///     `projects/{project_number}/locations/global/hubs/{hub_id}/routeTables/{route_table_id}`
    ///
    /// This field is read-only. Network Connectivity Center automatically
    /// populates it based on the route tables nested under the hub.
    #[prost(string, repeated, tag = "11")]
    pub route_tables: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. A summary of the spokes associated with a hub. The
    /// summary includes a count of spokes according to type
    /// and according to state. If any spokes are inactive,
    /// the summary also lists the reasons they are inactive,
    /// including a count for each reason.
    #[prost(message, optional, tag = "12")]
    pub spoke_summary: ::core::option::Option<SpokeSummary>,
}
/// RoutingVPC contains information about the VPC networks associated
/// with the spokes of a Network Connectivity Center hub.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoutingVpc {
    /// The URI of the VPC network.
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    /// Output only. If true, indicates that this VPC network is currently
    /// associated with spokes that use the data transfer feature (spokes where the
    /// site_to_site_data_transfer field is set to true). If you create new spokes
    /// that use data transfer, they must be associated with this VPC network. At
    /// most, one VPC network will have this field set to true.
    #[prost(bool, tag = "2")]
    pub required_for_new_site_to_site_data_transfer_spokes: bool,
}
/// A Network Connectivity Center spoke represents one or more network
/// connectivity resources.
///
/// When you create a spoke, you associate it with a hub. You must also
/// identify a value for exactly one of the following fields:
///
/// * linked_vpn_tunnels
/// * linked_interconnect_attachments
/// * linked_router_appliance_instances
/// * linked_vpc_network
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Spoke {
    /// Immutable. The name of the spoke. Spoke names must be unique. They use the
    /// following form:
    ///      `projects/{project_number}/locations/{region}/spokes/{spoke_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The time the spoke was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the spoke was last updated.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional labels in key-value pair format. For more information about
    /// labels, see [Requirements for
    /// labels](<https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements>).
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// An optional description of the spoke.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    /// Immutable. The name of the hub that this spoke is attached to.
    #[prost(string, tag = "6")]
    pub hub: ::prost::alloc::string::String,
    /// Optional. The name of the group that this spoke is associated with.
    #[prost(string, tag = "23")]
    pub group: ::prost::alloc::string::String,
    /// VPN tunnels that are associated with the spoke.
    #[prost(message, optional, tag = "17")]
    pub linked_vpn_tunnels: ::core::option::Option<LinkedVpnTunnels>,
    /// VLAN attachments that are associated with the spoke.
    #[prost(message, optional, tag = "18")]
    pub linked_interconnect_attachments: ::core::option::Option<
        LinkedInterconnectAttachments,
    >,
    /// Router appliance instances that are associated with the spoke.
    #[prost(message, optional, tag = "19")]
    pub linked_router_appliance_instances: ::core::option::Option<
        LinkedRouterApplianceInstances,
    >,
    /// Optional. VPC network that is associated with the spoke.
    #[prost(message, optional, tag = "20")]
    pub linked_vpc_network: ::core::option::Option<LinkedVpcNetwork>,
    /// Output only. The Google-generated UUID for the spoke. This value is unique
    /// across all spoke resources. If a spoke is deleted and another with the same
    /// name is created, the new spoke is assigned a different `unique_id`.
    #[prost(string, tag = "11")]
    pub unique_id: ::prost::alloc::string::String,
    /// Output only. The current lifecycle state of this spoke.
    #[prost(enumeration = "State", tag = "15")]
    pub state: i32,
    /// Output only. The reasons for current state of the spoke. Only present when
    /// the spoke is in the `INACTIVE` state.
    #[prost(message, repeated, tag = "21")]
    pub reasons: ::prost::alloc::vec::Vec<spoke::StateReason>,
    /// Output only. The type of resource associated with the spoke.
    #[prost(enumeration = "SpokeType", tag = "22")]
    pub spoke_type: i32,
}
/// Nested message and enum types in `Spoke`.
pub mod spoke {
    /// The reason a spoke is inactive.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StateReason {
        /// The code associated with this reason.
        #[prost(enumeration = "state_reason::Code", tag = "1")]
        pub code: i32,
        /// Human-readable details about this reason.
        #[prost(string, tag = "2")]
        pub message: ::prost::alloc::string::String,
        /// Additional information provided by the user in the RejectSpoke call.
        #[prost(string, tag = "3")]
        pub user_details: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `StateReason`.
    pub mod state_reason {
        /// The Code enum represents the various reasons a state can be `INACTIVE`.
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
        pub enum Code {
            /// No information available.
            Unspecified = 0,
            /// The proposed spoke is pending review.
            PendingReview = 1,
            /// The proposed spoke has been rejected by the hub administrator.
            Rejected = 2,
            /// The spoke has been deactivated internally.
            Paused = 3,
            /// Network Connectivity Center encountered errors while accepting
            /// the spoke.
            Failed = 4,
        }
        impl Code {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Code::Unspecified => "CODE_UNSPECIFIED",
                    Code::PendingReview => "PENDING_REVIEW",
                    Code::Rejected => "REJECTED",
                    Code::Paused => "PAUSED",
                    Code::Failed => "FAILED",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "CODE_UNSPECIFIED" => Some(Self::Unspecified),
                    "PENDING_REVIEW" => Some(Self::PendingReview),
                    "REJECTED" => Some(Self::Rejected),
                    "PAUSED" => Some(Self::Paused),
                    "FAILED" => Some(Self::Failed),
                    _ => None,
                }
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteTable {
    /// Immutable. The name of the route table. Route table names must be unique.
    /// They use the following form:
    ///       `projects/{project_number}/locations/global/hubs/{hub}/routeTables/{route_table_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The time the route table was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the route table was last updated.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional labels in key-value pair format. For more information about
    /// labels, see [Requirements for
    /// labels](<https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements>).
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// An optional description of the route table.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The Google-generated UUID for the route table. This value is
    /// unique across all route table resources. If a route table is deleted and
    /// another with the same name is created, the new route table is assigned
    /// a different `uid`.
    #[prost(string, tag = "6")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. The current lifecycle state of this route table.
    #[prost(enumeration = "State", tag = "7")]
    pub state: i32,
}
/// A route defines a path from VM instances within a spoke to a specific
/// destination resource. Only VPC spokes have routes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Route {
    /// Immutable. The name of the route. Route names must be unique. Route names
    /// use the following form:
    ///       `projects/{project_number}/locations/global/hubs/{hub}/routeTables/{route_table_id}/routes/{route_id}`
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The time the route was created.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the route was last updated.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The destination IP address range.
    #[prost(string, tag = "1")]
    pub ip_cidr_range: ::prost::alloc::string::String,
    /// Output only. The route's type. Its type is determined by the properties of
    /// its IP address range.
    #[prost(enumeration = "RouteType", tag = "10")]
    pub r#type: i32,
    /// Immutable. The destination VPC network for packets on this route.
    #[prost(message, optional, tag = "2")]
    pub next_hop_vpc_network: ::core::option::Option<NextHopVpcNetwork>,
    /// Optional labels in key-value pair format. For more information about
    /// labels, see [Requirements for
    /// labels](<https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements>).
    #[prost(map = "string, string", tag = "6")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// An optional description of the route.
    #[prost(string, tag = "7")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The Google-generated UUID for the route. This value is unique
    /// across all Network Connectivity Center route resources. If a
    /// route is deleted and another with the same name is created,
    /// the new route is assigned a different `uid`.
    #[prost(string, tag = "8")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. The current lifecycle state of the route.
    #[prost(enumeration = "State", tag = "9")]
    pub state: i32,
    /// Immutable. The spoke that this route leads to.
    /// Example: projects/12345/locations/global/spokes/SPOKE
    #[prost(string, tag = "11")]
    pub spoke: ::prost::alloc::string::String,
    /// Output only. The location of the route.
    /// Uses the following form: "projects/{project}/locations/{location}"
    /// Example: projects/1234/locations/us-central1
    #[prost(string, tag = "12")]
    pub location: ::prost::alloc::string::String,
}
/// A group represents a subset of spokes attached to a hub.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Group {
    /// Immutable. The name of the group. Group names must be unique. They
    /// use the following form:
    ///       `projects/{project_number}/locations/global/hubs/{hub}/groups/{group_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The time the group was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the group was last updated.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Labels in key-value pair format. For more information about
    /// labels, see [Requirements for
    /// labels](<https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements>).
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Optional. The description of the group.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The Google-generated UUID for the group. This value is unique
    /// across all group resources. If a group is deleted and
    /// another with the same name is created, the new route table is assigned
    /// a different unique_id.
    #[prost(string, tag = "6")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. The current lifecycle state of this group.
    #[prost(enumeration = "State", tag = "7")]
    pub state: i32,
}
/// Request for
/// [HubService.ListHubs][google.cloud.networkconnectivity.v1.HubService.ListHubs]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListHubsRequest {
    /// Required. The parent resource's name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of results per page to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// An expression that filters the list of results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Sort the results by a certain order.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response for
/// [HubService.ListHubs][google.cloud.networkconnectivity.v1.HubService.ListHubs]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListHubsResponse {
    /// The requested hubs.
    #[prost(message, repeated, tag = "1")]
    pub hubs: ::prost::alloc::vec::Vec<Hub>,
    /// The token for the next page of the response. To see more results,
    /// use this value as the page_token for your next request. If this value
    /// is empty, there are no more results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for
/// [HubService.GetHub][google.cloud.networkconnectivity.v1.HubService.GetHub]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetHubRequest {
    /// Required. The name of the hub resource to get.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for
/// [HubService.CreateHub][google.cloud.networkconnectivity.v1.HubService.CreateHub]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateHubRequest {
    /// Required. The parent resource.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. A unique identifier for the hub.
    #[prost(string, tag = "2")]
    pub hub_id: ::prost::alloc::string::String,
    /// Required. The initial values for a new hub.
    #[prost(message, optional, tag = "3")]
    pub hub: ::core::option::Option<Hub>,
    /// Optional. A request ID to identify requests. Specify a unique request ID so
    /// that if you must retry your request, the server knows to ignore the request
    /// if it has already been completed. The server guarantees that a request
    /// doesn't result in creation of duplicate commitments for at least 60
    /// minutes.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check to see whether the original operation
    /// was received. If it was, the server ignores the second request. This
    /// behavior prevents clients from mistakenly creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID, with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request for
/// [HubService.UpdateHub][google.cloud.networkconnectivity.v1.HubService.UpdateHub]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateHubRequest {
    /// Optional. In the case of an update to an existing hub, field mask is used
    /// to specify the fields to be overwritten. The fields specified in the
    /// update_mask are relative to the resource, not the full request. A field is
    /// overwritten if it is in the mask. If the user does not provide a mask, then
    /// all fields are overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The state that the hub should be in after the update.
    #[prost(message, optional, tag = "2")]
    pub hub: ::core::option::Option<Hub>,
    /// Optional. A request ID to identify requests. Specify a unique request ID so
    /// that if you must retry your request, the server knows to ignore the request
    /// if it has already been completed. The server guarantees that a request
    /// doesn't result in creation of duplicate commitments for at least 60
    /// minutes.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check to see whether the original operation
    /// was received. If it was, the server ignores the second request. This
    /// behavior prevents clients from mistakenly creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID, with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// The request for
/// [HubService.DeleteHub][google.cloud.networkconnectivity.v1.HubService.DeleteHub].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteHubRequest {
    /// Required. The name of the hub to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A request ID to identify requests. Specify a unique request ID so
    /// that if you must retry your request, the server knows to ignore the request
    /// if it has already been completed. The server guarantees that a request
    /// doesn't result in creation of duplicate commitments for at least 60
    /// minutes.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check to see whether the original operation
    /// was received. If it was, the server ignores the second request. This
    /// behavior prevents clients from mistakenly creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID, with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// The request for
/// [HubService.ListHubSpokes][google.cloud.networkconnectivity.v1.HubService.ListHubSpokes].
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListHubSpokesRequest {
    /// Required. The name of the hub.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A list of locations.
    /// Specify one of the following: `\[global\]`, a single region (for
    /// example, `\[us-central1\]`), or a combination of
    /// values (for example, `\[global, us-central1, us-west1\]`).
    /// If the spoke_locations field is populated, the list of results
    /// includes only spokes in the specified location.
    /// If the spoke_locations field is not populated, the list of results
    /// includes spokes in all locations.
    #[prost(string, repeated, tag = "2")]
    pub spoke_locations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The maximum number of results to return per page.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// The page token.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
    /// An expression that filters the list of results.
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
    /// Sort the results by name or create_time.
    #[prost(string, tag = "6")]
    pub order_by: ::prost::alloc::string::String,
    /// The view of the spoke to return.
    /// The view that you use determines which spoke fields are included in the
    /// response.
    #[prost(enumeration = "list_hub_spokes_request::SpokeView", tag = "7")]
    pub view: i32,
}
/// Nested message and enum types in `ListHubSpokesRequest`.
pub mod list_hub_spokes_request {
    /// Enum that controls which spoke fields are included in the response.
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
    pub enum SpokeView {
        /// The spoke view is unspecified. When the spoke view is unspecified, the
        /// API returns the same fields as the `BASIC` view.
        Unspecified = 0,
        /// Includes `name`, `create_time`, `hub`, `unique_id`, `state`, `reasons`,
        /// and `spoke_type`. This is the default value.
        Basic = 1,
        /// Includes all spoke fields except `labels`.
        /// You can use the `DETAILED` view only when you set the `spoke_locations`
        /// field to `\[global\]`.
        Detailed = 2,
    }
    impl SpokeView {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SpokeView::Unspecified => "SPOKE_VIEW_UNSPECIFIED",
                SpokeView::Basic => "BASIC",
                SpokeView::Detailed => "DETAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SPOKE_VIEW_UNSPECIFIED" => Some(Self::Unspecified),
                "BASIC" => Some(Self::Basic),
                "DETAILED" => Some(Self::Detailed),
                _ => None,
            }
        }
    }
}
/// The response for
/// [HubService.ListHubSpokes][google.cloud.networkconnectivity.v1.HubService.ListHubSpokes].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListHubSpokesResponse {
    /// The requested spokes.
    /// The spoke fields can be partially populated based on the `view` field in
    /// the request message.
    #[prost(message, repeated, tag = "1")]
    pub spokes: ::prost::alloc::vec::Vec<Spoke>,
    /// The token for the next page of the response. To see more results,
    /// use this value as the page_token for your next request. If this value
    /// is empty, there are no more results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The request for
/// [HubService.ListSpokes][google.cloud.networkconnectivity.v1.HubService.ListSpokes].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSpokesRequest {
    /// Required. The parent resource.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of results to return per page.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// An expression that filters the list of results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Sort the results by a certain order.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// The response for
/// [HubService.ListSpokes][google.cloud.networkconnectivity.v1.HubService.ListSpokes].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSpokesResponse {
    /// The requested spokes.
    #[prost(message, repeated, tag = "1")]
    pub spokes: ::prost::alloc::vec::Vec<Spoke>,
    /// The token for the next page of the response. To see more results,
    /// use this value as the page_token for your next request. If this value
    /// is empty, there are no more results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The request for
/// [HubService.GetSpoke][google.cloud.networkconnectivity.v1.HubService.GetSpoke].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSpokeRequest {
    /// Required. The name of the spoke resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request for
/// [HubService.CreateSpoke][google.cloud.networkconnectivity.v1.HubService.CreateSpoke].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSpokeRequest {
    /// Required. The parent resource.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Unique id for the spoke to create.
    #[prost(string, tag = "2")]
    pub spoke_id: ::prost::alloc::string::String,
    /// Required. The initial values for a new spoke.
    #[prost(message, optional, tag = "3")]
    pub spoke: ::core::option::Option<Spoke>,
    /// Optional. A request ID to identify requests. Specify a unique request ID so
    /// that if you must retry your request, the server knows to ignore the request
    /// if it has already been completed. The server guarantees that a request
    /// doesn't result in creation of duplicate commitments for at least 60
    /// minutes.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check to see whether the original operation
    /// was received. If it was, the server ignores the second request. This
    /// behavior prevents clients from mistakenly creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID, with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request for
/// [HubService.UpdateSpoke][google.cloud.networkconnectivity.v1.HubService.UpdateSpoke]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSpokeRequest {
    /// Optional. In the case of an update to an existing spoke, field mask is used
    /// to specify the fields to be overwritten. The fields specified in the
    /// update_mask are relative to the resource, not the full request. A field is
    /// overwritten if it is in the mask. If the user does not provide a mask, then
    /// all fields are overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The state that the spoke should be in after the update.
    #[prost(message, optional, tag = "2")]
    pub spoke: ::core::option::Option<Spoke>,
    /// Optional. A request ID to identify requests. Specify a unique request ID so
    /// that if you must retry your request, the server knows to ignore the request
    /// if it has already been completed. The server guarantees that a request
    /// doesn't result in creation of duplicate commitments for at least 60
    /// minutes.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check to see whether the original operation
    /// was received. If it was, the server ignores the second request. This
    /// behavior prevents clients from mistakenly creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID, with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// The request for
/// [HubService.DeleteSpoke][google.cloud.networkconnectivity.v1.HubService.DeleteSpoke].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSpokeRequest {
    /// Required. The name of the spoke to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A request ID to identify requests. Specify a unique request ID so
    /// that if you must retry your request, the server knows to ignore the request
    /// if it has already been completed. The server guarantees that a request
    /// doesn't result in creation of duplicate commitments for at least 60
    /// minutes.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check to see whether the original operation
    /// was received. If it was, the server ignores the second request. This
    /// behavior prevents clients from mistakenly creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID, with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// The request for
/// [HubService.AcceptHubSpoke][google.cloud.networkconnectivity.v1.HubService.AcceptHubSpoke].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcceptHubSpokeRequest {
    /// Required. The name of the hub into which to accept the spoke.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The URI of the spoke to accept into the hub.
    #[prost(string, tag = "2")]
    pub spoke_uri: ::prost::alloc::string::String,
    /// Optional. A request ID to identify requests. Specify a unique request ID so
    /// that if you must retry your request, the server knows to ignore the request
    /// if it has already been completed. The server guarantees that a request
    /// doesn't result in creation of duplicate commitments for at least 60
    /// minutes.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check to see whether the original operation
    /// was received. If it was, the server ignores the second request. This
    /// behavior prevents clients from mistakenly creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID, with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// The response for
/// [HubService.AcceptHubSpoke][google.cloud.networkconnectivity.v1.HubService.AcceptHubSpoke].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcceptHubSpokeResponse {
    /// The spoke that was operated on.
    #[prost(message, optional, tag = "1")]
    pub spoke: ::core::option::Option<Spoke>,
}
/// The request for
/// [HubService.RejectHubSpoke][google.cloud.networkconnectivity.v1.HubService.RejectHubSpoke].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RejectHubSpokeRequest {
    /// Required. The name of the hub from which to reject the spoke.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The URI of the spoke to reject from the hub.
    #[prost(string, tag = "2")]
    pub spoke_uri: ::prost::alloc::string::String,
    /// Optional. A request ID to identify requests. Specify a unique request ID so
    /// that if you must retry your request, the server knows to ignore the request
    /// if it has already been completed. The server guarantees that a request
    /// doesn't result in creation of duplicate commitments for at least 60
    /// minutes.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check to see whether the original operation
    /// was received. If it was, the server ignores the second request. This
    /// behavior prevents clients from mistakenly creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID, with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. Additional information provided by the hub administrator.
    #[prost(string, tag = "4")]
    pub details: ::prost::alloc::string::String,
}
/// The response for
/// [HubService.RejectHubSpoke][google.cloud.networkconnectivity.v1.HubService.RejectHubSpoke].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RejectHubSpokeResponse {
    /// The spoke that was operated on.
    #[prost(message, optional, tag = "1")]
    pub spoke: ::core::option::Option<Spoke>,
}
/// The request for
/// [HubService.GetRouteTable][google.cloud.networkconnectivity.v1.HubService.GetRouteTable].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRouteTableRequest {
    /// Required. The name of the route table resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request for
/// [HubService.GetRoute][google.cloud.networkconnectivity.v1.HubService.GetRoute].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRouteRequest {
    /// Required. The name of the route resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for
/// [HubService.ListRoutes][google.cloud.networkconnectivity.v1.HubService.ListRoutes]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRoutesRequest {
    /// Required. The parent resource's name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of results to return per page.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// An expression that filters the list of results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Sort the results by a certain order.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response for
/// [HubService.ListRoutes][google.cloud.networkconnectivity.v1.HubService.ListRoutes]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRoutesResponse {
    /// The requested routes.
    #[prost(message, repeated, tag = "1")]
    pub routes: ::prost::alloc::vec::Vec<Route>,
    /// The token for the next page of the response. To see more results,
    /// use this value as the page_token for your next request. If this value
    /// is empty, there are no more results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// RouteTables that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for
/// [HubService.ListRouteTables][google.cloud.networkconnectivity.v1.HubService.ListRouteTables]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRouteTablesRequest {
    /// Required. The parent resource's name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of results to return per page.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// An expression that filters the list of results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Sort the results by a certain order.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response for
/// [HubService.ListRouteTables][google.cloud.networkconnectivity.v1.HubService.ListRouteTables]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRouteTablesResponse {
    /// The requested route tables.
    #[prost(message, repeated, tag = "1")]
    pub route_tables: ::prost::alloc::vec::Vec<RouteTable>,
    /// The token for the next page of the response. To see more results,
    /// use this value as the page_token for your next request. If this value
    /// is empty, there are no more results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Hubs that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for
/// [HubService.ListGroups][google.cloud.networkconnectivity.v1.HubService.ListGroups]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGroupsRequest {
    /// Required. The parent resource's name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of results to return per page.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// An expression that filters the list of results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Sort the results by a certain order.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response for
/// [HubService.ListGroups][google.cloud.networkconnectivity.v1.HubService.ListGroups]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGroupsResponse {
    /// The requested groups.
    #[prost(message, repeated, tag = "1")]
    pub groups: ::prost::alloc::vec::Vec<Group>,
    /// The token for the next page of the response. To see more results,
    /// use this value as the page_token for your next request. If this value
    /// is empty, there are no more results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Hubs that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A collection of Cloud VPN tunnel resources. These resources should be
/// redundant HA VPN tunnels that all advertise the same prefixes to Google
/// Cloud. Alternatively, in a passive/active configuration, all tunnels
/// should be capable of advertising the same prefixes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinkedVpnTunnels {
    /// The URIs of linked VPN tunnel resources.
    #[prost(string, repeated, tag = "1")]
    pub uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A value that controls whether site-to-site data transfer is enabled for
    /// these resources. Data transfer is available only in [supported
    /// locations](<https://cloud.google.com/network-connectivity/docs/network-connectivity-center/concepts/locations>).
    #[prost(bool, tag = "2")]
    pub site_to_site_data_transfer: bool,
    /// Output only. The VPC network where these VPN tunnels are located.
    #[prost(string, tag = "3")]
    pub vpc_network: ::prost::alloc::string::String,
}
/// A collection of VLAN attachment resources. These resources should
/// be redundant attachments that all advertise the same prefixes to Google
/// Cloud. Alternatively, in active/passive configurations, all attachments
/// should be capable of advertising the same prefixes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinkedInterconnectAttachments {
    /// The URIs of linked interconnect attachment resources
    #[prost(string, repeated, tag = "1")]
    pub uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A value that controls whether site-to-site data transfer is enabled for
    /// these resources. Data transfer is available only in [supported
    /// locations](<https://cloud.google.com/network-connectivity/docs/network-connectivity-center/concepts/locations>).
    #[prost(bool, tag = "2")]
    pub site_to_site_data_transfer: bool,
    /// Output only. The VPC network where these VLAN attachments are located.
    #[prost(string, tag = "3")]
    pub vpc_network: ::prost::alloc::string::String,
}
/// A collection of router appliance instances. If you configure multiple router
/// appliance instances to receive data from the same set of sites outside of
/// Google Cloud, we recommend that you associate those instances with the same
/// spoke.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinkedRouterApplianceInstances {
    /// The list of router appliance instances.
    #[prost(message, repeated, tag = "1")]
    pub instances: ::prost::alloc::vec::Vec<RouterApplianceInstance>,
    /// A value that controls whether site-to-site data transfer is enabled for
    /// these resources. Data transfer is available only in [supported
    /// locations](<https://cloud.google.com/network-connectivity/docs/network-connectivity-center/concepts/locations>).
    #[prost(bool, tag = "2")]
    pub site_to_site_data_transfer: bool,
    /// Output only. The VPC network where these router appliance instances are
    /// located.
    #[prost(string, tag = "3")]
    pub vpc_network: ::prost::alloc::string::String,
}
/// An existing VPC network.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinkedVpcNetwork {
    /// Required. The URI of the VPC network resource.
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    /// Optional. IP ranges encompassing the subnets to be excluded from peering.
    #[prost(string, repeated, tag = "2")]
    pub exclude_export_ranges: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A router appliance instance is a Compute Engine virtual machine (VM) instance
/// that acts as a BGP speaker. A router appliance instance is specified by the
/// URI of the VM and the internal IP address of one of the VM's network
/// interfaces.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouterApplianceInstance {
    /// The URI of the VM.
    #[prost(string, tag = "1")]
    pub virtual_machine: ::prost::alloc::string::String,
    /// The IP address on the VM to use for peering.
    #[prost(string, tag = "3")]
    pub ip_address: ::prost::alloc::string::String,
}
/// Metadata about locations
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationMetadata {
    /// List of supported features
    #[prost(enumeration = "LocationFeature", repeated, tag = "1")]
    pub location_features: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NextHopVpcNetwork {
    /// The URI of the VPC network resource
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
}
/// Summarizes information about the spokes associated with a hub.
/// The summary includes a count of spokes according to type
/// and according to state. If any spokes are inactive,
/// the summary also lists the reasons they are inactive,
/// including a count for each reason.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpokeSummary {
    /// Output only. Counts the number of spokes of each type that are
    /// associated with a specific hub.
    #[prost(message, repeated, tag = "1")]
    pub spoke_type_counts: ::prost::alloc::vec::Vec<spoke_summary::SpokeTypeCount>,
    /// Output only. Counts the number of spokes that are in each state
    /// and associated with a given hub.
    #[prost(message, repeated, tag = "2")]
    pub spoke_state_counts: ::prost::alloc::vec::Vec<spoke_summary::SpokeStateCount>,
    /// Output only. Counts the number of spokes that are inactive for each
    /// possible reason and associated with a given hub.
    #[prost(message, repeated, tag = "3")]
    pub spoke_state_reason_counts: ::prost::alloc::vec::Vec<
        spoke_summary::SpokeStateReasonCount,
    >,
}
/// Nested message and enum types in `SpokeSummary`.
pub mod spoke_summary {
    /// The number of spokes of a given type that are associated
    /// with a specific hub. The type indicates what kind of
    /// resource is associated with the spoke.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SpokeTypeCount {
        /// Output only. The type of the spokes.
        #[prost(enumeration = "super::SpokeType", tag = "1")]
        pub spoke_type: i32,
        /// Output only. The total number of spokes of this type that are
        /// associated with the hub.
        #[prost(int64, tag = "2")]
        pub count: i64,
    }
    /// The number of spokes that are in a particular state
    /// and associated with a given hub.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SpokeStateCount {
        /// Output only. The state of the spokes.
        #[prost(enumeration = "super::State", tag = "1")]
        pub state: i32,
        /// Output only. The total number of spokes that are in this state
        /// and associated with a given hub.
        #[prost(int64, tag = "2")]
        pub count: i64,
    }
    /// The number of spokes in the hub that are inactive for this reason.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SpokeStateReasonCount {
        /// Output only. The reason that a spoke is inactive.
        #[prost(enumeration = "super::spoke::state_reason::Code", tag = "1")]
        pub state_reason_code: i32,
        /// Output only. The total number of spokes that are inactive for a
        /// particular reason and associated with a given hub.
        #[prost(int64, tag = "2")]
        pub count: i64,
    }
}
/// The request for
/// [HubService.GetGroup][google.cloud.networkconnectivity.v1.HubService.GetGroup].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGroupRequest {
    /// Required. The name of the route table resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Supported features for a location
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LocationFeature {
    /// No publicly supported feature in this location
    Unspecified = 0,
    /// Site-to-cloud spokes are supported in this location
    SiteToCloudSpokes = 1,
    /// Site-to-site spokes are supported in this location
    SiteToSiteSpokes = 2,
}
impl LocationFeature {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LocationFeature::Unspecified => "LOCATION_FEATURE_UNSPECIFIED",
            LocationFeature::SiteToCloudSpokes => "SITE_TO_CLOUD_SPOKES",
            LocationFeature::SiteToSiteSpokes => "SITE_TO_SITE_SPOKES",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LOCATION_FEATURE_UNSPECIFIED" => Some(Self::Unspecified),
            "SITE_TO_CLOUD_SPOKES" => Some(Self::SiteToCloudSpokes),
            "SITE_TO_SITE_SPOKES" => Some(Self::SiteToSiteSpokes),
            _ => None,
        }
    }
}
/// The route's type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RouteType {
    /// No route type information specified
    Unspecified = 0,
    /// The route leads to a destination within the primary address range of the
    /// VPC network's subnet.
    VpcPrimarySubnet = 1,
    /// The route leads to a destination within the secondary address range of the
    /// VPC network's subnet.
    VpcSecondarySubnet = 2,
}
impl RouteType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RouteType::Unspecified => "ROUTE_TYPE_UNSPECIFIED",
            RouteType::VpcPrimarySubnet => "VPC_PRIMARY_SUBNET",
            RouteType::VpcSecondarySubnet => "VPC_SECONDARY_SUBNET",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ROUTE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "VPC_PRIMARY_SUBNET" => Some(Self::VpcPrimarySubnet),
            "VPC_SECONDARY_SUBNET" => Some(Self::VpcSecondarySubnet),
            _ => None,
        }
    }
}
/// The State enum represents the lifecycle stage of a Network Connectivity
/// Center resource.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum State {
    /// No state information available
    Unspecified = 0,
    /// The resource's create operation is in progress.
    Creating = 1,
    /// The resource is active
    Active = 2,
    /// The resource's delete operation is in progress.
    Deleting = 3,
    /// The resource's accept operation is in progress.
    Accepting = 8,
    /// The resource's reject operation is in progress.
    Rejecting = 9,
    /// The resource's update operation is in progress.
    Updating = 6,
    /// The resource is inactive.
    Inactive = 7,
    /// The hub associated with this spoke resource has been deleted.
    /// This state applies to spoke resources only.
    Obsolete = 10,
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
            State::Deleting => "DELETING",
            State::Accepting => "ACCEPTING",
            State::Rejecting => "REJECTING",
            State::Updating => "UPDATING",
            State::Inactive => "INACTIVE",
            State::Obsolete => "OBSOLETE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "CREATING" => Some(Self::Creating),
            "ACTIVE" => Some(Self::Active),
            "DELETING" => Some(Self::Deleting),
            "ACCEPTING" => Some(Self::Accepting),
            "REJECTING" => Some(Self::Rejecting),
            "UPDATING" => Some(Self::Updating),
            "INACTIVE" => Some(Self::Inactive),
            "OBSOLETE" => Some(Self::Obsolete),
            _ => None,
        }
    }
}
/// The SpokeType enum represents the type of spoke. The type
/// reflects the kind of resource that a spoke is associated with.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SpokeType {
    /// Unspecified spoke type.
    Unspecified = 0,
    /// Spokes associated with VPN tunnels.
    VpnTunnel = 1,
    /// Spokes associated with VLAN attachments.
    InterconnectAttachment = 2,
    /// Spokes associated with router appliance instances.
    RouterAppliance = 3,
    /// Spokes associated with VPC networks.
    VpcNetwork = 4,
}
impl SpokeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SpokeType::Unspecified => "SPOKE_TYPE_UNSPECIFIED",
            SpokeType::VpnTunnel => "VPN_TUNNEL",
            SpokeType::InterconnectAttachment => "INTERCONNECT_ATTACHMENT",
            SpokeType::RouterAppliance => "ROUTER_APPLIANCE",
            SpokeType::VpcNetwork => "VPC_NETWORK",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SPOKE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "VPN_TUNNEL" => Some(Self::VpnTunnel),
            "INTERCONNECT_ATTACHMENT" => Some(Self::InterconnectAttachment),
            "ROUTER_APPLIANCE" => Some(Self::RouterAppliance),
            "VPC_NETWORK" => Some(Self::VpcNetwork),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod hub_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Network Connectivity Center is a hub-and-spoke abstraction for network
    /// connectivity management in Google Cloud. It reduces operational complexity
    /// through a simple, centralized connectivity management model.
    #[derive(Debug, Clone)]
    pub struct HubServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl HubServiceClient<tonic::transport::Channel> {
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
    impl<T> HubServiceClient<T>
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
        ) -> HubServiceClient<InterceptedService<T, F>>
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
            HubServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists the Network Connectivity Center hubs associated with a given project.
        pub async fn list_hubs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListHubsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListHubsResponse>,
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
                "/google.cloud.networkconnectivity.v1.HubService/ListHubs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkconnectivity.v1.HubService",
                        "ListHubs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details about a Network Connectivity Center hub.
        pub async fn get_hub(
            &mut self,
            request: impl tonic::IntoRequest<super::GetHubRequest>,
        ) -> std::result::Result<tonic::Response<super::Hub>, tonic::Status> {
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
                "/google.cloud.networkconnectivity.v1.HubService/GetHub",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkconnectivity.v1.HubService",
                        "GetHub",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new Network Connectivity Center hub in the specified project.
        pub async fn create_hub(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateHubRequest>,
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
                "/google.cloud.networkconnectivity.v1.HubService/CreateHub",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkconnectivity.v1.HubService",
                        "CreateHub",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the description and/or labels of a Network Connectivity Center
        /// hub.
        pub async fn update_hub(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateHubRequest>,
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
                "/google.cloud.networkconnectivity.v1.HubService/UpdateHub",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkconnectivity.v1.HubService",
                        "UpdateHub",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a Network Connectivity Center hub.
        pub async fn delete_hub(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteHubRequest>,
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
                "/google.cloud.networkconnectivity.v1.HubService/DeleteHub",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkconnectivity.v1.HubService",
                        "DeleteHub",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists the Network Connectivity Center spokes associated with a
        /// specified hub and location. The list includes both spokes that are attached
        /// to the hub and spokes that have been proposed but not yet accepted.
        pub async fn list_hub_spokes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListHubSpokesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListHubSpokesResponse>,
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
                "/google.cloud.networkconnectivity.v1.HubService/ListHubSpokes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkconnectivity.v1.HubService",
                        "ListHubSpokes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists the Network Connectivity Center spokes in a specified project and
        /// location.
        pub async fn list_spokes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSpokesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListSpokesResponse>,
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
                "/google.cloud.networkconnectivity.v1.HubService/ListSpokes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkconnectivity.v1.HubService",
                        "ListSpokes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details about a Network Connectivity Center spoke.
        pub async fn get_spoke(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSpokeRequest>,
        ) -> std::result::Result<tonic::Response<super::Spoke>, tonic::Status> {
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
                "/google.cloud.networkconnectivity.v1.HubService/GetSpoke",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkconnectivity.v1.HubService",
                        "GetSpoke",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a Network Connectivity Center spoke.
        pub async fn create_spoke(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSpokeRequest>,
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
                "/google.cloud.networkconnectivity.v1.HubService/CreateSpoke",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkconnectivity.v1.HubService",
                        "CreateSpoke",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the parameters of a Network Connectivity Center spoke.
        pub async fn update_spoke(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSpokeRequest>,
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
                "/google.cloud.networkconnectivity.v1.HubService/UpdateSpoke",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkconnectivity.v1.HubService",
                        "UpdateSpoke",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Rejects a Network Connectivity Center spoke from being attached to a hub.
        /// If the spoke was previously in the `ACTIVE` state, it
        /// transitions to the `INACTIVE` state and is no longer able to
        /// connect to other spokes that are attached to the hub.
        pub async fn reject_hub_spoke(
            &mut self,
            request: impl tonic::IntoRequest<super::RejectHubSpokeRequest>,
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
                "/google.cloud.networkconnectivity.v1.HubService/RejectHubSpoke",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkconnectivity.v1.HubService",
                        "RejectHubSpoke",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Accepts a proposal to attach a Network Connectivity Center spoke
        /// to a hub.
        pub async fn accept_hub_spoke(
            &mut self,
            request: impl tonic::IntoRequest<super::AcceptHubSpokeRequest>,
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
                "/google.cloud.networkconnectivity.v1.HubService/AcceptHubSpoke",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkconnectivity.v1.HubService",
                        "AcceptHubSpoke",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a Network Connectivity Center spoke.
        pub async fn delete_spoke(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSpokeRequest>,
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
                "/google.cloud.networkconnectivity.v1.HubService/DeleteSpoke",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkconnectivity.v1.HubService",
                        "DeleteSpoke",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details about a Network Connectivity Center route table.
        pub async fn get_route_table(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRouteTableRequest>,
        ) -> std::result::Result<tonic::Response<super::RouteTable>, tonic::Status> {
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
                "/google.cloud.networkconnectivity.v1.HubService/GetRouteTable",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkconnectivity.v1.HubService",
                        "GetRouteTable",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details about the specified route.
        pub async fn get_route(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRouteRequest>,
        ) -> std::result::Result<tonic::Response<super::Route>, tonic::Status> {
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
                "/google.cloud.networkconnectivity.v1.HubService/GetRoute",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkconnectivity.v1.HubService",
                        "GetRoute",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists routes in a given project.
        pub async fn list_routes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRoutesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRoutesResponse>,
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
                "/google.cloud.networkconnectivity.v1.HubService/ListRoutes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkconnectivity.v1.HubService",
                        "ListRoutes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists route tables in a given project.
        pub async fn list_route_tables(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRouteTablesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRouteTablesResponse>,
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
                "/google.cloud.networkconnectivity.v1.HubService/ListRouteTables",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkconnectivity.v1.HubService",
                        "ListRouteTables",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details about a Network Connectivity Center group.
        pub async fn get_group(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGroupRequest>,
        ) -> std::result::Result<tonic::Response<super::Group>, tonic::Status> {
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
                "/google.cloud.networkconnectivity.v1.HubService/GetGroup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkconnectivity.v1.HubService",
                        "GetGroup",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists groups in a given hub.
        pub async fn list_groups(
            &mut self,
            request: impl tonic::IntoRequest<super::ListGroupsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListGroupsResponse>,
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
                "/google.cloud.networkconnectivity.v1.HubService/ListGroups",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkconnectivity.v1.HubService",
                        "ListGroups",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Policy Based Routes (PBR) are more powerful routes that allows GCP customers
/// to route their L4 network traffic based on not just destination IP, but also
/// source IP, protocol and more. A PBR always take precedence when it conflicts
/// with other types of routes.
/// Next id: 22
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyBasedRoute {
    /// Immutable. A unique name of the resource in the form of
    /// `projects/{project_number}/locations/global/PolicyBasedRoutes/{policy_based_route_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Time when the PolicyBasedRoute was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when the PolicyBasedRoute was updated.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// User-defined labels.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Optional. An optional description of this resource. Provide this field when
    /// you create the resource.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    /// Required. Fully-qualified URL of the network that this route applies to.
    /// e.g. projects/my-project/global/networks/my-network.
    #[prost(string, tag = "6")]
    pub network: ::prost::alloc::string::String,
    /// Required. The filter to match L4 traffic.
    #[prost(message, optional, tag = "10")]
    pub filter: ::core::option::Option<policy_based_route::Filter>,
    /// Optional. The priority of this policy based route. Priority is used to
    /// break ties in cases where there are more than one matching policy based
    /// routes found. In cases where multiple policy based routes are matched, the
    /// one with the lowest-numbered priority value wins. The default value is
    /// 1000. The priority value must be from 1 to 65535, inclusive.
    #[prost(int32, tag = "11")]
    pub priority: i32,
    /// Output only. If potential misconfigurations are detected for this route,
    /// this field will be populated with warning messages.
    #[prost(message, repeated, tag = "14")]
    pub warnings: ::prost::alloc::vec::Vec<policy_based_route::Warnings>,
    /// Output only. Server-defined fully-qualified URL for this resource.
    #[prost(string, tag = "15")]
    pub self_link: ::prost::alloc::string::String,
    /// Output only. Type of this resource. Always
    /// networkconnectivity#policyBasedRoute for Policy Based Route resources.
    #[prost(string, tag = "16")]
    pub kind: ::prost::alloc::string::String,
    /// Target specifies network endpoints to which this policy based route applies
    /// to. If none of the target is specified, the PBR will be installed on all
    /// network endpoints (e.g. VMs, VPNs, and Interconnects) in the VPC.
    #[prost(oneof = "policy_based_route::Target", tags = "18, 9")]
    pub target: ::core::option::Option<policy_based_route::Target>,
    #[prost(oneof = "policy_based_route::NextHop", tags = "12, 21")]
    pub next_hop: ::core::option::Option<policy_based_route::NextHop>,
}
/// Nested message and enum types in `PolicyBasedRoute`.
pub mod policy_based_route {
    /// VM instances to which this policy based route applies to.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct VirtualMachine {
        /// Optional. A list of VM instance tags to which this policy based route
        /// applies to. VM instances that have ANY of tags specified here will
        /// install this PBR.
        #[prost(string, repeated, tag = "1")]
        pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// InterconnectAttachment to which this route applies to.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InterconnectAttachment {
        /// Optional. Cloud region to install this policy based route on interconnect
        /// attachment. Use `all` to install it on all interconnect attachments.
        #[prost(string, tag = "1")]
        pub region: ::prost::alloc::string::String,
    }
    /// Filter matches L4 traffic.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Filter {
        /// Optional. The IP protocol that this policy based route applies to. Valid
        /// values are 'TCP', 'UDP', and 'ALL'. Default is 'ALL'.
        #[prost(string, tag = "1")]
        pub ip_protocol: ::prost::alloc::string::String,
        /// Optional. The source IP range of outgoing packets that this policy based
        /// route applies to. Default is "0.0.0.0/0" if protocol version is IPv4.
        #[prost(string, tag = "2")]
        pub src_range: ::prost::alloc::string::String,
        /// Optional. The destination IP range of outgoing packets that this policy
        /// based route applies to. Default is "0.0.0.0/0" if protocol version is
        /// IPv4.
        #[prost(string, tag = "3")]
        pub dest_range: ::prost::alloc::string::String,
        /// Required. Internet protocol versions this policy based route applies to.
        /// For this version, only IPV4 is supported.
        #[prost(enumeration = "filter::ProtocolVersion", tag = "6")]
        pub protocol_version: i32,
    }
    /// Nested message and enum types in `Filter`.
    pub mod filter {
        /// The internet protocol version.
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
        pub enum ProtocolVersion {
            /// Default value.
            Unspecified = 0,
            /// The PBR is for IPv4 internet protocol traffic.
            Ipv4 = 1,
        }
        impl ProtocolVersion {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    ProtocolVersion::Unspecified => "PROTOCOL_VERSION_UNSPECIFIED",
                    ProtocolVersion::Ipv4 => "IPV4",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "PROTOCOL_VERSION_UNSPECIFIED" => Some(Self::Unspecified),
                    "IPV4" => Some(Self::Ipv4),
                    _ => None,
                }
            }
        }
    }
    /// Informational warning message.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Warnings {
        /// Output only. A warning code, if applicable.
        #[prost(enumeration = "warnings::Code", tag = "1")]
        pub code: i32,
        /// Output only. Metadata about this warning in key: value format. The key
        /// should provides more detail on the warning being returned. For example,
        /// for warnings where there are no results in a list request for a
        /// particular zone, this key might be scope and the key value might be the
        /// zone name. Other examples might be a key indicating a deprecated resource
        /// and a suggested replacement.
        #[prost(map = "string, string", tag = "2")]
        pub data: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
        /// Output only. A human-readable description of the warning code.
        #[prost(string, tag = "3")]
        pub warning_message: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `Warnings`.
    pub mod warnings {
        /// Warning code for Policy Based Routing. Expect to add values in the
        /// future.
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
        pub enum Code {
            /// Default value.
            WarningUnspecified = 0,
            /// The policy based route is not active and functioning. Common causes are
            /// the dependent network was deleted or the resource project was turned
            /// off.
            ResourceNotActive = 1,
            /// The policy based route is being modified (e.g. created/deleted) at this
            /// time.
            ResourceBeingModified = 2,
        }
        impl Code {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Code::WarningUnspecified => "WARNING_UNSPECIFIED",
                    Code::ResourceNotActive => "RESOURCE_NOT_ACTIVE",
                    Code::ResourceBeingModified => "RESOURCE_BEING_MODIFIED",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "WARNING_UNSPECIFIED" => Some(Self::WarningUnspecified),
                    "RESOURCE_NOT_ACTIVE" => Some(Self::ResourceNotActive),
                    "RESOURCE_BEING_MODIFIED" => Some(Self::ResourceBeingModified),
                    _ => None,
                }
            }
        }
    }
    /// The other routing cases.
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
    pub enum OtherRoutes {
        /// Default value.
        Unspecified = 0,
        /// Use the routes from the default routing tables (system-generated routes,
        /// custom routes, peering route) to determine the next hop. This will
        /// effectively exclude matching packets being applied on other PBRs with a
        /// lower priority.
        DefaultRouting = 1,
    }
    impl OtherRoutes {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                OtherRoutes::Unspecified => "OTHER_ROUTES_UNSPECIFIED",
                OtherRoutes::DefaultRouting => "DEFAULT_ROUTING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OTHER_ROUTES_UNSPECIFIED" => Some(Self::Unspecified),
                "DEFAULT_ROUTING" => Some(Self::DefaultRouting),
                _ => None,
            }
        }
    }
    /// Target specifies network endpoints to which this policy based route applies
    /// to. If none of the target is specified, the PBR will be installed on all
    /// network endpoints (e.g. VMs, VPNs, and Interconnects) in the VPC.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Target {
        /// Optional. VM instances to which this policy based route applies to.
        #[prost(message, tag = "18")]
        VirtualMachine(VirtualMachine),
        /// Optional. The interconnect attachments to which this route applies to.
        #[prost(message, tag = "9")]
        InterconnectAttachment(InterconnectAttachment),
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum NextHop {
        /// Optional. The IP of a global access enabled L4 ILB that should be the
        /// next hop to handle matching packets. For this version, only
        /// next_hop_ilb_ip is supported.
        #[prost(string, tag = "12")]
        NextHopIlbIp(::prost::alloc::string::String),
        /// Optional. Other routes that will be referenced to determine the next hop
        /// of the packet.
        #[prost(enumeration = "OtherRoutes", tag = "21")]
        NextHopOtherRoutes(i32),
    }
}
/// Request for [PolicyBasedRouting.ListPolicyBasedRoutes][] method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPolicyBasedRoutesRequest {
    /// Required. The parent resource's name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of results per page that should be returned.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// A filter expression that filters the results listed in the response.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Sort the results by a certain order.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response for [PolicyBasedRouting.ListPolicyBasedRoutes][] method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPolicyBasedRoutesResponse {
    /// Policy based routes to be returned.
    #[prost(message, repeated, tag = "1")]
    pub policy_based_routes: ::prost::alloc::vec::Vec<PolicyBasedRoute>,
    /// The next pagination token in the List response. It should be used as
    /// page_token for the following request. An empty value means no more result.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for [PolicyBasedRouting.GetPolicyBasedRoute][] method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPolicyBasedRouteRequest {
    /// Required. Name of the PolicyBasedRoute resource to get.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for [PolicyBasedRouting.CreatePolicyBasedRoute][] method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePolicyBasedRouteRequest {
    /// Required. The parent resource's name of the PolicyBasedRoute.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Unique id for the Policy Based Route to create.
    #[prost(string, tag = "2")]
    pub policy_based_route_id: ::prost::alloc::string::String,
    /// Required. Initial values for a new Policy Based Route.
    #[prost(message, optional, tag = "3")]
    pub policy_based_route: ::core::option::Option<PolicyBasedRoute>,
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
}
/// Request for [PolicyBasedRouting.DeletePolicyBasedRoute][] method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePolicyBasedRouteRequest {
    /// Required. Name of the PolicyBasedRoute resource to delete.
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
}
/// Generated client implementations.
pub mod policy_based_routing_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Policy-Based Routing allows GCP customers to specify flexibile routing
    /// policies for Layer 4 traffic traversing through the connected service.
    #[derive(Debug, Clone)]
    pub struct PolicyBasedRoutingServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PolicyBasedRoutingServiceClient<tonic::transport::Channel> {
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
    impl<T> PolicyBasedRoutingServiceClient<T>
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
        ) -> PolicyBasedRoutingServiceClient<InterceptedService<T, F>>
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
            PolicyBasedRoutingServiceClient::new(
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
        /// Lists PolicyBasedRoutes in a given project and location.
        pub async fn list_policy_based_routes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPolicyBasedRoutesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPolicyBasedRoutesResponse>,
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
                "/google.cloud.networkconnectivity.v1.PolicyBasedRoutingService/ListPolicyBasedRoutes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkconnectivity.v1.PolicyBasedRoutingService",
                        "ListPolicyBasedRoutes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single PolicyBasedRoute.
        pub async fn get_policy_based_route(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPolicyBasedRouteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PolicyBasedRoute>,
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
                "/google.cloud.networkconnectivity.v1.PolicyBasedRoutingService/GetPolicyBasedRoute",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkconnectivity.v1.PolicyBasedRoutingService",
                        "GetPolicyBasedRoute",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new PolicyBasedRoute in a given project and location.
        pub async fn create_policy_based_route(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePolicyBasedRouteRequest>,
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
                "/google.cloud.networkconnectivity.v1.PolicyBasedRoutingService/CreatePolicyBasedRoute",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkconnectivity.v1.PolicyBasedRoutingService",
                        "CreatePolicyBasedRoute",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single PolicyBasedRoute.
        pub async fn delete_policy_based_route(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePolicyBasedRouteRequest>,
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
                "/google.cloud.networkconnectivity.v1.PolicyBasedRoutingService/DeletePolicyBasedRoute",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkconnectivity.v1.PolicyBasedRoutingService",
                        "DeletePolicyBasedRoute",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
