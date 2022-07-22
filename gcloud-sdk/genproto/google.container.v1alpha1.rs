/// Parameters that describe the nodes in a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeConfig {
    /// The name of a Google Compute Engine [machine
    /// type](/compute/docs/machine-types) (e.g.
    /// `n1-standard-1`).
    ///
    /// If unspecified, the default machine type is
    /// `n1-standard-1`.
    #[prost(string, tag="1")]
    pub machine_type: ::prost::alloc::string::String,
    /// Size of the disk attached to each node, specified in GB.
    /// The smallest allowed disk size is 10GB.
    ///
    /// If unspecified, the default disk size is 100GB.
    #[prost(int32, tag="2")]
    pub disk_size_gb: i32,
    /// The set of Google API scopes to be made available on all of the
    /// node VMs under the "default" service account.
    ///
    /// The following scopes are recommended, but not required, and by default are
    /// not included:
    ///
    /// * `<https://www.googleapis.com/auth/compute`> is required for mounting
    /// persistent storage on your nodes.
    /// * `<https://www.googleapis.com/auth/devstorage.read_only`> is required for
    /// communicating with **gcr.io**
    /// (the [Google Container Registry](/container-registry/)).
    ///
    /// If unspecified, no scopes are added, unless Cloud Logging or Cloud
    /// Monitoring are enabled, in which case their required scopes will be added.
    #[prost(string, repeated, tag="3")]
    pub oauth_scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The Google Cloud Platform Service Account to be used by the node VMs. If
    /// no Service Account is specified, the "default" service account is used.
    #[prost(string, tag="9")]
    pub service_account: ::prost::alloc::string::String,
    /// The metadata key/value pairs assigned to instances in the cluster.
    ///
    /// Keys must conform to the regexp \[a-zA-Z0-9-_\]+ and be less than 128 bytes
    /// in length. These are reflected as part of a URL in the metadata server.
    /// Additionally, to avoid ambiguity, keys must not conflict with any other
    /// metadata keys for the project or be one of the four reserved keys:
    /// "instance-template", "kube-env", "startup-script", and "user-data"
    ///
    /// Values are free-form strings, and only have meaning as interpreted by
    /// the image running in the instance. The only restriction placed on them is
    /// that each value's size must be less than or equal to 32 KB.
    ///
    /// The total size of all keys and values must be less than 512 KB.
    #[prost(map="string, string", tag="4")]
    pub metadata: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// The image type to use for this node. Note that for a given image type,
    /// the latest version of it will be used.
    #[prost(string, tag="5")]
    pub image_type: ::prost::alloc::string::String,
    /// The map of Kubernetes labels (key/value pairs) to be applied to each node.
    /// These will added in addition to any default label(s) that
    /// Kubernetes may apply to the node.
    /// In case of conflict in label keys, the applied set may differ depending on
    /// the Kubernetes version -- it's best to assume the behavior is undefined
    /// and conflicts should be avoided.
    /// For more information, including usage and the valid values, see:
    /// <https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/>
    #[prost(map="string, string", tag="6")]
    pub labels: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// The number of local SSD disks to be attached to the node.
    ///
    /// The limit for this value is dependant upon the maximum number of
    /// disks available on a machine per zone. See:
    /// <https://cloud.google.com/compute/docs/disks/local-ssd#local_ssd_limits>
    /// for more information.
    #[prost(int32, tag="7")]
    pub local_ssd_count: i32,
    /// The list of instance tags applied to all nodes. Tags are used to identify
    /// valid sources or targets for network firewalls and are specified by
    /// the client during cluster or node pool creation. Each tag within the list
    /// must comply with RFC1035.
    #[prost(string, repeated, tag="8")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Whether the nodes are created as preemptible VM instances. See:
    /// <https://cloud.google.com/compute/docs/instances/preemptible> for more
    /// inforamtion about preemptible VM instances.
    #[prost(bool, tag="10")]
    pub preemptible: bool,
    /// A list of hardware accelerators to be attached to each node.
    /// See <https://cloud.google.com/compute/docs/gpus> for more information about
    /// support for GPUs.
    #[prost(message, repeated, tag="11")]
    pub accelerators: ::prost::alloc::vec::Vec<AcceleratorConfig>,
    /// Minimum CPU platform to be used by this instance. The instance may be
    /// scheduled on the specified or newer CPU platform. Applicable values are the
    /// friendly names of CPU platforms, such as
    /// <code>minCpuPlatform: &quot;Intel Haswell&quot;</code> or
    /// <code>minCpuPlatform: &quot;Intel Sandy Bridge&quot;</code>. For more
    /// information, read [how to specify min CPU platform](<https://cloud.google.com/compute/docs/instances/specify-min-cpu-platform>)
    #[prost(string, tag="13")]
    pub min_cpu_platform: ::prost::alloc::string::String,
    /// List of kubernetes taints to be applied to each node.
    ///
    /// For more information, including usage and the valid values, see:
    /// <https://kubernetes.io/docs/concepts/configuration/taint-and-toleration/>
    #[prost(message, repeated, tag="15")]
    pub taints: ::prost::alloc::vec::Vec<NodeTaint>,
}
/// Kubernetes taint is comprised of three fields: key, value, and effect. Effect
/// can only be one of three types:  NoSchedule, PreferNoSchedule or NoExecute.
///
/// For more information, including usage and the valid values, see:
/// <https://kubernetes.io/docs/concepts/configuration/taint-and-toleration/>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeTaint {
    /// Key for taint.
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    /// Value for taint.
    #[prost(string, tag="2")]
    pub value: ::prost::alloc::string::String,
    /// Effect for taint.
    #[prost(enumeration="node_taint::Effect", tag="3")]
    pub effect: i32,
}
/// Nested message and enum types in `NodeTaint`.
pub mod node_taint {
    /// Possible values for Effect in taint.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Effect {
        /// Not set
        Unspecified = 0,
        /// NoSchedule
        NoSchedule = 1,
        /// PreferNoSchedule
        PreferNoSchedule = 2,
        /// NoExecute
        NoExecute = 3,
    }
}
/// The authentication information for accessing the master endpoint.
/// Authentication can be done using HTTP basic auth or using client
/// certificates.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MasterAuth {
    /// The username to use for HTTP basic authentication to the master endpoint.
    /// For clusters v1.6.0 and later, you can disable basic authentication by
    /// providing an empty username.
    #[prost(string, tag="1")]
    pub username: ::prost::alloc::string::String,
    /// The password to use for HTTP basic authentication to the master endpoint.
    /// Because the master endpoint is open to the Internet, you should create a
    /// strong password.  If a password is provided for cluster creation, username
    /// must be non-empty.
    #[prost(string, tag="2")]
    pub password: ::prost::alloc::string::String,
    /// Configuration for client certificate authentication on the cluster.  If no
    /// configuration is specified, a client certificate is issued.
    #[prost(message, optional, tag="3")]
    pub client_certificate_config: ::core::option::Option<ClientCertificateConfig>,
    /// [Output only] Base64-encoded public certificate that is the root of
    /// trust for the cluster.
    #[prost(string, tag="100")]
    pub cluster_ca_certificate: ::prost::alloc::string::String,
    /// [Output only] Base64-encoded public certificate used by clients to
    /// authenticate to the cluster endpoint.
    #[prost(string, tag="101")]
    pub client_certificate: ::prost::alloc::string::String,
    /// [Output only] Base64-encoded private key used by clients to authenticate
    /// to the cluster endpoint.
    #[prost(string, tag="102")]
    pub client_key: ::prost::alloc::string::String,
}
/// Configuration for client certificates on the cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientCertificateConfig {
    /// Issue a client certificate.
    #[prost(bool, tag="1")]
    pub issue_client_certificate: bool,
}
/// Configuration for the addons that can be automatically spun up in the
/// cluster, enabling additional functionality.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddonsConfig {
    /// Configuration for the HTTP (L7) load balancing controller addon, which
    /// makes it easy to set up HTTP load balancers for services in a cluster.
    #[prost(message, optional, tag="1")]
    pub http_load_balancing: ::core::option::Option<HttpLoadBalancing>,
    /// Configuration for the horizontal pod autoscaling feature, which
    /// increases or decreases the number of replica pods a replication controller
    /// has based on the resource usage of the existing pods.
    #[prost(message, optional, tag="2")]
    pub horizontal_pod_autoscaling: ::core::option::Option<HorizontalPodAutoscaling>,
    /// Configuration for the Kubernetes Dashboard.
    #[prost(message, optional, tag="3")]
    pub kubernetes_dashboard: ::core::option::Option<KubernetesDashboard>,
    /// Configuration for NetworkPolicy. This only tracks whether the addon
    /// is enabled or not on the Master, it does not track whether network policy
    /// is enabled for the nodes.
    #[prost(message, optional, tag="4")]
    pub network_policy_config: ::core::option::Option<NetworkPolicyConfig>,
}
/// Configuration options for the HTTP (L7) load balancing controller addon,
/// which makes it easy to set up HTTP load balancers for services in a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpLoadBalancing {
    /// Whether the HTTP Load Balancing controller is enabled in the cluster.
    /// When enabled, it runs a small pod in the cluster that manages the load
    /// balancers.
    #[prost(bool, tag="1")]
    pub disabled: bool,
}
/// Configuration options for the horizontal pod autoscaling feature, which
/// increases or decreases the number of replica pods a replication controller
/// has based on the resource usage of the existing pods.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HorizontalPodAutoscaling {
    /// Whether the Horizontal Pod Autoscaling feature is enabled in the cluster.
    /// When enabled, it ensures that a Heapster pod is running in the cluster,
    /// which is also used by the Cloud Monitoring service.
    #[prost(bool, tag="1")]
    pub disabled: bool,
}
/// Configuration for the Kubernetes Dashboard.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KubernetesDashboard {
    /// Whether the Kubernetes Dashboard is enabled for this cluster.
    #[prost(bool, tag="1")]
    pub disabled: bool,
}
/// Configuration for NetworkPolicy. This only tracks whether the addon
/// is enabled or not on the Master, it does not track whether network policy
/// is enabled for the nodes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkPolicyConfig {
    /// Whether NetworkPolicy is enabled for this cluster.
    #[prost(bool, tag="1")]
    pub disabled: bool,
}
/// Configuration options for the master authorized networks feature. Enabled
/// master authorized networks will disallow all external traffic to access
/// Kubernetes master through HTTPS except traffic from the given CIDR blocks,
/// Google Compute Engine Public IPs and Google Prod IPs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MasterAuthorizedNetworksConfig {
    /// Whether or not master authorized networks is enabled.
    #[prost(bool, tag="1")]
    pub enabled: bool,
    /// cidr_blocks define up to 10 external networks that could access
    /// Kubernetes master through HTTPS.
    #[prost(message, repeated, tag="2")]
    pub cidr_blocks: ::prost::alloc::vec::Vec<master_authorized_networks_config::CidrBlock>,
}
/// Nested message and enum types in `MasterAuthorizedNetworksConfig`.
pub mod master_authorized_networks_config {
    /// CidrBlock contains an optional name and one CIDR block.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CidrBlock {
        /// display_name is an optional field for users to identify CIDR blocks.
        #[prost(string, tag="1")]
        pub display_name: ::prost::alloc::string::String,
        /// cidr_block must be specified in CIDR notation.
        #[prost(string, tag="2")]
        pub cidr_block: ::prost::alloc::string::String,
    }
}
/// Configuration options for the NetworkPolicy feature.
/// <https://kubernetes.io/docs/concepts/services-networking/networkpolicies/>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkPolicy {
    /// The selected network policy provider.
    #[prost(enumeration="network_policy::Provider", tag="1")]
    pub provider: i32,
    /// Whether network policy is enabled on the cluster.
    #[prost(bool, tag="2")]
    pub enabled: bool,
}
/// Nested message and enum types in `NetworkPolicy`.
pub mod network_policy {
    /// Allowed Network Policy providers.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Provider {
        /// Not set
        Unspecified = 0,
        /// Tigera (Calico Felix).
        Calico = 1,
    }
}
/// Configuration for controlling how IPs are allocated in the cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpAllocationPolicy {
    /// Whether alias IPs will be used for pod IPs in the cluster.
    #[prost(bool, tag="1")]
    pub use_ip_aliases: bool,
    /// Whether a new subnetwork will be created automatically for the cluster.
    ///
    /// This field is only applicable when `use_ip_aliases` is true.
    #[prost(bool, tag="2")]
    pub create_subnetwork: bool,
    /// A custom subnetwork name to be used if `create_subnetwork` is true.  If
    /// this field is empty, then an automatic name will be chosen for the new
    /// subnetwork.
    #[prost(string, tag="3")]
    pub subnetwork_name: ::prost::alloc::string::String,
    /// This field is deprecated, use cluster_ipv4_cidr_block.
    #[prost(string, tag="4")]
    pub cluster_ipv4_cidr: ::prost::alloc::string::String,
    /// This field is deprecated, use node_ipv4_cidr_block.
    #[prost(string, tag="5")]
    pub node_ipv4_cidr: ::prost::alloc::string::String,
    /// This field is deprecated, use services_ipv4_cidr_block.
    #[prost(string, tag="6")]
    pub services_ipv4_cidr: ::prost::alloc::string::String,
    /// The name of the secondary range to be used for the cluster CIDR
    /// block.  The secondary range will be used for pod IP
    /// addresses. This must be an existing secondary range associated
    /// with the cluster subnetwork.
    ///
    /// This field is only applicable if use_ip_aliases is true and
    /// create_subnetwork is false.
    #[prost(string, tag="7")]
    pub cluster_secondary_range_name: ::prost::alloc::string::String,
    /// The name of the secondary range to be used as for the services
    /// CIDR block.  The secondary range will be used for service
    /// ClusterIPs. This must be an existing secondary range associated
    /// with the cluster subnetwork.
    ///
    /// This field is only applicable with use_ip_aliases is true and
    /// create_subnetwork is false.
    #[prost(string, tag="8")]
    pub services_secondary_range_name: ::prost::alloc::string::String,
    /// The IP address range for the cluster pod IPs. If this field is set, then
    /// `cluster.cluster_ipv4_cidr` must be left blank.
    ///
    /// This field is only applicable when `use_ip_aliases` is true.
    ///
    /// Set to blank to have a range chosen with the default size.
    ///
    /// Set to /netmask (e.g. `/14`) to have a range chosen with a specific
    /// netmask.
    ///
    /// Set to a
    /// \[CIDR\](<http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing>)
    /// notation (e.g. `10.96.0.0/14`) from the RFC-1918 private networks (e.g.
    /// `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`) to pick a specific range
    /// to use.
    #[prost(string, tag="9")]
    pub cluster_ipv4_cidr_block: ::prost::alloc::string::String,
    /// The IP address range of the instance IPs in this cluster.
    ///
    /// This is applicable only if `create_subnetwork` is true.
    ///
    /// Set to blank to have a range chosen with the default size.
    ///
    /// Set to /netmask (e.g. `/14`) to have a range chosen with a specific
    /// netmask.
    ///
    /// Set to a
    /// \[CIDR\](<http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing>)
    /// notation (e.g. `10.96.0.0/14`) from the RFC-1918 private networks (e.g.
    /// `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`) to pick a specific range
    /// to use.
    #[prost(string, tag="10")]
    pub node_ipv4_cidr_block: ::prost::alloc::string::String,
    /// The IP address range of the services IPs in this cluster. If blank, a range
    /// will be automatically chosen with the default size.
    ///
    /// This field is only applicable when `use_ip_aliases` is true.
    ///
    /// Set to blank to have a range chosen with the default size.
    ///
    /// Set to /netmask (e.g. `/14`) to have a range chosen with a specific
    /// netmask.
    ///
    /// Set to a
    /// \[CIDR\](<http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing>)
    /// notation (e.g. `10.96.0.0/14`) from the RFC-1918 private networks (e.g.
    /// `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`) to pick a specific range
    /// to use.
    #[prost(string, tag="11")]
    pub services_ipv4_cidr_block: ::prost::alloc::string::String,
}
/// Configuration for the PodSecurityPolicy feature.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodSecurityPolicyConfig {
    /// Enable the PodSecurityPolicy controller for this cluster. If enabled, pods
    /// must be valid under a PodSecurityPolicy to be created.
    #[prost(bool, tag="1")]
    pub enabled: bool,
}
/// A Google Container Engine cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cluster {
    /// The name of this cluster. The name must be unique within this project
    /// and zone, and can be up to 40 characters with the following restrictions:
    ///
    /// * Lowercase letters, numbers, and hyphens only.
    /// * Must start with a letter.
    /// * Must end with a number or a letter.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// An optional description of this cluster.
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// The number of nodes to create in this cluster. You must ensure that your
    /// Compute Engine <a href="/compute/docs/resource-quotas">resource quota</a>
    /// is sufficient for this number of instances. You must also have available
    /// firewall and routes quota.
    /// For requests, this field should only be used in lieu of a
    /// "node_pool" object, since this configuration (along with the
    /// "node_config") will be used to create a "NodePool" object with an
    /// auto-generated name. Do not use this and a node_pool at the same time.
    #[prost(int32, tag="3")]
    pub initial_node_count: i32,
    /// Parameters used in creating the cluster's nodes.
    /// See `nodeConfig` for the description of its properties.
    /// For requests, this field should only be used in lieu of a
    /// "node_pool" object, since this configuration (along with the
    /// "initial_node_count") will be used to create a "NodePool" object with an
    /// auto-generated name. Do not use this and a node_pool at the same time.
    /// For responses, this field will be populated with the node configuration of
    /// the first node pool.
    ///
    /// If unspecified, the defaults are used.
    #[prost(message, optional, tag="4")]
    pub node_config: ::core::option::Option<NodeConfig>,
    /// The authentication information for accessing the master endpoint.
    #[prost(message, optional, tag="5")]
    pub master_auth: ::core::option::Option<MasterAuth>,
    /// The logging service the cluster should use to write logs.
    /// Currently available options:
    ///
    /// * `logging.googleapis.com` - the Google Cloud Logging service.
    /// * `none` - no logs will be exported from the cluster.
    /// * if left as an empty string,`logging.googleapis.com` will be used.
    #[prost(string, tag="6")]
    pub logging_service: ::prost::alloc::string::String,
    /// The monitoring service the cluster should use to write metrics.
    /// Currently available options:
    ///
    /// * `monitoring.googleapis.com` - the Google Cloud Monitoring service.
    /// * `none` - no metrics will be exported from the cluster.
    /// * if left as an empty string, `monitoring.googleapis.com` will be used.
    #[prost(string, tag="7")]
    pub monitoring_service: ::prost::alloc::string::String,
    /// The name of the Google Compute Engine
    /// \[network\](/compute/docs/networks-and-firewalls#networks) to which the
    /// cluster is connected. If left unspecified, the `default` network
    /// will be used.
    #[prost(string, tag="8")]
    pub network: ::prost::alloc::string::String,
    /// The IP address range of the container pods in this cluster, in
    /// \[CIDR\](<http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing>)
    /// notation (e.g. `10.96.0.0/14`). Leave blank to have
    /// one automatically chosen or specify a `/14` block in `10.0.0.0/8`.
    #[prost(string, tag="9")]
    pub cluster_ipv4_cidr: ::prost::alloc::string::String,
    /// Configurations for the various addons available to run in the cluster.
    #[prost(message, optional, tag="10")]
    pub addons_config: ::core::option::Option<AddonsConfig>,
    /// The name of the Google Compute Engine
    /// \[subnetwork\](/compute/docs/subnetworks) to which the
    /// cluster is connected.
    #[prost(string, tag="11")]
    pub subnetwork: ::prost::alloc::string::String,
    /// The node pools associated with this cluster.
    /// This field should not be set if "node_config" or "initial_node_count" are
    /// specified.
    #[prost(message, repeated, tag="12")]
    pub node_pools: ::prost::alloc::vec::Vec<NodePool>,
    /// The list of Google Compute Engine
    /// \[locations\](/compute/docs/zones#available) in which the cluster's nodes
    /// should be located.
    #[prost(string, repeated, tag="13")]
    pub locations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Kubernetes alpha features are enabled on this cluster. This includes alpha
    /// API groups (e.g. v1alpha1) and features that may not be production ready in
    /// the kubernetes version of the master and nodes.
    /// The cluster has no SLA for uptime and master/node upgrades are disabled.
    /// Alpha enabled clusters are automatically deleted thirty days after
    /// creation.
    #[prost(bool, tag="14")]
    pub enable_kubernetes_alpha: bool,
    /// Configuration options for the NetworkPolicy feature.
    #[prost(message, optional, tag="19")]
    pub network_policy: ::core::option::Option<NetworkPolicy>,
    /// Configuration for cluster IP allocation.
    #[prost(message, optional, tag="20")]
    pub ip_allocation_policy: ::core::option::Option<IpAllocationPolicy>,
    /// The configuration options for master authorized networks feature.
    #[prost(message, optional, tag="22")]
    pub master_authorized_networks_config: ::core::option::Option<MasterAuthorizedNetworksConfig>,
    /// Configure the maintenance policy for this cluster.
    #[prost(message, optional, tag="23")]
    pub maintenance_policy: ::core::option::Option<MaintenancePolicy>,
    /// Configuration for the PodSecurityPolicy feature.
    #[prost(message, optional, tag="25")]
    pub pod_security_policy_config: ::core::option::Option<PodSecurityPolicyConfig>,
    /// [Output only] Server-defined URL for the resource.
    #[prost(string, tag="100")]
    pub self_link: ::prost::alloc::string::String,
    /// [Output only] The name of the Google Compute Engine
    /// \[zone\](/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field is deprecated, use location instead.
    #[prost(string, tag="101")]
    pub zone: ::prost::alloc::string::String,
    /// [Output only] The IP address of this cluster's master endpoint.
    /// The endpoint can be accessed from the internet at
    /// `<https://username:password@endpoint/`.>
    ///
    /// See the `masterAuth` property of this resource for username and
    /// password information.
    #[prost(string, tag="102")]
    pub endpoint: ::prost::alloc::string::String,
    /// The initial Kubernetes version for this cluster.  Valid versions are those
    /// found in validMasterVersions returned by getServerConfig.  The version can
    /// be upgraded over time; such upgrades are reflected in
    /// currentMasterVersion and currentNodeVersion.
    #[prost(string, tag="103")]
    pub initial_cluster_version: ::prost::alloc::string::String,
    /// [Output only] The current software version of the master endpoint.
    #[prost(string, tag="104")]
    pub current_master_version: ::prost::alloc::string::String,
    /// [Output only] The current version of the node software components.
    /// If they are currently at multiple versions because they're in the process
    /// of being upgraded, this reflects the minimum version of all nodes.
    #[prost(string, tag="105")]
    pub current_node_version: ::prost::alloc::string::String,
    /// [Output only] The time the cluster was created, in
    /// \[RFC3339\](<https://www.ietf.org/rfc/rfc3339.txt>) text format.
    #[prost(string, tag="106")]
    pub create_time: ::prost::alloc::string::String,
    /// [Output only] The current status of this cluster.
    #[prost(enumeration="cluster::Status", tag="107")]
    pub status: i32,
    /// [Output only] Additional information about the current status of this
    /// cluster, if available.
    #[prost(string, tag="108")]
    pub status_message: ::prost::alloc::string::String,
    /// [Output only] The size of the address space on each node for hosting
    /// containers. This is provisioned from within the `container_ipv4_cidr`
    /// range.
    #[prost(int32, tag="109")]
    pub node_ipv4_cidr_size: i32,
    /// [Output only] The IP address range of the Kubernetes services in
    /// this cluster, in
    /// \[CIDR\](<http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing>)
    /// notation (e.g. `1.2.3.4/29`). Service addresses are
    /// typically put in the last `/16` from the container CIDR.
    #[prost(string, tag="110")]
    pub services_ipv4_cidr: ::prost::alloc::string::String,
    /// [Output only] The resource URLs of [instance
    /// groups](/compute/docs/instance-groups/) associated with this
    /// cluster.
    #[prost(string, repeated, tag="111")]
    pub instance_group_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// [Output only] The number of nodes currently in the cluster.
    #[prost(int32, tag="112")]
    pub current_node_count: i32,
    /// [Output only] The time the cluster will be automatically
    /// deleted in \[RFC3339\](<https://www.ietf.org/rfc/rfc3339.txt>) text format.
    #[prost(string, tag="113")]
    pub expire_time: ::prost::alloc::string::String,
    /// [Output only] The name of the Google Compute Engine
    /// \[zone\](/compute/docs/regions-zones/regions-zones#available) or
    /// \[region\](/compute/docs/regions-zones/regions-zones#available) in which
    /// the cluster resides.
    #[prost(string, tag="114")]
    pub location: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Cluster`.
pub mod cluster {
    /// The current status of the cluster.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        /// Not set.
        Unspecified = 0,
        /// The PROVISIONING state indicates the cluster is being created.
        Provisioning = 1,
        /// The RUNNING state indicates the cluster has been created and is fully
        /// usable.
        Running = 2,
        /// The RECONCILING state indicates that some work is actively being done on
        /// the cluster, such as upgrading the master or node software. Details can
        /// be found in the `statusMessage` field.
        Reconciling = 3,
        /// The STOPPING state indicates the cluster is being deleted.
        Stopping = 4,
        /// The ERROR state indicates the cluster may be unusable. Details
        /// can be found in the `statusMessage` field.
        Error = 5,
    }
}
/// ClusterUpdate describes an update to the cluster. Exactly one update can
/// be applied to a cluster with each request, so at most one field can be
/// provided.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterUpdate {
    /// The Kubernetes version to change the nodes to (typically an
    /// upgrade). Use `-` to upgrade to the latest version supported by
    /// the server.
    #[prost(string, tag="4")]
    pub desired_node_version: ::prost::alloc::string::String,
    /// The monitoring service the cluster should use to write metrics.
    /// Currently available options:
    ///
    /// * "monitoring.googleapis.com" - the Google Cloud Monitoring service
    /// * "none" - no metrics will be exported from the cluster
    #[prost(string, tag="5")]
    pub desired_monitoring_service: ::prost::alloc::string::String,
    /// Configurations for the various addons available to run in the cluster.
    #[prost(message, optional, tag="6")]
    pub desired_addons_config: ::core::option::Option<AddonsConfig>,
    /// The node pool to be upgraded. This field is mandatory if
    /// "desired_node_version", "desired_image_family" or
    /// "desired_node_pool_autoscaling" is specified and there is more than one
    /// node pool on the cluster.
    #[prost(string, tag="7")]
    pub desired_node_pool_id: ::prost::alloc::string::String,
    /// The desired image type for the node pool.
    /// NOTE: Set the "desired_node_pool" field as well.
    #[prost(string, tag="8")]
    pub desired_image_type: ::prost::alloc::string::String,
    /// Autoscaler configuration for the node pool specified in
    /// desired_node_pool_id. If there is only one pool in the
    /// cluster and desired_node_pool_id is not provided then
    /// the change applies to that single node pool.
    #[prost(message, optional, tag="9")]
    pub desired_node_pool_autoscaling: ::core::option::Option<NodePoolAutoscaling>,
    /// The desired list of Google Compute Engine
    /// \[locations\](/compute/docs/zones#available) in which the cluster's nodes
    /// should be located. Changing the locations a cluster is in will result
    /// in nodes being either created or removed from the cluster, depending on
    /// whether locations are being added or removed.
    ///
    /// This list must always include the cluster's primary zone.
    #[prost(string, repeated, tag="10")]
    pub desired_locations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The desired configuration options for master authorized networks feature.
    #[prost(message, optional, tag="12")]
    pub desired_master_authorized_networks_config: ::core::option::Option<MasterAuthorizedNetworksConfig>,
    /// The desired configuration options for the PodSecurityPolicy feature.
    #[prost(message, optional, tag="14")]
    pub desired_pod_security_policy_config: ::core::option::Option<PodSecurityPolicyConfig>,
    /// The Kubernetes version to change the master to. The only valid value is the
    /// latest supported version. Use "-" to have the server automatically select
    /// the latest version.
    #[prost(string, tag="100")]
    pub desired_master_version: ::prost::alloc::string::String,
}
/// This operation resource represents operations that may have happened or are
/// happening on the cluster. All fields are output only.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Operation {
    /// The server-assigned ID for the operation.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The name of the Google Compute Engine
    /// \[zone\](/compute/docs/zones#available) in which the operation
    /// is taking place.
    /// This field is deprecated, use location instead.
    #[prost(string, tag="2")]
    pub zone: ::prost::alloc::string::String,
    /// The operation type.
    #[prost(enumeration="operation::Type", tag="3")]
    pub operation_type: i32,
    /// The current status of the operation.
    #[prost(enumeration="operation::Status", tag="4")]
    pub status: i32,
    /// Detailed operation progress, if available.
    #[prost(string, tag="8")]
    pub detail: ::prost::alloc::string::String,
    /// If an error has occurred, a textual description of the error.
    #[prost(string, tag="5")]
    pub status_message: ::prost::alloc::string::String,
    /// Server-defined URL for the resource.
    #[prost(string, tag="6")]
    pub self_link: ::prost::alloc::string::String,
    /// Server-defined URL for the target of the operation.
    #[prost(string, tag="7")]
    pub target_link: ::prost::alloc::string::String,
    /// [Output only] The name of the Google Compute Engine
    /// \[zone\](/compute/docs/regions-zones/regions-zones#available) or
    /// \[region\](/compute/docs/regions-zones/regions-zones#available) in which
    /// the cluster resides.
    #[prost(string, tag="9")]
    pub location: ::prost::alloc::string::String,
    /// [Output only] The time the operation started, in
    /// \[RFC3339\](<https://www.ietf.org/rfc/rfc3339.txt>) text format.
    #[prost(string, tag="10")]
    pub start_time: ::prost::alloc::string::String,
    /// [Output only] The time the operation completed, in
    /// \[RFC3339\](<https://www.ietf.org/rfc/rfc3339.txt>) text format.
    #[prost(string, tag="11")]
    pub end_time: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Operation`.
