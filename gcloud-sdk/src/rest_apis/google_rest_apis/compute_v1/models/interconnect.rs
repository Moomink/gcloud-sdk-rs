use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// Interconnect : Represents an Interconnect resource. An Interconnect resource is a dedicated connection between the Google Cloud network and your on-premises network. For more information, read the Dedicated Interconnect Overview.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Interconnect {
    /// Administrative status of the interconnect. When this is set to true, the Interconnect is functional and can carry traffic. When set to false, no packets can be carried over the interconnect and no BGP routes are exchanged over it. By default, the status is set to true.
    #[serde(rename = "adminEnabled", skip_serializing_if = "Option::is_none")]
    pub admin_enabled: Option<bool>,
    /// [Output only] List of features available for this Interconnect connection, which can take one of the following values: - MACSEC If present then the Interconnect connection is provisioned on MACsec capable hardware ports. If not present then the Interconnect connection is provisioned on non-MACsec capable ports and MACsec isn't supported and enabling MACsec fails.
    #[serde(rename = "availableFeatures", skip_serializing_if = "Option::is_none")]
    pub available_features: Option<Vec<AvailableFeatures>>,
    /// [Output Only] A list of CircuitInfo objects, that describe the individual circuits in this LAG.
    #[serde(rename = "circuitInfos", skip_serializing_if = "Option::is_none")]
    pub circuit_infos:
        Option<Vec<crate::google_rest_apis::compute_v1::models::InterconnectCircuitInfo>>,
    /// [Output Only] Creation timestamp in RFC3339 text format.
    #[serde(rename = "creationTimestamp", skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// Customer name, to put in the Letter of Authorization as the party authorized to request a crossconnect.
    #[serde(rename = "customerName", skip_serializing_if = "Option::is_none")]
    pub customer_name: Option<String>,
    /// An optional description of this resource. Provide this property when you create the resource.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// [Output Only] A list of outages expected for this Interconnect.
    #[serde(rename = "expectedOutages", skip_serializing_if = "Option::is_none")]
    pub expected_outages:
        Option<Vec<crate::google_rest_apis::compute_v1::models::InterconnectOutageNotification>>,
    /// [Output Only] IP address configured on the Google side of the Interconnect link. This can be used only for ping tests.
    #[serde(rename = "googleIpAddress", skip_serializing_if = "Option::is_none")]
    pub google_ip_address: Option<String>,
    /// [Output Only] Google reference ID to be used when raising support tickets with Google or otherwise to debug backend connectivity issues.
    #[serde(rename = "googleReferenceId", skip_serializing_if = "Option::is_none")]
    pub google_reference_id: Option<String>,
    /// [Output Only] The unique identifier for the resource. This identifier is defined by the server.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// [Output Only] A list of the URLs of all InterconnectAttachments configured to use this Interconnect.
    #[serde(
        rename = "interconnectAttachments",
        skip_serializing_if = "Option::is_none"
    )]
    pub interconnect_attachments: Option<Vec<String>>,
    /// Type of interconnect, which can take one of the following values: - PARTNER: A partner-managed interconnection shared between customers though a partner. - DEDICATED: A dedicated physical interconnection with the customer. Note that a value IT_PRIVATE has been deprecated in favor of DEDICATED.
    #[serde(rename = "interconnectType", skip_serializing_if = "Option::is_none")]
    pub interconnect_type: Option<InterconnectType>,
    /// [Output Only] Type of the resource. Always compute#interconnect for interconnects.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// A fingerprint for the labels being applied to this Interconnect, which is essentially a hash of the labels set used for optimistic locking. The fingerprint is initially generated by Compute Engine and changes after every request to modify or update labels. You must always provide an up-to-date fingerprint hash in order to update or change labels, otherwise the request will fail with error 412 conditionNotMet. To see the latest fingerprint, make a get() request to retrieve an Interconnect.
    #[serde(rename = "labelFingerprint", skip_serializing_if = "Option::is_none")]
    pub label_fingerprint: Option<String>,
    /// Labels for this resource. These can only be added or modified by the setLabels method. Each label key/value pair must comply with RFC1035. Label values may be empty.
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    /// Type of link requested, which can take one of the following values: - LINK_TYPE_ETHERNET_10G_LR: A 10G Ethernet with LR optics - LINK_TYPE_ETHERNET_100G_LR: A 100G Ethernet with LR optics. Note that this field indicates the speed of each of the links in the bundle, not the speed of the entire bundle.
    #[serde(rename = "linkType", skip_serializing_if = "Option::is_none")]
    pub link_type: Option<LinkType>,
    /// URL of the InterconnectLocation object that represents where this connection is to be provisioned.
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "macsec", skip_serializing_if = "Option::is_none")]
    pub macsec: Option<Box<crate::google_rest_apis::compute_v1::models::InterconnectMacsec>>,
    /// Enable or disable MACsec on this Interconnect connection. MACsec enablement fails if the MACsec object is not specified.
    #[serde(rename = "macsecEnabled", skip_serializing_if = "Option::is_none")]
    pub macsec_enabled: Option<bool>,
    /// Name of the resource. Provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Email address to contact the customer NOC for operations and maintenance notifications regarding this Interconnect. If specified, this will be used for notifications in addition to all other forms described, such as Cloud Monitoring logs alerting and Cloud Notifications. This field is required for users who sign up for Cloud Interconnect using workforce identity federation.
    #[serde(rename = "nocContactEmail", skip_serializing_if = "Option::is_none")]
    pub noc_contact_email: Option<String>,
    /// [Output Only] The current status of this Interconnect's functionality, which can take one of the following values: - OS_ACTIVE: A valid Interconnect, which is turned up and is ready to use. Attachments may be provisioned on this Interconnect. - OS_UNPROVISIONED: An Interconnect that has not completed turnup. No attachments may be provisioned on this Interconnect. - OS_UNDER_MAINTENANCE: An Interconnect that is undergoing internal maintenance. No attachments may be provisioned or updated on this Interconnect.
    #[serde(rename = "operationalStatus", skip_serializing_if = "Option::is_none")]
    pub operational_status: Option<OperationalStatus>,
    /// [Output Only] IP address configured on the customer side of the Interconnect link. The customer should configure this IP address during turnup when prompted by Google NOC. This can be used only for ping tests.
    #[serde(rename = "peerIpAddress", skip_serializing_if = "Option::is_none")]
    pub peer_ip_address: Option<String>,
    /// [Output Only] Number of links actually provisioned in this interconnect.
    #[serde(
        rename = "provisionedLinkCount",
        skip_serializing_if = "Option::is_none"
    )]
    pub provisioned_link_count: Option<i32>,
    /// Indicates that this is a Cross-Cloud Interconnect. This field specifies the location outside of Google's network that the interconnect is connected to.
    #[serde(rename = "remoteLocation", skip_serializing_if = "Option::is_none")]
    pub remote_location: Option<String>,
    /// Optional. List of features requested for this Interconnect connection, which can take one of the following values: - MACSEC If specified then the connection is created on MACsec capable hardware ports. If not specified, the default value is false, which allocates non-MACsec capable ports first if available. This parameter can be provided only with Interconnect INSERT. It isn't valid for Interconnect PATCH.
    #[serde(rename = "requestedFeatures", skip_serializing_if = "Option::is_none")]
    pub requested_features: Option<Vec<RequestedFeatures>>,
    /// Target number of physical links in the link bundle, as requested by the customer.
    #[serde(rename = "requestedLinkCount", skip_serializing_if = "Option::is_none")]
    pub requested_link_count: Option<i32>,
    /// [Output Only] Reserved for future use.
    #[serde(rename = "satisfiesPzs", skip_serializing_if = "Option::is_none")]
    pub satisfies_pzs: Option<bool>,
    /// [Output Only] Server-defined URL for the resource.
    #[serde(rename = "selfLink", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    /// [Output Only] The current state of Interconnect functionality, which can take one of the following values: - ACTIVE: The Interconnect is valid, turned up and ready to use. Attachments may be provisioned on this Interconnect. - UNPROVISIONED: The Interconnect has not completed turnup. No attachments may be provisioned on this Interconnect. - UNDER_MAINTENANCE: The Interconnect is undergoing internal maintenance. No attachments may be provisioned or updated on this Interconnect.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
}

