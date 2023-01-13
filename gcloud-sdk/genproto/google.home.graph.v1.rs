/// Third-party device definition.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Device {
    /// Third-party device ID.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Hardware type of the device.
    /// See [device
    /// types](<https://developers.home.google.com/cloud-to-cloud/guides>).
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
    /// Traits supported by the device.
    /// See [device
    /// traits](<https://developers.home.google.com/cloud-to-cloud/traits>).
    #[prost(string, repeated, tag = "3")]
    pub traits: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Names given to this device by your smart home Action.
    #[prost(message, optional, tag = "4")]
    pub name: ::core::option::Option<DeviceNames>,
    /// Indicates whether your smart home Action will report state of this device
    /// to Google via
    /// \[ReportStateAndNotification][google.home.graph.v1.HomeGraphApiService.ReportStateAndNotification\].
    #[prost(bool, tag = "5")]
    pub will_report_state: bool,
    /// Suggested name for the room where this device is installed.
    /// Google attempts to use this value during user setup.
    #[prost(string, tag = "6")]
    pub room_hint: ::prost::alloc::string::String,
    /// Suggested name for the structure where this device is installed.
    /// Google attempts to use this value during user setup.
    #[prost(string, tag = "7")]
    pub structure_hint: ::prost::alloc::string::String,
    /// Device manufacturer, model, hardware version, and software version.
    #[prost(message, optional, tag = "8")]
    pub device_info: ::core::option::Option<DeviceInfo>,
    /// Attributes for the traits supported by the device.
    #[prost(message, optional, tag = "9")]
    pub attributes: ::core::option::Option<::prost_types::Struct>,
    /// Custom device attributes stored in Home Graph and provided to your
    /// smart home Action in each
    /// \[QUERY\](<https://developers.home.google.com/cloud-to-cloud/intents/query>)
    /// and
    /// \[EXECUTE\](<https://developers.home.google.com/cloud-to-cloud/intents/execute>)
    /// intent.
    /// Data in this object has a few constraints: No sensitive information,
    /// including but not limited to Personally Identifiable Information.
    #[prost(message, optional, tag = "10")]
    pub custom_data: ::core::option::Option<::prost_types::Struct>,
    /// Alternate IDs associated with this device.
    /// This is used to identify cloud synced devices enabled for [local
    /// fulfillment](<https://developers.home.google.com/local-home/overview>).
    #[prost(message, repeated, tag = "11")]
    pub other_device_ids: ::prost::alloc::vec::Vec<AgentOtherDeviceId>,
    /// Indicates whether your smart home Action will report notifications
    /// to Google for this device via
    /// \[ReportStateAndNotification][google.home.graph.v1.HomeGraphApiService.ReportStateAndNotification\].
    ///
    /// If your smart home Action enables users to control device notifications,
    /// you should update this field and call
    /// \[RequestSyncDevices][google.home.graph.v1.HomeGraphApiService.RequestSyncDevices\].
    #[prost(bool, tag = "12")]
    pub notification_supported_by_agent: bool,
}
/// Identifiers used to describe the device.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceNames {
    /// Primary name of the device, generally provided by the user.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Additional names provided by the user for the device.
    #[prost(string, repeated, tag = "2")]
    pub nicknames: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// List of names provided by the manufacturer rather than the user, such as
    /// serial numbers, SKUs, etc.
    #[prost(string, repeated, tag = "3")]
    pub default_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Device information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceInfo {
    /// Device manufacturer.
    #[prost(string, tag = "1")]
    pub manufacturer: ::prost::alloc::string::String,
    /// Device model.
    #[prost(string, tag = "2")]
    pub model: ::prost::alloc::string::String,
    /// Device hardware version.
    #[prost(string, tag = "3")]
    pub hw_version: ::prost::alloc::string::String,
    /// Device software version.
    #[prost(string, tag = "4")]
    pub sw_version: ::prost::alloc::string::String,
}
/// Alternate third-party device ID.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AgentOtherDeviceId {
    /// Project ID for your smart home Action.
    #[prost(string, tag = "1")]
    pub agent_id: ::prost::alloc::string::String,
    /// Unique third-party device ID.
    #[prost(string, tag = "2")]
    pub device_id: ::prost::alloc::string::String,
}
/// Request type for the
/// \[`RequestSyncDevices`\](#google.home.graph.v1.HomeGraphApiService.RequestSyncDevices)
/// call.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestSyncDevicesRequest {
    /// Required. Third-party user ID.
    #[prost(string, tag = "1")]
    pub agent_user_id: ::prost::alloc::string::String,
    /// Optional. If set, the request will be added to a queue and a response will
    /// be returned immediately. This enables concurrent requests for the given
    /// `agent_user_id`, but the caller will not receive any error responses.
    #[prost(bool, tag = "2")]
    pub r#async: bool,
}
/// Response type for the
/// \[`RequestSyncDevices`\](#google.home.graph.v1.HomeGraphApiService.RequestSyncDevices)
/// call.
///
/// Intentionally empty upon success. An HTTP response code is returned
/// with more details upon failure.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestSyncDevicesResponse {}
/// Request type for the
/// \[`ReportStateAndNotification`\](#google.home.graph.v1.HomeGraphApiService.ReportStateAndNotification)
/// call. It may include states, notifications, or both. States and notifications
/// are defined per `device_id` (for example, "123" and "456" in the following
/// example).
///
/// Example:
///
/// ```json
/// {
///    "requestId": "ff36a3cc-ec34-11e6-b1a0-64510650abcf",
///    "agentUserId": "1234",
///    "payload": {
///      "devices": {
///        "states": {
///          "123": {
///            "on": true
///          },
///          "456": {
///            "on": true,
///            "brightness": 10
///          }
///        },
///      }
///    }
/// }
/// ```
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportStateAndNotificationRequest {
    /// Request ID used for debugging.
    #[prost(string, tag = "1")]
    pub request_id: ::prost::alloc::string::String,
    /// Unique identifier per event (for example, a doorbell press).
    #[prost(string, tag = "4")]
    pub event_id: ::prost::alloc::string::String,
    /// Required. Third-party user ID.
    #[prost(string, tag = "2")]
    pub agent_user_id: ::prost::alloc::string::String,
    /// Deprecated.
    #[deprecated]
    #[prost(string, tag = "5")]
    pub follow_up_token: ::prost::alloc::string::String,
    /// Required. State of devices to update and notification metadata for devices.
    #[prost(message, optional, tag = "3")]
    pub payload: ::core::option::Option<StateAndNotificationPayload>,
}
/// Response type for the
/// \[`ReportStateAndNotification`\](#google.home.graph.v1.HomeGraphApiService.ReportStateAndNotification)
/// call.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportStateAndNotificationResponse {
    /// Request ID copied from
    /// \[ReportStateAndNotificationRequest][google.home.graph.v1.ReportStateAndNotificationRequest\].
    #[prost(string, tag = "1")]
    pub request_id: ::prost::alloc::string::String,
}
/// Payload containing the state and notification information for devices.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StateAndNotificationPayload {
    /// The devices for updating state and sending notifications.
    #[prost(message, optional, tag = "1")]
    pub devices: ::core::option::Option<ReportStateAndNotificationDevice>,
}
/// The states and notifications specific to a device.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportStateAndNotificationDevice {
    /// States of devices to update. See the **Device STATES** section
    /// of the individual trait [reference
    /// guides](<https://developers.home.google.com/cloud-to-cloud/traits>).
    #[prost(message, optional, tag = "1")]
    pub states: ::core::option::Option<::prost_types::Struct>,
    /// Notifications metadata for devices. See the **Device NOTIFICATIONS**
    /// section of the individual trait [reference
    /// guides](<https://developers.home.google.com/cloud-to-cloud/traits>).
    #[prost(message, optional, tag = "2")]
    pub notifications: ::core::option::Option<::prost_types::Struct>,
}
/// Request type for the
/// \[`DeleteAgentUser`\](#google.home.graph.v1.HomeGraphApiService.DeleteAgentUser)
/// call.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAgentUserRequest {
    /// Request ID used for debugging.
    #[prost(string, tag = "1")]
    pub request_id: ::prost::alloc::string::String,
    /// Required. Third-party user ID.
    #[prost(string, tag = "2")]
    pub agent_user_id: ::prost::alloc::string::String,
}
/// Request type for the
/// \[`Query`\](#google.home.graph.v1.HomeGraphApiService.Query) call.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRequest {
    /// Request ID used for debugging.
    #[prost(string, tag = "1")]
    pub request_id: ::prost::alloc::string::String,
    /// Required. Third-party user ID.
    #[prost(string, tag = "2")]
    pub agent_user_id: ::prost::alloc::string::String,
    /// Required. Inputs containing third-party device IDs for which to
    /// get the device states.
    #[prost(message, repeated, tag = "3")]
    pub inputs: ::prost::alloc::vec::Vec<QueryRequestInput>,
}
/// Device ID inputs to \[QueryRequest][google.home.graph.v1.QueryRequest\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRequestInput {
    /// Payload containing third-party device IDs.
    #[prost(message, optional, tag = "1")]
    pub payload: ::core::option::Option<QueryRequestPayload>,
}
/// Payload containing device IDs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRequestPayload {
    /// Third-party device IDs for which to get the device states.
    #[prost(message, repeated, tag = "1")]
    pub devices: ::prost::alloc::vec::Vec<AgentDeviceId>,
}
/// Third-party device ID for one device.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AgentDeviceId {
    /// Third-party device ID.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// Response type for the