pub mod operation {
    /// Current status of the operation.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        /// Not set.
        Unspecified = 0,
        /// The operation has been created.
        Pending = 1,
        /// The operation is currently running.
        Running = 2,
        /// The operation is done, either cancelled or completed.
        Done = 3,
        /// The operation is aborting.
        Aborting = 4,
    }
    /// Operation type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Not set.
        Unspecified = 0,
        /// Cluster create.
        CreateCluster = 1,
        /// Cluster delete.
        DeleteCluster = 2,
        /// A master upgrade.
        UpgradeMaster = 3,
        /// A node upgrade.
        UpgradeNodes = 4,
        /// Cluster repair.
        RepairCluster = 5,
        /// Cluster update.
        UpdateCluster = 6,
        /// Node pool create.
        CreateNodePool = 7,
        /// Node pool delete.
        DeleteNodePool = 8,
        /// Set node pool management.
        SetNodePoolManagement = 9,
        /// Automatic node pool repair.
        AutoRepairNodes = 10,
        /// Automatic node upgrade.
        AutoUpgradeNodes = 11,
        /// Set labels.
        SetLabels = 12,
        /// Set/generate master auth materials
        SetMasterAuth = 13,
        /// Set node pool size.
        SetNodePoolSize = 14,
        /// Updates network policy for a cluster.
        SetNetworkPolicy = 15,
        /// Set the maintenance policy.
        SetMaintenancePolicy = 16,
    }
}
/// CreateClusterRequest creates a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateClusterRequest {
    /// The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    /// This field is deprecated, use parent instead.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// The name of the Google Compute Engine
    /// \[zone\](/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field is deprecated, use parent instead.
    #[prost(string, tag="2")]
    pub zone: ::prost::alloc::string::String,
    /// A [cluster
    /// resource](/container-engine/reference/rest/v1alpha1/projects.zones.clusters)
    #[prost(message, optional, tag="3")]
    pub cluster: ::core::option::Option<Cluster>,
    /// The parent (project and location) where the cluster will be created.
    /// Specified in the format 'projects/*/locations/*'.
    #[prost(string, tag="5")]
    pub parent: ::prost::alloc::string::String,
}
/// GetClusterRequest gets the settings of a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClusterRequest {
    /// The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    /// This field is deprecated, use name instead.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// The name of the Google Compute Engine
    /// \[zone\](/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="2")]
    pub zone: ::prost::alloc::string::String,
    /// The name of the cluster to retrieve.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// The name (project, location, cluster) of the cluster to retrieve.
    /// Specified in the format 'projects/*/locations/*/clusters/*'.
    #[prost(string, tag="5")]
    pub name: ::prost::alloc::string::String,
}
/// UpdateClusterRequest updates the settings of a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateClusterRequest {
    /// The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    /// This field is deprecated, use name instead.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// The name of the Google Compute Engine
    /// \[zone\](/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="2")]
    pub zone: ::prost::alloc::string::String,
    /// The name of the cluster to upgrade.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// A description of the update.
    #[prost(message, optional, tag="4")]
    pub update: ::core::option::Option<ClusterUpdate>,
    /// The name (project, location, cluster) of the cluster to update.
    /// Specified in the format 'projects/*/locations/*/clusters/*'.
    #[prost(string, tag="5")]
    pub name: ::prost::alloc::string::String,
}
/// SetNodePoolVersionRequest updates the version of a node pool.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateNodePoolRequest {
    /// The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    /// This field is deprecated, use name instead.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// The name of the Google Compute Engine
    /// \[zone\](/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="2")]
    pub zone: ::prost::alloc::string::String,
    /// The name of the cluster to upgrade.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// The name of the node pool to upgrade.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="4")]
    pub node_pool_id: ::prost::alloc::string::String,
    /// The Kubernetes version to change the nodes to (typically an
    /// upgrade). Use `-` to upgrade to the latest version supported by
    /// the server.
    #[prost(string, tag="5")]
    pub node_version: ::prost::alloc::string::String,
    /// The desired image type for the node pool.
    #[prost(string, tag="6")]
    pub image_type: ::prost::alloc::string::String,
    /// The name (project, location, cluster, node pool) of the node pool to update.
    /// Specified in the format 'projects/*/locations/*/clusters/*/nodePools/*'.
    #[prost(string, tag="8")]
    pub name: ::prost::alloc::string::String,
}
/// SetNodePoolAutoscalingRequest sets the autoscaler settings of a node pool.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetNodePoolAutoscalingRequest {
    /// The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    /// This field is deprecated, use name instead.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// The name of the Google Compute Engine
    /// \[zone\](/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="2")]
    pub zone: ::prost::alloc::string::String,
    /// The name of the cluster to upgrade.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// The name of the node pool to upgrade.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="4")]
    pub node_pool_id: ::prost::alloc::string::String,
    /// Autoscaling configuration for the node pool.
    #[prost(message, optional, tag="5")]
    pub autoscaling: ::core::option::Option<NodePoolAutoscaling>,
    /// The name (project, location, cluster, node pool) of the node pool to set
    /// autoscaler settings. Specified in the format
    /// 'projects/*/locations/*/clusters/*/nodePools/*'.
    #[prost(string, tag="6")]
    pub name: ::prost::alloc::string::String,
}
/// SetLoggingServiceRequest sets the logging service of a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetLoggingServiceRequest {
    /// The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    /// This field is deprecated, use name instead.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// The name of the Google Compute Engine
    /// \[zone\](/compute/docs/zones#available) in which the cluster
    /// resides.
    #[prost(string, tag="2")]
    pub zone: ::prost::alloc::string::String,
    /// The name of the cluster to upgrade.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// The logging service the cluster should use to write metrics.
    /// Currently available options:
    ///
    /// * "logging.googleapis.com" - the Google Cloud Logging service
    /// * "none" - no metrics will be exported from the cluster
    #[prost(string, tag="4")]
    pub logging_service: ::prost::alloc::string::String,
    /// The name (project, location, cluster) of the cluster to set logging.
    /// Specified in the format 'projects/*/locations/*/clusters/*'.
    #[prost(string, tag="5")]
    pub name: ::prost::alloc::string::String,
}
/// SetMonitoringServiceRequest sets the monitoring service of a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMonitoringServiceRequest {
    /// The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    /// This field is deprecated, use name instead.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// The name of the Google Compute Engine
    /// \[zone\](/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="2")]
    pub zone: ::prost::alloc::string::String,
    /// The name of the cluster to upgrade.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// The monitoring service the cluster should use to write metrics.
    /// Currently available options:
    ///
    /// * "monitoring.googleapis.com" - the Google Cloud Monitoring service
    /// * "none" - no metrics will be exported from the cluster
    #[prost(string, tag="4")]
    pub monitoring_service: ::prost::alloc::string::String,
    /// The name (project, location, cluster) of the cluster to set monitoring.
    /// Specified in the format 'projects/*/locations/*/clusters/*'.
    #[prost(string, tag="6")]
    pub name: ::prost::alloc::string::String,
}
/// SetAddonsRequest sets the addons associated with the cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAddonsConfigRequest {
    /// The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    /// This field is deprecated, use name instead.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// The name of the Google Compute Engine
    /// \[zone\](/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="2")]
    pub zone: ::prost::alloc::string::String,
    /// The name of the cluster to upgrade.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// The desired configurations for the various addons available to run in the
    /// cluster.
    #[prost(message, optional, tag="4")]
    pub addons_config: ::core::option::Option<AddonsConfig>,
    /// The name (project, location, cluster) of the cluster to set addons.
    /// Specified in the format 'projects/*/locations/*/clusters/*'.
    #[prost(string, tag="6")]
    pub name: ::prost::alloc::string::String,
}
/// SetLocationsRequest sets the locations of the cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetLocationsRequest {
    /// The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    /// This field is deprecated, use name instead.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// The name of the Google Compute Engine
    /// \[zone\](/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="2")]
    pub zone: ::prost::alloc::string::String,
    /// The name of the cluster to upgrade.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// The desired list of Google Compute Engine
    /// \[locations\](/compute/docs/zones#available) in which the cluster's nodes
    /// should be located. Changing the locations a cluster is in will result
    /// in nodes being either created or removed from the cluster, depending on
    /// whether locations are being added or removed.
    ///
    /// This list must always include the cluster's primary zone.
    #[prost(string, repeated, tag="4")]
    pub locations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The name (project, location, cluster) of the cluster to set locations.
    /// Specified in the format 'projects/*/locations/*/clusters/*'.
    #[prost(string, tag="6")]
    pub name: ::prost::alloc::string::String,
}
/// UpdateMasterRequest updates the master of the cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMasterRequest {
    /// The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// The name of the Google Compute Engine
    /// \[zone\](/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="2")]
    pub zone: ::prost::alloc::string::String,
    /// The name of the cluster to upgrade.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// The Kubernetes version to change the master to. The only valid value is the
    /// latest supported version. Use "-" to have the server automatically select
    /// the latest version.
    #[prost(string, tag="4")]
    pub master_version: ::prost::alloc::string::String,
    /// The name (project, location, cluster) of the cluster to update.
    /// Specified in the format 'projects/*/locations/*/clusters/*'.
    #[prost(string, tag="7")]
    pub name: ::prost::alloc::string::String,
}
/// SetMasterAuthRequest updates the admin password of a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMasterAuthRequest {
    /// The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    /// This field is deprecated, use name instead.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// The name of the Google Compute Engine
    /// \[zone\](/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="2")]
    pub zone: ::prost::alloc::string::String,
    /// The name of the cluster to upgrade.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// The exact form of action to be taken on the master auth.
    #[prost(enumeration="set_master_auth_request::Action", tag="4")]
    pub action: i32,
    /// A description of the update.
    #[prost(message, optional, tag="5")]
    pub update: ::core::option::Option<MasterAuth>,
    /// The name (project, location, cluster) of the cluster to set auth.
    /// Specified in the format 'projects/*/locations/*/clusters/*'.
    #[prost(string, tag="7")]
    pub name: ::prost::alloc::string::String,
}
/// Nested message and enum types in `SetMasterAuthRequest`.
pub mod set_master_auth_request {
    /// Operation type: what type update to perform.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Action {
        /// Operation is unknown and will error out.
        Unknown = 0,
        /// Set the password to a user generated value.
        SetPassword = 1,
        /// Generate a new password and set it to that.
        GeneratePassword = 2,
        /// Set the username.  If an empty username is provided, basic authentication
        /// is disabled for the cluster.  If a non-empty username is provided, basic
        /// authentication is enabled, with either a provided password or a generated
        /// one.
        SetUsername = 3,
    }
}
/// DeleteClusterRequest deletes a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteClusterRequest {
    /// The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    /// This field is deprecated, use name instead.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// The name of the Google Compute Engine
    /// \[zone\](/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="2")]
    pub zone: ::prost::alloc::string::String,
    /// The name of the cluster to delete.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// The name (project, location, cluster) of the cluster to delete.
    /// Specified in the format 'projects/*/locations/*/clusters/*'.
    #[prost(string, tag="4")]
    pub name: ::prost::alloc::string::String,
}
/// ListClustersRequest lists clusters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListClustersRequest {
    /// The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    /// This field is deprecated, use parent instead.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// The name of the Google Compute Engine
    /// \[zone\](/compute/docs/zones#available) in which the cluster
    /// resides, or "-" for all zones.
    /// This field is deprecated, use parent instead.
    #[prost(string, tag="2")]
    pub zone: ::prost::alloc::string::String,
    /// The parent (project and location) where the clusters will be listed.
    /// Specified in the format 'projects/*/locations/*'.
    /// Location "-" matches all zones and all regions.
    #[prost(string, tag="4")]
    pub parent: ::prost::alloc::string::String,
}
/// ListClustersResponse is the result of ListClustersRequest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListClustersResponse {
    /// A list of clusters in the project in the specified zone, or
    /// across all ones.
    #[prost(message, repeated, tag="1")]
    pub clusters: ::prost::alloc::vec::Vec<Cluster>,
    /// If any zones are listed here, the list of clusters returned
    /// may be missing those zones.
    #[prost(string, repeated, tag="2")]
    pub missing_zones: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// GetOperationRequest gets a single operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOperationRequest {
    /// The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    /// This field is deprecated, use name instead.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// The name of the Google Compute Engine
    /// \[zone\](/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="2")]
    pub zone: ::prost::alloc::string::String,
    /// The server-assigned `name` of the operation.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="3")]
    pub operation_id: ::prost::alloc::string::String,
    /// The name (project, location, operation id) of the operation to get.
    /// Specified in the format 'projects/*/locations/*/operations/*'.
    #[prost(string, tag="5")]
    pub name: ::prost::alloc::string::String,
}
/// ListOperationsRequest lists operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOperationsRequest {
    /// The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    /// This field is deprecated, use parent instead.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// The name of the Google Compute Engine \[zone\](/compute/docs/zones#available)
    /// to return operations for, or `-` for all zones.
    /// This field is deprecated, use parent instead.
    #[prost(string, tag="2")]
    pub zone: ::prost::alloc::string::String,
    /// The parent (project and location) where the operations will be listed.
    /// Specified in the format 'projects/*/locations/*'.
    /// Location "-" matches all zones and all regions.
    #[prost(string, tag="4")]
    pub parent: ::prost::alloc::string::String,
}
/// CancelOperationRequest cancels a single operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelOperationRequest {
    /// The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    /// This field is deprecated, use name instead.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// The name of the Google Compute Engine
    /// \[zone\](/compute/docs/zones#available) in which the operation resides.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="2")]
    pub zone: ::prost::alloc::string::String,
    /// The server-assigned `name` of the operation.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="3")]
    pub operation_id: ::prost::alloc::string::String,
    /// The name (project, location, operation id) of the operation to cancel.
    /// Specified in the format 'projects/*/locations/*/operations/*'.
    #[prost(string, tag="4")]
    pub name: ::prost::alloc::string::String,
}
/// ListOperationsResponse is the result of ListOperationsRequest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOperationsResponse {
    /// A list of operations in the project in the specified zone.
    #[prost(message, repeated, tag="1")]
    pub operations: ::prost::alloc::vec::Vec<Operation>,
    /// If any zones are listed here, the list of operations returned
    /// may be missing the operations from those zones.
    #[prost(string, repeated, tag="2")]
    pub missing_zones: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Gets the current Container Engine service configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServerConfigRequest {
    /// The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    /// This field is deprecated, use name instead.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// The name of the Google Compute Engine \[zone\](/compute/docs/zones#available)
    /// to return operations for.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="2")]
    pub zone: ::prost::alloc::string::String,
    /// The name (project and location) of the server config to get
    /// Specified in the format 'projects/*/locations/*'.
    #[prost(string, tag="4")]
    pub name: ::prost::alloc::string::String,
}
/// Container Engine service configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerConfig {
    /// Version of Kubernetes the service deploys by default.
    #[prost(string, tag="1")]
    pub default_cluster_version: ::prost::alloc::string::String,
    /// List of valid node upgrade target versions.
    #[prost(string, repeated, tag="3")]
    pub valid_node_versions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Default image type.
    #[prost(string, tag="4")]
    pub default_image_type: ::prost::alloc::string::String,
    /// List of valid image types.
    #[prost(string, repeated, tag="5")]
    pub valid_image_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// List of valid master versions.
    #[prost(string, repeated, tag="6")]
    pub valid_master_versions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// CreateNodePoolRequest creates a node pool for a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateNodePoolRequest {
    /// The Google Developers Console [project ID or project
    /// number](<https://developers.google.com/console/help/new/#projectnumber>).
    /// This field is deprecated, use parent instead.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// The name of the Google Compute Engine
    /// \[zone\](/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field is deprecated, use parent instead.
    #[prost(string, tag="2")]
    pub zone: ::prost::alloc::string::String,
    /// The name of the cluster.
    /// This field is deprecated, use parent instead.
    #[prost(string, tag="3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// The node pool to create.
    #[prost(message, optional, tag="4")]
    pub node_pool: ::core::option::Option<NodePool>,
    /// The parent (project, location, cluster id) where the node pool will be created.
    /// Specified in the format 'projects/*/locations/*/clusters/*/nodePools/*'.
    #[prost(string, tag="6")]
    pub parent: ::prost::alloc::string::String,
}
/// DeleteNodePoolRequest deletes a node pool for a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteNodePoolRequest {
    /// The Google Developers Console [project ID or project
    /// number](<https://developers.google.com/console/help/new/#projectnumber>).
    /// This field is deprecated, use name instead.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// The name of the Google Compute Engine
    /// \[zone\](/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="2")]
    pub zone: ::prost::alloc::string::String,
    /// The name of the cluster.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// The name of the node pool to delete.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="4")]
    pub node_pool_id: ::prost::alloc::string::String,
    /// The name (project, location, cluster, node pool id) of the node pool to delete.
    /// Specified in the format 'projects/*/locations/*/clusters/*/nodePools/*'.
    #[prost(string, tag="6")]
    pub name: ::prost::alloc::string::String,
}
/// ListNodePoolsRequest lists the node pool(s) for a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNodePoolsRequest {
    /// The Google Developers Console [project ID or project
    /// number](<https://developers.google.com/console/help/new/#projectnumber>).
    /// This field is deprecated, use parent instead.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// The name of the Google Compute Engine
    /// \[zone\](/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field is deprecated, use parent instead.
    #[prost(string, tag="2")]
    pub zone: ::prost::alloc::string::String,
    /// The name of the cluster.
    /// This field is deprecated, use parent instead.
    #[prost(string, tag="3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// The parent (project, location, cluster id) where the node pools will be listed.
    /// Specified in the format 'projects/*/locations/*/clusters/*'.
    #[prost(string, tag="5")]
    pub parent: ::prost::alloc::string::String,
}
/// GetNodePoolRequest retrieves a node pool for a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNodePoolRequest {
    /// The Google Developers Console [project ID or project
    /// number](<https://developers.google.com/console/help/new/#projectnumber>).
    /// This field is deprecated, use name instead.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// The name of the Google Compute Engine
    /// \[zone\](/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="2")]
    pub zone: ::prost::alloc::string::String,
    /// The name of the cluster.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// The name of the node pool.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="4")]
    pub node_pool_id: ::prost::alloc::string::String,
    /// The name (project, location, cluster, node pool id) of the node pool to get.
    /// Specified in the format 'projects/*/locations/*/clusters/*/nodePools/*'.
    #[prost(string, tag="6")]
    pub name: ::prost::alloc::string::String,
}
/// NodePool contains the name and configuration for a cluster's node pool.
/// Node pools are a set of nodes (i.e. VM's), with a common configuration and
/// specification, under the control of the cluster master. They may have a set
/// of Kubernetes labels applied to them, which may be used to reference them
/// during pod scheduling. They may also be resized up or down, to accommodate
/// the workload.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodePool {
    /// The name of the node pool.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The node configuration of the pool.
    #[prost(message, optional, tag="2")]
    pub config: ::core::option::Option<NodeConfig>,
    /// The initial node count for the pool. You must ensure that your
    /// Compute Engine <a href="/compute/docs/resource-quotas">resource quota</a>
    /// is sufficient for this number of instances. You must also have available
    /// firewall and routes quota.
    #[prost(int32, tag="3")]
    pub initial_node_count: i32,
    /// Autoscaler configuration for this NodePool. Autoscaler is enabled
    /// only if a valid configuration is present.
    #[prost(message, optional, tag="4")]
    pub autoscaling: ::core::option::Option<NodePoolAutoscaling>,
    /// NodeManagement configuration for this NodePool.
    #[prost(message, optional, tag="5")]
    pub management: ::core::option::Option<NodeManagement>,
    /// [Output only] Server-defined URL for the resource.
    #[prost(string, tag="100")]
    pub self_link: ::prost::alloc::string::String,
    /// [Output only] The version of the Kubernetes of this node.
    #[prost(string, tag="101")]
    pub version: ::prost::alloc::string::String,
    /// [Output only] The resource URLs of [instance
    /// groups](/compute/docs/instance-groups/) associated with this
    /// node pool.
    #[prost(string, repeated, tag="102")]
    pub instance_group_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// [Output only] The status of the nodes in this pool instance.
    #[prost(enumeration="node_pool::Status", tag="103")]
    pub status: i32,
    /// [Output only] Additional information about the current status of this
    /// node pool instance, if available.
    #[prost(string, tag="104")]
    pub status_message: ::prost::alloc::string::String,
}
/// Nested message and enum types in `NodePool`.
pub mod node_pool {
    /// The current status of the node pool instance.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        /// Not set.
        Unspecified = 0,
        /// The PROVISIONING state indicates the node pool is being created.
        Provisioning = 1,
        /// The RUNNING state indicates the node pool has been created
        /// and is fully usable.
        Running = 2,
        /// The RUNNING_WITH_ERROR state indicates the node pool has been created
        /// and is partially usable. Some error state has occurred and some
        /// functionality may be impaired. Customer may need to reissue a request
        /// or trigger a new update.
        RunningWithError = 3,
        /// The RECONCILING state indicates that some work is actively being done on
        /// the node pool, such as upgrading node software. Details can
        /// be found in the `statusMessage` field.
        Reconciling = 4,
        /// The STOPPING state indicates the node pool is being deleted.
        Stopping = 5,
        /// The ERROR state indicates the node pool may be unusable. Details
        /// can be found in the `statusMessage` field.
        Error = 6,
    }
}
/// NodeManagement defines the set of node management services turned on for the
/// node pool.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeManagement {
    /// Whether the nodes will be automatically upgraded.
    #[prost(bool, tag="1")]
    pub auto_upgrade: bool,
    /// Whether the nodes will be automatically repaired.
    #[prost(bool, tag="2")]
    pub auto_repair: bool,
    /// Specifies the Auto Upgrade knobs for the node pool.
    #[prost(message, optional, tag="10")]
    pub upgrade_options: ::core::option::Option<AutoUpgradeOptions>,
}
/// AutoUpgradeOptions defines the set of options for the user to control how
/// the Auto Upgrades will proceed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoUpgradeOptions {
    /// [Output only] This field is set when upgrades are about to commence
    /// with the approximate start time for the upgrades, in
    /// \[RFC3339\](<https://www.ietf.org/rfc/rfc3339.txt>) text format.
    #[prost(string, tag="1")]
    pub auto_upgrade_start_time: ::prost::alloc::string::String,
    /// [Output only] This field is set when upgrades are about to commence
    /// with the description of the upgrade.
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
}
/// MaintenancePolicy defines the maintenance policy to be used for the cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaintenancePolicy {
    /// Specifies the maintenance window in which maintenance may be performed.
    #[prost(message, optional, tag="1")]
    pub window: ::core::option::Option<MaintenanceWindow>,
}
/// MaintenanceWindow defines the maintenance window to be used for the cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaintenanceWindow {
    /// Unimplemented, reserved for future use.
    /// HourlyMaintenanceWindow hourly_maintenance_window = 1;
    #[prost(oneof="maintenance_window::Policy", tags="2")]
    pub policy: ::core::option::Option<maintenance_window::Policy>,
}
/// Nested message and enum types in `MaintenanceWindow`.
pub mod maintenance_window {
    /// Unimplemented, reserved for future use.
    /// HourlyMaintenanceWindow hourly_maintenance_window = 1;
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Policy {
        /// DailyMaintenanceWindow specifies a daily maintenance operation window.
        #[prost(message, tag="2")]
        DailyMaintenanceWindow(super::DailyMaintenanceWindow),
    }
}
/// Time window specified for daily maintenance operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailyMaintenanceWindow {
    /// Time within the maintenance window to start the maintenance operations.
    /// It must be in format "HH:MM”, where HH : \[00-23\] and MM : \[00-59\] GMT.
    #[prost(string, tag="2")]
    pub start_time: ::prost::alloc::string::String,
    /// [Output only] Duration of the time window, automatically chosen to be
    /// smallest possible in the given scenario.
    #[prost(string, tag="3")]
    pub duration: ::prost::alloc::string::String,
}
/// SetNodePoolManagementRequest sets the node management properties of a node
/// pool.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetNodePoolManagementRequest {
    /// The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    /// This field is deprecated, use name instead.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// The name of the Google Compute Engine
    /// \[zone\](/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="2")]
    pub zone: ::prost::alloc::string::String,
    /// The name of the cluster to update.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// The name of the node pool to update.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="4")]
    pub node_pool_id: ::prost::alloc::string::String,
    /// NodeManagement configuration for the node pool.
    #[prost(message, optional, tag="5")]
    pub management: ::core::option::Option<NodeManagement>,
    /// The name (project, location, cluster, node pool id) of the node pool to set
    /// management properties. Specified in the format
    /// 'projects/*/locations/*/clusters/*/nodePools/*'.
    #[prost(string, tag="7")]
    pub name: ::prost::alloc::string::String,
}
/// SetNodePoolSizeRequest sets the size a node
/// pool.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetNodePoolSizeRequest {
    /// The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// The name of the Google Compute Engine
    /// \[zone\](/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="2")]
    pub zone: ::prost::alloc::string::String,
    /// The name of the cluster to update.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// The name of the node pool to update.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="4")]
    pub node_pool_id: ::prost::alloc::string::String,
    /// The desired node count for the pool.
    #[prost(int32, tag="5")]
    pub node_count: i32,
    /// The name (project, location, cluster, node pool id) of the node pool to set
    /// size.
    /// Specified in the format 'projects/*/locations/*/clusters/*/nodePools/*'.
    #[prost(string, tag="7")]
    pub name: ::prost::alloc::string::String,
}
/// RollbackNodePoolUpgradeRequest rollbacks the previously Aborted or Failed
/// NodePool upgrade. This will be an no-op if the last upgrade successfully
/// completed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollbackNodePoolUpgradeRequest {
    /// The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    /// This field is deprecated, use name instead.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// The name of the Google Compute Engine
    /// \[zone\](/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="2")]
    pub zone: ::prost::alloc::string::String,
    /// The name of the cluster to rollback.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// The name of the node pool to rollback.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="4")]
    pub node_pool_id: ::prost::alloc::string::String,
    /// The name (project, location, cluster, node pool id) of the node poll to
    /// rollback upgrade.
    /// Specified in the format 'projects/*/locations/*/clusters/*/nodePools/*'.
    #[prost(string, tag="6")]
    pub name: ::prost::alloc::string::String,
}
/// ListNodePoolsResponse is the result of ListNodePoolsRequest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNodePoolsResponse {
    /// A list of node pools for a cluster.
    #[prost(message, repeated, tag="1")]
    pub node_pools: ::prost::alloc::vec::Vec<NodePool>,
}
/// NodePoolAutoscaling contains information required by cluster autoscaler to
/// adjust the size of the node pool to the current cluster usage.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodePoolAutoscaling {
    /// Is autoscaling enabled for this node pool.
    #[prost(bool, tag="1")]
    pub enabled: bool,
    /// Minimum number of nodes in the NodePool. Must be >= 1 and <=
    /// max_node_count.
    #[prost(int32, tag="2")]
    pub min_node_count: i32,
    /// Maximum number of nodes in the NodePool. Must be >= min_node_count. There
    /// has to enough quota to scale up the cluster.
    #[prost(int32, tag="3")]
    pub max_node_count: i32,
}
/// SetLabelsRequest sets the Google Cloud Platform labels on a Google Container
/// Engine cluster, which will in turn set them for Google Compute Engine
/// resources used by that cluster
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetLabelsRequest {
    /// The Google Developers Console [project ID or project
    /// number](<https://developers.google.com/console/help/new/#projectnumber>).
    /// This field is deprecated, use name instead.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// The name of the Google Compute Engine
    /// \[zone\](/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="2")]
    pub zone: ::prost::alloc::string::String,
    /// The name of the cluster.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// The labels to set for that cluster.
    #[prost(map="string, string", tag="4")]
    pub resource_labels: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// The fingerprint of the previous set of labels for this resource,
    /// used to detect conflicts. The fingerprint is initially generated by
    /// Container Engine and changes after every request to modify or update
    /// labels. You must always provide an up-to-date fingerprint hash when
    /// updating or changing labels. Make a <code>get()</code> request to the
    /// resource to get the latest fingerprint.
    #[prost(string, tag="5")]
    pub label_fingerprint: ::prost::alloc::string::String,
    /// The name (project, location, cluster id) of the cluster to set labels.
    /// Specified in the format 'projects/*/locations/*/clusters/*'.
    #[prost(string, tag="7")]
    pub name: ::prost::alloc::string::String,
}
/// SetLegacyAbacRequest enables or disables the ABAC authorization mechanism for
/// a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetLegacyAbacRequest {
    /// The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    /// This field is deprecated, use name instead.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// The name of the Google Compute Engine
    /// \[zone\](/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="2")]
    pub zone: ::prost::alloc::string::String,
    /// The name of the cluster to update.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Whether ABAC authorization will be enabled in the cluster.
    #[prost(bool, tag="4")]
    pub enabled: bool,
    /// The name (project, location, cluster id) of the cluster to set legacy abac.
    /// Specified in the format 'projects/*/locations/*/clusters/*'.
    #[prost(string, tag="6")]
    pub name: ::prost::alloc::string::String,
}
/// StartIPRotationRequest creates a new IP for the cluster and then performs
/// a node upgrade on each node pool to point to the new IP.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartIpRotationRequest {
    /// The Google Developers Console [project ID or project
    /// number](<https://developers.google.com/console/help/new/#projectnumber>).
    /// This field is deprecated, use name instead.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// The name of the Google Compute Engine
    /// \[zone\](/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="2")]
    pub zone: ::prost::alloc::string::String,
    /// The name of the cluster.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// The name (project, location, cluster id) of the cluster to start IP rotation.
    /// Specified in the format 'projects/*/locations/*/clusters/*'.
    #[prost(string, tag="6")]
    pub name: ::prost::alloc::string::String,
}
/// CompleteIPRotationRequest moves the cluster master back into single-IP mode.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompleteIpRotationRequest {
    /// The Google Developers Console [project ID or project
    /// number](<https://developers.google.com/console/help/new/#projectnumber>).
    /// This field is deprecated, use name instead.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// The name of the Google Compute Engine
    /// \[zone\](/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="2")]
    pub zone: ::prost::alloc::string::String,
    /// The name of the cluster.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// The name (project, location, cluster id) of the cluster to complete IP rotation.
    /// Specified in the format 'projects/*/locations/*/clusters/*'.
    #[prost(string, tag="7")]
    pub name: ::prost::alloc::string::String,
}
/// AcceleratorConfig represents a Hardware Accelerator request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcceleratorConfig {
    /// The number of the accelerator cards exposed to an instance.
    #[prost(int64, tag="1")]
    pub accelerator_count: i64,
    /// The accelerator type resource name. List of supported accelerators
    /// \[here\](/compute/docs/gpus/#Introduction)
    #[prost(string, tag="2")]
    pub accelerator_type: ::prost::alloc::string::String,
}
/// SetNetworkPolicyRequest enables/disables network policy for a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetNetworkPolicyRequest {
    /// The Google Developers Console [project ID or project
    /// number](<https://developers.google.com/console/help/new/#projectnumber>).
    /// This field is deprecated, use name instead.
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// The name of the Google Compute Engine
    /// \[zone\](/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="2")]
    pub zone: ::prost::alloc::string::String,
    /// The name of the cluster.
    /// This field is deprecated, use name instead.
    #[prost(string, tag="3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Configuration options for the NetworkPolicy feature.
    #[prost(message, optional, tag="4")]
    pub network_policy: ::core::option::Option<NetworkPolicy>,
    /// The name (project, location, cluster id) of the cluster to set networking
    /// policy.
    /// Specified in the format 'projects/*/locations/*/clusters/*'.
    #[prost(string, tag="6")]
    pub name: ::prost::alloc::string::String,
}
/// SetMaintenancePolicyRequest sets the maintenance policy for a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMaintenancePolicyRequest {
    /// The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// The name of the Google Compute Engine
    /// \[zone\](/compute/docs/zones#available) in which the cluster
    /// resides.
    #[prost(string, tag="2")]
    pub zone: ::prost::alloc::string::String,
    /// The name of the cluster to update.
    #[prost(string, tag="3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// The maintenance policy to be set for the cluster. An empty field
    /// clears the existing maintenance policy.
    #[prost(message, optional, tag="4")]
    pub maintenance_policy: ::core::option::Option<MaintenancePolicy>,
    /// The name (project, location, cluster id) of the cluster to set maintenance
    /// policy.
    /// Specified in the format 'projects/*/locations/*/clusters/*'.
    #[prost(string, tag="5")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod cluster_manager_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Google Container Engine Cluster Manager v1alpha1
    #[derive(Debug, Clone)]
    pub struct ClusterManagerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ClusterManagerClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ClusterManagerClient<T>
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
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ClusterManagerClient<InterceptedService<T, F>>
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
            ClusterManagerClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        /// Lists all clusters owned by a project in either the specified zone or all
        /// zones.
        pub async fn list_clusters(
            &mut self,
            request: impl tonic::IntoRequest<super::ListClustersRequest>,
        ) -> Result<tonic::Response<super::ListClustersResponse>, tonic::Status> {
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
                "/google.container.v1alpha1.ClusterManager/ListClusters",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the details of a specific cluster.
        pub async fn get_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::GetClusterRequest>,
        ) -> Result<tonic::Response<super::Cluster>, tonic::Status> {
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
                "/google.container.v1alpha1.ClusterManager/GetCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a cluster, consisting of the specified number and type of Google
        /// Compute Engine instances.
        ///
        /// By default, the cluster is created in the project's
        /// [default network](/compute/docs/networks-and-firewalls#networks).
        ///
        /// One firewall is added for the cluster. After cluster creation,
        /// the cluster creates routes for each node to allow the containers
        /// on that node to communicate with all other instances in the
        /// cluster.
        ///
        /// Finally, an entry is added to the project's global metadata indicating
        /// which CIDR range is being used by the cluster.
        pub async fn create_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateClusterRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1alpha1.ClusterManager/CreateCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the settings of a specific cluster.
        pub async fn update_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateClusterRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1alpha1.ClusterManager/UpdateCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the version and/or iamge type of a specific node pool.
        pub async fn update_node_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateNodePoolRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1alpha1.ClusterManager/UpdateNodePool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sets the autoscaling settings of a specific node pool.
        pub async fn set_node_pool_autoscaling(
            &mut self,
            request: impl tonic::IntoRequest<super::SetNodePoolAutoscalingRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1alpha1.ClusterManager/SetNodePoolAutoscaling",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sets the logging service of a specific cluster.
        pub async fn set_logging_service(
            &mut self,
            request: impl tonic::IntoRequest<super::SetLoggingServiceRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1alpha1.ClusterManager/SetLoggingService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sets the monitoring service of a specific cluster.
        pub async fn set_monitoring_service(
            &mut self,
            request: impl tonic::IntoRequest<super::SetMonitoringServiceRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1alpha1.ClusterManager/SetMonitoringService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sets the addons of a specific cluster.
        pub async fn set_addons_config(
            &mut self,
            request: impl tonic::IntoRequest<super::SetAddonsConfigRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1alpha1.ClusterManager/SetAddonsConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sets the locations of a specific cluster.
        pub async fn set_locations(
            &mut self,
            request: impl tonic::IntoRequest<super::SetLocationsRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1alpha1.ClusterManager/SetLocations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the master of a specific cluster.
        pub async fn update_master(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateMasterRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1alpha1.ClusterManager/UpdateMaster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Used to set master auth materials. Currently supports :-
        /// Changing the admin password of a specific cluster.
        /// This can be either via password generation or explicitly set.
        /// Modify basic_auth.csv and reset the K8S API server.
        pub async fn set_master_auth(
            &mut self,
            request: impl tonic::IntoRequest<super::SetMasterAuthRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1alpha1.ClusterManager/SetMasterAuth",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes the cluster, including the Kubernetes endpoint and all worker
        /// nodes.
        ///
        /// Firewalls and routes that were configured during cluster creation
        /// are also deleted.
        ///
        /// Other Google Compute Engine resources that might be in use by the cluster
        /// (e.g. load balancer resources) will not be deleted if they weren't present
        /// at the initial create time.
        pub async fn delete_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteClusterRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1alpha1.ClusterManager/DeleteCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists all operations in a project in a specific zone or all zones.
        pub async fn list_operations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListOperationsRequest>,
        ) -> Result<tonic::Response<super::ListOperationsResponse>, tonic::Status> {
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
                "/google.container.v1alpha1.ClusterManager/ListOperations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the specified operation.
        pub async fn get_operation(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOperationRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1alpha1.ClusterManager/GetOperation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Cancels the specified operation.
        pub async fn cancel_operation(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelOperationRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.container.v1alpha1.ClusterManager/CancelOperation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns configuration info about the Container Engine service.
        pub async fn get_server_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetServerConfigRequest>,
        ) -> Result<tonic::Response<super::ServerConfig>, tonic::Status> {
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
                "/google.container.v1alpha1.ClusterManager/GetServerConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists the node pools for a cluster.
        pub async fn list_node_pools(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNodePoolsRequest>,
        ) -> Result<tonic::Response<super::ListNodePoolsResponse>, tonic::Status> {
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
                "/google.container.v1alpha1.ClusterManager/ListNodePools",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Retrieves the node pool requested.
        pub async fn get_node_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNodePoolRequest>,
        ) -> Result<tonic::Response<super::NodePool>, tonic::Status> {
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
                "/google.container.v1alpha1.ClusterManager/GetNodePool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a node pool for a cluster.
        pub async fn create_node_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateNodePoolRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1alpha1.ClusterManager/CreateNodePool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a node pool from a cluster.
        pub async fn delete_node_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteNodePoolRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1alpha1.ClusterManager/DeleteNodePool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Roll back the previously Aborted or Failed NodePool upgrade.
        /// This will be an no-op if the last upgrade successfully completed.
        pub async fn rollback_node_pool_upgrade(
            &mut self,
            request: impl tonic::IntoRequest<super::RollbackNodePoolUpgradeRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1alpha1.ClusterManager/RollbackNodePoolUpgrade",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sets the NodeManagement options for a node pool.
        pub async fn set_node_pool_management(
            &mut self,
            request: impl tonic::IntoRequest<super::SetNodePoolManagementRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1alpha1.ClusterManager/SetNodePoolManagement",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sets labels on a cluster.
        pub async fn set_labels(
            &mut self,
            request: impl tonic::IntoRequest<super::SetLabelsRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1alpha1.ClusterManager/SetLabels",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Enables or disables the ABAC authorization mechanism on a cluster.
        pub async fn set_legacy_abac(
            &mut self,
            request: impl tonic::IntoRequest<super::SetLegacyAbacRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1alpha1.ClusterManager/SetLegacyAbac",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Start master IP rotation.
        pub async fn start_ip_rotation(
            &mut self,
            request: impl tonic::IntoRequest<super::StartIpRotationRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1alpha1.ClusterManager/StartIPRotation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Completes master IP rotation.
        pub async fn complete_ip_rotation(
            &mut self,
            request: impl tonic::IntoRequest<super::CompleteIpRotationRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1alpha1.ClusterManager/CompleteIPRotation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sets the size of a specific node pool.
        pub async fn set_node_pool_size(
            &mut self,
            request: impl tonic::IntoRequest<super::SetNodePoolSizeRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1alpha1.ClusterManager/SetNodePoolSize",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Enables/Disables Network Policy for a cluster.
        pub async fn set_network_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::SetNetworkPolicyRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1alpha1.ClusterManager/SetNetworkPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sets the maintenance policy for a cluster.
        pub async fn set_maintenance_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::SetMaintenancePolicyRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1alpha1.ClusterManager/SetMaintenancePolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
