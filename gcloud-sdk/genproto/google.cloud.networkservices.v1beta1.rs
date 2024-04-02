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
/// Specification of a port-based selector.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrafficPortSelector {
    /// Optional. A list of ports. Can be port numbers or port range
    /// (example, \[80-90\] specifies all ports from 80 to 90, including
    /// 80 and 90) or named ports or * to specify all ports. If the
    /// list is empty, all ports are selected.
    #[prost(string, repeated, tag = "1")]
    pub ports: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A definition of a matcher that selects endpoints to which the policies
/// should be applied.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndpointMatcher {
    /// Specifies type of the matcher used for this endpoint matcher.
    #[prost(oneof = "endpoint_matcher::MatcherType", tags = "1")]
    pub matcher_type: ::core::option::Option<endpoint_matcher::MatcherType>,
}
/// Nested message and enum types in `EndpointMatcher`.
pub mod endpoint_matcher {
    /// The matcher that is based on node metadata presented by xDS clients.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MetadataLabelMatcher {
        /// Specifies how matching should be done.
        ///
        /// Supported values are:
        /// MATCH_ANY: At least one of the Labels specified in the
        ///    matcher should match the metadata presented by xDS client.
        /// MATCH_ALL: The metadata presented by the xDS client should
        ///    contain all of the labels specified here.
        ///
        /// The selection is determined based on the best match. For
        /// example, suppose there are three EndpointPolicy
        /// resources P1, P2 and P3 and if P1 has a the matcher as
        /// MATCH_ANY <A:1, B:1>, P2 has MATCH_ALL <A:1,B:1>, and P3 has
        /// MATCH_ALL <A:1,B:1,C:1>.
        ///
        /// If a client with label <A:1> connects, the config from P1
        /// will be selected.
        ///
        /// If a client with label <A:1,B:1> connects, the config from P2
        /// will be selected.
        ///
        /// If a client with label <A:1,B:1,C:1> connects, the config
        /// from P3 will be selected.
        ///
        /// If there is more than one best match, (for example, if a
        /// config P4 with selector <A:1,D:1> exists and if a client with
        /// label <A:1,B:1,D:1> connects), an error will be thrown.
        #[prost(
            enumeration = "metadata_label_matcher::MetadataLabelMatchCriteria",
            tag = "1"
        )]
        pub metadata_label_match_criteria: i32,
        /// The list of label value pairs that must match labels in the
        /// provided metadata based on filterMatchCriteria This list can
        /// have at most 64 entries. The list can be empty if the match
        /// criteria is MATCH_ANY, to specify a wildcard match (i.e this
        /// matches any client).
        #[prost(message, repeated, tag = "2")]
        pub metadata_labels: ::prost::alloc::vec::Vec<
            metadata_label_matcher::MetadataLabels,
        >,
    }
    /// Nested message and enum types in `MetadataLabelMatcher`.
    pub mod metadata_label_matcher {
        /// Defines a name-pair value for a single label.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MetadataLabels {
            /// Required. Label name presented as key in xDS Node Metadata.
            #[prost(string, tag = "1")]
            pub label_name: ::prost::alloc::string::String,
            /// Required. Label value presented as value corresponding to the above
            /// key, in xDS Node Metadata.
            #[prost(string, tag = "2")]
            pub label_value: ::prost::alloc::string::String,
        }
        /// Possible criteria values that define logic of how matching is made.
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
        pub enum MetadataLabelMatchCriteria {
            /// Default value. Should not be used.
            Unspecified = 0,
            /// At least one of the Labels specified in the matcher should match the
            /// metadata presented by xDS client.
            MatchAny = 1,
            /// The metadata presented by the xDS client should contain all of the
            /// labels specified here.
            MatchAll = 2,
        }
        impl MetadataLabelMatchCriteria {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    MetadataLabelMatchCriteria::Unspecified => {
                        "METADATA_LABEL_MATCH_CRITERIA_UNSPECIFIED"
                    }
                    MetadataLabelMatchCriteria::MatchAny => "MATCH_ANY",
                    MetadataLabelMatchCriteria::MatchAll => "MATCH_ALL",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "METADATA_LABEL_MATCH_CRITERIA_UNSPECIFIED" => {
                        Some(Self::Unspecified)
                    }
                    "MATCH_ANY" => Some(Self::MatchAny),
                    "MATCH_ALL" => Some(Self::MatchAll),
                    _ => None,
                }
            }
        }
    }
    /// Specifies type of the matcher used for this endpoint matcher.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MatcherType {
        /// The matcher is based on node metadata presented by xDS clients.
        #[prost(message, tag = "1")]
        MetadataLabelMatcher(MetadataLabelMatcher),
    }
}
/// A single extension chain wrapper that contains the match conditions and
/// extensions to execute.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtensionChain {
    /// Required. The name for this extension chain.
    /// The name is logged as part of the HTTP request logs.
    /// The name must conform with RFC-1034, is restricted to lower-cased letters,
    /// numbers and hyphens, and can have a maximum length of 63 characters.
    /// Additionally, the first character must be a letter and the last a letter or
    /// a number.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Conditions under which this chain is invoked for a request.
    #[prost(message, optional, tag = "2")]
    pub match_condition: ::core::option::Option<extension_chain::MatchCondition>,
    /// Required. A set of extensions to execute for the matching request.
    /// At least one extension is required.
    /// Up to 3 extensions can be defined for each extension chain
    /// for `LbTrafficExtension` resource.
    /// `LbRouteExtension` chains are limited to 1 extension per extension chain.
    #[prost(message, repeated, tag = "3")]
    pub extensions: ::prost::alloc::vec::Vec<extension_chain::Extension>,
}
/// Nested message and enum types in `ExtensionChain`.
pub mod extension_chain {
    /// Conditions under which this chain is invoked for a request.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MatchCondition {
        /// Required. A Common Expression Language (CEL) expression that is used to
        /// match requests for which the extension chain is executed.
        ///
        /// For more information, see [CEL matcher language
        /// reference](<https://cloud.google.com/service-extensions/docs/cel-matcher-language-reference>).
        #[prost(string, tag = "1")]
        pub cel_expression: ::prost::alloc::string::String,
    }
    /// A single extension in the chain to execute for the matching request.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Extension {
        /// Required. The name for this extension.
        /// The name is logged as part of the HTTP request logs.
        /// The name must conform with RFC-1034, is restricted to lower-cased
        /// letters, numbers and hyphens, and can have a maximum length of 63
        /// characters. Additionally, the first character must be a letter and the
        /// last a letter or a number.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// Optional. The `:authority` header in the gRPC request sent from Envoy
        /// to the extension service.
        /// Required for Callout extensions.
        #[prost(string, tag = "2")]
        pub authority: ::prost::alloc::string::String,
        /// Required. The reference to the service that runs the extension.
        ///
        /// Currently only callout extensions are supported here.
        ///
        /// To configure a callout extension, `service` must be a fully-qualified
        /// reference
        /// to a [backend
        /// service](<https://cloud.google.com/compute/docs/reference/rest/v1/backendServices>)
        /// in the format:
        /// `<https://www.googleapis.com/compute/v1/projects/{project}/regions/{region}/backendServices/{backendService}`>
        /// or
        /// `<https://www.googleapis.com/compute/v1/projects/{project}/global/backendServices/{backendService}`.>
        #[prost(string, tag = "3")]
        pub service: ::prost::alloc::string::String,
        /// Optional. A set of events during request or response processing for which
        /// this extension is called. This field is required for the
        /// `LbTrafficExtension` resource. It's not relevant for the
        /// `LbRouteExtension` resource.
        #[prost(enumeration = "super::EventType", repeated, packed = "false", tag = "4")]
        pub supported_events: ::prost::alloc::vec::Vec<i32>,
        /// Optional. Specifies the timeout for each individual message on the
        /// stream. The timeout must be between 10-1000 milliseconds. Required for
        /// Callout extensions.
        #[prost(message, optional, tag = "5")]
        pub timeout: ::core::option::Option<::prost_types::Duration>,
        /// Optional. Determines how the proxy behaves if the call to the extension
        /// fails or times out.
        ///
        /// When set to `TRUE`, request or response processing continues without
        /// error. Any subsequent extensions in the extension chain are also
        /// executed. When set to `FALSE` or the default setting of `FALSE` is used,
        /// one of the following happens:
        /// * If response headers have not been delivered to the downstream client,
        /// a generic 500 error is returned to the client. The error response can be
        /// tailored by configuring a custom error response in the load balancer.
        ///
        /// * If response headers have been delivered, then the HTTP stream to the
        /// downstream client is reset.
        #[prost(bool, tag = "6")]
        pub fail_open: bool,
        /// Optional. List of the HTTP headers to forward to the extension
        /// (from the client or backend). If omitted, all headers are sent.
        /// Each element is a string indicating the header name.
        #[prost(string, repeated, tag = "7")]
        pub forward_headers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