/// \[`Query`\](#google.home.graph.v1.HomeGraphApiService.Query) call.
/// This should follow the same format as the Google smart home
/// `action.devices.QUERY`
/// \[response\](<https://developers.home.google.com/cloud-to-cloud/intents/query>).
///
/// Example:
///
/// ```json
/// {
///    "requestId": "ff36a3cc-ec34-11e6-b1a0-64510650abcf",
///    "payload": {
///      "devices": {
///        "123": {
///          "on": true,
///          "online": true
///        },
///        "456": {
///          "on": true,
///          "online": true,
///          "brightness": 80,
///          "color": {
///            "name": "cerulean",
///            "spectrumRGB": 31655
///          }
///        }
///      }
///    }
/// }
/// ```
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResponse {
    /// Request ID used for debugging. Copied from the request.
    #[prost(string, tag = "1")]
    pub request_id: ::prost::alloc::string::String,
    /// Device states for the devices given in the request.
    #[prost(message, optional, tag = "2")]
    pub payload: ::core::option::Option<QueryResponsePayload>,
}
/// Payload containing device states information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResponsePayload {
    /// States of the devices. Map of third-party device ID to struct of device
    /// states.
    #[prost(map = "string, message", tag = "1")]
    pub devices: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost_types::Struct,
    >,
}
/// Request type for the \[`Sync`\](#google.home.graph.v1.HomeGraphApiService.Sync)
/// call.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncRequest {
    /// Request ID used for debugging.
    #[prost(string, tag = "1")]
    pub request_id: ::prost::alloc::string::String,
    /// Required. Third-party user ID.
    #[prost(string, tag = "2")]
    pub agent_user_id: ::prost::alloc::string::String,
}
/// Response type for the
/// \[`Sync`\](#google.home.graph.v1.HomeGraphApiService.Sync) call.
/// This should follow the same format as the Google smart home
/// `action.devices.SYNC`
/// \[response\](<https://developers.home.google.com/cloud-to-cloud/intents/sync>).
///
/// Example:
///
/// ```json
/// {
///    "requestId": "ff36a3cc-ec34-11e6-b1a0-64510650abcf",
///    "payload": {
///      "agentUserId": "1836.15267389",
///      "devices": [{
///        "id": "123",
///        "type": "action.devices.types.OUTLET",
///        "traits": [
///          "action.devices.traits.OnOff"
///        ],
///        "name": {
///          "defaultNames": ["My Outlet 1234"],
///          "name": "Night light",
///          "nicknames": ["wall plug"]
///        },
///        "willReportState": false,
///        "deviceInfo": {
///          "manufacturer": "lights-out-inc",
///          "model": "hs1234",
///          "hwVersion": "3.2",
///          "swVersion": "11.4"
///        },
///        "customData": {
///          "fooValue": 74,
///          "barValue": true,
///          "bazValue": "foo"
///        }
///      }]
///    }
/// }
/// ```
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncResponse {
    /// Request ID used for debugging. Copied from the request.
    #[prost(string, tag = "1")]
    pub request_id: ::prost::alloc::string::String,
    /// Devices associated with the third-party user.
    #[prost(message, optional, tag = "2")]
    pub payload: ::core::option::Option<SyncResponsePayload>,
}
/// Payload containing device information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncResponsePayload {
    /// Third-party user ID
    #[prost(string, tag = "1")]
    pub agent_user_id: ::prost::alloc::string::String,
    /// Devices associated with the third-party user.
    #[prost(message, repeated, tag = "2")]
    pub devices: ::prost::alloc::vec::Vec<Device>,
}
/// Generated client implementations.
pub mod home_graph_api_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Google Home Graph API service. The Home Graph service provides support for
    /// accessing first-party and third-party devices stored in Google's Home Graph.
    /// The Home Graph database provides contextual data about the relationships
    /// between devices and the home.
    ///
    /// For more details, see the [Home Graph developer
    /// guide](https://developers.home.google.com/cloud-to-cloud/primer/home-graph).
    #[derive(Debug, Clone)]
    pub struct HomeGraphApiServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl HomeGraphApiServiceClient<tonic::transport::Channel> {
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
    impl<T> HomeGraphApiServiceClient<T>
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
        ) -> HomeGraphApiServiceClient<InterceptedService<T, F>>
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
            HomeGraphApiServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Requests Google to send an `action.devices.SYNC`
        /// [intent](https://developers.home.google.com/cloud-to-cloud/intents/sync)
        /// to your smart home Action to update device metadata for the given user.
        ///
        ///
        /// The third-party user's identity is passed via the `agent_user_id`
        /// (see
        /// [RequestSyncDevicesRequest][google.home.graph.v1.RequestSyncDevicesRequest]).
        /// This request must be authorized using service account credentials from your
        /// Actions console project.
        pub async fn request_sync_devices(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestSyncDevicesRequest>,
        ) -> Result<tonic::Response<super::RequestSyncDevicesResponse>, tonic::Status> {
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
                "/google.home.graph.v1.HomeGraphApiService/RequestSyncDevices",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Reports device state and optionally sends device notifications.
        /// Called by your smart home Action when the state of a third-party device
        /// changes or you need to send a notification about the device.
        /// See [Implement Report
        /// State](https://developers.home.google.com/cloud-to-cloud/integration/report-state)
        /// for more information.
        ///
        /// This method updates the device state according to its declared
        /// [traits](https://developers.home.google.com/cloud-to-cloud/primer/device-types-and-traits).
        /// Publishing a new state value outside of these traits will result in an
        /// `INVALID_ARGUMENT` error response.
        ///
        /// The third-party user's identity is passed in via the `agent_user_id`
        /// (see
        /// [ReportStateAndNotificationRequest][google.home.graph.v1.ReportStateAndNotificationRequest]).
        /// This request must be authorized using service account credentials from your
        /// Actions console project.
        pub async fn report_state_and_notification(
            &mut self,
            request: impl tonic::IntoRequest<super::ReportStateAndNotificationRequest>,
        ) -> Result<
            tonic::Response<super::ReportStateAndNotificationResponse>,
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
                "/google.home.graph.v1.HomeGraphApiService/ReportStateAndNotification",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Unlinks the given third-party user from your smart home Action.
        /// All data related to this user will be deleted.
        ///
        /// For more details on how users link their accounts, see
        /// [fulfillment and
        /// authentication](https://developers.home.google.com/cloud-to-cloud/primer/fulfillment).
        ///
        /// The third-party user's identity is passed in via the `agent_user_id`
        /// (see
        /// [DeleteAgentUserRequest][google.home.graph.v1.DeleteAgentUserRequest]).
        /// This request must be authorized using service account credentials from your
        /// Actions console project.
        pub async fn delete_agent_user(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAgentUserRequest>,
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
                "/google.home.graph.v1.HomeGraphApiService/DeleteAgentUser",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the current states in Home Graph for the given set of the third-party
        /// user's devices.
        ///
        /// The third-party user's identity is passed in via the `agent_user_id`
        /// (see [QueryRequest][google.home.graph.v1.QueryRequest]).
        /// This request must be authorized using service account credentials from your
        /// Actions console project.
        pub async fn query(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryRequest>,
        ) -> Result<tonic::Response<super::QueryResponse>, tonic::Status> {
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
                "/google.home.graph.v1.HomeGraphApiService/Query",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets all the devices associated with the given third-party user.
        ///
        /// The third-party user's identity is passed in via the `agent_user_id`
        /// (see [SyncRequest][google.home.graph.v1.SyncRequest]).
        /// This request must be authorized using service account credentials from your
        /// Actions console project.
        pub async fn sync(
            &mut self,
            request: impl tonic::IntoRequest<super::SyncRequest>,
        ) -> Result<tonic::Response<super::SyncResponse>, tonic::Status> {
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
                "/google.home.graph.v1.HomeGraphApiService/Sync",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
