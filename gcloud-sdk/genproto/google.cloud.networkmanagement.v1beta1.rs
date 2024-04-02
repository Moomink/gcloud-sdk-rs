/// Trace represents one simulated packet forwarding path.
///
///    * Each trace contains multiple ordered steps.
///    * Each step is in a particular state with associated configuration.
///    * State is categorized as final or non-final states.
///    * Each final state has a reason associated.
///    * Each trace must end with a final state (the last step).
/// ```
///    |---------------------Trace----------------------|
///    Step1(State) Step2(State) ---  StepN(State(final))
/// ```
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trace {
    /// Derived from the source and destination endpoints definition specified by
    /// user request, and validated by the data plane model.
    /// If there are multiple traces starting from different source locations, then
    /// the endpoint_info may be different between traces.
    #[prost(message, optional, tag = "1")]
    pub endpoint_info: ::core::option::Option<EndpointInfo>,
    /// A trace of a test contains multiple steps from the initial state to the
    /// final state (delivered, dropped, forwarded, or aborted).
    ///
    /// The steps are ordered by the processing sequence within the simulated
    /// network state machine. It is critical to preserve the order of the steps
    /// and avoid reordering or sorting them.
    #[prost(message, repeated, tag = "2")]
    pub steps: ::prost::alloc::vec::Vec<Step>,
    /// ID of trace. For forward traces, this ID is unique for each trace. For
    /// return traces, it matches ID of associated forward trace. A single forward
    /// trace can be associated with none, one or more than one return trace.
    #[prost(int32, tag = "4")]
    pub forward_trace_id: i32,
}
/// A simulated forwarding path is composed of multiple steps.
/// Each step has a well-defined state and an associated configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Step {
    /// A description of the step. Usually this is a summary of the state.
    #[prost(string, tag = "1")]
    pub description: ::prost::alloc::string::String,
    /// Each step is in one of the pre-defined states.
    #[prost(enumeration = "step::State", tag = "2")]
    pub state: i32,
    /// This is a step that leads to the final state Drop.
    #[prost(bool, tag = "3")]
    pub causes_drop: bool,
    /// Project ID that contains the configuration this step is validating.
    #[prost(string, tag = "4")]
    pub project_id: ::prost::alloc::string::String,
    /// Configuration or metadata associated with each step.
    /// The configuration is filtered based on viewer's permission. If a viewer
    /// has no permission to view the configuration in this step, for non-final
    /// states a special state is populated (VIEWER_PERMISSION_MISSING), and for
    /// final state the configuration is cleared.
    #[prost(
        oneof = "step::StepInfo",
        tags = "5, 6, 7, 8, 24, 9, 10, 11, 21, 12, 13, 14, 15, 16, 17, 18, 19, 20, 22, 23, 25, 26, 27, 28"
    )]
    pub step_info: ::core::option::Option<step::StepInfo>,
}
/// Nested message and enum types in `Step`.
pub mod step {
    /// Type of states that are defined in the network state machine.
    /// Each step in the packet trace is in a specific state.
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
        /// Unspecified state.
        Unspecified = 0,
        /// Initial state: packet originating from a Compute Engine instance.
        /// An InstanceInfo is populated with starting instance information.
        StartFromInstance = 1,
        /// Initial state: packet originating from the internet.
        /// The endpoint information is populated.
        StartFromInternet = 2,
        /// Initial state: packet originating from a Google service.
        /// The google_service information is populated.
        StartFromGoogleService = 27,
        /// Initial state: packet originating from a VPC or on-premises network
        /// with internal source IP.
        /// If the source is a VPC network visible to the user, a NetworkInfo
        /// is populated with details of the network.
        StartFromPrivateNetwork = 3,
        /// Initial state: packet originating from a Google Kubernetes Engine cluster
        /// master. A GKEMasterInfo is populated with starting instance information.
        StartFromGkeMaster = 21,
        /// Initial state: packet originating from a Cloud SQL instance.
        /// A CloudSQLInstanceInfo is populated with starting instance information.
        StartFromCloudSqlInstance = 22,
        /// Initial state: packet originating from a Cloud Function.
        /// A CloudFunctionInfo is populated with starting function information.
        StartFromCloudFunction = 23,
        /// Initial state: packet originating from an App Engine service version.
        /// An AppEngineVersionInfo is populated with starting version information.
        StartFromAppEngineVersion = 25,
        /// Initial state: packet originating from a Cloud Run revision.
        /// A CloudRunRevisionInfo is populated with starting revision information.
        StartFromCloudRunRevision = 26,
        /// Initial state: packet originating from a Storage Bucket. Used only for
        /// return traces.
        /// The storage_bucket information is populated.
        StartFromStorageBucket = 29,
        /// Initial state: packet originating from a published service that uses
        /// Private Service Connect. Used only for return traces.
        StartFromPscPublishedService = 30,
        /// Config checking state: verify ingress firewall rule.
        ApplyIngressFirewallRule = 4,
        /// Config checking state: verify egress firewall rule.
        ApplyEgressFirewallRule = 5,
        /// Config checking state: verify route.
        ApplyRoute = 6,
        /// Config checking state: match forwarding rule.
        ApplyForwardingRule = 7,
        /// Config checking state: verify load balancer backend configuration.
        AnalyzeLoadBalancerBackend = 28,
        /// Config checking state: packet sent or received under foreign IP
        /// address and allowed.
        SpoofingApproved = 8,
        /// Forwarding state: arriving at a Compute Engine instance.
        ArriveAtInstance = 9,
        /// Forwarding state: arriving at a Compute Engine internal load balancer.
        /// Deprecated in favor of the `ANALYZE_LOAD_BALANCER_BACKEND` state, not
        /// used in new tests.
        ArriveAtInternalLoadBalancer = 10,
        /// Forwarding state: arriving at a Compute Engine external load balancer.
        /// Deprecated in favor of the `ANALYZE_LOAD_BALANCER_BACKEND` state, not
        /// used in new tests.
        ArriveAtExternalLoadBalancer = 11,
        /// Forwarding state: arriving at a Cloud VPN gateway.
        ArriveAtVpnGateway = 12,
        /// Forwarding state: arriving at a Cloud VPN tunnel.
        ArriveAtVpnTunnel = 13,
        /// Forwarding state: arriving at a VPC connector.
        ArriveAtVpcConnector = 24,
        /// Transition state: packet header translated.
        Nat = 14,
        /// Transition state: original connection is terminated and a new proxied
        /// connection is initiated.
        ProxyConnection = 15,
        /// Final state: packet could be delivered.
        Deliver = 16,
        /// Final state: packet could be dropped.
        Drop = 17,
        /// Final state: packet could be forwarded to a network with an unknown
        /// configuration.
        Forward = 18,
        /// Final state: analysis is aborted.
        Abort = 19,
        /// Special state: viewer of the test result does not have permission to
        /// see the configuration in this step.
        ViewerPermissionMissing = 20,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::StartFromInstance => "START_FROM_INSTANCE",
                State::StartFromInternet => "START_FROM_INTERNET",
                State::StartFromGoogleService => "START_FROM_GOOGLE_SERVICE",
                State::StartFromPrivateNetwork => "START_FROM_PRIVATE_NETWORK",
                State::StartFromGkeMaster => "START_FROM_GKE_MASTER",
                State::StartFromCloudSqlInstance => "START_FROM_CLOUD_SQL_INSTANCE",
                State::StartFromCloudFunction => "START_FROM_CLOUD_FUNCTION",
                State::StartFromAppEngineVersion => "START_FROM_APP_ENGINE_VERSION",
                State::StartFromCloudRunRevision => "START_FROM_CLOUD_RUN_REVISION",
                State::StartFromStorageBucket => "START_FROM_STORAGE_BUCKET",
                State::StartFromPscPublishedService => "START_FROM_PSC_PUBLISHED_SERVICE",
                State::ApplyIngressFirewallRule => "APPLY_INGRESS_FIREWALL_RULE",
                State::ApplyEgressFirewallRule => "APPLY_EGRESS_FIREWALL_RULE",
                State::ApplyRoute => "APPLY_ROUTE",
                State::ApplyForwardingRule => "APPLY_FORWARDING_RULE",
                State::AnalyzeLoadBalancerBackend => "ANALYZE_LOAD_BALANCER_BACKEND",
                State::SpoofingApproved => "SPOOFING_APPROVED",
                State::ArriveAtInstance => "ARRIVE_AT_INSTANCE",
                State::ArriveAtInternalLoadBalancer => "ARRIVE_AT_INTERNAL_LOAD_BALANCER",
                State::ArriveAtExternalLoadBalancer => "ARRIVE_AT_EXTERNAL_LOAD_BALANCER",
                State::ArriveAtVpnGateway => "ARRIVE_AT_VPN_GATEWAY",
                State::ArriveAtVpnTunnel => "ARRIVE_AT_VPN_TUNNEL",
                State::ArriveAtVpcConnector => "ARRIVE_AT_VPC_CONNECTOR",
                State::Nat => "NAT",
                State::ProxyConnection => "PROXY_CONNECTION",
                State::Deliver => "DELIVER",
                State::Drop => "DROP",
                State::Forward => "FORWARD",
                State::Abort => "ABORT",
                State::ViewerPermissionMissing => "VIEWER_PERMISSION_MISSING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "START_FROM_INSTANCE" => Some(Self::StartFromInstance),
                "START_FROM_INTERNET" => Some(Self::StartFromInternet),
                "START_FROM_GOOGLE_SERVICE" => Some(Self::StartFromGoogleService),
                "START_FROM_PRIVATE_NETWORK" => Some(Self::StartFromPrivateNetwork),
                "START_FROM_GKE_MASTER" => Some(Self::StartFromGkeMaster),
                "START_FROM_CLOUD_SQL_INSTANCE" => Some(Self::StartFromCloudSqlInstance),
                "START_FROM_CLOUD_FUNCTION" => Some(Self::StartFromCloudFunction),
                "START_FROM_APP_ENGINE_VERSION" => Some(Self::StartFromAppEngineVersion),
                "START_FROM_CLOUD_RUN_REVISION" => Some(Self::StartFromCloudRunRevision),
                "START_FROM_STORAGE_BUCKET" => Some(Self::StartFromStorageBucket),
                "START_FROM_PSC_PUBLISHED_SERVICE" => {
                    Some(Self::StartFromPscPublishedService)
                }
                "APPLY_INGRESS_FIREWALL_RULE" => Some(Self::ApplyIngressFirewallRule),
                "APPLY_EGRESS_FIREWALL_RULE" => Some(Self::ApplyEgressFirewallRule),
                "APPLY_ROUTE" => Some(Self::ApplyRoute),
                "APPLY_FORWARDING_RULE" => Some(Self::ApplyForwardingRule),
                "ANALYZE_LOAD_BALANCER_BACKEND" => Some(Self::AnalyzeLoadBalancerBackend),
                "SPOOFING_APPROVED" => Some(Self::SpoofingApproved),
                "ARRIVE_AT_INSTANCE" => Some(Self::ArriveAtInstance),
                "ARRIVE_AT_INTERNAL_LOAD_BALANCER" => {
                    Some(Self::ArriveAtInternalLoadBalancer)
                }
                "ARRIVE_AT_EXTERNAL_LOAD_BALANCER" => {
                    Some(Self::ArriveAtExternalLoadBalancer)
                }
                "ARRIVE_AT_VPN_GATEWAY" => Some(Self::ArriveAtVpnGateway),
                "ARRIVE_AT_VPN_TUNNEL" => Some(Self::ArriveAtVpnTunnel),
                "ARRIVE_AT_VPC_CONNECTOR" => Some(Self::ArriveAtVpcConnector),
                "NAT" => Some(Self::Nat),
                "PROXY_CONNECTION" => Some(Self::ProxyConnection),
                "DELIVER" => Some(Self::Deliver),
                "DROP" => Some(Self::Drop),
                "FORWARD" => Some(Self::Forward),
                "ABORT" => Some(Self::Abort),
                "VIEWER_PERMISSION_MISSING" => Some(Self::ViewerPermissionMissing),
                _ => None,
            }
        }
    }
    /// Configuration or metadata associated with each step.
    /// The configuration is filtered based on viewer's permission. If a viewer
    /// has no permission to view the configuration in this step, for non-final
    /// states a special state is populated (VIEWER_PERMISSION_MISSING), and for
    /// final state the configuration is cleared.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum StepInfo {
        /// Display information of a Compute Engine instance.
        #[prost(message, tag = "5")]
        Instance(super::InstanceInfo),
        /// Display information of a Compute Engine firewall rule.
        #[prost(message, tag = "6")]
        Firewall(super::FirewallInfo),
        /// Display information of a Compute Engine route.
        #[prost(message, tag = "7")]
        Route(super::RouteInfo),
        /// Display information of the source and destination under analysis.
        /// The endpoint information in an intermediate state may differ with the
        /// initial input, as it might be modified by state like NAT,
        /// or Connection Proxy.
        #[prost(message, tag = "8")]
        Endpoint(super::EndpointInfo),
        /// Display information of a Google service
        #[prost(message, tag = "24")]
        GoogleService(super::GoogleServiceInfo),
        /// Display information of a Compute Engine forwarding rule.
        #[prost(message, tag = "9")]
        ForwardingRule(super::ForwardingRuleInfo),
        /// Display information of a Compute Engine VPN gateway.
        #[prost(message, tag = "10")]
        VpnGateway(super::VpnGatewayInfo),
        /// Display information of a Compute Engine VPN tunnel.
        #[prost(message, tag = "11")]
        VpnTunnel(super::VpnTunnelInfo),
        /// Display information of a VPC connector.
        #[prost(message, tag = "21")]
        VpcConnector(super::VpcConnectorInfo),
        /// Display information of the final state "deliver" and reason.
        #[prost(message, tag = "12")]
        Deliver(super::DeliverInfo),
        /// Display information of the final state "forward" and reason.
        #[prost(message, tag = "13")]
        Forward(super::ForwardInfo),
        /// Display information of the final state "abort" and reason.
        #[prost(message, tag = "14")]
        Abort(super::AbortInfo),
        /// Display information of the final state "drop" and reason.
        #[prost(message, tag = "15")]
        Drop(super::DropInfo),
        /// Display information of the load balancers. Deprecated in favor of the
        /// `load_balancer_backend_info` field, not used in new tests.
        #[prost(message, tag = "16")]
        LoadBalancer(super::LoadBalancerInfo),
        /// Display information of a Google Cloud network.
        #[prost(message, tag = "17")]
        Network(super::NetworkInfo),
        /// Display information of a Google Kubernetes Engine cluster master.
        #[prost(message, tag = "18")]
        GkeMaster(super::GkeMasterInfo),
        /// Display information of a Cloud SQL instance.
        #[prost(message, tag = "19")]
        CloudSqlInstance(super::CloudSqlInstanceInfo),
        /// Display information of a Cloud Function.
        #[prost(message, tag = "20")]
        CloudFunction(super::CloudFunctionInfo),
        /// Display information of an App Engine service version.
        #[prost(message, tag = "22")]
        AppEngineVersion(super::AppEngineVersionInfo),
        /// Display information of a Cloud Run revision.
        #[prost(message, tag = "23")]
        CloudRunRevision(super::CloudRunRevisionInfo),
        /// Display information of a NAT.
        #[prost(message, tag = "25")]
        Nat(super::NatInfo),
        /// Display information of a ProxyConnection.
        #[prost(message, tag = "26")]
        ProxyConnection(super::ProxyConnectionInfo),
        /// Display information of a specific load balancer backend.
        #[prost(message, tag = "27")]
        LoadBalancerBackendInfo(super::LoadBalancerBackendInfo),
        /// Display information of a Storage Bucket. Used only for return traces.
        #[prost(message, tag = "28")]
        StorageBucket(super::StorageBucketInfo),
    }
}
/// For display only. Metadata associated with a Compute Engine instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstanceInfo {
    /// Name of a Compute Engine instance.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// URI of a Compute Engine instance.
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// Name of the network interface of a Compute Engine instance.
    #[prost(string, tag = "3")]
    pub interface: ::prost::alloc::string::String,
    /// URI of a Compute Engine network.
    #[prost(string, tag = "4")]
    pub network_uri: ::prost::alloc::string::String,
    /// Internal IP address of the network interface.
    #[prost(string, tag = "5")]
    pub internal_ip: ::prost::alloc::string::String,
    /// External IP address of the network interface.
    #[prost(string, tag = "6")]
    pub external_ip: ::prost::alloc::string::String,
    /// Network tags configured on the instance.
    #[prost(string, repeated, tag = "7")]
    pub network_tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Service account authorized for the instance.
    #[deprecated]
    #[prost(string, tag = "8")]
    pub service_account: ::prost::alloc::string::String,
}
/// For display only. Metadata associated with a Compute Engine network.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkInfo {
    /// Name of a Compute Engine network.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// URI of a Compute Engine network.
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// The IP range that matches the test.
    #[prost(string, tag = "4")]
    pub matched_ip_range: ::prost::alloc::string::String,
}
/// For display only. Metadata associated with a VPC firewall rule, an implied
/// VPC firewall rule, or a hierarchical firewall policy rule.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FirewallInfo {
    /// The display name of the VPC firewall rule. This field is not applicable
    /// to hierarchical firewall policy rules.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// The URI of the VPC firewall rule. This field is not applicable to
    /// implied firewall rules or hierarchical firewall policy rules.
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// Possible values: INGRESS, EGRESS
    #[prost(string, tag = "3")]
    pub direction: ::prost::alloc::string::String,
    /// Possible values: ALLOW, DENY, APPLY_SECURITY_PROFILE_GROUP
    #[prost(string, tag = "4")]
    pub action: ::prost::alloc::string::String,
    /// The priority of the firewall rule.
    #[prost(int32, tag = "5")]
    pub priority: i32,
    /// The URI of the VPC network that the firewall rule is associated with.
    /// This field is not applicable to hierarchical firewall policy rules.
    #[prost(string, tag = "6")]
    pub network_uri: ::prost::alloc::string::String,
    /// The target tags defined by the VPC firewall rule. This field is not
    /// applicable to hierarchical firewall policy rules.
    #[prost(string, repeated, tag = "7")]
    pub target_tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The target service accounts specified by the firewall rule.
    #[prost(string, repeated, tag = "8")]
    pub target_service_accounts: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// The hierarchical firewall policy that this rule is associated with.
    /// This field is not applicable to VPC firewall rules.
    #[prost(string, tag = "9")]
    pub policy: ::prost::alloc::string::String,
    /// The firewall rule's type.
    #[prost(enumeration = "firewall_info::FirewallRuleType", tag = "10")]
    pub firewall_rule_type: i32,
}
/// Nested message and enum types in `FirewallInfo`.
pub mod firewall_info {
    /// The firewall rule's type.
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
    pub enum FirewallRuleType {
        /// Unspecified type.
        Unspecified = 0,
        /// Hierarchical firewall policy rule. For details, see
        /// [Hierarchical firewall policies
        /// overview](<https://cloud.google.com/vpc/docs/firewall-policies>).
        HierarchicalFirewallPolicyRule = 1,
        /// VPC firewall rule. For details, see
        /// [VPC firewall rules
        /// overview](<https://cloud.google.com/vpc/docs/firewalls>).
        VpcFirewallRule = 2,
        /// Implied VPC firewall rule. For details, see
        /// [Implied
        /// rules](<https://cloud.google.com/vpc/docs/firewalls#default_firewall_rules>).
        ImpliedVpcFirewallRule = 3,
        /// Implicit firewall rules that are managed by serverless VPC access to
        /// allow ingress access. They are not visible in the Google Cloud console.
        /// For details, see [VPC connector's implicit
        /// rules](<https://cloud.google.com/functions/docs/networking/connecting-vpc#restrict-access>).
        ServerlessVpcAccessManagedFirewallRule = 4,
        /// Global network firewall policy rule.
        /// For details, see [Network firewall
        /// policies](<https://cloud.google.com/vpc/docs/network-firewall-policies>).
        NetworkFirewallPolicyRule = 5,
        /// Regional network firewall policy rule.
        /// For details, see [Regional network firewall
        /// policies](<https://cloud.google.com/firewall/docs/regional-firewall-policies>).
        NetworkRegionalFirewallPolicyRule = 6,
        /// Firewall policy rule containing attributes not yet supported in
        /// Connectivity tests. Firewall analysis is skipped if such a rule can
        /// potentially be matched. Please see the [list of unsupported
        /// configurations](<https://cloud.google.com/network-intelligence-center/docs/connectivity-tests/concepts/overview#unsupported-configs>).
        UnsupportedFirewallPolicyRule = 100,
        /// Tracking state for response traffic created when request traffic goes
        /// through allow firewall rule.
        /// For details, see [firewall rules
        /// specifications](<https://cloud.google.com/firewall/docs/firewalls#specifications>)
        TrackingState = 101,
    }
    impl FirewallRuleType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FirewallRuleType::Unspecified => "FIREWALL_RULE_TYPE_UNSPECIFIED",
                FirewallRuleType::HierarchicalFirewallPolicyRule => {
                    "HIERARCHICAL_FIREWALL_POLICY_RULE"
                }
                FirewallRuleType::VpcFirewallRule => "VPC_FIREWALL_RULE",
                FirewallRuleType::ImpliedVpcFirewallRule => "IMPLIED_VPC_FIREWALL_RULE",
                FirewallRuleType::ServerlessVpcAccessManagedFirewallRule => {
                    "SERVERLESS_VPC_ACCESS_MANAGED_FIREWALL_RULE"
                }
                FirewallRuleType::NetworkFirewallPolicyRule => {
                    "NETWORK_FIREWALL_POLICY_RULE"
                }
                FirewallRuleType::NetworkRegionalFirewallPolicyRule => {
                    "NETWORK_REGIONAL_FIREWALL_POLICY_RULE"
                }
                FirewallRuleType::UnsupportedFirewallPolicyRule => {
                    "UNSUPPORTED_FIREWALL_POLICY_RULE"
                }
                FirewallRuleType::TrackingState => "TRACKING_STATE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FIREWALL_RULE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "HIERARCHICAL_FIREWALL_POLICY_RULE" => {
                    Some(Self::HierarchicalFirewallPolicyRule)
                }
                "VPC_FIREWALL_RULE" => Some(Self::VpcFirewallRule),
                "IMPLIED_VPC_FIREWALL_RULE" => Some(Self::ImpliedVpcFirewallRule),
                "SERVERLESS_VPC_ACCESS_MANAGED_FIREWALL_RULE" => {
                    Some(Self::ServerlessVpcAccessManagedFirewallRule)
                }
                "NETWORK_FIREWALL_POLICY_RULE" => Some(Self::NetworkFirewallPolicyRule),
                "NETWORK_REGIONAL_FIREWALL_POLICY_RULE" => {
                    Some(Self::NetworkRegionalFirewallPolicyRule)
                }
                "UNSUPPORTED_FIREWALL_POLICY_RULE" => {
                    Some(Self::UnsupportedFirewallPolicyRule)
                }
                "TRACKING_STATE" => Some(Self::TrackingState),
                _ => None,
            }
        }
    }
}
/// For display only. Metadata associated with a Compute Engine route.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteInfo {
    /// Type of route.
    #[prost(enumeration = "route_info::RouteType", tag = "8")]
    pub route_type: i32,
    /// Type of next hop.
    #[prost(enumeration = "route_info::NextHopType", tag = "9")]
    pub next_hop_type: i32,
    /// Indicates where route is applicable.
    #[prost(enumeration = "route_info::RouteScope", tag = "14")]
    pub route_scope: i32,
    /// Name of a route.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// URI of a route.
    /// Dynamic, peering static and peering dynamic routes do not have an URI.
    /// Advertised route from Google Cloud VPC to on-premises network also does
    /// not have an URI.
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// Destination IP range of the route.
    #[prost(string, tag = "3")]
    pub dest_ip_range: ::prost::alloc::string::String,
    /// Next hop of the route.
    #[prost(string, tag = "4")]
    pub next_hop: ::prost::alloc::string::String,
    /// URI of a Compute Engine network. NETWORK routes only.
    #[prost(string, tag = "5")]
    pub network_uri: ::prost::alloc::string::String,
    /// Priority of the route.
    #[prost(int32, tag = "6")]
    pub priority: i32,
    /// Instance tags of the route.
    #[prost(string, repeated, tag = "7")]
    pub instance_tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Source IP address range of the route. Policy based routes only.
    #[prost(string, tag = "10")]
    pub src_ip_range: ::prost::alloc::string::String,
    /// Destination port ranges of the route. Policy based routes only.
    #[prost(string, repeated, tag = "11")]
    pub dest_port_ranges: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Source port ranges of the route. Policy based routes only.
    #[prost(string, repeated, tag = "12")]
    pub src_port_ranges: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Protocols of the route. Policy based routes only.
    #[prost(string, repeated, tag = "13")]
    pub protocols: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// URI of a NCC Hub. NCC_HUB routes only.
    #[prost(string, optional, tag = "15")]
    pub ncc_hub_uri: ::core::option::Option<::prost::alloc::string::String>,
    /// URI of a NCC Spoke. NCC_HUB routes only.
    #[prost(string, optional, tag = "16")]
    pub ncc_spoke_uri: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `RouteInfo`.