impl Interconnect {
    /// Represents an Interconnect resource. An Interconnect resource is a dedicated connection between the Google Cloud network and your on-premises network. For more information, read the Dedicated Interconnect Overview.
    pub fn new() -> Interconnect {
        Interconnect {
            admin_enabled: None,
            available_features: None,
            circuit_infos: None,
            creation_timestamp: None,
            customer_name: None,
            description: None,
            expected_outages: None,
            google_ip_address: None,
            google_reference_id: None,
            id: None,
            interconnect_attachments: None,
            interconnect_type: None,
            kind: None,
            label_fingerprint: None,
            labels: None,
            link_type: None,
            location: None,
            macsec: None,
            macsec_enabled: None,
            name: None,
            noc_contact_email: None,
            operational_status: None,
            peer_ip_address: None,
            provisioned_link_count: None,
            remote_location: None,
            requested_features: None,
            requested_link_count: None,
            satisfies_pzs: None,
            self_link: None,
            state: None,
        }
    }
}

/// [Output only] List of features available for this Interconnect connection, which can take one of the following values: - MACSEC If present then the Interconnect connection is provisioned on MACsec capable hardware ports. If not present then the Interconnect connection is provisioned on non-MACsec capable ports and MACsec isn't supported and enabling MACsec fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AvailableFeatures {
    #[serde(rename = "IF_MACSEC")]
    IfMacsec,
}

