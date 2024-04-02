/// A subscription to receive events about a Google Workspace resource. To learn
/// more about subscriptions, see the [Google Workspace Events API
/// overview](<https://developers.google.com/workspace/events>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Subscription {
    /// Optional. Immutable. Identifier. Resource name of the subscription.
    ///
    /// Format: `subscriptions/{subscription}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. System-assigned unique identifier for the subscription.
    #[prost(string, tag = "2")]
    pub uid: ::prost::alloc::string::String,
    /// Required. Immutable. The Google Workspace resource that's monitored for
    /// events, formatted as the [full resource
    /// name](<https://google.aip.dev/122#full-resource-names>). To learn about
    /// target resources and the events that they support, see [Supported Google
    /// Workspace
    /// events](<https://developers.google.com/workspace/events#supported-events>).
    ///
    /// A user can only authorize your app to create one subscription for a given
    /// target resource. If your app tries to create another subscription with the
    /// same user credentials, the request returns an `ALREADY_EXISTS` error.
    #[prost(string, tag = "4")]
    pub target_resource: ::prost::alloc::string::String,
    /// Required. Immutable. Unordered list. Input for creating a subscription.
    /// Otherwise, output only. One or more types of events to receive about the
    /// target resource. Formatted according to the CloudEvents specification.
    ///
    /// The supported event types depend on the target resource of your
    /// subscription. For details, see [Supported Google Workspace
    /// events](<https://developers.google.com/workspace/events/guides#supported-events>).
    ///
    /// By default, you also receive events about the [lifecycle of your
    /// subscription](<https://developers.google.com/workspace/events/guides/events-lifecycle>).
    /// You don't need to specify lifecycle events for this field.
    ///
    /// If you specify an event type that doesn't exist for the target resource,
    /// the request returns an HTTP `400 Bad Request` status code.
    #[prost(string, repeated, tag = "5")]
    pub event_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. Options about what data to include in the event payload. Only
    /// supported for Google Chat events.
    #[prost(message, optional, tag = "6")]
    pub payload_options: ::core::option::Option<PayloadOptions>,
    /// Required. Immutable. The endpoint where the subscription delivers events,
    /// such as a Pub/Sub topic.
    #[prost(message, optional, tag = "7")]
    pub notification_endpoint: ::core::option::Option<NotificationEndpoint>,
    /// Output only. The state of the subscription. Determines whether the
    /// subscription can receive events and deliver them to the notification
    /// endpoint.
    #[prost(enumeration = "subscription::State", tag = "8")]
    pub state: i32,
    /// Output only. The error that suspended the subscription.
    ///
    /// To reactivate the subscription, resolve the error and call the
    /// [`ReactivateSubscription`][google.apps.events.subscriptions.v1.SubscriptionsService.ReactivateSubscription]
    /// method.
    #[prost(enumeration = "subscription::ErrorType", tag = "18")]
    pub suspension_reason: i32,
    /// Output only. The user who authorized the creation of the
    /// subscription.
    ///
    /// Format: `users/{user}`
    ///
    /// For Google Workspace users, the `{user}` value is the
    /// [`user.id`](<https://developers.google.com/admin-sdk/directory/reference/rest/v1/users#User.FIELDS.ids>)
    /// field from the Directory API.
    #[prost(string, tag = "10")]
    pub authority: ::prost::alloc::string::String,
    /// Output only. The time when the subscription is created.
    #[prost(message, optional, tag = "11")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last time that the subscription is updated.
    #[prost(message, optional, tag = "12")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. If `true`, the subscription is in the process of being
    /// updated.
    #[prost(bool, tag = "15")]
    pub reconciling: bool,
    /// Optional. This checksum is computed by the server based on the value of
    /// other fields, and might be sent on update requests to ensure the client has
    /// an up-to-date value before proceeding.
    #[prost(string, tag = "17")]
    pub etag: ::prost::alloc::string::String,
    /// The time when the subscription expires.
    ///
    /// The maximum expiration time depends on whether your subscription includes
    /// resource data in event payloads (specified in the
    /// [PayloadOptions][google.apps.events.subscriptions.v1.PayloadOptions]
    /// field):
    ///
    /// * If payloads omit resource data, up to 7 days.
    /// * If payloads include resource data, up to 4 hours. If your Google
    /// Workspace organization grants access to the resource through [domain-wide
    /// delegation](<https://support.google.com/a/answer/162106>), you can extend the
    /// subscription's expiration time to up to 24 hours.
    ///
    /// After a subscription expires, it's deleted automatically. You receive
    /// lifecycle events to the
    /// [notification_endpoint][google.apps.events.subscriptions.v1.Subscription.notification_endpoint]
    /// 12 hours and one hour before the subscription expires. For details, see
    /// [Receive and respond to lifecycle
    /// events](<https://developers.google.com/workspace/events/guides/events-lifecycle>).
    ///
    /// To prevent a subscription from expiring, you can use the
    /// [`UpdateSubscription`][google.apps.events.subscriptions.v1.SubscriptionsService.UpdateSubscription]
    /// method to extend its expiration date. For details, see [Update or renew a
    /// subscription](<https://developers.google.com/workspace/events/guides/update-subscription>).
    #[prost(oneof = "subscription::Expiration", tags = "13, 14")]
    pub expiration: ::core::option::Option<subscription::Expiration>,
}
/// Nested message and enum types in `Subscription`.
pub mod subscription {
    /// Possible states for the subscription.
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
        /// Default value. This value is unused.
        Unspecified = 0,
        /// The subscription is active and can receive and deliver events to its
        /// notification endpoint.
        Active = 1,
        /// The subscription is unable to receive events due to an error.
        /// To identify the error, see the
        /// [`suspension_reason`][google.apps.events.subscriptions.v1.Subscription.suspension_reason]
        /// field.
        Suspended = 2,
        /// The subscription is deleted.
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
                State::Active => "ACTIVE",
                State::Suspended => "SUSPENDED",
                State::Deleted => "DELETED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ACTIVE" => Some(Self::Active),
                "SUSPENDED" => Some(Self::Suspended),
                "DELETED" => Some(Self::Deleted),
                _ => None,
            }
        }
    }
    /// Possible errors for a subscription.
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
    pub enum ErrorType {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// The authorizing user has revoked the grant of one or more OAuth scopes.
        /// To learn more about authorization for Google Workspace, see [Configure
        /// the OAuth consent
        /// screen](<https://developers.google.com/workspace/guides/configure-oauth-consent#choose-scopes>).
        UserScopeRevoked = 1,
        /// The target resource for the subscription no longer exists.
        ResourceDeleted = 2,
        /// The user that authorized the creation of the subscription no longer has
        /// access to the subscription's target resource.
        UserAuthorizationFailure = 3,
        /// The Google Workspace application doesn't have access to deliver
        /// events to your subscription's notification endpoint.
        EndpointPermissionDenied = 4,
        /// The subscription's notification endpoint doesn't exist, or the endpoint
        /// can't be found in the Google Cloud project where you created the
        /// subscription.
        EndpointNotFound = 6,
        /// The subscription's notification endpoint failed to receive events due to
        /// insufficient quota or reaching rate limiting.
        EndpointResourceExhausted = 7,
        /// An unidentified error has occurred.
        Other = 5,
    }
    impl ErrorType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ErrorType::Unspecified => "ERROR_TYPE_UNSPECIFIED",
                ErrorType::UserScopeRevoked => "USER_SCOPE_REVOKED",
                ErrorType::ResourceDeleted => "RESOURCE_DELETED",
                ErrorType::UserAuthorizationFailure => "USER_AUTHORIZATION_FAILURE",
                ErrorType::EndpointPermissionDenied => "ENDPOINT_PERMISSION_DENIED",
                ErrorType::EndpointNotFound => "ENDPOINT_NOT_FOUND",
                ErrorType::EndpointResourceExhausted => "ENDPOINT_RESOURCE_EXHAUSTED",
                ErrorType::Other => "OTHER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ERROR_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "USER_SCOPE_REVOKED" => Some(Self::UserScopeRevoked),
                "RESOURCE_DELETED" => Some(Self::ResourceDeleted),
                "USER_AUTHORIZATION_FAILURE" => Some(Self::UserAuthorizationFailure),
                "ENDPOINT_PERMISSION_DENIED" => Some(Self::EndpointPermissionDenied),
                "ENDPOINT_NOT_FOUND" => Some(Self::EndpointNotFound),
                "ENDPOINT_RESOURCE_EXHAUSTED" => Some(Self::EndpointResourceExhausted),
                "OTHER" => Some(Self::Other),
                _ => None,
            }
        }
    }
    /// The time when the subscription expires.
    ///
    /// The maximum expiration time depends on whether your subscription includes
    /// resource data in event payloads (specified in the
    /// [PayloadOptions][google.apps.events.subscriptions.v1.PayloadOptions]
    /// field):
    ///
    /// * If payloads omit resource data, up to 7 days.
    /// * If payloads include resource data, up to 4 hours. If your Google
    /// Workspace organization grants access to the resource through [domain-wide
    /// delegation](<https://support.google.com/a/answer/162106>), you can extend the
    /// subscription's expiration time to up to 24 hours.
    ///
    /// After a subscription expires, it's deleted automatically. You receive
    /// lifecycle events to the
    /// [notification_endpoint][google.apps.events.subscriptions.v1.Subscription.notification_endpoint]
    /// 12 hours and one hour before the subscription expires. For details, see
    /// [Receive and respond to lifecycle
    /// events](<https://developers.google.com/workspace/events/guides/events-lifecycle>).
    ///
    /// To prevent a subscription from expiring, you can use the
    /// [`UpdateSubscription`][google.apps.events.subscriptions.v1.SubscriptionsService.UpdateSubscription]
    /// method to extend its expiration date. For details, see [Update or renew a
    /// subscription](<https://developers.google.com/workspace/events/guides/update-subscription>).
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Expiration {
        /// Non-empty default. The timestamp in UTC when the subscription expires.
        /// Always displayed on output, regardless of what was used on input.
        #[prost(message, tag = "13")]
        ExpireTime(::prost_types::Timestamp),
        /// Input only. The time-to-live (TTL) or duration for the subscription. If
        /// unspecified or set to `0`, uses the maximum possible duration.
        #[prost(message, tag = "14")]
        Ttl(::prost_types::Duration),
    }
}
/// Options about what data to include in the event payload. Only supported for
/// Google Chat events.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PayloadOptions {
    /// Optional. Whether the event payload includes data about the resource that
    /// changed. For example, for an event where a Google Chat message was created,
    /// whether the payload contains data about the
    /// [`Message`](<https://developers.google.com/chat/api/reference/rest/v1/spaces.messages>)
    /// resource. If false, the event payload only includes the name of the changed
    /// resource.
    #[prost(bool, tag = "1")]
    pub include_resource: bool,
    /// Optional. If `include_resource` is set to `true`, the list of fields to
    /// include in the event payload. Separate fields with a comma. For example, to
    /// include a Google Chat message's sender and create time, enter
    /// `message.sender,message.createTime`. If omitted, the payload includes all
    /// fields for the resource.
    ///
    /// If you specify a field that doesn't exist for the resource, the system
    /// ignores the field.
    #[prost(message, optional, tag = "2")]
    pub field_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The endpoint where the subscription delivers events.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationEndpoint {
    #[prost(oneof = "notification_endpoint::Endpoint", tags = "1")]
    pub endpoint: ::core::option::Option<notification_endpoint::Endpoint>,
}
/// Nested message and enum types in `NotificationEndpoint`.
pub mod notification_endpoint {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Endpoint {
        /// Immutable. The Cloud Pub/Sub topic that receives events for the
        /// subscription.
        ///
        /// Format: `projects/{project}/topics/{topic}`
        ///
        ///
        /// You must create the topic in the same Google Cloud project where
        /// you create this subscription.
        ///
        /// When the topic receives events, the events are encoded as Cloud Pub/Sub
        /// messages. For details, see the [Google Cloud Pub/Sub Protocol Binding for
        /// CloudEvents](<https://github.com/googleapis/google-cloudevents/blob/main/docs/spec/pubsub.md>).
        #[prost(string, tag = "1")]
        PubsubTopic(::prost::alloc::string::String),
    }
}
/// The request message for
/// [SubscriptionsService.CreateSubscription][google.apps.events.subscriptions.v1.SubscriptionsService.CreateSubscription].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSubscriptionRequest {
    /// Required. The subscription resource to create.
    #[prost(message, optional, tag = "1")]
    pub subscription: ::core::option::Option<Subscription>,
    /// Optional. If set to `true`, validates and previews the request, but doesn't
    /// create the subscription.
    #[prost(bool, tag = "2")]
    pub validate_only: bool,
}
/// The request message for
/// [SubscriptionsService.DeleteSubscription][google.apps.events.subscriptions.v1.SubscriptionsService.DeleteSubscription].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSubscriptionRequest {
    /// Required. Resource name of the subscription to delete.
    ///
    /// Format: `subscriptions/{subscription}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. If set to `true`, validates and previews the request, but doesn't
    /// delete the subscription.
    #[prost(bool, tag = "2")]
    pub validate_only: bool,
    /// Optional. If set to `true` and the subscription isn't found, the request
    /// succeeds but doesn't delete the subscription.
    #[prost(bool, tag = "3")]
    pub allow_missing: bool,
    /// Optional. Etag of the subscription.
    ///
    /// If present, it must match with the server's etag. Otherwise, request
    /// fails with the status `ABORTED`.
    #[prost(string, tag = "4")]
    pub etag: ::prost::alloc::string::String,
}
/// The request message for
/// [SubscriptionsService.GetSubscription][google.apps.events.subscriptions.v1.SubscriptionsService.GetSubscription].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSubscriptionRequest {
    /// Required. Resource name of the subscription.
    ///
    /// Format: `subscriptions/{subscription}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for