pub mod route_info {
    /// Type of route:
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
    pub enum RouteType {
        /// Unspecified type. Default value.
        Unspecified = 0,
        /// Route is a subnet route automatically created by the system.
        Subnet = 1,
        /// Static route created by the user, including the default route to the
        /// internet.
        Static = 2,
        /// Dynamic route exchanged between BGP peers.
        Dynamic = 3,
        /// A subnet route received from peering network.
        PeeringSubnet = 4,
        /// A static route received from peering network.
        PeeringStatic = 5,
        /// A dynamic route received from peering network.
        PeeringDynamic = 6,
        /// Policy based route.
        PolicyBased = 7,
    }
    impl RouteType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RouteType::Unspecified => "ROUTE_TYPE_UNSPECIFIED",
                RouteType::Subnet => "SUBNET",
                RouteType::Static => "STATIC",
                RouteType::Dynamic => "DYNAMIC",
                RouteType::PeeringSubnet => "PEERING_SUBNET",
                RouteType::PeeringStatic => "PEERING_STATIC",
                RouteType::PeeringDynamic => "PEERING_DYNAMIC",
                RouteType::PolicyBased => "POLICY_BASED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ROUTE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "SUBNET" => Some(Self::Subnet),
                "STATIC" => Some(Self::Static),
                "DYNAMIC" => Some(Self::Dynamic),
                "PEERING_SUBNET" => Some(Self::PeeringSubnet),
                "PEERING_STATIC" => Some(Self::PeeringStatic),
                "PEERING_DYNAMIC" => Some(Self::PeeringDynamic),
                "POLICY_BASED" => Some(Self::PolicyBased),
                _ => None,
            }
        }
    }
    /// Type of next hop:
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
    pub enum NextHopType {
        /// Unspecified type. Default value.
        Unspecified = 0,
        /// Next hop is an IP address.
        NextHopIp = 1,
        /// Next hop is a Compute Engine instance.
        NextHopInstance = 2,
        /// Next hop is a VPC network gateway.
        NextHopNetwork = 3,
        /// Next hop is a peering VPC.
        NextHopPeering = 4,
        /// Next hop is an interconnect.
        NextHopInterconnect = 5,
        /// Next hop is a VPN tunnel.
        NextHopVpnTunnel = 6,
        /// Next hop is a VPN gateway. This scenario only happens when tracing
        /// connectivity from an on-premises network to Google Cloud through a VPN.
        /// The analysis simulates a packet departing from the on-premises network
        /// through a VPN tunnel and arriving at a Cloud VPN gateway.
        NextHopVpnGateway = 7,
        /// Next hop is an internet gateway.
        NextHopInternetGateway = 8,
        /// Next hop is blackhole; that is, the next hop either does not exist or is
        /// not running.
        NextHopBlackhole = 9,
        /// Next hop is the forwarding rule of an Internal Load Balancer.
        NextHopIlb = 10,
        /// Next hop is a
        /// [router appliance
        /// instance](<https://cloud.google.com/network-connectivity/docs/network-connectivity-center/concepts/ra-overview>).
        NextHopRouterAppliance = 11,
        /// Next hop is an NCC hub.
        NextHopNccHub = 12,
    }
    impl NextHopType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                NextHopType::Unspecified => "NEXT_HOP_TYPE_UNSPECIFIED",
                NextHopType::NextHopIp => "NEXT_HOP_IP",
                NextHopType::NextHopInstance => "NEXT_HOP_INSTANCE",
                NextHopType::NextHopNetwork => "NEXT_HOP_NETWORK",
                NextHopType::NextHopPeering => "NEXT_HOP_PEERING",
                NextHopType::NextHopInterconnect => "NEXT_HOP_INTERCONNECT",
                NextHopType::NextHopVpnTunnel => "NEXT_HOP_VPN_TUNNEL",
                NextHopType::NextHopVpnGateway => "NEXT_HOP_VPN_GATEWAY",
                NextHopType::NextHopInternetGateway => "NEXT_HOP_INTERNET_GATEWAY",
                NextHopType::NextHopBlackhole => "NEXT_HOP_BLACKHOLE",
                NextHopType::NextHopIlb => "NEXT_HOP_ILB",
                NextHopType::NextHopRouterAppliance => "NEXT_HOP_ROUTER_APPLIANCE",
                NextHopType::NextHopNccHub => "NEXT_HOP_NCC_HUB",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NEXT_HOP_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "NEXT_HOP_IP" => Some(Self::NextHopIp),
                "NEXT_HOP_INSTANCE" => Some(Self::NextHopInstance),
                "NEXT_HOP_NETWORK" => Some(Self::NextHopNetwork),
                "NEXT_HOP_PEERING" => Some(Self::NextHopPeering),
                "NEXT_HOP_INTERCONNECT" => Some(Self::NextHopInterconnect),
                "NEXT_HOP_VPN_TUNNEL" => Some(Self::NextHopVpnTunnel),
                "NEXT_HOP_VPN_GATEWAY" => Some(Self::NextHopVpnGateway),
                "NEXT_HOP_INTERNET_GATEWAY" => Some(Self::NextHopInternetGateway),
                "NEXT_HOP_BLACKHOLE" => Some(Self::NextHopBlackhole),
                "NEXT_HOP_ILB" => Some(Self::NextHopIlb),
                "NEXT_HOP_ROUTER_APPLIANCE" => Some(Self::NextHopRouterAppliance),
                "NEXT_HOP_NCC_HUB" => Some(Self::NextHopNccHub),
                _ => None,
            }
        }
    }
    /// Indicates where routes are applicable.
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
    pub enum RouteScope {
        /// Unspecified scope. Default value.
        Unspecified = 0,
        /// Route is applicable to packets in Network.
        Network = 1,
        /// Route is applicable to packets using NCC Hub's routing table.
        NccHub = 2,
    }
    impl RouteScope {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RouteScope::Unspecified => "ROUTE_SCOPE_UNSPECIFIED",
                RouteScope::Network => "NETWORK",
                RouteScope::NccHub => "NCC_HUB",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ROUTE_SCOPE_UNSPECIFIED" => Some(Self::Unspecified),
                "NETWORK" => Some(Self::Network),
                "NCC_HUB" => Some(Self::NccHub),
                _ => None,
            }
        }
    }
}
/// For display only. Details of a Google Service sending packets to a
/// VPC network. Although the source IP might be a publicly routable address,
/// some Google Services use special routes within Google production
/// infrastructure to reach Compute Engine Instances.
/// <https://cloud.google.com/vpc/docs/routes#special_return_paths>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoogleServiceInfo {
    /// Source IP address.
    #[prost(string, tag = "1")]
    pub source_ip: ::prost::alloc::string::String,
    /// Recognized type of a Google Service.
    #[prost(enumeration = "google_service_info::GoogleServiceType", tag = "2")]
    pub google_service_type: i32,
}
/// Nested message and enum types in `GoogleServiceInfo`.
pub mod google_service_info {
    /// Recognized type of a Google Service.
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
    pub enum GoogleServiceType {
        /// Unspecified Google Service.
        Unspecified = 0,
        /// Identity aware proxy.
        /// <https://cloud.google.com/iap/docs/using-tcp-forwarding>
        Iap = 1,
        /// One of two services sharing IP ranges:
        /// * Load Balancer proxy
        /// * Centralized Health Check prober
        /// <https://cloud.google.com/load-balancing/docs/firewall-rules>
        GfeProxyOrHealthCheckProber = 2,
        /// Connectivity from Cloud DNS to forwarding targets or alternate name
        /// servers that use private routing.
        /// <https://cloud.google.com/dns/docs/zones/forwarding-zones#firewall-rules>
        /// <https://cloud.google.com/dns/docs/policies#firewall-rules>
        CloudDns = 3,
        /// private.googleapis.com and restricted.googleapis.com
        GoogleApi = 4,
        /// Google API via Private Service Connect.
        /// <https://cloud.google.com/vpc/docs/configure-private-service-connect-apis>
        GoogleApiPsc = 5,
        /// Google API via VPC Service Controls.
        /// <https://cloud.google.com/vpc/docs/configure-private-service-connect-apis>
        GoogleApiVpcSc = 6,
    }
    impl GoogleServiceType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                GoogleServiceType::Unspecified => "GOOGLE_SERVICE_TYPE_UNSPECIFIED",
                GoogleServiceType::Iap => "IAP",
                GoogleServiceType::GfeProxyOrHealthCheckProber => {
                    "GFE_PROXY_OR_HEALTH_CHECK_PROBER"
                }
                GoogleServiceType::CloudDns => "CLOUD_DNS",
                GoogleServiceType::GoogleApi => "GOOGLE_API",
                GoogleServiceType::GoogleApiPsc => "GOOGLE_API_PSC",
                GoogleServiceType::GoogleApiVpcSc => "GOOGLE_API_VPC_SC",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "GOOGLE_SERVICE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "IAP" => Some(Self::Iap),
                "GFE_PROXY_OR_HEALTH_CHECK_PROBER" => {
                    Some(Self::GfeProxyOrHealthCheckProber)
                }
                "CLOUD_DNS" => Some(Self::CloudDns),
                "GOOGLE_API" => Some(Self::GoogleApi),
                "GOOGLE_API_PSC" => Some(Self::GoogleApiPsc),
                "GOOGLE_API_VPC_SC" => Some(Self::GoogleApiVpcSc),
                _ => None,
            }
        }
    }
}
/// For display only. Metadata associated with a Compute Engine forwarding rule.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ForwardingRuleInfo {
    /// Name of a Compute Engine forwarding rule.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// URI of a Compute Engine forwarding rule.
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// Protocol defined in the forwarding rule that matches the test.
    #[prost(string, tag = "3")]
    pub matched_protocol: ::prost::alloc::string::String,
    /// Port range defined in the forwarding rule that matches the test.
    #[prost(string, tag = "6")]
    pub matched_port_range: ::prost::alloc::string::String,
    /// VIP of the forwarding rule.
    #[prost(string, tag = "4")]
    pub vip: ::prost::alloc::string::String,
    /// Target type of the forwarding rule.
    #[prost(string, tag = "5")]
    pub target: ::prost::alloc::string::String,
    /// Network URI. Only valid for Internal Load Balancer.
    #[prost(string, tag = "7")]
    pub network_uri: ::prost::alloc::string::String,
}
/// For display only. Metadata associated with a load balancer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadBalancerInfo {
    /// Type of the load balancer.
    #[prost(enumeration = "load_balancer_info::LoadBalancerType", tag = "1")]
    pub load_balancer_type: i32,
    /// URI of the health check for the load balancer. Deprecated and no longer
    /// populated as different load balancer backends might have different health
    /// checks.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub health_check_uri: ::prost::alloc::string::String,
    /// Information for the loadbalancer backends.
    #[prost(message, repeated, tag = "3")]
    pub backends: ::prost::alloc::vec::Vec<LoadBalancerBackend>,
    /// Type of load balancer's backend configuration.
    #[prost(enumeration = "load_balancer_info::BackendType", tag = "4")]
    pub backend_type: i32,
    /// Backend configuration URI.
    #[prost(string, tag = "5")]
    pub backend_uri: ::prost::alloc::string::String,
}
/// Nested message and enum types in `LoadBalancerInfo`.
pub mod load_balancer_info {
    /// The type definition for a load balancer:
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
    pub enum LoadBalancerType {
        /// Type is unspecified.
        Unspecified = 0,
        /// Internal TCP/UDP load balancer.
        InternalTcpUdp = 1,
        /// Network TCP/UDP load balancer.
        NetworkTcpUdp = 2,
        /// HTTP(S) proxy load balancer.
        HttpProxy = 3,
        /// TCP proxy load balancer.
        TcpProxy = 4,
        /// SSL proxy load balancer.
        SslProxy = 5,
    }
    impl LoadBalancerType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LoadBalancerType::Unspecified => "LOAD_BALANCER_TYPE_UNSPECIFIED",
                LoadBalancerType::InternalTcpUdp => "INTERNAL_TCP_UDP",
                LoadBalancerType::NetworkTcpUdp => "NETWORK_TCP_UDP",
                LoadBalancerType::HttpProxy => "HTTP_PROXY",
                LoadBalancerType::TcpProxy => "TCP_PROXY",
                LoadBalancerType::SslProxy => "SSL_PROXY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LOAD_BALANCER_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "INTERNAL_TCP_UDP" => Some(Self::InternalTcpUdp),
                "NETWORK_TCP_UDP" => Some(Self::NetworkTcpUdp),
                "HTTP_PROXY" => Some(Self::HttpProxy),
                "TCP_PROXY" => Some(Self::TcpProxy),
                "SSL_PROXY" => Some(Self::SslProxy),
                _ => None,
            }
        }
    }
    /// The type definition for a load balancer backend configuration:
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
    pub enum BackendType {
        /// Type is unspecified.
        Unspecified = 0,
        /// Backend Service as the load balancer's backend.
        BackendService = 1,
        /// Target Pool as the load balancer's backend.
        TargetPool = 2,
        /// Target Instance as the load balancer's backend.
        TargetInstance = 3,
    }
    impl BackendType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                BackendType::Unspecified => "BACKEND_TYPE_UNSPECIFIED",
                BackendType::BackendService => "BACKEND_SERVICE",
                BackendType::TargetPool => "TARGET_POOL",
                BackendType::TargetInstance => "TARGET_INSTANCE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BACKEND_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "BACKEND_SERVICE" => Some(Self::BackendService),
                "TARGET_POOL" => Some(Self::TargetPool),
                "TARGET_INSTANCE" => Some(Self::TargetInstance),
                _ => None,
            }
        }
    }
}
/// For display only. Metadata associated with a specific load balancer backend.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadBalancerBackend {
    /// Name of a Compute Engine instance or network endpoint.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// URI of a Compute Engine instance or network endpoint.
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// State of the health check firewall configuration.
    #[prost(enumeration = "load_balancer_backend::HealthCheckFirewallState", tag = "3")]
    pub health_check_firewall_state: i32,
    /// A list of firewall rule URIs allowing probes from health check IP ranges.
    #[prost(string, repeated, tag = "4")]
    pub health_check_allowing_firewall_rules: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// A list of firewall rule URIs blocking probes from health check IP ranges.
    #[prost(string, repeated, tag = "5")]
    pub health_check_blocking_firewall_rules: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
}
/// Nested message and enum types in `LoadBalancerBackend`.
pub mod load_balancer_backend {
    /// State of a health check firewall configuration:
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
    pub enum HealthCheckFirewallState {
        /// State is unspecified. Default state if not populated.
        Unspecified = 0,
        /// There are configured firewall rules to allow health check probes to the
        /// backend.
        Configured = 1,
        /// There are firewall rules configured to allow partial health check ranges
        /// or block all health check ranges.
        /// If a health check probe is sent from denied IP ranges,
        /// the health check to the backend will fail. Then, the backend will be
        /// marked unhealthy and will not receive traffic sent to the load balancer.
        Misconfigured = 2,
    }
    impl HealthCheckFirewallState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                HealthCheckFirewallState::Unspecified => {
                    "HEALTH_CHECK_FIREWALL_STATE_UNSPECIFIED"
                }
                HealthCheckFirewallState::Configured => "CONFIGURED",
                HealthCheckFirewallState::Misconfigured => "MISCONFIGURED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "HEALTH_CHECK_FIREWALL_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CONFIGURED" => Some(Self::Configured),
                "MISCONFIGURED" => Some(Self::Misconfigured),
                _ => None,
            }
        }
    }
}
/// For display only. Metadata associated with a Compute Engine VPN gateway.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VpnGatewayInfo {
    /// Name of a VPN gateway.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// URI of a VPN gateway.
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// URI of a Compute Engine network where the VPN gateway is configured.
    #[prost(string, tag = "3")]
    pub network_uri: ::prost::alloc::string::String,
    /// IP address of the VPN gateway.
    #[prost(string, tag = "4")]
    pub ip_address: ::prost::alloc::string::String,
    /// A VPN tunnel that is associated with this VPN gateway.
    /// There may be multiple VPN tunnels configured on a VPN gateway, and only
    /// the one relevant to the test is displayed.
    #[prost(string, tag = "5")]
    pub vpn_tunnel_uri: ::prost::alloc::string::String,
    /// Name of a Google Cloud region where this VPN gateway is configured.
    #[prost(string, tag = "6")]
    pub region: ::prost::alloc::string::String,
}
/// For display only. Metadata associated with a Compute Engine VPN tunnel.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VpnTunnelInfo {
    /// Name of a VPN tunnel.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// URI of a VPN tunnel.
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// URI of the VPN gateway at local end of the tunnel.
    #[prost(string, tag = "3")]
    pub source_gateway: ::prost::alloc::string::String,
    /// URI of a VPN gateway at remote end of the tunnel.
    #[prost(string, tag = "4")]
    pub remote_gateway: ::prost::alloc::string::String,
    /// Remote VPN gateway's IP address.
    #[prost(string, tag = "5")]
    pub remote_gateway_ip: ::prost::alloc::string::String,
    /// Local VPN gateway's IP address.
    #[prost(string, tag = "6")]
    pub source_gateway_ip: ::prost::alloc::string::String,
    /// URI of a Compute Engine network where the VPN tunnel is configured.
    #[prost(string, tag = "7")]
    pub network_uri: ::prost::alloc::string::String,
    /// Name of a Google Cloud region where this VPN tunnel is configured.
    #[prost(string, tag = "8")]
    pub region: ::prost::alloc::string::String,
    /// Type of the routing policy.
    #[prost(enumeration = "vpn_tunnel_info::RoutingType", tag = "9")]
    pub routing_type: i32,
}
/// Nested message and enum types in `VpnTunnelInfo`.
pub mod vpn_tunnel_info {
    /// Types of VPN routing policy. For details, refer to [Networks and Tunnel
    /// routing](<https://cloud.google.com/network-connectivity/docs/vpn/concepts/choosing-networks-routing/>).
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
    pub enum RoutingType {
        /// Unspecified type. Default value.
        Unspecified = 0,
        /// Route based VPN.
        RouteBased = 1,
        /// Policy based routing.
        PolicyBased = 2,
        /// Dynamic (BGP) routing.
        Dynamic = 3,
    }
    impl RoutingType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RoutingType::Unspecified => "ROUTING_TYPE_UNSPECIFIED",
                RoutingType::RouteBased => "ROUTE_BASED",
                RoutingType::PolicyBased => "POLICY_BASED",
                RoutingType::Dynamic => "DYNAMIC",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ROUTING_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "ROUTE_BASED" => Some(Self::RouteBased),
                "POLICY_BASED" => Some(Self::PolicyBased),
                "DYNAMIC" => Some(Self::Dynamic),
                _ => None,
            }
        }
    }
}
/// For display only. The specification of the endpoints for the test.
/// EndpointInfo is derived from source and destination Endpoint and validated
/// by the backend data plane model.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndpointInfo {
    /// Source IP address.
    #[prost(string, tag = "1")]
    pub source_ip: ::prost::alloc::string::String,
    /// Destination IP address.
    #[prost(string, tag = "2")]
    pub destination_ip: ::prost::alloc::string::String,
    /// IP protocol in string format, for example: "TCP", "UDP", "ICMP".
    #[prost(string, tag = "3")]
    pub protocol: ::prost::alloc::string::String,
    /// Source port. Only valid when protocol is TCP or UDP.
    #[prost(int32, tag = "4")]
    pub source_port: i32,
    /// Destination port. Only valid when protocol is TCP or UDP.
    #[prost(int32, tag = "5")]
    pub destination_port: i32,
    /// URI of the network where this packet originates from.
    #[prost(string, tag = "6")]
    pub source_network_uri: ::prost::alloc::string::String,
    /// URI of the network where this packet is sent to.
    #[prost(string, tag = "7")]
    pub destination_network_uri: ::prost::alloc::string::String,
    /// URI of the source telemetry agent this packet originates from.
    #[prost(string, tag = "8")]
    pub source_agent_uri: ::prost::alloc::string::String,
}
/// Details of the final state "deliver" and associated resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeliverInfo {
    /// Target type where the packet is delivered to.
    #[prost(enumeration = "deliver_info::Target", tag = "1")]
    pub target: i32,
    /// URI of the resource that the packet is delivered to.
    #[prost(string, tag = "2")]
    pub resource_uri: ::prost::alloc::string::String,
    /// IP address of the target (if applicable).
    #[prost(string, tag = "3")]
    pub ip_address: ::prost::alloc::string::String,
}
/// Nested message and enum types in `DeliverInfo`.
pub mod deliver_info {
    /// Deliver target types:
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
    pub enum Target {
        /// Target not specified.
        Unspecified = 0,
        /// Target is a Compute Engine instance.
        Instance = 1,
        /// Target is the internet.
        Internet = 2,
        /// Target is a Google API.
        GoogleApi = 3,
        /// Target is a Google Kubernetes Engine cluster master.
        GkeMaster = 4,
        /// Target is a Cloud SQL instance.
        CloudSqlInstance = 5,
        /// Target is a published service that uses [Private Service
        /// Connect](<https://cloud.google.com/vpc/docs/configure-private-service-connect-services>).
        PscPublishedService = 6,
        /// Target is all Google APIs that use [Private Service
        /// Connect](<https://cloud.google.com/vpc/docs/configure-private-service-connect-apis>).
        PscGoogleApi = 7,
        /// Target is a VPC-SC that uses [Private Service
        /// Connect](<https://cloud.google.com/vpc/docs/configure-private-service-connect-apis>).
        PscVpcSc = 8,
        /// Target is a serverless network endpoint group.
        ServerlessNeg = 9,
        /// Target is a Cloud Storage bucket.
        StorageBucket = 10,
        /// Target is a private network. Used only for return traces.
        PrivateNetwork = 11,
        /// Target is a Cloud Function. Used only for return traces.
        CloudFunction = 12,
        /// Target is a App Engine service version. Used only for return traces.
        AppEngineVersion = 13,
        /// Target is a Cloud Run revision. Used only for return traces.
        CloudRunRevision = 14,
    }
    impl Target {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Target::Unspecified => "TARGET_UNSPECIFIED",
                Target::Instance => "INSTANCE",
                Target::Internet => "INTERNET",
                Target::GoogleApi => "GOOGLE_API",
                Target::GkeMaster => "GKE_MASTER",
                Target::CloudSqlInstance => "CLOUD_SQL_INSTANCE",
                Target::PscPublishedService => "PSC_PUBLISHED_SERVICE",
                Target::PscGoogleApi => "PSC_GOOGLE_API",
                Target::PscVpcSc => "PSC_VPC_SC",
                Target::ServerlessNeg => "SERVERLESS_NEG",
                Target::StorageBucket => "STORAGE_BUCKET",
                Target::PrivateNetwork => "PRIVATE_NETWORK",
                Target::CloudFunction => "CLOUD_FUNCTION",
                Target::AppEngineVersion => "APP_ENGINE_VERSION",
                Target::CloudRunRevision => "CLOUD_RUN_REVISION",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TARGET_UNSPECIFIED" => Some(Self::Unspecified),
                "INSTANCE" => Some(Self::Instance),
                "INTERNET" => Some(Self::Internet),
                "GOOGLE_API" => Some(Self::GoogleApi),
                "GKE_MASTER" => Some(Self::GkeMaster),
                "CLOUD_SQL_INSTANCE" => Some(Self::CloudSqlInstance),
                "PSC_PUBLISHED_SERVICE" => Some(Self::PscPublishedService),
                "PSC_GOOGLE_API" => Some(Self::PscGoogleApi),
                "PSC_VPC_SC" => Some(Self::PscVpcSc),
                "SERVERLESS_NEG" => Some(Self::ServerlessNeg),
                "STORAGE_BUCKET" => Some(Self::StorageBucket),
                "PRIVATE_NETWORK" => Some(Self::PrivateNetwork),
                "CLOUD_FUNCTION" => Some(Self::CloudFunction),
                "APP_ENGINE_VERSION" => Some(Self::AppEngineVersion),
                "CLOUD_RUN_REVISION" => Some(Self::CloudRunRevision),
                _ => None,
            }
        }
    }
}
/// Details of the final state "forward" and associated resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ForwardInfo {
    /// Target type where this packet is forwarded to.
    #[prost(enumeration = "forward_info::Target", tag = "1")]
    pub target: i32,
    /// URI of the resource that the packet is forwarded to.
    #[prost(string, tag = "2")]
    pub resource_uri: ::prost::alloc::string::String,
    /// IP address of the target (if applicable).
    #[prost(string, tag = "3")]
    pub ip_address: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ForwardInfo`.
pub mod forward_info {
    /// Forward target types.
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
    pub enum Target {
        /// Target not specified.
        Unspecified = 0,
        /// Forwarded to a VPC peering network.
        PeeringVpc = 1,
        /// Forwarded to a Cloud VPN gateway.
        VpnGateway = 2,
        /// Forwarded to a Cloud Interconnect connection.
        Interconnect = 3,
        /// Forwarded to a Google Kubernetes Engine Container cluster master.
        GkeMaster = 4,
        /// Forwarded to the next hop of a custom route imported from a peering VPC.
        ImportedCustomRouteNextHop = 5,
        /// Forwarded to a Cloud SQL instance.
        CloudSqlInstance = 6,
        /// Forwarded to a VPC network in another project.
        AnotherProject = 7,
        /// Forwarded to an NCC Hub.
        NccHub = 8,
        /// Forwarded to a router appliance.
        RouterAppliance = 9,
    }
    impl Target {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Target::Unspecified => "TARGET_UNSPECIFIED",
                Target::PeeringVpc => "PEERING_VPC",
                Target::VpnGateway => "VPN_GATEWAY",
                Target::Interconnect => "INTERCONNECT",
                Target::GkeMaster => "GKE_MASTER",
                Target::ImportedCustomRouteNextHop => "IMPORTED_CUSTOM_ROUTE_NEXT_HOP",
                Target::CloudSqlInstance => "CLOUD_SQL_INSTANCE",
                Target::AnotherProject => "ANOTHER_PROJECT",
                Target::NccHub => "NCC_HUB",
                Target::RouterAppliance => "ROUTER_APPLIANCE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TARGET_UNSPECIFIED" => Some(Self::Unspecified),
                "PEERING_VPC" => Some(Self::PeeringVpc),
                "VPN_GATEWAY" => Some(Self::VpnGateway),
                "INTERCONNECT" => Some(Self::Interconnect),
                "GKE_MASTER" => Some(Self::GkeMaster),
                "IMPORTED_CUSTOM_ROUTE_NEXT_HOP" => {
                    Some(Self::ImportedCustomRouteNextHop)
                }
                "CLOUD_SQL_INSTANCE" => Some(Self::CloudSqlInstance),
                "ANOTHER_PROJECT" => Some(Self::AnotherProject),
                "NCC_HUB" => Some(Self::NccHub),
                "ROUTER_APPLIANCE" => Some(Self::RouterAppliance),
                _ => None,
            }
        }
    }
}
/// Details of the final state "abort" and associated resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AbortInfo {
    /// Causes that the analysis is aborted.
    #[prost(enumeration = "abort_info::Cause", tag = "1")]
    pub cause: i32,
    /// URI of the resource that caused the abort.
    #[prost(string, tag = "2")]
    pub resource_uri: ::prost::alloc::string::String,
    /// IP address that caused the abort.
    #[prost(string, tag = "4")]
    pub ip_address: ::prost::alloc::string::String,
    /// List of project IDs the user specified in the request but lacks access to.
    /// In this case, analysis is aborted with the PERMISSION_DENIED cause.
    #[prost(string, repeated, tag = "3")]
    pub projects_missing_permission: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
}
/// Nested message and enum types in `AbortInfo`.
pub mod abort_info {
    /// Abort cause types:
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
    pub enum Cause {
        /// Cause is unspecified.
        Unspecified = 0,
        /// Aborted due to unknown network. Deprecated, not used in the new tests.
        UnknownNetwork = 1,
        /// Aborted because no project information can be derived from the test
        /// input. Deprecated, not used in the new tests.
        UnknownProject = 3,
        /// Aborted because traffic is sent from a public IP to an instance without
        /// an external IP. Deprecated, not used in the new tests.
        NoExternalIp = 7,
        /// Aborted because none of the traces matches destination information
        /// specified in the input test request. Deprecated, not used in the new
        /// tests.
        UnintendedDestination = 8,
        /// Aborted because the source endpoint could not be found. Deprecated, not
        /// used in the new tests.
        SourceEndpointNotFound = 11,
        /// Aborted because the source network does not match the source endpoint.
        /// Deprecated, not used in the new tests.
        MismatchedSourceNetwork = 12,
        /// Aborted because the destination endpoint could not be found. Deprecated,
        /// not used in the new tests.
        DestinationEndpointNotFound = 13,
        /// Aborted because the destination network does not match the destination
        /// endpoint. Deprecated, not used in the new tests.
        MismatchedDestinationNetwork = 14,
        /// Aborted because no endpoint with the packet's destination IP address is
        /// found.
        UnknownIp = 2,
        /// Aborted because the source IP address doesn't belong to any of the
        /// subnets of the source VPC network.
        SourceIpAddressNotInSourceNetwork = 23,
        /// Aborted because user lacks permission to access all or part of the
        /// network configurations required to run the test.
        PermissionDenied = 4,
        /// Aborted because user lacks permission to access Cloud NAT configs
        /// required to run the test.
        PermissionDeniedNoCloudNatConfigs = 28,
        /// Aborted because user lacks permission to access Network endpoint group
        /// endpoint configs required to run the test.
        PermissionDeniedNoNegEndpointConfigs = 29,
        /// Aborted because no valid source or destination endpoint is derived from
        /// the input test request.
        NoSourceLocation = 5,
        /// Aborted because the source or destination endpoint specified in
        /// the request is invalid. Some examples:
        /// - The request might contain malformed resource URI, project ID, or IP
        /// address.
        /// - The request might contain inconsistent information (for example, the
        /// request might include both the instance and the network, but the instance
        /// might not have a NIC in that network).
        InvalidArgument = 6,
        /// Aborted because the number of steps in the trace exceeds a certain
        /// limit. It might be caused by a routing loop.
        TraceTooLong = 9,
        /// Aborted due to internal server error.
        InternalError = 10,
        /// Aborted because the test scenario is not supported.
        Unsupported = 15,
        /// Aborted because the source and destination resources have no common IP
        /// version.
        MismatchedIpVersion = 16,
        /// Aborted because the connection between the control plane and the node of
        /// the source cluster is initiated by the node and managed by the
        /// Konnectivity proxy.
        GkeKonnectivityProxyUnsupported = 17,
        /// Aborted because expected resource configuration was missing.
        ResourceConfigNotFound = 18,
        /// Aborted because expected VM instance configuration was missing.
        VmInstanceConfigNotFound = 24,
        /// Aborted because expected network configuration was missing.
        NetworkConfigNotFound = 25,
        /// Aborted because expected firewall configuration was missing.
        FirewallConfigNotFound = 26,
        /// Aborted because expected route configuration was missing.
        RouteConfigNotFound = 27,
        /// Aborted because a PSC endpoint selection for the Google-managed service
        /// is ambiguous (several PSC endpoints satisfy test input).
        GoogleManagedServiceAmbiguousPscEndpoint = 19,
        /// Aborted because tests with a PSC-based Cloud SQL instance as a source are
        /// not supported.
        SourcePscCloudSqlUnsupported = 20,
        /// Aborted because tests with a forwarding rule as a source are not
        /// supported.
        SourceForwardingRuleUnsupported = 21,
        /// Aborted because one of the endpoints is a non-routable IP address
        /// (loopback, link-local, etc).
        NonRoutableIpAddress = 22,
        /// Aborted due to an unknown issue in the Google-managed project.
        UnknownIssueInGoogleManagedProject = 30,
        /// Aborted due to an unsupported configuration of the Google-managed
        /// project.
        UnsupportedGoogleManagedProjectConfig = 31,
    }
    impl Cause {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Cause::Unspecified => "CAUSE_UNSPECIFIED",
                Cause::UnknownNetwork => "UNKNOWN_NETWORK",
                Cause::UnknownProject => "UNKNOWN_PROJECT",
                Cause::NoExternalIp => "NO_EXTERNAL_IP",
                Cause::UnintendedDestination => "UNINTENDED_DESTINATION",
                Cause::SourceEndpointNotFound => "SOURCE_ENDPOINT_NOT_FOUND",
                Cause::MismatchedSourceNetwork => "MISMATCHED_SOURCE_NETWORK",
                Cause::DestinationEndpointNotFound => "DESTINATION_ENDPOINT_NOT_FOUND",
                Cause::MismatchedDestinationNetwork => "MISMATCHED_DESTINATION_NETWORK",
                Cause::UnknownIp => "UNKNOWN_IP",
                Cause::SourceIpAddressNotInSourceNetwork => {
                    "SOURCE_IP_ADDRESS_NOT_IN_SOURCE_NETWORK"
                }
                Cause::PermissionDenied => "PERMISSION_DENIED",
                Cause::PermissionDeniedNoCloudNatConfigs => {
                    "PERMISSION_DENIED_NO_CLOUD_NAT_CONFIGS"
                }
                Cause::PermissionDeniedNoNegEndpointConfigs => {
                    "PERMISSION_DENIED_NO_NEG_ENDPOINT_CONFIGS"
                }
                Cause::NoSourceLocation => "NO_SOURCE_LOCATION",
                Cause::InvalidArgument => "INVALID_ARGUMENT",
                Cause::TraceTooLong => "TRACE_TOO_LONG",
                Cause::InternalError => "INTERNAL_ERROR",
                Cause::Unsupported => "UNSUPPORTED",
                Cause::MismatchedIpVersion => "MISMATCHED_IP_VERSION",
                Cause::GkeKonnectivityProxyUnsupported => {
                    "GKE_KONNECTIVITY_PROXY_UNSUPPORTED"
                }
                Cause::ResourceConfigNotFound => "RESOURCE_CONFIG_NOT_FOUND",
                Cause::VmInstanceConfigNotFound => "VM_INSTANCE_CONFIG_NOT_FOUND",
                Cause::NetworkConfigNotFound => "NETWORK_CONFIG_NOT_FOUND",
                Cause::FirewallConfigNotFound => "FIREWALL_CONFIG_NOT_FOUND",
                Cause::RouteConfigNotFound => "ROUTE_CONFIG_NOT_FOUND",
                Cause::GoogleManagedServiceAmbiguousPscEndpoint => {
                    "GOOGLE_MANAGED_SERVICE_AMBIGUOUS_PSC_ENDPOINT"
                }
                Cause::SourcePscCloudSqlUnsupported => "SOURCE_PSC_CLOUD_SQL_UNSUPPORTED",
                Cause::SourceForwardingRuleUnsupported => {
                    "SOURCE_FORWARDING_RULE_UNSUPPORTED"
                }
                Cause::NonRoutableIpAddress => "NON_ROUTABLE_IP_ADDRESS",
                Cause::UnknownIssueInGoogleManagedProject => {
                    "UNKNOWN_ISSUE_IN_GOOGLE_MANAGED_PROJECT"
                }
                Cause::UnsupportedGoogleManagedProjectConfig => {
                    "UNSUPPORTED_GOOGLE_MANAGED_PROJECT_CONFIG"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CAUSE_UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN_NETWORK" => Some(Self::UnknownNetwork),
                "UNKNOWN_PROJECT" => Some(Self::UnknownProject),
                "NO_EXTERNAL_IP" => Some(Self::NoExternalIp),
                "UNINTENDED_DESTINATION" => Some(Self::UnintendedDestination),
                "SOURCE_ENDPOINT_NOT_FOUND" => Some(Self::SourceEndpointNotFound),
                "MISMATCHED_SOURCE_NETWORK" => Some(Self::MismatchedSourceNetwork),
                "DESTINATION_ENDPOINT_NOT_FOUND" => {
                    Some(Self::DestinationEndpointNotFound)
                }
                "MISMATCHED_DESTINATION_NETWORK" => {
                    Some(Self::MismatchedDestinationNetwork)
                }
                "UNKNOWN_IP" => Some(Self::UnknownIp),
                "SOURCE_IP_ADDRESS_NOT_IN_SOURCE_NETWORK" => {
                    Some(Self::SourceIpAddressNotInSourceNetwork)
                }
                "PERMISSION_DENIED" => Some(Self::PermissionDenied),
                "PERMISSION_DENIED_NO_CLOUD_NAT_CONFIGS" => {
                    Some(Self::PermissionDeniedNoCloudNatConfigs)
                }
                "PERMISSION_DENIED_NO_NEG_ENDPOINT_CONFIGS" => {
                    Some(Self::PermissionDeniedNoNegEndpointConfigs)
                }
                "NO_SOURCE_LOCATION" => Some(Self::NoSourceLocation),
                "INVALID_ARGUMENT" => Some(Self::InvalidArgument),
                "TRACE_TOO_LONG" => Some(Self::TraceTooLong),
                "INTERNAL_ERROR" => Some(Self::InternalError),
                "UNSUPPORTED" => Some(Self::Unsupported),
                "MISMATCHED_IP_VERSION" => Some(Self::MismatchedIpVersion),
                "GKE_KONNECTIVITY_PROXY_UNSUPPORTED" => {
                    Some(Self::GkeKonnectivityProxyUnsupported)
                }
                "RESOURCE_CONFIG_NOT_FOUND" => Some(Self::ResourceConfigNotFound),
                "VM_INSTANCE_CONFIG_NOT_FOUND" => Some(Self::VmInstanceConfigNotFound),
                "NETWORK_CONFIG_NOT_FOUND" => Some(Self::NetworkConfigNotFound),
                "FIREWALL_CONFIG_NOT_FOUND" => Some(Self::FirewallConfigNotFound),
                "ROUTE_CONFIG_NOT_FOUND" => Some(Self::RouteConfigNotFound),
                "GOOGLE_MANAGED_SERVICE_AMBIGUOUS_PSC_ENDPOINT" => {
                    Some(Self::GoogleManagedServiceAmbiguousPscEndpoint)
                }
                "SOURCE_PSC_CLOUD_SQL_UNSUPPORTED" => {
                    Some(Self::SourcePscCloudSqlUnsupported)
                }
                "SOURCE_FORWARDING_RULE_UNSUPPORTED" => {
                    Some(Self::SourceForwardingRuleUnsupported)
                }
                "NON_ROUTABLE_IP_ADDRESS" => Some(Self::NonRoutableIpAddress),
                "UNKNOWN_ISSUE_IN_GOOGLE_MANAGED_PROJECT" => {
                    Some(Self::UnknownIssueInGoogleManagedProject)
                }
                "UNSUPPORTED_GOOGLE_MANAGED_PROJECT_CONFIG" => {
                    Some(Self::UnsupportedGoogleManagedProjectConfig)
                }
                _ => None,
            }
        }
    }
}
/// Details of the final state "drop" and associated resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DropInfo {
    /// Cause that the packet is dropped.
    #[prost(enumeration = "drop_info::Cause", tag = "1")]
    pub cause: i32,
    /// URI of the resource that caused the drop.
    #[prost(string, tag = "2")]
    pub resource_uri: ::prost::alloc::string::String,
    /// Source IP address of the dropped packet (if relevant).
    #[prost(string, tag = "3")]
    pub source_ip: ::prost::alloc::string::String,
    /// Destination IP address of the dropped packet (if relevant).
    #[prost(string, tag = "4")]
    pub destination_ip: ::prost::alloc::string::String,
    /// Region of the dropped packet (if relevant).
    #[prost(string, tag = "5")]
    pub region: ::prost::alloc::string::String,
}
/// Nested message and enum types in `DropInfo`.
pub mod drop_info {
    /// Drop cause types:
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
    pub enum Cause {
        /// Cause is unspecified.
        Unspecified = 0,
        /// Destination external address cannot be resolved to a known target. If
        /// the address is used in a Google Cloud project, provide the project ID
        /// as test input.
        UnknownExternalAddress = 1,
        /// A Compute Engine instance can only send or receive a packet with a
        /// foreign IP address if ip_forward is enabled.
        ForeignIpDisallowed = 2,
        /// Dropped due to a firewall rule, unless allowed due to connection
        /// tracking.
        FirewallRule = 3,
        /// Dropped due to no matching routes.
        NoRoute = 4,
        /// Dropped due to invalid route. Route's next hop is a blackhole.
        RouteBlackhole = 5,
        /// Packet is sent to a wrong (unintended) network. Example: you trace a
        /// packet from VM1:Network1 to VM2:Network2, however, the route configured
        /// in Network1 sends the packet destined for VM2's IP address to Network3.
        RouteWrongNetwork = 6,
        /// Route's next hop IP address cannot be resolved to a GCP resource.
        RouteNextHopIpAddressNotResolved = 42,
        /// Route's next hop resource is not found.
        RouteNextHopResourceNotFound = 43,
        /// Route's next hop instance doesn't have a NIC in the route's network.
        RouteNextHopInstanceWrongNetwork = 49,
        /// Route's next hop IP address is not a primary IP address of the next hop
        /// instance.
        RouteNextHopInstanceNonPrimaryIp = 50,
        /// Route's next hop forwarding rule doesn't match next hop IP address.
        RouteNextHopForwardingRuleIpMismatch = 51,
        /// Route's next hop VPN tunnel is down (does not have valid IKE SAs).
        RouteNextHopVpnTunnelNotEstablished = 52,
        /// Route's next hop forwarding rule type is invalid (it's not a forwarding
        /// rule of the internal passthrough load balancer).
        RouteNextHopForwardingRuleTypeInvalid = 53,
        /// Packet is sent from the Internet to the private IPv6 address.
        NoRouteFromInternetToPrivateIpv6Address = 44,
        /// The packet does not match a policy-based VPN tunnel local selector.
        VpnTunnelLocalSelectorMismatch = 45,
        /// The packet does not match a policy-based VPN tunnel remote selector.
        VpnTunnelRemoteSelectorMismatch = 46,
        /// Packet with internal destination address sent to the internet gateway.
        PrivateTrafficToInternet = 7,
        /// Instance with only an internal IP address tries to access Google API and
        /// services, but private Google access is not enabled in the subnet.
        PrivateGoogleAccessDisallowed = 8,
        /// Source endpoint tries to access Google API and services through the VPN
        /// tunnel to another network, but Private Google Access needs to be enabled
        /// in the source endpoint network.
        PrivateGoogleAccessViaVpnTunnelUnsupported = 47,
        /// Instance with only an internal IP address tries to access external hosts,
        /// but Cloud NAT is not enabled in the subnet, unless special configurations
        /// on a VM allow this connection.
        NoExternalAddress = 9,
        /// Destination internal address cannot be resolved to a known target. If
        /// this is a shared VPC scenario, verify if the service project ID is
        /// provided as test input. Otherwise, verify if the IP address is being
        /// used in the project.
        UnknownInternalAddress = 10,
        /// Forwarding rule's protocol and ports do not match the packet header.
        ForwardingRuleMismatch = 11,
        /// Forwarding rule does not have backends configured.
        ForwardingRuleNoInstances = 12,
        /// Firewalls block the health check probes to the backends and cause
        /// the backends to be unavailable for traffic from the load balancer.
        /// For more details, see [Health check firewall
        /// rules](<https://cloud.google.com/load-balancing/docs/health-checks#firewall_rules>).
        FirewallBlockingLoadBalancerBackendHealthCheck = 13,
        /// Packet is sent from or to a Compute Engine instance that is not in a
        /// running state.
        InstanceNotRunning = 14,
        /// Packet sent from or to a GKE cluster that is not in running state.
        GkeClusterNotRunning = 27,
        /// Packet sent from or to a Cloud SQL instance that is not in running state.
        CloudSqlInstanceNotRunning = 28,
        /// The type of traffic is blocked and the user cannot configure a firewall
        /// rule to enable it. See [Always blocked
        /// traffic](<https://cloud.google.com/vpc/docs/firewalls#blockedtraffic>) for
        /// more details.
        TrafficTypeBlocked = 15,
        /// Access to Google Kubernetes Engine cluster master's endpoint is not
        /// authorized. See [Access to the cluster
        /// endpoints](<https://cloud.google.com/kubernetes-engine/docs/how-to/private-clusters#access_to_the_cluster_endpoints>)
        /// for more details.
        GkeMasterUnauthorizedAccess = 16,
        /// Access to the Cloud SQL instance endpoint is not authorized.
        /// See [Authorizing with authorized
        /// networks](<https://cloud.google.com/sql/docs/mysql/authorize-networks>) for
        /// more details.
        CloudSqlInstanceUnauthorizedAccess = 17,
        /// Packet was dropped inside Google Kubernetes Engine Service.
        DroppedInsideGkeService = 18,
        /// Packet was dropped inside Cloud SQL Service.
        DroppedInsideCloudSqlService = 19,
        /// Packet was dropped because there is no peering between the originating
        /// network and the Google Managed Services Network.
        GoogleManagedServiceNoPeering = 20,
        /// Packet was dropped because the Google-managed service uses Private
        /// Service Connect (PSC), but the PSC endpoint is not found in the project.
        GoogleManagedServiceNoPscEndpoint = 38,
        /// Packet was dropped because the GKE cluster uses Private Service Connect
        /// (PSC), but the PSC endpoint is not found in the project.
        GkePscEndpointMissing = 36,
        /// Packet was dropped because the Cloud SQL instance has neither a private
        /// nor a public IP address.
        CloudSqlInstanceNoIpAddress = 21,
        /// Packet was dropped because a GKE cluster private endpoint is
        /// unreachable from a region different from the cluster's region.
        GkeControlPlaneRegionMismatch = 30,
        /// Packet sent from a public GKE cluster control plane to a private
        /// IP address.
        PublicGkeControlPlaneToPrivateDestination = 31,
        /// Packet was dropped because there is no route from a GKE cluster
        /// control plane to a destination network.
        GkeControlPlaneNoRoute = 32,
        /// Packet sent from a Cloud SQL instance to an external IP address is not
        /// allowed. The Cloud SQL instance is not configured to send packets to
        /// external IP addresses.
        CloudSqlInstanceNotConfiguredForExternalTraffic = 33,
        /// Packet sent from a Cloud SQL instance with only a public IP address to a
        /// private IP address.
        PublicCloudSqlInstanceToPrivateDestination = 34,
        /// Packet was dropped because there is no route from a Cloud SQL
        /// instance to a destination network.
        CloudSqlInstanceNoRoute = 35,
        /// Packet could be dropped because the Cloud Function is not in an active
        /// status.
        CloudFunctionNotActive = 22,
        /// Packet could be dropped because no VPC connector is set.
        VpcConnectorNotSet = 23,
        /// Packet could be dropped because the VPC connector is not in a running
        /// state.
        VpcConnectorNotRunning = 24,
        /// Packet could be dropped because it was sent from a different region
        /// to a regional forwarding without global access.
        ForwardingRuleRegionMismatch = 25,
        /// The Private Service Connect endpoint is in a project that is not approved
        /// to connect to the service.
        PscConnectionNotAccepted = 26,
        /// The packet is sent to the Private Service Connect endpoint over the
        /// peering, but [it's not
        /// supported](<https://cloud.google.com/vpc/docs/configure-private-service-connect-services#on-premises>).
        PscEndpointAccessedFromPeeredNetwork = 41,
        /// The packet is sent to the Private Service Connect backend (network
        /// endpoint group), but the producer PSC forwarding rule does not have
        /// global access enabled.
        PscNegProducerEndpointNoGlobalAccess = 48,
        /// The packet is sent to the Private Service Connect backend (network
        /// endpoint group), but the producer PSC forwarding rule has multiple ports
        /// specified.
        PscNegProducerForwardingRuleMultiplePorts = 54,
        /// The packet is sent to the Private Service Connect backend (network
        /// endpoint group) targeting a Cloud SQL service attachment, but this
        /// configuration is not supported.
        CloudSqlPscNegUnsupported = 58,
        /// No NAT subnets are defined for the PSC service attachment.
        NoNatSubnetsForPscServiceAttachment = 57,
        /// The packet sent from the hybrid NEG proxy matches a non-dynamic route,
        /// but such a configuration is not supported.
        HybridNegNonDynamicRouteMatched = 55,
        /// The packet sent from the hybrid NEG proxy matches a dynamic route with a
        /// next hop in a different region, but such a configuration is not
        /// supported.
        HybridNegNonLocalDynamicRouteMatched = 56,
        /// Packet sent from a Cloud Run revision that is not ready.
        CloudRunRevisionNotReady = 29,
        /// Packet was dropped inside Private Service Connect service producer.
        DroppedInsidePscServiceProducer = 37,
        /// Packet sent to a load balancer, which requires a proxy-only subnet and
        /// the subnet is not found.
        LoadBalancerHasNoProxySubnet = 39,
        /// Packet sent to Cloud Nat without active NAT IPs.
        CloudNatNoAddresses = 40,
        /// Packet is stuck in a routing loop.
        RoutingLoop = 59,
    }
    impl Cause {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Cause::Unspecified => "CAUSE_UNSPECIFIED",
                Cause::UnknownExternalAddress => "UNKNOWN_EXTERNAL_ADDRESS",
                Cause::ForeignIpDisallowed => "FOREIGN_IP_DISALLOWED",
                Cause::FirewallRule => "FIREWALL_RULE",
                Cause::NoRoute => "NO_ROUTE",
                Cause::RouteBlackhole => "ROUTE_BLACKHOLE",
                Cause::RouteWrongNetwork => "ROUTE_WRONG_NETWORK",
                Cause::RouteNextHopIpAddressNotResolved => {
                    "ROUTE_NEXT_HOP_IP_ADDRESS_NOT_RESOLVED"
                }
                Cause::RouteNextHopResourceNotFound => {
                    "ROUTE_NEXT_HOP_RESOURCE_NOT_FOUND"
                }
                Cause::RouteNextHopInstanceWrongNetwork => {
                    "ROUTE_NEXT_HOP_INSTANCE_WRONG_NETWORK"
                }
                Cause::RouteNextHopInstanceNonPrimaryIp => {
                    "ROUTE_NEXT_HOP_INSTANCE_NON_PRIMARY_IP"
                }
                Cause::RouteNextHopForwardingRuleIpMismatch => {
                    "ROUTE_NEXT_HOP_FORWARDING_RULE_IP_MISMATCH"
                }
                Cause::RouteNextHopVpnTunnelNotEstablished => {
                    "ROUTE_NEXT_HOP_VPN_TUNNEL_NOT_ESTABLISHED"
                }
                Cause::RouteNextHopForwardingRuleTypeInvalid => {
                    "ROUTE_NEXT_HOP_FORWARDING_RULE_TYPE_INVALID"
                }
                Cause::NoRouteFromInternetToPrivateIpv6Address => {
                    "NO_ROUTE_FROM_INTERNET_TO_PRIVATE_IPV6_ADDRESS"
                }
                Cause::VpnTunnelLocalSelectorMismatch => {
                    "VPN_TUNNEL_LOCAL_SELECTOR_MISMATCH"
                }
                Cause::VpnTunnelRemoteSelectorMismatch => {
                    "VPN_TUNNEL_REMOTE_SELECTOR_MISMATCH"
                }
                Cause::PrivateTrafficToInternet => "PRIVATE_TRAFFIC_TO_INTERNET",
                Cause::PrivateGoogleAccessDisallowed => {
                    "PRIVATE_GOOGLE_ACCESS_DISALLOWED"
                }
                Cause::PrivateGoogleAccessViaVpnTunnelUnsupported => {
                    "PRIVATE_GOOGLE_ACCESS_VIA_VPN_TUNNEL_UNSUPPORTED"
                }
                Cause::NoExternalAddress => "NO_EXTERNAL_ADDRESS",
                Cause::UnknownInternalAddress => "UNKNOWN_INTERNAL_ADDRESS",
                Cause::ForwardingRuleMismatch => "FORWARDING_RULE_MISMATCH",
                Cause::ForwardingRuleNoInstances => "FORWARDING_RULE_NO_INSTANCES",
                Cause::FirewallBlockingLoadBalancerBackendHealthCheck => {
                    "FIREWALL_BLOCKING_LOAD_BALANCER_BACKEND_HEALTH_CHECK"
                }
                Cause::InstanceNotRunning => "INSTANCE_NOT_RUNNING",
                Cause::GkeClusterNotRunning => "GKE_CLUSTER_NOT_RUNNING",
                Cause::CloudSqlInstanceNotRunning => "CLOUD_SQL_INSTANCE_NOT_RUNNING",
                Cause::TrafficTypeBlocked => "TRAFFIC_TYPE_BLOCKED",
                Cause::GkeMasterUnauthorizedAccess => "GKE_MASTER_UNAUTHORIZED_ACCESS",
                Cause::CloudSqlInstanceUnauthorizedAccess => {
                    "CLOUD_SQL_INSTANCE_UNAUTHORIZED_ACCESS"
                }
                Cause::DroppedInsideGkeService => "DROPPED_INSIDE_GKE_SERVICE",
                Cause::DroppedInsideCloudSqlService => "DROPPED_INSIDE_CLOUD_SQL_SERVICE",
                Cause::GoogleManagedServiceNoPeering => {
                    "GOOGLE_MANAGED_SERVICE_NO_PEERING"
                }
                Cause::GoogleManagedServiceNoPscEndpoint => {
                    "GOOGLE_MANAGED_SERVICE_NO_PSC_ENDPOINT"
                }
                Cause::GkePscEndpointMissing => "GKE_PSC_ENDPOINT_MISSING",
                Cause::CloudSqlInstanceNoIpAddress => "CLOUD_SQL_INSTANCE_NO_IP_ADDRESS",
                Cause::GkeControlPlaneRegionMismatch => {
                    "GKE_CONTROL_PLANE_REGION_MISMATCH"
                }
                Cause::PublicGkeControlPlaneToPrivateDestination => {
                    "PUBLIC_GKE_CONTROL_PLANE_TO_PRIVATE_DESTINATION"
                }
                Cause::GkeControlPlaneNoRoute => "GKE_CONTROL_PLANE_NO_ROUTE",
                Cause::CloudSqlInstanceNotConfiguredForExternalTraffic => {
                    "CLOUD_SQL_INSTANCE_NOT_CONFIGURED_FOR_EXTERNAL_TRAFFIC"
                }
                Cause::PublicCloudSqlInstanceToPrivateDestination => {
                    "PUBLIC_CLOUD_SQL_INSTANCE_TO_PRIVATE_DESTINATION"
                }
                Cause::CloudSqlInstanceNoRoute => "CLOUD_SQL_INSTANCE_NO_ROUTE",
                Cause::CloudFunctionNotActive => "CLOUD_FUNCTION_NOT_ACTIVE",
                Cause::VpcConnectorNotSet => "VPC_CONNECTOR_NOT_SET",
                Cause::VpcConnectorNotRunning => "VPC_CONNECTOR_NOT_RUNNING",
                Cause::ForwardingRuleRegionMismatch => "FORWARDING_RULE_REGION_MISMATCH",
                Cause::PscConnectionNotAccepted => "PSC_CONNECTION_NOT_ACCEPTED",
                Cause::PscEndpointAccessedFromPeeredNetwork => {
                    "PSC_ENDPOINT_ACCESSED_FROM_PEERED_NETWORK"
                }
                Cause::PscNegProducerEndpointNoGlobalAccess => {
                    "PSC_NEG_PRODUCER_ENDPOINT_NO_GLOBAL_ACCESS"
                }
                Cause::PscNegProducerForwardingRuleMultiplePorts => {
                    "PSC_NEG_PRODUCER_FORWARDING_RULE_MULTIPLE_PORTS"
                }
                Cause::CloudSqlPscNegUnsupported => "CLOUD_SQL_PSC_NEG_UNSUPPORTED",
                Cause::NoNatSubnetsForPscServiceAttachment => {
                    "NO_NAT_SUBNETS_FOR_PSC_SERVICE_ATTACHMENT"
                }
                Cause::HybridNegNonDynamicRouteMatched => {
                    "HYBRID_NEG_NON_DYNAMIC_ROUTE_MATCHED"
                }
                Cause::HybridNegNonLocalDynamicRouteMatched => {
                    "HYBRID_NEG_NON_LOCAL_DYNAMIC_ROUTE_MATCHED"
                }
                Cause::CloudRunRevisionNotReady => "CLOUD_RUN_REVISION_NOT_READY",
                Cause::DroppedInsidePscServiceProducer => {
                    "DROPPED_INSIDE_PSC_SERVICE_PRODUCER"
                }
                Cause::LoadBalancerHasNoProxySubnet => {
                    "LOAD_BALANCER_HAS_NO_PROXY_SUBNET"
                }
                Cause::CloudNatNoAddresses => "CLOUD_NAT_NO_ADDRESSES",
                Cause::RoutingLoop => "ROUTING_LOOP",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CAUSE_UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN_EXTERNAL_ADDRESS" => Some(Self::UnknownExternalAddress),
                "FOREIGN_IP_DISALLOWED" => Some(Self::ForeignIpDisallowed),
                "FIREWALL_RULE" => Some(Self::FirewallRule),
                "NO_ROUTE" => Some(Self::NoRoute),
                "ROUTE_BLACKHOLE" => Some(Self::RouteBlackhole),
                "ROUTE_WRONG_NETWORK" => Some(Self::RouteWrongNetwork),
                "ROUTE_NEXT_HOP_IP_ADDRESS_NOT_RESOLVED" => {
                    Some(Self::RouteNextHopIpAddressNotResolved)
                }
                "ROUTE_NEXT_HOP_RESOURCE_NOT_FOUND" => {
                    Some(Self::RouteNextHopResourceNotFound)
                }
                "ROUTE_NEXT_HOP_INSTANCE_WRONG_NETWORK" => {
                    Some(Self::RouteNextHopInstanceWrongNetwork)
                }
                "ROUTE_NEXT_HOP_INSTANCE_NON_PRIMARY_IP" => {
                    Some(Self::RouteNextHopInstanceNonPrimaryIp)
                }
                "ROUTE_NEXT_HOP_FORWARDING_RULE_IP_MISMATCH" => {
                    Some(Self::RouteNextHopForwardingRuleIpMismatch)
                }
                "ROUTE_NEXT_HOP_VPN_TUNNEL_NOT_ESTABLISHED" => {
                    Some(Self::RouteNextHopVpnTunnelNotEstablished)
                }
                "ROUTE_NEXT_HOP_FORWARDING_RULE_TYPE_INVALID" => {
                    Some(Self::RouteNextHopForwardingRuleTypeInvalid)
                }
                "NO_ROUTE_FROM_INTERNET_TO_PRIVATE_IPV6_ADDRESS" => {
                    Some(Self::NoRouteFromInternetToPrivateIpv6Address)
                }
                "VPN_TUNNEL_LOCAL_SELECTOR_MISMATCH" => {
                    Some(Self::VpnTunnelLocalSelectorMismatch)
                }
                "VPN_TUNNEL_REMOTE_SELECTOR_MISMATCH" => {
                    Some(Self::VpnTunnelRemoteSelectorMismatch)
                }
                "PRIVATE_TRAFFIC_TO_INTERNET" => Some(Self::PrivateTrafficToInternet),
                "PRIVATE_GOOGLE_ACCESS_DISALLOWED" => {
                    Some(Self::PrivateGoogleAccessDisallowed)
                }
                "PRIVATE_GOOGLE_ACCESS_VIA_VPN_TUNNEL_UNSUPPORTED" => {
                    Some(Self::PrivateGoogleAccessViaVpnTunnelUnsupported)
                }
                "NO_EXTERNAL_ADDRESS" => Some(Self::NoExternalAddress),
                "UNKNOWN_INTERNAL_ADDRESS" => Some(Self::UnknownInternalAddress),
                "FORWARDING_RULE_MISMATCH" => Some(Self::ForwardingRuleMismatch),
                "FORWARDING_RULE_NO_INSTANCES" => Some(Self::ForwardingRuleNoInstances),
                "FIREWALL_BLOCKING_LOAD_BALANCER_BACKEND_HEALTH_CHECK" => {
                    Some(Self::FirewallBlockingLoadBalancerBackendHealthCheck)
                }
                "INSTANCE_NOT_RUNNING" => Some(Self::InstanceNotRunning),
                "GKE_CLUSTER_NOT_RUNNING" => Some(Self::GkeClusterNotRunning),
                "CLOUD_SQL_INSTANCE_NOT_RUNNING" => {
                    Some(Self::CloudSqlInstanceNotRunning)
                }
                "TRAFFIC_TYPE_BLOCKED" => Some(Self::TrafficTypeBlocked),
                "GKE_MASTER_UNAUTHORIZED_ACCESS" => {
                    Some(Self::GkeMasterUnauthorizedAccess)
                }
                "CLOUD_SQL_INSTANCE_UNAUTHORIZED_ACCESS" => {
                    Some(Self::CloudSqlInstanceUnauthorizedAccess)
                }
                "DROPPED_INSIDE_GKE_SERVICE" => Some(Self::DroppedInsideGkeService),
                "DROPPED_INSIDE_CLOUD_SQL_SERVICE" => {
                    Some(Self::DroppedInsideCloudSqlService)
                }
                "GOOGLE_MANAGED_SERVICE_NO_PEERING" => {
                    Some(Self::GoogleManagedServiceNoPeering)
                }
                "GOOGLE_MANAGED_SERVICE_NO_PSC_ENDPOINT" => {
                    Some(Self::GoogleManagedServiceNoPscEndpoint)
                }
                "GKE_PSC_ENDPOINT_MISSING" => Some(Self::GkePscEndpointMissing),
                "CLOUD_SQL_INSTANCE_NO_IP_ADDRESS" => {
                    Some(Self::CloudSqlInstanceNoIpAddress)
                }
                "GKE_CONTROL_PLANE_REGION_MISMATCH" => {
                    Some(Self::GkeControlPlaneRegionMismatch)
                }
                "PUBLIC_GKE_CONTROL_PLANE_TO_PRIVATE_DESTINATION" => {
                    Some(Self::PublicGkeControlPlaneToPrivateDestination)
                }
                "GKE_CONTROL_PLANE_NO_ROUTE" => Some(Self::GkeControlPlaneNoRoute),
                "CLOUD_SQL_INSTANCE_NOT_CONFIGURED_FOR_EXTERNAL_TRAFFIC" => {
                    Some(Self::CloudSqlInstanceNotConfiguredForExternalTraffic)
                }
                "PUBLIC_CLOUD_SQL_INSTANCE_TO_PRIVATE_DESTINATION" => {
                    Some(Self::PublicCloudSqlInstanceToPrivateDestination)
                }
                "CLOUD_SQL_INSTANCE_NO_ROUTE" => Some(Self::CloudSqlInstanceNoRoute),
                "CLOUD_FUNCTION_NOT_ACTIVE" => Some(Self::CloudFunctionNotActive),
                "VPC_CONNECTOR_NOT_SET" => Some(Self::VpcConnectorNotSet),
                "VPC_CONNECTOR_NOT_RUNNING" => Some(Self::VpcConnectorNotRunning),
                "FORWARDING_RULE_REGION_MISMATCH" => {
                    Some(Self::ForwardingRuleRegionMismatch)
                }
                "PSC_CONNECTION_NOT_ACCEPTED" => Some(Self::PscConnectionNotAccepted),
                "PSC_ENDPOINT_ACCESSED_FROM_PEERED_NETWORK" => {
                    Some(Self::PscEndpointAccessedFromPeeredNetwork)
                }
                "PSC_NEG_PRODUCER_ENDPOINT_NO_GLOBAL_ACCESS" => {
                    Some(Self::PscNegProducerEndpointNoGlobalAccess)
                }
                "PSC_NEG_PRODUCER_FORWARDING_RULE_MULTIPLE_PORTS" => {
                    Some(Self::PscNegProducerForwardingRuleMultiplePorts)
                }
                "CLOUD_SQL_PSC_NEG_UNSUPPORTED" => Some(Self::CloudSqlPscNegUnsupported),
                "NO_NAT_SUBNETS_FOR_PSC_SERVICE_ATTACHMENT" => {
                    Some(Self::NoNatSubnetsForPscServiceAttachment)
                }
                "HYBRID_NEG_NON_DYNAMIC_ROUTE_MATCHED" => {
                    Some(Self::HybridNegNonDynamicRouteMatched)
                }
                "HYBRID_NEG_NON_LOCAL_DYNAMIC_ROUTE_MATCHED" => {
                    Some(Self::HybridNegNonLocalDynamicRouteMatched)
                }
                "CLOUD_RUN_REVISION_NOT_READY" => Some(Self::CloudRunRevisionNotReady),
                "DROPPED_INSIDE_PSC_SERVICE_PRODUCER" => {
                    Some(Self::DroppedInsidePscServiceProducer)
                }
                "LOAD_BALANCER_HAS_NO_PROXY_SUBNET" => {
                    Some(Self::LoadBalancerHasNoProxySubnet)
                }
                "CLOUD_NAT_NO_ADDRESSES" => Some(Self::CloudNatNoAddresses),
                "ROUTING_LOOP" => Some(Self::RoutingLoop),
                _ => None,
            }
        }
    }
}
/// For display only. Metadata associated with a Google Kubernetes Engine (GKE)
/// cluster master.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GkeMasterInfo {
    /// URI of a GKE cluster.
    #[prost(string, tag = "2")]
    pub cluster_uri: ::prost::alloc::string::String,
    /// URI of a GKE cluster network.
    #[prost(string, tag = "4")]
    pub cluster_network_uri: ::prost::alloc::string::String,
    /// Internal IP address of a GKE cluster master.
    #[prost(string, tag = "5")]
    pub internal_ip: ::prost::alloc::string::String,
    /// External IP address of a GKE cluster master.
    #[prost(string, tag = "6")]
    pub external_ip: ::prost::alloc::string::String,
}
/// For display only. Metadata associated with a Cloud SQL instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudSqlInstanceInfo {
    /// Name of a Cloud SQL instance.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// URI of a Cloud SQL instance.
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// URI of a Cloud SQL instance network or empty string if the instance does
    /// not have one.
    #[prost(string, tag = "4")]
    pub network_uri: ::prost::alloc::string::String,
    /// Internal IP address of a Cloud SQL instance.
    #[prost(string, tag = "5")]
    pub internal_ip: ::prost::alloc::string::String,
    /// External IP address of a Cloud SQL instance.
    #[prost(string, tag = "6")]
    pub external_ip: ::prost::alloc::string::String,
    /// Region in which the Cloud SQL instance is running.
    #[prost(string, tag = "7")]
    pub region: ::prost::alloc::string::String,
}
/// For display only. Metadata associated with a Cloud Function.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudFunctionInfo {
    /// Name of a Cloud Function.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// URI of a Cloud Function.
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// Location in which the Cloud Function is deployed.
    #[prost(string, tag = "3")]
    pub location: ::prost::alloc::string::String,
    /// Latest successfully deployed version id of the Cloud Function.
    #[prost(int64, tag = "4")]
    pub version_id: i64,
}
/// For display only. Metadata associated with a Cloud Run revision.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudRunRevisionInfo {
    /// Name of a Cloud Run revision.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// URI of a Cloud Run revision.
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// Location in which this revision is deployed.
    #[prost(string, tag = "4")]
    pub location: ::prost::alloc::string::String,
    /// URI of Cloud Run service this revision belongs to.
    #[prost(string, tag = "5")]
    pub service_uri: ::prost::alloc::string::String,
}
/// For display only. Metadata associated with an App Engine version.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppEngineVersionInfo {
    /// Name of an App Engine version.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// URI of an App Engine version.
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// Runtime of the App Engine version.
    #[prost(string, tag = "3")]
    pub runtime: ::prost::alloc::string::String,
    /// App Engine execution environment for a version.
    #[prost(string, tag = "4")]
    pub environment: ::prost::alloc::string::String,
}
/// For display only. Metadata associated with a VPC connector.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VpcConnectorInfo {
    /// Name of a VPC connector.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// URI of a VPC connector.
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// Location in which the VPC connector is deployed.
    #[prost(string, tag = "3")]
    pub location: ::prost::alloc::string::String,
}
/// For display only. Metadata associated with NAT.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatInfo {
    /// Type of NAT.
    #[prost(enumeration = "nat_info::Type", tag = "1")]
    pub r#type: i32,
    /// IP protocol in string format, for example: "TCP", "UDP", "ICMP".
    #[prost(string, tag = "2")]
    pub protocol: ::prost::alloc::string::String,
    /// URI of the network where NAT translation takes place.
    #[prost(string, tag = "3")]
    pub network_uri: ::prost::alloc::string::String,
    /// Source IP address before NAT translation.
    #[prost(string, tag = "4")]
    pub old_source_ip: ::prost::alloc::string::String,
    /// Source IP address after NAT translation.
    #[prost(string, tag = "5")]
    pub new_source_ip: ::prost::alloc::string::String,
    /// Destination IP address before NAT translation.
    #[prost(string, tag = "6")]
    pub old_destination_ip: ::prost::alloc::string::String,
    /// Destination IP address after NAT translation.
    #[prost(string, tag = "7")]
    pub new_destination_ip: ::prost::alloc::string::String,
    /// Source port before NAT translation. Only valid when protocol is TCP or UDP.
    #[prost(int32, tag = "8")]
    pub old_source_port: i32,
    /// Source port after NAT translation. Only valid when protocol is TCP or UDP.
    #[prost(int32, tag = "9")]
    pub new_source_port: i32,
    /// Destination port before NAT translation. Only valid when protocol is TCP or
    /// UDP.
    #[prost(int32, tag = "10")]
    pub old_destination_port: i32,
    /// Destination port after NAT translation. Only valid when protocol is TCP or
    /// UDP.
    #[prost(int32, tag = "11")]
    pub new_destination_port: i32,
    /// Uri of the Cloud Router. Only valid when type is CLOUD_NAT.
    #[prost(string, tag = "12")]
    pub router_uri: ::prost::alloc::string::String,
    /// The name of Cloud NAT Gateway. Only valid when type is CLOUD_NAT.
    #[prost(string, tag = "13")]
    pub nat_gateway_name: ::prost::alloc::string::String,
}
/// Nested message and enum types in `NatInfo`.
pub mod nat_info {
    /// Types of NAT.
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
        /// Type is unspecified.
        Unspecified = 0,
        /// From Compute Engine instance's internal address to external address.
        InternalToExternal = 1,
        /// From Compute Engine instance's external address to internal address.
        ExternalToInternal = 2,
        /// Cloud NAT Gateway.
        CloudNat = 3,
        /// Private service connect NAT.
        PrivateServiceConnect = 4,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::InternalToExternal => "INTERNAL_TO_EXTERNAL",
                Type::ExternalToInternal => "EXTERNAL_TO_INTERNAL",
                Type::CloudNat => "CLOUD_NAT",
                Type::PrivateServiceConnect => "PRIVATE_SERVICE_CONNECT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "INTERNAL_TO_EXTERNAL" => Some(Self::InternalToExternal),
                "EXTERNAL_TO_INTERNAL" => Some(Self::ExternalToInternal),
                "CLOUD_NAT" => Some(Self::CloudNat),
                "PRIVATE_SERVICE_CONNECT" => Some(Self::PrivateServiceConnect),
                _ => None,
            }
        }
    }
}
/// For display only. Metadata associated with ProxyConnection.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProxyConnectionInfo {
    /// IP protocol in string format, for example: "TCP", "UDP", "ICMP".
    #[prost(string, tag = "1")]
    pub protocol: ::prost::alloc::string::String,
    /// Source IP address of an original connection.
    #[prost(string, tag = "2")]
    pub old_source_ip: ::prost::alloc::string::String,
    /// Source IP address of a new connection.
    #[prost(string, tag = "3")]
    pub new_source_ip: ::prost::alloc::string::String,
    /// Destination IP address of an original connection
    #[prost(string, tag = "4")]
    pub old_destination_ip: ::prost::alloc::string::String,
    /// Destination IP address of a new connection.
    #[prost(string, tag = "5")]
    pub new_destination_ip: ::prost::alloc::string::String,
    /// Source port of an original connection. Only valid when protocol is TCP or
    /// UDP.
    #[prost(int32, tag = "6")]
    pub old_source_port: i32,
    /// Source port of a new connection. Only valid when protocol is TCP or UDP.
    #[prost(int32, tag = "7")]
    pub new_source_port: i32,
    /// Destination port of an original connection. Only valid when protocol is TCP
    /// or UDP.
    #[prost(int32, tag = "8")]
    pub old_destination_port: i32,
    /// Destination port of a new connection. Only valid when protocol is TCP or
    /// UDP.
    #[prost(int32, tag = "9")]
    pub new_destination_port: i32,
    /// Uri of proxy subnet.
    #[prost(string, tag = "10")]
    pub subnet_uri: ::prost::alloc::string::String,
    /// URI of the network where connection is proxied.
    #[prost(string, tag = "11")]
    pub network_uri: ::prost::alloc::string::String,
}
/// For display only. Metadata associated with the load balancer backend.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadBalancerBackendInfo {
    /// Display name of the backend. For example, it might be an instance name for
    /// the instance group backends, or an IP address and port for zonal network
    /// endpoint group backends.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// URI of the backend instance (if applicable). Populated for instance group
    /// backends, and zonal NEG backends.
    #[prost(string, tag = "2")]
    pub instance_uri: ::prost::alloc::string::String,
    /// URI of the backend service this backend belongs to (if applicable).
    #[prost(string, tag = "3")]
    pub backend_service_uri: ::prost::alloc::string::String,
    /// URI of the instance group this backend belongs to (if applicable).
    #[prost(string, tag = "4")]
    pub instance_group_uri: ::prost::alloc::string::String,
    /// URI of the network endpoint group this backend belongs to (if applicable).
    #[prost(string, tag = "5")]
    pub network_endpoint_group_uri: ::prost::alloc::string::String,
    /// URI of the backend bucket this backend targets (if applicable).
    #[prost(string, tag = "8")]
    pub backend_bucket_uri: ::prost::alloc::string::String,
    /// URI of the PSC service attachment this PSC NEG backend targets (if
    /// applicable).
    #[prost(string, tag = "9")]
    pub psc_service_attachment_uri: ::prost::alloc::string::String,
    /// PSC Google API target this PSC NEG backend targets (if applicable).
    #[prost(string, tag = "10")]
    pub psc_google_api_target: ::prost::alloc::string::String,
    /// URI of the health check attached to this backend (if applicable).
    #[prost(string, tag = "6")]
    pub health_check_uri: ::prost::alloc::string::String,
    /// Output only. Health check firewalls configuration state for the backend.
    /// This is a result of the static firewall analysis (verifying that health
    /// check traffic from required IP ranges to the backend is allowed or not).
    /// The backend might still be unhealthy even if these firewalls are
    /// configured. Please refer to the documentation for more information:
    /// <https://cloud.google.com/load-balancing/docs/firewall-rules>
    #[prost(
        enumeration = "load_balancer_backend_info::HealthCheckFirewallsConfigState",
        tag = "7"
    )]
    pub health_check_firewalls_config_state: i32,
}
/// Nested message and enum types in `LoadBalancerBackendInfo`.
pub mod load_balancer_backend_info {
    /// Health check firewalls configuration state enum.
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
    pub enum HealthCheckFirewallsConfigState {
        /// Configuration state unspecified. It usually means that the backend has
        /// no health check attached, or there was an unexpected configuration error
        /// preventing Connectivity tests from verifying health check configuration.
        Unspecified = 0,
        /// Firewall rules (policies) allowing health check traffic from all required
        /// IP ranges to the backend are configured.
        FirewallsConfigured = 1,
        /// Firewall rules (policies) allow health check traffic only from a part of
        /// required IP ranges.
        FirewallsPartiallyConfigured = 2,
        /// Firewall rules (policies) deny health check traffic from all required
        /// IP ranges to the backend.
        FirewallsNotConfigured = 3,
        /// The network contains firewall rules of unsupported types, so Connectivity
        /// tests were not able to verify health check configuration status. Please
        /// refer to the documentation for the list of unsupported configurations:
        /// <https://cloud.google.com/network-intelligence-center/docs/connectivity-tests/concepts/overview#unsupported-configs>
        FirewallsUnsupported = 4,
    }
    impl HealthCheckFirewallsConfigState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                HealthCheckFirewallsConfigState::Unspecified => {
                    "HEALTH_CHECK_FIREWALLS_CONFIG_STATE_UNSPECIFIED"
                }
                HealthCheckFirewallsConfigState::FirewallsConfigured => {
                    "FIREWALLS_CONFIGURED"
                }
                HealthCheckFirewallsConfigState::FirewallsPartiallyConfigured => {
                    "FIREWALLS_PARTIALLY_CONFIGURED"
                }
                HealthCheckFirewallsConfigState::FirewallsNotConfigured => {
                    "FIREWALLS_NOT_CONFIGURED"
                }
                HealthCheckFirewallsConfigState::FirewallsUnsupported => {
                    "FIREWALLS_UNSUPPORTED"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "HEALTH_CHECK_FIREWALLS_CONFIG_STATE_UNSPECIFIED" => {
                    Some(Self::Unspecified)
                }
                "FIREWALLS_CONFIGURED" => Some(Self::FirewallsConfigured),
                "FIREWALLS_PARTIALLY_CONFIGURED" => {
                    Some(Self::FirewallsPartiallyConfigured)
                }
                "FIREWALLS_NOT_CONFIGURED" => Some(Self::FirewallsNotConfigured),
                "FIREWALLS_UNSUPPORTED" => Some(Self::FirewallsUnsupported),
                _ => None,
            }
        }
    }
}
/// For display only. Metadata associated with Storage Bucket.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageBucketInfo {
    /// Cloud Storage Bucket name.
    #[prost(string, tag = "1")]
    pub bucket: ::prost::alloc::string::String,
}
/// Type of a load balancer. For more information, see [Summary of Google Cloud
/// load
/// balancers](<https://cloud.google.com/load-balancing/docs/load-balancing-overview#summary-of-google-cloud-load-balancers>).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LoadBalancerType {
    /// Forwarding rule points to a different target than a load balancer or a
    /// load balancer type is unknown.
    Unspecified = 0,
    /// Global external HTTP(S) load balancer.
    HttpsAdvancedLoadBalancer = 1,
    /// Global external HTTP(S) load balancer (classic)
    HttpsLoadBalancer = 2,
    /// Regional external HTTP(S) load balancer.
    RegionalHttpsLoadBalancer = 3,
    /// Internal HTTP(S) load balancer.
    InternalHttpsLoadBalancer = 4,
    /// External SSL proxy load balancer.
    SslProxyLoadBalancer = 5,
    /// External TCP proxy load balancer.
    TcpProxyLoadBalancer = 6,
    /// Internal regional TCP proxy load balancer.
    InternalTcpProxyLoadBalancer = 7,
    /// External TCP/UDP Network load balancer.
    NetworkLoadBalancer = 8,
    /// Target-pool based external TCP/UDP Network load balancer.
    LegacyNetworkLoadBalancer = 9,
    /// Internal TCP/UDP load balancer.
    TcpUdpInternalLoadBalancer = 10,
}
impl LoadBalancerType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LoadBalancerType::Unspecified => "LOAD_BALANCER_TYPE_UNSPECIFIED",
            LoadBalancerType::HttpsAdvancedLoadBalancer => "HTTPS_ADVANCED_LOAD_BALANCER",
            LoadBalancerType::HttpsLoadBalancer => "HTTPS_LOAD_BALANCER",
            LoadBalancerType::RegionalHttpsLoadBalancer => "REGIONAL_HTTPS_LOAD_BALANCER",
            LoadBalancerType::InternalHttpsLoadBalancer => "INTERNAL_HTTPS_LOAD_BALANCER",
            LoadBalancerType::SslProxyLoadBalancer => "SSL_PROXY_LOAD_BALANCER",
            LoadBalancerType::TcpProxyLoadBalancer => "TCP_PROXY_LOAD_BALANCER",
            LoadBalancerType::InternalTcpProxyLoadBalancer => {
                "INTERNAL_TCP_PROXY_LOAD_BALANCER"
            }
            LoadBalancerType::NetworkLoadBalancer => "NETWORK_LOAD_BALANCER",
            LoadBalancerType::LegacyNetworkLoadBalancer => "LEGACY_NETWORK_LOAD_BALANCER",
            LoadBalancerType::TcpUdpInternalLoadBalancer => {
                "TCP_UDP_INTERNAL_LOAD_BALANCER"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LOAD_BALANCER_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "HTTPS_ADVANCED_LOAD_BALANCER" => Some(Self::HttpsAdvancedLoadBalancer),
            "HTTPS_LOAD_BALANCER" => Some(Self::HttpsLoadBalancer),
            "REGIONAL_HTTPS_LOAD_BALANCER" => Some(Self::RegionalHttpsLoadBalancer),
            "INTERNAL_HTTPS_LOAD_BALANCER" => Some(Self::InternalHttpsLoadBalancer),
            "SSL_PROXY_LOAD_BALANCER" => Some(Self::SslProxyLoadBalancer),
            "TCP_PROXY_LOAD_BALANCER" => Some(Self::TcpProxyLoadBalancer),
            "INTERNAL_TCP_PROXY_LOAD_BALANCER" => {
                Some(Self::InternalTcpProxyLoadBalancer)
            }
            "NETWORK_LOAD_BALANCER" => Some(Self::NetworkLoadBalancer),
            "LEGACY_NETWORK_LOAD_BALANCER" => Some(Self::LegacyNetworkLoadBalancer),
            "TCP_UDP_INTERNAL_LOAD_BALANCER" => Some(Self::TcpUdpInternalLoadBalancer),
            _ => None,
        }
    }
}
/// A Connectivity Test for a network reachability analysis.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectivityTest {
    /// Required. Unique name of the resource using the form:
    ///      `projects/{project_id}/locations/global/connectivityTests/{test}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The user-supplied description of the Connectivity Test.
    /// Maximum of 512 characters.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Required. Source specification of the Connectivity Test.
    ///
    /// You can use a combination of source IP address, virtual machine
    /// (VM) instance, or Compute Engine network to uniquely identify
    /// the source location.
    ///
    /// Examples:
    /// If the source IP address is an internal IP address within a Google Cloud
    /// Virtual Private Cloud (VPC) network, then you must also specify the VPC
    /// network. Otherwise, specify the VM instance, which already contains its
    /// internal IP address and VPC network information.
    ///
    /// If the source of the test is within an on-premises network, then you must
    /// provide the destination VPC network.
    ///
    /// If the source endpoint is a Compute Engine VM instance with multiple
    /// network interfaces, the instance itself is not sufficient to identify the
    /// endpoint. So, you must also specify the source IP address or VPC network.
    ///
    /// A reachability analysis proceeds even if the source location is
    /// ambiguous. However, the test result may include endpoints that you don't
    /// intend to test.
    #[prost(message, optional, tag = "3")]
    pub source: ::core::option::Option<Endpoint>,
    /// Required. Destination specification of the Connectivity Test.
    ///
    /// You can use a combination of destination IP address, Compute Engine
    /// VM instance, or VPC network to uniquely identify the destination
    /// location.
    ///
    /// Even if the destination IP address is not unique, the source IP
    /// location is unique. Usually, the analysis can infer the destination
    /// endpoint from route information.
    ///
    /// If the destination you specify is a VM instance and the instance has
    /// multiple network interfaces, then you must also specify either
    /// a destination IP address  or VPC network to identify the destination
    /// interface.
    ///
    /// A reachability analysis proceeds even if the destination location is
    /// ambiguous. However, the result can include endpoints that you don't
    /// intend to test.
    #[prost(message, optional, tag = "4")]
    pub destination: ::core::option::Option<Endpoint>,
    /// IP Protocol of the test. When not provided, "TCP" is assumed.
    #[prost(string, tag = "5")]
    pub protocol: ::prost::alloc::string::String,
    /// Other projects that may be relevant for reachability analysis.
    /// This is applicable to scenarios where a test can cross project boundaries.
    #[prost(string, repeated, tag = "6")]
    pub related_projects: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. The display name of a Connectivity Test.
    #[prost(string, tag = "7")]
    pub display_name: ::prost::alloc::string::String,
    /// Resource labels to represent user-provided metadata.
    #[prost(map = "string, string", tag = "8")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. The time the test was created.
    #[prost(message, optional, tag = "10")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the test's configuration was updated.
    #[prost(message, optional, tag = "11")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The reachability details of this test from the latest run.
    /// The details are updated when creating a new test, updating an
    /// existing test, or triggering a one-time rerun of an existing test.
    #[prost(message, optional, tag = "12")]
    pub reachability_details: ::core::option::Option<ReachabilityDetails>,
    /// Output only. The probing details of this test from the latest run, present
    /// for applicable tests only. The details are updated when creating a new
    /// test, updating an existing test, or triggering a one-time rerun of an
    /// existing test.
    #[prost(message, optional, tag = "14")]
    pub probing_details: ::core::option::Option<ProbingDetails>,
    /// Whether the test should skip firewall checking.
    /// If not provided, we assume false.
    #[prost(bool, tag = "17")]
    pub bypass_firewall_checks: bool,
}
/// Source or destination of the Connectivity Test.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Endpoint {
    /// The IP address of the endpoint, which can be an external or internal IP.
    #[prost(string, tag = "1")]
    pub ip_address: ::prost::alloc::string::String,
    /// The IP protocol port of the endpoint.
    /// Only applicable when protocol is TCP or UDP.
    #[prost(int32, tag = "2")]
    pub port: i32,
    /// A Compute Engine instance URI.
    #[prost(string, tag = "3")]
    pub instance: ::prost::alloc::string::String,
    /// A forwarding rule and its corresponding IP address represent the frontend
    /// configuration of a Google Cloud load balancer. Forwarding rules are also
    /// used for protocol forwarding, Private Service Connect and other network
    /// services to provide forwarding information in the control plane. Format:
    ///   projects/{project}/global/forwardingRules/{id} or
    ///   projects/{project}/regions/{region}/forwardingRules/{id}
    #[prost(string, tag = "13")]
    pub forwarding_rule: ::prost::alloc::string::String,
    /// Output only. Specifies the type of the target of the forwarding rule.
    #[prost(enumeration = "endpoint::ForwardingRuleTarget", optional, tag = "14")]
    pub forwarding_rule_target: ::core::option::Option<i32>,
    /// Output only. ID of the load balancer the forwarding rule points to. Empty
    /// for forwarding rules not related to load balancers.
    #[prost(string, optional, tag = "15")]
    pub load_balancer_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Type of the load balancer the forwarding rule points to.
    #[prost(enumeration = "LoadBalancerType", optional, tag = "16")]
    pub load_balancer_type: ::core::option::Option<i32>,
    /// A cluster URI for [Google Kubernetes Engine
    /// master](<https://cloud.google.com/kubernetes-engine/docs/concepts/cluster-architecture>).
    #[prost(string, tag = "7")]
    pub gke_master_cluster: ::prost::alloc::string::String,
    /// A [Cloud SQL](<https://cloud.google.com/sql>) instance URI.
    #[prost(string, tag = "8")]
    pub cloud_sql_instance: ::prost::alloc::string::String,
    /// A [Cloud Function](<https://cloud.google.com/functions>).
    #[prost(message, optional, tag = "10")]
    pub cloud_function: ::core::option::Option<endpoint::CloudFunctionEndpoint>,
    /// An [App Engine](<https://cloud.google.com/appengine>) [service
    /// version](<https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions>).
    #[prost(message, optional, tag = "11")]
    pub app_engine_version: ::core::option::Option<endpoint::AppEngineVersionEndpoint>,
    /// A [Cloud Run](<https://cloud.google.com/run>)
    /// [revision](<https://cloud.google.com/run/docs/reference/rest/v1/namespaces.revisions/get>)
    #[prost(message, optional, tag = "12")]
    pub cloud_run_revision: ::core::option::Option<endpoint::CloudRunRevisionEndpoint>,
    /// A Compute Engine network URI.
    #[prost(string, tag = "4")]
    pub network: ::prost::alloc::string::String,
    /// Type of the network where the endpoint is located.
    /// Applicable only to source endpoint, as destination network type can be
    /// inferred from the source.
    #[prost(enumeration = "endpoint::NetworkType", tag = "5")]
    pub network_type: i32,
    /// Project ID where the endpoint is located.
    /// The Project ID can be derived from the URI if you provide a VM instance or
    /// network URI.
    /// The following are two cases where you must provide the project ID:
    /// 1. Only the IP address is specified, and the IP address is within a Google
    /// Cloud project.
    /// 2. When you are using Shared VPC and the IP address that you provide is
    /// from the service project. In this case, the network that the IP address
    /// resides in is defined in the host project.
    #[prost(string, tag = "6")]
    pub project_id: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Endpoint`.
pub mod endpoint {
    /// Wrapper for Cloud Function attributes.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CloudFunctionEndpoint {
        /// A [Cloud Function](<https://cloud.google.com/functions>) name.
        #[prost(string, tag = "1")]
        pub uri: ::prost::alloc::string::String,
    }
    /// Wrapper for the App Engine service version attributes.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AppEngineVersionEndpoint {
        /// An [App Engine](<https://cloud.google.com/appengine>) [service
        /// version](<https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions>)
        /// name.
        #[prost(string, tag = "1")]
        pub uri: ::prost::alloc::string::String,
    }
    /// Wrapper for Cloud Run revision attributes.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CloudRunRevisionEndpoint {
        /// A [Cloud Run](<https://cloud.google.com/run>)
        /// [revision](<https://cloud.google.com/run/docs/reference/rest/v1/namespaces.revisions/get>)
        /// URI. The format is:
        /// projects/{project}/locations/{location}/revisions/{revision}
        #[prost(string, tag = "1")]
        pub uri: ::prost::alloc::string::String,
    }
    /// The type definition of an endpoint's network. Use one of the
    /// following choices:
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
    pub enum NetworkType {
        /// Default type if unspecified.
        Unspecified = 0,
        /// A network hosted within Google Cloud.
        /// To receive more detailed output, specify the URI for the source or
        /// destination network.
        GcpNetwork = 1,
        /// A network hosted outside of Google Cloud.
        /// This can be an on-premises network, or a network hosted by another cloud
        /// provider.
        NonGcpNetwork = 2,
    }
    impl NetworkType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                NetworkType::Unspecified => "NETWORK_TYPE_UNSPECIFIED",
                NetworkType::GcpNetwork => "GCP_NETWORK",
                NetworkType::NonGcpNetwork => "NON_GCP_NETWORK",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NETWORK_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "GCP_NETWORK" => Some(Self::GcpNetwork),
                "NON_GCP_NETWORK" => Some(Self::NonGcpNetwork),
                _ => None,
            }
        }
    }
    /// Type of the target of a forwarding rule.
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
    pub enum ForwardingRuleTarget {
        /// Forwarding rule target is unknown.
        Unspecified = 0,
        /// Compute Engine instance for protocol forwarding.
        Instance = 1,
        /// Load Balancer. The specific type can be found from \[load_balancer_type\]
        /// \[google.cloud.networkmanagement.v1beta1.Endpoint.load_balancer_type\].
        LoadBalancer = 2,
        /// Classic Cloud VPN Gateway.
        VpnGateway = 3,
        /// Forwarding Rule is a Private Service Connect endpoint.
        Psc = 4,
    }
    impl ForwardingRuleTarget {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ForwardingRuleTarget::Unspecified => "FORWARDING_RULE_TARGET_UNSPECIFIED",
                ForwardingRuleTarget::Instance => "INSTANCE",
                ForwardingRuleTarget::LoadBalancer => "LOAD_BALANCER",
                ForwardingRuleTarget::VpnGateway => "VPN_GATEWAY",
                ForwardingRuleTarget::Psc => "PSC",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FORWARDING_RULE_TARGET_UNSPECIFIED" => Some(Self::Unspecified),
                "INSTANCE" => Some(Self::Instance),
                "LOAD_BALANCER" => Some(Self::LoadBalancer),
                "VPN_GATEWAY" => Some(Self::VpnGateway),
                "PSC" => Some(Self::Psc),
                _ => None,
            }
        }
    }
}
/// Results of the configuration analysis from the last run of the test.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReachabilityDetails {
    /// The overall result of the test's configuration analysis.
    #[prost(enumeration = "reachability_details::Result", tag = "1")]
    pub result: i32,
    /// The time of the configuration analysis.
    #[prost(message, optional, tag = "2")]
    pub verify_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The details of a failure or a cancellation of reachability analysis.
    #[prost(message, optional, tag = "3")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
    /// Result may contain a list of traces if a test has multiple possible
    /// paths in the network, such as when destination endpoint is a load balancer
    /// with multiple backends.
    #[prost(message, repeated, tag = "5")]
    pub traces: ::prost::alloc::vec::Vec<Trace>,
}
/// Nested message and enum types in `ReachabilityDetails`.
pub mod reachability_details {
    /// The overall result of the test's configuration analysis.
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
    pub enum Result {
        /// No result was specified.
        Unspecified = 0,
        /// Possible scenarios are:
        ///
        /// * The configuration analysis determined that a packet originating from
        ///    the source is expected to reach the destination.
        /// * The analysis didn't complete because the user lacks permission for
        ///    some of the resources in the trace. However, at the time the user's
        ///    permission became insufficient, the trace had been successful so far.
        Reachable = 1,
        /// A packet originating from the source is expected to be dropped before
        /// reaching the destination.
        Unreachable = 2,
        /// The source and destination endpoints do not uniquely identify
        /// the test location in the network, and the reachability result contains
        /// multiple traces. For some traces, a packet could be delivered, and for
        /// others, it would not be. This result is also assigned to
        /// configuration analysis of return path if on its own it should be
        /// REACHABLE, but configuration analysis of forward path is AMBIGUOUS.
        Ambiguous = 4,
        /// The configuration analysis did not complete. Possible reasons are:
        ///
        /// * A permissions error occurred--for example, the user might not have
        ///    read permission for all of the resources named in the test.
        /// * An internal error occurred.
        /// * The analyzer received an invalid or unsupported argument or was unable
        ///    to identify a known endpoint.
        Undetermined = 5,
    }
    impl Result {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Result::Unspecified => "RESULT_UNSPECIFIED",
                Result::Reachable => "REACHABLE",
                Result::Unreachable => "UNREACHABLE",
                Result::Ambiguous => "AMBIGUOUS",
                Result::Undetermined => "UNDETERMINED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RESULT_UNSPECIFIED" => Some(Self::Unspecified),
                "REACHABLE" => Some(Self::Reachable),
                "UNREACHABLE" => Some(Self::Unreachable),
                "AMBIGUOUS" => Some(Self::Ambiguous),
                "UNDETERMINED" => Some(Self::Undetermined),
                _ => None,
            }
        }
    }
}
/// Latency percentile rank and value.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LatencyPercentile {
    /// Percentage of samples this data point applies to.
    #[prost(int32, tag = "1")]
    pub percent: i32,
    /// percent-th percentile of latency observed, in microseconds.
    /// Fraction of percent/100 of samples have latency lower or
    /// equal to the value of this field.
    #[prost(int64, tag = "2")]
    pub latency_micros: i64,
}
/// Describes measured latency distribution.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LatencyDistribution {
    /// Representative latency percentiles.
    #[prost(message, repeated, tag = "1")]
    pub latency_percentiles: ::prost::alloc::vec::Vec<LatencyPercentile>,
}
/// Results of active probing from the last run of the test.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProbingDetails {
    /// The overall result of active probing.
    #[prost(enumeration = "probing_details::ProbingResult", tag = "1")]
    pub result: i32,
    /// The time that reachability was assessed through active probing.
    #[prost(message, optional, tag = "2")]
    pub verify_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Details about an internal failure or the cancellation of active probing.
    #[prost(message, optional, tag = "3")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
    /// The reason probing was aborted.
    #[prost(enumeration = "probing_details::ProbingAbortCause", tag = "4")]
    pub abort_cause: i32,
    /// Number of probes sent.
    #[prost(int32, tag = "5")]
    pub sent_probe_count: i32,
    /// Number of probes that reached the destination.
    #[prost(int32, tag = "6")]
    pub successful_probe_count: i32,
    /// The source and destination endpoints derived from the test input and used
    /// for active probing.
    #[prost(message, optional, tag = "7")]
    pub endpoint_info: ::core::option::Option<EndpointInfo>,
    /// Latency as measured by active probing in one direction:
    /// from the source to the destination endpoint.
    #[prost(message, optional, tag = "8")]
    pub probing_latency: ::core::option::Option<LatencyDistribution>,
    /// The EdgeLocation from which a packet destined for/originating from the
    /// internet will egress/ingress the Google network.
    /// This will only be populated for a connectivity test which has an internet
    /// destination/source address.
    /// The absence of this field *must not* be used as an indication that the
    /// destination/source is part of the Google network.
    #[prost(message, optional, tag = "9")]
    pub destination_egress_location: ::core::option::Option<
        probing_details::EdgeLocation,
    >,
}
/// Nested message and enum types in `ProbingDetails`.
pub mod probing_details {
    /// Representation of a network edge location as per
    /// <https://cloud.google.com/vpc/docs/edge-locations.>
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EdgeLocation {
        /// Name of the metropolitan area.
        #[prost(string, tag = "1")]
        pub metropolitan_area: ::prost::alloc::string::String,
    }
    /// Overall probing result of the test.
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
    pub enum ProbingResult {
        /// No result was specified.
        Unspecified = 0,
        /// At least 95% of packets reached the destination.
        Reachable = 1,
        /// No packets reached the destination.
        Unreachable = 2,
        /// Less than 95% of packets reached the destination.
        ReachabilityInconsistent = 3,
        /// Reachability could not be determined. Possible reasons are:
        /// * The user lacks permission to access some of the network resources
        ///    required to run the test.
        /// * No valid source endpoint could be derived from the request.
        /// * An internal error occurred.
        Undetermined = 4,
    }
    impl ProbingResult {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ProbingResult::Unspecified => "PROBING_RESULT_UNSPECIFIED",
                ProbingResult::Reachable => "REACHABLE",
                ProbingResult::Unreachable => "UNREACHABLE",
                ProbingResult::ReachabilityInconsistent => "REACHABILITY_INCONSISTENT",
                ProbingResult::Undetermined => "UNDETERMINED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PROBING_RESULT_UNSPECIFIED" => Some(Self::Unspecified),
                "REACHABLE" => Some(Self::Reachable),
                "UNREACHABLE" => Some(Self::Unreachable),
                "REACHABILITY_INCONSISTENT" => Some(Self::ReachabilityInconsistent),
                "UNDETERMINED" => Some(Self::Undetermined),
                _ => None,
            }
        }
    }
    /// Abort cause types.
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
    pub enum ProbingAbortCause {
        /// No reason was specified.
        Unspecified = 0,
        /// The user lacks permission to access some of the
        /// network resources required to run the test.
        PermissionDenied = 1,
        /// No valid source endpoint could be derived from the request.
        NoSourceLocation = 2,
    }
    impl ProbingAbortCause {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ProbingAbortCause::Unspecified => "PROBING_ABORT_CAUSE_UNSPECIFIED",
                ProbingAbortCause::PermissionDenied => "PERMISSION_DENIED",
                ProbingAbortCause::NoSourceLocation => "NO_SOURCE_LOCATION",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PROBING_ABORT_CAUSE_UNSPECIFIED" => Some(Self::Unspecified),
                "PERMISSION_DENIED" => Some(Self::PermissionDenied),
                "NO_SOURCE_LOCATION" => Some(Self::NoSourceLocation),
                _ => None,
            }
        }
    }
}
/// Request for the `ListConnectivityTests` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConnectivityTestsRequest {
    /// Required. The parent resource of the Connectivity Tests:
    ///      `projects/{project_id}/locations/global`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Number of `ConnectivityTests` to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Page token from an earlier query, as returned in `next_page_token`.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Lists the `ConnectivityTests` that match the filter expression. A filter
    /// expression filters the resources listed in the response. The expression
    /// must be of the form `<field> <operator> <value>` where operators: `<`, `>`,
    /// `<=`,
    /// `>=`,
    /// `!=`, `=`, `:` are supported (colon `:` represents a HAS operator which is
    /// roughly synonymous with equality). <field> can refer to a proto or JSON
    /// field, or a synthetic field. Field names can be camelCase or snake_case.
    ///
    /// Examples:
    /// - Filter by name:
    ///    name = "projects/proj-1/locations/global/connectivityTests/test-1
    ///
    /// - Filter by labels:
    ///    - Resources that have a key called `foo`
    ///      labels.foo:*
    ///    - Resources that have a key called `foo` whose value is `bar`
    ///      labels.foo = bar
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Field to use to sort the list.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response for the `ListConnectivityTests` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConnectivityTestsResponse {
    /// List of Connectivity Tests.
    #[prost(message, repeated, tag = "1")]
    pub resources: ::prost::alloc::vec::Vec<ConnectivityTest>,
    /// Page token to fetch the next set of Connectivity Tests.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached (when querying all locations with `-`).
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for the `GetConnectivityTest` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConnectivityTestRequest {
    /// Required. `ConnectivityTest` resource name using the form:
    ///      `projects/{project_id}/locations/global/connectivityTests/{test_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for the `CreateConnectivityTest` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateConnectivityTestRequest {
    /// Required. The parent resource of the Connectivity Test to create:
    ///      `projects/{project_id}/locations/global`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The logical name of the Connectivity Test in your project
    /// with the following restrictions:
    ///
    /// * Must contain only lowercase letters, numbers, and hyphens.
    /// * Must start with a letter.
    /// * Must be between 1-40 characters.
    /// * Must end with a number or a letter.
    /// * Must be unique within the customer project
    #[prost(string, tag = "2")]
    pub test_id: ::prost::alloc::string::String,
    /// Required. A `ConnectivityTest` resource
    #[prost(message, optional, tag = "3")]
    pub resource: ::core::option::Option<ConnectivityTest>,
}
/// Request for the `UpdateConnectivityTest` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateConnectivityTestRequest {
    /// Required. Mask of fields to update. At least one path must be supplied in
    /// this field.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. Only fields specified in update_mask are updated.
    #[prost(message, optional, tag = "2")]
    pub resource: ::core::option::Option<ConnectivityTest>,
}
/// Request for the `DeleteConnectivityTest` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteConnectivityTestRequest {
    /// Required. Connectivity Test resource name using the form:
    ///      `projects/{project_id}/locations/global/connectivityTests/{test_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for the `RerunConnectivityTest` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RerunConnectivityTestRequest {
    /// Required. Connectivity Test resource name using the form:
    ///      `projects/{project_id}/locations/global/connectivityTests/{test_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Metadata describing an [Operation][google.longrunning.Operation]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Target of the operation - for example
    /// projects/project-1/locations/global/connectivityTests/test-1
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: ::prost::alloc::string::String,
    /// Human-readable status of the operation, if any.
    #[prost(string, tag = "5")]
    pub status_detail: ::prost::alloc::string::String,
    /// Specifies if cancellation was requested for the operation.
    #[prost(bool, tag = "6")]
    pub cancel_requested: bool,
    /// API version.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod reachability_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// The Reachability service in the Google Cloud Network Management API provides
    /// services that analyze the reachability within a single Google Virtual Private
    /// Cloud (VPC) network, between peered VPC networks, between VPC and on-premises
    /// networks, or between VPC networks and internet hosts. A reachability analysis
    /// is based on Google Cloud network configurations.
    ///
    /// You can use the analysis results to verify these configurations and
    /// to troubleshoot connectivity issues.
    #[derive(Debug, Clone)]
    pub struct ReachabilityServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ReachabilityServiceClient<tonic::transport::Channel> {
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
    impl<T> ReachabilityServiceClient<T>
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
        ) -> ReachabilityServiceClient<InterceptedService<T, F>>
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
            ReachabilityServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists all Connectivity Tests owned by a project.
        pub async fn list_connectivity_tests(
            &mut self,
            request: impl tonic::IntoRequest<super::ListConnectivityTestsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListConnectivityTestsResponse>,
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
                "/google.cloud.networkmanagement.v1beta1.ReachabilityService/ListConnectivityTests",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkmanagement.v1beta1.ReachabilityService",
                        "ListConnectivityTests",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the details of a specific Connectivity Test.
        pub async fn get_connectivity_test(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConnectivityTestRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ConnectivityTest>,
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
                "/google.cloud.networkmanagement.v1beta1.ReachabilityService/GetConnectivityTest",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkmanagement.v1beta1.ReachabilityService",
                        "GetConnectivityTest",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new Connectivity Test.
        /// After you create a test, the reachability analysis is performed as part
        /// of the long running operation, which completes when the analysis completes.
        ///
        /// If the endpoint specifications in `ConnectivityTest` are invalid
        /// (for example, containing non-existent resources in the network, or you
        /// don't have read permissions to the network configurations of listed
        /// projects), then the reachability result returns a value of `UNKNOWN`.
        ///
        /// If the endpoint specifications in `ConnectivityTest` are
        /// incomplete, the reachability result returns a value of
        /// <code>AMBIGUOUS</code>. For more information,
        /// see the Connectivity Test documentation.
        pub async fn create_connectivity_test(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateConnectivityTestRequest>,
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
                "/google.cloud.networkmanagement.v1beta1.ReachabilityService/CreateConnectivityTest",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkmanagement.v1beta1.ReachabilityService",
                        "CreateConnectivityTest",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the configuration of an existing `ConnectivityTest`.
        /// After you update a test, the reachability analysis is performed as part
        /// of the long running operation, which completes when the analysis completes.
        /// The Reachability state in the test resource is updated with the new result.
        ///
        /// If the endpoint specifications in `ConnectivityTest` are invalid
        /// (for example, they contain non-existent resources in the network, or the
        /// user does not have read permissions to the network configurations of
        /// listed projects), then the reachability result returns a value of
        /// <code>UNKNOWN</code>.
        ///
        /// If the endpoint specifications in `ConnectivityTest` are incomplete, the
        /// reachability result returns a value of `AMBIGUOUS`. See the documentation
        /// in `ConnectivityTest` for for more details.
        pub async fn update_connectivity_test(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateConnectivityTestRequest>,
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
                "/google.cloud.networkmanagement.v1beta1.ReachabilityService/UpdateConnectivityTest",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkmanagement.v1beta1.ReachabilityService",
                        "UpdateConnectivityTest",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Rerun an existing `ConnectivityTest`.
        /// After the user triggers the rerun, the reachability analysis is performed
        /// as part of the long running operation, which completes when the analysis
        /// completes.
        ///
        /// Even though the test configuration remains the same, the reachability
        /// result may change due to underlying network configuration changes.
        ///
        /// If the endpoint specifications in `ConnectivityTest` become invalid (for
        /// example, specified resources are deleted in the network, or you lost
        /// read permissions to the network configurations of listed projects), then
        /// the reachability result returns a value of `UNKNOWN`.
        pub async fn rerun_connectivity_test(
            &mut self,
            request: impl tonic::IntoRequest<super::RerunConnectivityTestRequest>,
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
                "/google.cloud.networkmanagement.v1beta1.ReachabilityService/RerunConnectivityTest",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkmanagement.v1beta1.ReachabilityService",
                        "RerunConnectivityTest",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a specific `ConnectivityTest`.
        pub async fn delete_connectivity_test(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteConnectivityTestRequest>,
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
                "/google.cloud.networkmanagement.v1beta1.ReachabilityService/DeleteConnectivityTest",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.networkmanagement.v1beta1.ReachabilityService",
                        "DeleteConnectivityTest",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