impl Default for AvailableFeatures {
    fn default() -> AvailableFeatures {
        Self::IfMacsec
    }
}
/// Type of interconnect, which can take one of the following values: - PARTNER: A partner-managed interconnection shared between customers though a partner. - DEDICATED: A dedicated physical interconnection with the customer. Note that a value IT_PRIVATE has been deprecated in favor of DEDICATED.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InterconnectType {
    #[serde(rename = "DEDICATED")]
    Dedicated,
    #[serde(rename = "IT_PRIVATE")]
    ItPrivate,
    #[serde(rename = "PARTNER")]
    Partner,
}

impl Default for InterconnectType {
    fn default() -> InterconnectType {
        Self::Dedicated
    }
}
/// Type of link requested, which can take one of the following values: - LINK_TYPE_ETHERNET_10G_LR: A 10G Ethernet with LR optics - LINK_TYPE_ETHERNET_100G_LR: A 100G Ethernet with LR optics. Note that this field indicates the speed of each of the links in the bundle, not the speed of the entire bundle.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LinkType {
    #[serde(rename = "LINK_TYPE_ETHERNET_100G_LR")]
    Variant100Glr,
    #[serde(rename = "LINK_TYPE_ETHERNET_10G_LR")]
    Variant10Glr,
}

impl Default for LinkType {
    fn default() -> LinkType {
        Self::Variant100Glr
    }
}
/// [Output Only] The current status of this Interconnect's functionality, which can take one of the following values: - OS_ACTIVE: A valid Interconnect, which is turned up and is ready to use. Attachments may be provisioned on this Interconnect. - OS_UNPROVISIONED: An Interconnect that has not completed turnup. No attachments may be provisioned on this Interconnect. - OS_UNDER_MAINTENANCE: An Interconnect that is undergoing internal maintenance. No attachments may be provisioned or updated on this Interconnect.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OperationalStatus {
    #[serde(rename = "OS_ACTIVE")]
    Active,
    #[serde(rename = "OS_UNPROVISIONED")]
    Unprovisioned,
}

impl Default for OperationalStatus {
    fn default() -> OperationalStatus {
        Self::Active
    }
}
/// Optional. List of features requested for this Interconnect connection, which can take one of the following values: - MACSEC If specified then the connection is created on MACsec capable hardware ports. If not specified, the default value is false, which allocates non-MACsec capable ports first if available. This parameter can be provided only with Interconnect INSERT. It isn't valid for Interconnect PATCH.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RequestedFeatures {
    #[serde(rename = "IF_MACSEC")]
    IfMacsec,
}

impl Default for RequestedFeatures {
    fn default() -> RequestedFeatures {
        Self::IfMacsec
    }
}
/// [Output Only] The current state of Interconnect functionality, which can take one of the following values: - ACTIVE: The Interconnect is valid, turned up and ready to use. Attachments may be provisioned on this Interconnect. - UNPROVISIONED: The Interconnect has not completed turnup. No attachments may be provisioned on this Interconnect. - UNDER_MAINTENANCE: The Interconnect is undergoing internal maintenance. No attachments may be provisioned or updated on this Interconnect.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "UNPROVISIONED")]
    Unprovisioned,
}

impl Default for State {
    fn default() -> State {
        Self::Active
    }
}