/// [SubscriptionsService.UpdateSubscription][google.apps.events.subscriptions.v1.SubscriptionsService.UpdateSubscription].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSubscriptionRequest {
    /// Required. The subscription to update.
    ///
    /// The subscription's `name` field is used to identify the subscription to
    /// update.
    #[prost(message, optional, tag = "1")]
    pub subscription: ::core::option::Option<Subscription>,
    /// Optional. Required. The field to update.
    ///
    /// You can update one of the following fields in a subscription:
    ///
    /// * [`expire_time`][google.apps.events.subscriptions.v1.Subscription.expire_time]: The timestamp when the
    ///    subscription expires.
    /// * [`ttl`][google.apps.events.subscriptions.v1.Subscription.ttl]: The
    /// time-to-live (TTL) or duration of the
    ///    subscription.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Optional. If set to `true`, validates and previews the request, but doesn't
    /// update the subscription.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
}
/// The request message for
/// [SubscriptionsService.ReactivateSubscription][google.apps.events.subscriptions.v1.SubscriptionsService.ReactivateSubscription].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReactivateSubscriptionRequest {
    /// Required. Resource name of the subscription.
    ///
    /// Format: `subscriptions/{subscription}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for
/// [SubscriptionsService.ListSubscriptions][google.apps.events.subscriptions.v1.SubscriptionsService.ListSubscriptions].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSubscriptionsRequest {
    /// Optional. The maximum number of subscriptions to return. The service might
    /// return fewer than this value.
    ///
    /// If unspecified or set to `0`, up to 50 subscriptions are returned.
    ///
    /// The maximum value is 100. If you specify a value more than 100, the system
    /// only returns 100 subscriptions.
    #[prost(int32, tag = "1")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous list subscriptions call.
    /// Provide this parameter to retrieve the subsequent page.
    ///
    /// When paginating, the filter value should match the call that provided the
    /// page token. Passing a different value might lead to unexpected results.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
    /// Required. A query filter.
    ///
    /// You can filter subscriptions by event type (`event_types`)
    /// and target resource (`target_resource`).
    ///
    /// You must specify at least one event type in your query. To filter for
    /// multiple event types, use the `OR` operator.
    ///
    /// To filter by both event type and target resource, use the `AND` operator
    /// and specify the full resource name, such as
    /// `//chat.googleapis.com/spaces/{space}`.
    ///
    /// For example, the following queries are valid:
    ///
    /// ```
    /// event_types:"google.workspace.chat.membership.v1.updated" OR
    ///    event_types:"google.workspace.chat.message.v1.created"
    ///
    /// event_types:"google.workspace.chat.message.v1.created" AND
    ///    target_resource="//chat.googleapis.com/spaces/{space}"
    ///
    /// ( event_types:"google.workspace.chat.membership.v1.updated" OR
    ///    event_types:"google.workspace.chat.message.v1.created" ) AND
    ///    target_resource="//chat.googleapis.com/spaces/{space}"
    /// ```
    ///
    /// The server rejects invalid queries with an `INVALID_ARGUMENT`
    /// error.
    #[prost(string, tag = "3")]
    pub filter: ::prost::alloc::string::String,
}
/// The response message for
/// [SubscriptionsService.ListSubscriptions][google.apps.events.subscriptions.v1.SubscriptionsService.ListSubscriptions].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSubscriptionsResponse {
    /// List of subscriptions.
    #[prost(message, repeated, tag = "1")]
    pub subscriptions: ::prost::alloc::vec::Vec<Subscription>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Metadata for UpdateSubscription LRO.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSubscriptionMetadata {}