/// `LbTrafficExtension` is a resource that lets the extension service modify the
/// headers and payloads of both requests and responses without impacting the
/// choice of backend services or any other security policies associated with the
/// backend service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LbTrafficExtension {
    /// Required. Identifier. Name of the `LbTrafficExtension` resource in the
    /// following format:
    /// `projects/{project}/locations/{location}/lbTrafficExtensions/{lb_traffic_extension}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The timestamp when the resource was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when the resource was updated.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. A human-readable description of the resource.
    #[prost(string, tag = "9")]
    pub description: ::prost::alloc::string::String,
    /// Optional. Set of labels associated with the `LbTrafficExtension` resource.
    ///
    /// The format must comply with [the requirements for
    /// labels](<https://cloud.google.com/compute/docs/labeling-resources#requirements>)
    /// for Google Cloud resources.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Required. A list of references to the forwarding rules to which this
    /// service extension is attached to. At least one forwarding rule is required.
    /// There can be only one `LBTrafficExtension` resource per forwarding rule.
    #[prost(string, repeated, tag = "5")]
    pub forwarding_rules: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. A set of ordered extension chains that contain the match
    /// conditions and extensions to execute. Match conditions for each extension
    /// chain are evaluated in sequence for a given request. The first extension
    /// chain that has a condition that matches the request is executed.
    /// Any subsequent extension chains do not execute.
    /// Limited to 5 extension chains per resource.
    #[prost(message, repeated, tag = "7")]
    pub extension_chains: ::prost::alloc::vec::Vec<ExtensionChain>,
    /// Required. All backend services and forwarding rules referenced by this
    /// extension must share the same load balancing scheme. Supported values:
    /// `INTERNAL_MANAGED`, `EXTERNAL_MANAGED`. For more information, refer to
    /// [Choosing a load
    /// balancer](<https://cloud.google.com/load-balancing/docs/backend-service>).
    #[prost(enumeration = "LoadBalancingScheme", tag = "8")]
    pub load_balancing_scheme: i32,
}
/// Message for requesting list of `LbTrafficExtension` resources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLbTrafficExtensionsRequest {
    /// Required. The project and location from which the `LbTrafficExtension`
    /// resources are listed, specified in the following format:
    /// `projects/{project}/locations/{location}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Requested page size. The server might return fewer items than
    /// requested. If unspecified, the server picks an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results that the server returns.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filtering results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Hint for how to order the results.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Message for response to listing `LbTrafficExtension` resources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLbTrafficExtensionsResponse {
    /// The list of `LbTrafficExtension` resources.
    #[prost(message, repeated, tag = "1")]
    pub lb_traffic_extensions: ::prost::alloc::vec::Vec<LbTrafficExtension>,
    /// A token identifying a page of results that the server returns.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for getting a `LbTrafficExtension` resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLbTrafficExtensionRequest {
    /// Required. A name of the `LbTrafficExtension` resource to get. Must be in
    /// the format
    /// `projects/{project}/locations/{location}/lbTrafficExtensions/{lb_traffic_extension}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for creating a `LbTrafficExtension` resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateLbTrafficExtensionRequest {
    /// Required. The parent resource of the `LbTrafficExtension` resource. Must be
    /// in the format `projects/{project}/locations/{location}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. User-provided ID of the `LbTrafficExtension` resource to be
    /// created.
    #[prost(string, tag = "2")]
    pub lb_traffic_extension_id: ::prost::alloc::string::String,
    /// Required. `LbTrafficExtension` resource to be created.
    #[prost(message, optional, tag = "3")]
    pub lb_traffic_extension: ::core::option::Option<LbTrafficExtension>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server can ignore
    /// the request if it has already been completed. The server guarantees
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, ignores the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for updating a `LbTrafficExtension` resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateLbTrafficExtensionRequest {
    /// Required. Used to specify the fields to be overwritten in the
    /// `LbTrafficExtension` resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field is overwritten if it is in the mask. If the
    /// user does not specify a mask, then all fields are overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. `LbTrafficExtension` resource being updated.
    #[prost(message, optional, tag = "2")]
    pub lb_traffic_extension: ::core::option::Option<LbTrafficExtension>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server can ignore
    /// the request if it has already been completed. The server guarantees
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, ignores the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for deleting a `LbTrafficExtension` resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteLbTrafficExtensionRequest {
    /// Required. The name of the `LbTrafficExtension` resource to delete. Must be
    /// in the format
    /// `projects/{project}/locations/{location}/lbTrafficExtensions/{lb_traffic_extension}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server can ignore
    /// the request if it has already been completed. The server guarantees
    /// that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, ignores the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// `LbRouteExtension` is a resource that lets you control where traffic is
/// routed to for a given request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LbRouteExtension {
    /// Required. Identifier. Name of the `LbRouteExtension` resource in the
    /// following format:
    /// `projects/{project}/locations/{location}/lbRouteExtensions/{lb_route_extension}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The timestamp when the resource was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when the resource was updated.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. A human-readable description of the resource.
    #[prost(string, tag = "9")]
    pub description: ::prost::alloc::string::String,
    /// Optional. Set of labels associated with the `LbRouteExtension` resource.
    ///
    /// The format must comply with [the requirements for
    /// labels](<https://cloud.google.com/compute/docs/labeling-resources#requirements>)
    /// for Google Cloud resources.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Required. A list of references to the forwarding rules to which this
    /// service extension is attached to. At least one forwarding rule is required.
    /// There can be only one `LbRouteExtension` resource per forwarding rule.
    #[prost(string, repeated, tag = "5")]
    pub forwarding_rules: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. A set of ordered extension chains that contain the match
    /// conditions and extensions to execute. Match conditions for each extension
    /// chain are evaluated in sequence for a given request. The first extension
    /// chain that has a condition that matches the request is executed.
    /// Any subsequent extension chains do not execute.
    /// Limited to 5 extension chains per resource.
    #[prost(message, repeated, tag = "7")]
    pub extension_chains: ::prost::alloc::vec::Vec<ExtensionChain>,
    /// Required. All backend services and forwarding rules referenced by this
    /// extension must share the same load balancing scheme. Supported values:
    /// `INTERNAL_MANAGED`, `EXTERNAL_MANAGED`. For more information, refer to
    /// [Choosing a load
    /// balancer](<https://cloud.google.com/load-balancing/docs/backend-service>).
    #[prost(enumeration = "LoadBalancingScheme", tag = "8")]
    pub load_balancing_scheme: i32,
}
/// Message for requesting list of `LbRouteExtension` resources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLbRouteExtensionsRequest {
    /// Required. The project and location from which the `LbRouteExtension`
    /// resources are listed, specified in the following format:
    /// `projects/{project}/locations/{location}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Requested page size. The server might return fewer items than
    /// requested. If unspecified, the server picks an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results that the server returns.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filtering results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Hint for how to order the results.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Message for response to listing `LbRouteExtension` resources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLbRouteExtensionsResponse {
    /// The list of `LbRouteExtension` resources.
    #[prost(message, repeated, tag = "1")]
    pub lb_route_extensions: ::prost::alloc::vec::Vec<LbRouteExtension>,
    /// A token identifying a page of results that the server returns.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for getting a `LbRouteExtension` resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLbRouteExtensionRequest {
    /// Required. A name of the `LbRouteExtension` resource to get. Must be in the
    /// format
    /// `projects/{project}/locations/{location}/lbRouteExtensions/{lb_route_extension}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for creating a `LbRouteExtension` resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateLbRouteExtensionRequest {
    /// Required. The parent resource of the `LbRouteExtension` resource. Must be
    /// in the format `projects/{project}/locations/{location}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. User-provided ID of the `LbRouteExtension` resource to be
    /// created.
    #[prost(string, tag = "2")]
    pub lb_route_extension_id: ::prost::alloc::string::String,
    /// Required. `LbRouteExtension` resource to be created.
    #[prost(message, optional, tag = "3")]
    pub lb_route_extension: ::core::option::Option<LbRouteExtension>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server can ignore
    /// the request if it has already been completed. The server guarantees
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, ignores the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for updating a `LbRouteExtension` resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateLbRouteExtensionRequest {
    /// Required. Used to specify the fields to be overwritten in the
    /// `LbRouteExtension` resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field is overwritten if it is in the mask. If the
    /// user does not specify a mask, then all fields are overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. `LbRouteExtension` resource being updated.
    #[prost(message, optional, tag = "2")]
    pub lb_route_extension: ::core::option::Option<LbRouteExtension>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server can ignore
    /// the request if it has already been completed. The server guarantees
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, ignores the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for deleting a `LbRouteExtension` resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteLbRouteExtensionRequest {
    /// Required. The name of the `LbRouteExtension` resource to delete. Must be in
    /// the format
    /// `projects/{project}/locations/{location}/lbRouteExtensions/{lb_route_extension}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server can ignore
    /// the request if it has already been completed. The server guarantees
    /// that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, ignores the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// The part of the request or response for which the extension is called.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EventType {
    /// Unspecified value. Do not use.
    Unspecified = 0,
    /// If included in `supported_events`,
    /// the extension is called when the HTTP request headers arrive.
    RequestHeaders = 1,
    /// If included in `supported_events`,
    /// the extension is called when the HTTP request body arrives.
    RequestBody = 2,
    /// If included in `supported_events`,
    /// the extension is called when the HTTP response headers arrive.
    ResponseHeaders = 3,
    /// If included in `supported_events`,
    /// the extension is called when the HTTP response body arrives.
    ResponseBody = 4,
    /// If included in `supported_events`,
    /// the extension is called when the HTTP request trailers arrives.
    RequestTrailers = 5,
    /// If included in `supported_events`,
    /// the extension is called when the HTTP response trailers arrives.
    ResponseTrailers = 6,
}
impl EventType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EventType::Unspecified => "EVENT_TYPE_UNSPECIFIED",
            EventType::RequestHeaders => "REQUEST_HEADERS",
            EventType::RequestBody => "REQUEST_BODY",
            EventType::ResponseHeaders => "RESPONSE_HEADERS",
            EventType::ResponseBody => "RESPONSE_BODY",
            EventType::RequestTrailers => "REQUEST_TRAILERS",
            EventType::ResponseTrailers => "RESPONSE_TRAILERS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EVENT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "REQUEST_HEADERS" => Some(Self::RequestHeaders),
            "REQUEST_BODY" => Some(Self::RequestBody),
            "RESPONSE_HEADERS" => Some(Self::ResponseHeaders),
            "RESPONSE_BODY" => Some(Self::ResponseBody),
            "REQUEST_TRAILERS" => Some(Self::RequestTrailers),
            "RESPONSE_TRAILERS" => Some(Self::ResponseTrailers),
            _ => None,
        }
    }
}
/// Load balancing schemes supported by the `LbTrafficExtension` resource and
/// `LbRouteExtension` resource.
/// For more information, refer to [Choosing a load
/// balancer](<https://cloud.google.com/load-balancing/docs/backend-service>).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LoadBalancingScheme {
    /// Default value. Do not use.
    Unspecified = 0,
    /// Signifies that this is used for Internal HTTP(S) Load Balancing.
    InternalManaged = 1,
    /// Signifies that this is used for External Managed HTTP(S) Load
    /// Balancing.
    ExternalManaged = 2,
}
impl LoadBalancingScheme {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LoadBalancingScheme::Unspecified => "LOAD_BALANCING_SCHEME_UNSPECIFIED",
            LoadBalancingScheme::InternalManaged => "INTERNAL_MANAGED",
            LoadBalancingScheme::ExternalManaged => "EXTERNAL_MANAGED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LOAD_BALANCING_SCHEME_UNSPECIFIED" => Some(Self::Unspecified),
            "INTERNAL_MANAGED" => Some(Self::InternalManaged),
            "EXTERNAL_MANAGED" => Some(Self::ExternalManaged),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod dep_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service describing handlers for resources.
    #[derive(Debug, Clone)]
    pub struct DepServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DepServiceClient<tonic::transport::Channel> {
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
    impl<T> DepServiceClient<T>
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
        ) -> DepServiceClient<InterceptedService<T, F>>
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
            DepServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists `LbTrafficExtension` resources in a given project and location.
        pub async fn list_lb_traffic_extensions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLbTrafficExtensionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListLbTrafficExtensionsResponse>,
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
                "/google.cloud.networkservices.v1beta1.DepService/ListLbTrafficExtensions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkservices.v1beta1.DepService",
                        "ListLbTrafficExtensions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of the specified `LbTrafficExtension` resource.
        pub async fn get_lb_traffic_extension(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLbTrafficExtensionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::LbTrafficExtension>,
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
                "/google.cloud.networkservices.v1beta1.DepService/GetLbTrafficExtension",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkservices.v1beta1.DepService",
                        "GetLbTrafficExtension",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new `LbTrafficExtension` resource in a given project and
        /// location.
        pub async fn create_lb_traffic_extension(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateLbTrafficExtensionRequest>,
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
                "/google.cloud.networkservices.v1beta1.DepService/CreateLbTrafficExtension",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkservices.v1beta1.DepService",
                        "CreateLbTrafficExtension",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the parameters of the specified `LbTrafficExtension` resource.
        pub async fn update_lb_traffic_extension(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateLbTrafficExtensionRequest>,
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
                "/google.cloud.networkservices.v1beta1.DepService/UpdateLbTrafficExtension",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkservices.v1beta1.DepService",
                        "UpdateLbTrafficExtension",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the specified `LbTrafficExtension` resource.
        pub async fn delete_lb_traffic_extension(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteLbTrafficExtensionRequest>,
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
                "/google.cloud.networkservices.v1beta1.DepService/DeleteLbTrafficExtension",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkservices.v1beta1.DepService",
                        "DeleteLbTrafficExtension",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists `LbRouteExtension` resources in a given project and location.
        pub async fn list_lb_route_extensions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLbRouteExtensionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListLbRouteExtensionsResponse>,
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
                "/google.cloud.networkservices.v1beta1.DepService/ListLbRouteExtensions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkservices.v1beta1.DepService",
                        "ListLbRouteExtensions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of the specified `LbRouteExtension` resource.
        pub async fn get_lb_route_extension(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLbRouteExtensionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::LbRouteExtension>,
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
                "/google.cloud.networkservices.v1beta1.DepService/GetLbRouteExtension",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkservices.v1beta1.DepService",
                        "GetLbRouteExtension",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new `LbRouteExtension` resource in a given project and location.
        pub async fn create_lb_route_extension(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateLbRouteExtensionRequest>,
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
                "/google.cloud.networkservices.v1beta1.DepService/CreateLbRouteExtension",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkservices.v1beta1.DepService",
                        "CreateLbRouteExtension",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the parameters of the specified `LbRouteExtension` resource.
        pub async fn update_lb_route_extension(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateLbRouteExtensionRequest>,
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
                "/google.cloud.networkservices.v1beta1.DepService/UpdateLbRouteExtension",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkservices.v1beta1.DepService",
                        "UpdateLbRouteExtension",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the specified `LbRouteExtension` resource.
        pub async fn delete_lb_route_extension(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteLbRouteExtensionRequest>,
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
                "/google.cloud.networkservices.v1beta1.DepService/DeleteLbRouteExtension",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkservices.v1beta1.DepService",
                        "DeleteLbRouteExtension",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// EndpointPolicy is a resource that helps apply desired configuration
/// on the endpoints that match specific criteria.
/// For example, this resource can be used to apply "authentication config"
/// an all endpoints that serve on port 8080.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndpointPolicy {
    /// Required. Name of the EndpointPolicy resource. It matches pattern
    /// `projects/{project}/locations/global/endpointPolicies/{endpoint_policy}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The timestamp when the resource was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when the resource was updated.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Set of label tags associated with the EndpointPolicy resource.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Required. The type of endpoint policy. This is primarily used to validate
    /// the configuration.
    #[prost(enumeration = "endpoint_policy::EndpointPolicyType", tag = "5")]
    pub r#type: i32,
    /// Optional. This field specifies the URL of AuthorizationPolicy resource that
    /// applies authorization policies to the inbound traffic at the
    /// matched endpoints. Refer to Authorization. If this field is not
    /// specified, authorization is disabled(no authz checks) for this
    /// endpoint.
    #[prost(string, tag = "7")]
    pub authorization_policy: ::prost::alloc::string::String,
    /// Required. A matcher that selects endpoints to which the policies should be
    /// applied.
    #[prost(message, optional, tag = "9")]
    pub endpoint_matcher: ::core::option::Option<EndpointMatcher>,
    /// Optional. Port selector for the (matched) endpoints. If no port selector is
    /// provided, the matched config is applied to all ports.
    #[prost(message, optional, tag = "10")]
    pub traffic_port_selector: ::core::option::Option<TrafficPortSelector>,
    /// Optional. A free-text description of the resource. Max length 1024
    /// characters.
    #[prost(string, tag = "11")]
    pub description: ::prost::alloc::string::String,
    /// Optional. A URL referring to ServerTlsPolicy resource. ServerTlsPolicy is
    /// used to determine the authentication policy to be applied to terminate the
    /// inbound traffic at the identified backends. If this field is not set,
    /// authentication is disabled(open) for this endpoint.
    #[prost(string, tag = "12")]
    pub server_tls_policy: ::prost::alloc::string::String,
    /// Optional. A URL referring to a ClientTlsPolicy resource. ClientTlsPolicy
    /// can be set to specify the authentication for traffic from the proxy to the
    /// actual endpoints. More specifically, it is applied to the outgoing traffic
    /// from the proxy to the endpoint. This is typically used for sidecar model
    /// where the proxy identifies itself as endpoint to the control plane, with
    /// the connection between sidecar and endpoint requiring authentication. If
    /// this field is not set, authentication is disabled(open). Applicable only
    /// when EndpointPolicyType is SIDECAR_PROXY.
    #[prost(string, tag = "13")]
    pub client_tls_policy: ::prost::alloc::string::String,
}
/// Nested message and enum types in `EndpointPolicy`.
pub mod endpoint_policy {
    /// The type of endpoint policy.
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
    pub enum EndpointPolicyType {
        /// Default value. Must not be used.
        Unspecified = 0,
        /// Represents a proxy deployed as a sidecar.
        SidecarProxy = 1,
        /// Represents a proxyless gRPC backend.
        GrpcServer = 2,
    }
    impl EndpointPolicyType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EndpointPolicyType::Unspecified => "ENDPOINT_POLICY_TYPE_UNSPECIFIED",
                EndpointPolicyType::SidecarProxy => "SIDECAR_PROXY",
                EndpointPolicyType::GrpcServer => "GRPC_SERVER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ENDPOINT_POLICY_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "SIDECAR_PROXY" => Some(Self::SidecarProxy),
                "GRPC_SERVER" => Some(Self::GrpcServer),
                _ => None,
            }
        }
    }
}
/// Request used with the ListEndpointPolicies method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEndpointPoliciesRequest {
    /// Required. The project and location from which the EndpointPolicies should
    /// be listed, specified in the format `projects/*/locations/global`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of EndpointPolicies to return per call.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The value returned by the last `ListEndpointPoliciesResponse`
    /// Indicates that this is a continuation of a prior
    /// `ListEndpointPolicies` call, and that the system should return the
    /// next page of data.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response returned by the ListEndpointPolicies method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEndpointPoliciesResponse {
    /// List of EndpointPolicy resources.
    #[prost(message, repeated, tag = "1")]
    pub endpoint_policies: ::prost::alloc::vec::Vec<EndpointPolicy>,
    /// If there might be more results than those appearing in this response, then
    /// `next_page_token` is included. To get the next set of results, call this
    /// method again using the value of `next_page_token` as `page_token`.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request used with the GetEndpointPolicy method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEndpointPolicyRequest {
    /// Required. A name of the EndpointPolicy to get. Must be in the format
    /// `projects/*/locations/global/endpointPolicies/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request used with the CreateEndpointPolicy method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEndpointPolicyRequest {
    /// Required. The parent resource of the EndpointPolicy. Must be in the
    /// format `projects/*/locations/global`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Short name of the EndpointPolicy resource to be created.
    /// E.g. "CustomECS".
    #[prost(string, tag = "2")]
    pub endpoint_policy_id: ::prost::alloc::string::String,
    /// Required. EndpointPolicy resource to be created.
    #[prost(message, optional, tag = "3")]
    pub endpoint_policy: ::core::option::Option<EndpointPolicy>,
}
/// Request used with the UpdateEndpointPolicy method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateEndpointPolicyRequest {
    /// Optional. Field mask is used to specify the fields to be overwritten in the
    /// EndpointPolicy resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. Updated EndpointPolicy resource.
    #[prost(message, optional, tag = "2")]
    pub endpoint_policy: ::core::option::Option<EndpointPolicy>,
}
/// Request used with the DeleteEndpointPolicy method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEndpointPolicyRequest {
    /// Required. A name of the EndpointPolicy to delete. Must be in the format
    /// `projects/*/locations/global/endpointPolicies/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod network_services_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct NetworkServicesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl NetworkServicesClient<tonic::transport::Channel> {
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
    impl<T> NetworkServicesClient<T>
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
        ) -> NetworkServicesClient<InterceptedService<T, F>>
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
            NetworkServicesClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists EndpointPolicies in a given project and location.
        pub async fn list_endpoint_policies(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEndpointPoliciesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListEndpointPoliciesResponse>,
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
                "/google.cloud.networkservices.v1beta1.NetworkServices/ListEndpointPolicies",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkservices.v1beta1.NetworkServices",
                        "ListEndpointPolicies",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single EndpointPolicy.
        pub async fn get_endpoint_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEndpointPolicyRequest>,
        ) -> std::result::Result<tonic::Response<super::EndpointPolicy>, tonic::Status> {
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
                "/google.cloud.networkservices.v1beta1.NetworkServices/GetEndpointPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkservices.v1beta1.NetworkServices",
                        "GetEndpointPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new EndpointPolicy in a given project and location.
        pub async fn create_endpoint_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateEndpointPolicyRequest>,
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
                "/google.cloud.networkservices.v1beta1.NetworkServices/CreateEndpointPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkservices.v1beta1.NetworkServices",
                        "CreateEndpointPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the parameters of a single EndpointPolicy.
        pub async fn update_endpoint_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateEndpointPolicyRequest>,
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
                "/google.cloud.networkservices.v1beta1.NetworkServices/UpdateEndpointPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkservices.v1beta1.NetworkServices",
                        "UpdateEndpointPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single EndpointPolicy.
        pub async fn delete_endpoint_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteEndpointPolicyRequest>,
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
                "/google.cloud.networkservices.v1beta1.NetworkServices/DeleteEndpointPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkservices.v1beta1.NetworkServices",
                        "DeleteEndpointPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