/// Metadata for CreateSubscription LRO.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSubscriptionMetadata {}
/// Metadata for DeleteSubscription LRO.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSubscriptionMetadata {}
/// Metadata for ReactivateSubscription LRO.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReactivateSubscriptionMetadata {}
/// Generated client implementations.
pub mod subscriptions_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// A service that manages subscriptions to Google Workspace events.
    #[derive(Debug, Clone)]
    pub struct SubscriptionsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SubscriptionsServiceClient<tonic::transport::Channel> {
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
    impl<T> SubscriptionsServiceClient<T>
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
        ) -> SubscriptionsServiceClient<InterceptedService<T, F>>
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
            SubscriptionsServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a Google Workspace subscription. To learn how to use this
        /// method, see [Create a Google Workspace
        /// subscription](https://developers.google.com/workspace/events/guides/create-subscription).
        pub async fn create_subscription(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSubscriptionRequest>,
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
                "/google.apps.events.subscriptions.v1.SubscriptionsService/CreateSubscription",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.apps.events.subscriptions.v1.SubscriptionsService",
                        "CreateSubscription",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a Google Workspace subscription.
        /// To learn how to use this method, see [Delete a Google Workspace
        /// subscription](https://developers.google.com/workspace/events/guides/delete-subscription).
        pub async fn delete_subscription(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSubscriptionRequest>,
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
                "/google.apps.events.subscriptions.v1.SubscriptionsService/DeleteSubscription",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.apps.events.subscriptions.v1.SubscriptionsService",
                        "DeleteSubscription",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details about a Google Workspace subscription. To learn how to use
        /// this method, see [Get details about a Google Workspace
        /// subscription](https://developers.google.com/workspace/events/guides/get-subscription).
        pub async fn get_subscription(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSubscriptionRequest>,
        ) -> std::result::Result<tonic::Response<super::Subscription>, tonic::Status> {
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
                "/google.apps.events.subscriptions.v1.SubscriptionsService/GetSubscription",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.apps.events.subscriptions.v1.SubscriptionsService",
                        "GetSubscription",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists Google Workspace subscriptions. To learn how to use this method, see
        /// [List Google Workspace
        /// subscriptions](https://developers.google.com/workspace/events/guides/list-subscriptions).
        pub async fn list_subscriptions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSubscriptionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListSubscriptionsResponse>,
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
                "/google.apps.events.subscriptions.v1.SubscriptionsService/ListSubscriptions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.apps.events.subscriptions.v1.SubscriptionsService",
                        "ListSubscriptions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates or renews a Google Workspace subscription. To learn how to use this
        /// method, see [Update or renew a Google Workspace
        /// subscription](https://developers.google.com/workspace/events/guides/update-subscription).
        pub async fn update_subscription(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSubscriptionRequest>,
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
                "/google.apps.events.subscriptions.v1.SubscriptionsService/UpdateSubscription",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.apps.events.subscriptions.v1.SubscriptionsService",
                        "UpdateSubscription",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Reactivates a suspended Google Workspace subscription.
        ///
        /// This method resets your subscription's `State` field to `ACTIVE`. Before
        /// you use this method, you must fix the error that suspended the
        /// subscription. To learn how to use this method, see [Reactivate a Google
        /// Workspace
        /// subscription](https://developers.google.com/workspace/events/guides/reactivate-subscription).
        pub async fn reactivate_subscription(
            &mut self,
            request: impl tonic::IntoRequest<super::ReactivateSubscriptionRequest>,
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
                "/google.apps.events.subscriptions.v1.SubscriptionsService/ReactivateSubscription",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.apps.events.subscriptions.v1.SubscriptionsService",
                        "ReactivateSubscription",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
