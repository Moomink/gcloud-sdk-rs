/// The notification categories that an essential contact can be subscribed to.
/// Each notification will be categorized by the sender into one of the following
/// categories. All contacts that are subscribed to that category will receive
/// the notification.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NotificationCategory {
    /// Notification category is unrecognized or unspecified.
    Unspecified = 0,
    /// All notifications related to the resource, including notifications
    /// pertaining to categories added in the future.
    All = 2,
    /// Notifications related to imminent account suspension.
    Suspension = 3,
    /// Notifications related to security/privacy incidents, notifications, and
    /// vulnerabilities.
    Security = 5,
    /// Notifications related to technical events and issues such as outages,
    /// errors, or bugs.
    Technical = 6,
    /// Notifications related to billing and payments notifications, price updates,
    /// errors, or credits.
    Billing = 7,
    /// Notifications related to enforcement actions, regulatory compliance, or
    /// government notices.
    Legal = 8,
    /// Notifications related to new versions, product terms updates, or
    /// deprecations.
    ProductUpdates = 9,
    /// Child category of TECHNICAL. If assigned, technical incident notifications
    /// will go to these contacts instead of TECHNICAL.
    TechnicalIncidents = 10,
}
/// A contact's validation state indicates whether or not it is the correct
/// contact to be receiving notifications for a particular resource.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ValidationState {
    /// The validation state is unknown or unspecified.
    Unspecified = 0,
    /// The contact is marked as valid. This is usually done manually by the
    /// contact admin. All new contacts begin in the valid state.
    Valid = 1,
    /// The contact is considered invalid. This may become the state if the
    /// contact's email is found to be unreachable.
    Invalid = 2,
}
/// A contact that will receive notifications from Google Cloud.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contact {
    /// The identifier for the contact.
    /// Format: {resource_type}/{resource_id}/contacts/{contact_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The email address to send notifications to. This does not need to
    /// be a Google account.
    #[prost(string, tag = "2")]
    pub email: ::prost::alloc::string::String,
    /// The categories of notifications that the contact will receive
    /// communications for.
    #[prost(enumeration = "NotificationCategory", repeated, tag = "3")]
    pub notification_category_subscriptions: ::prost::alloc::vec::Vec<i32>,
    /// The preferred language for notifications, as a ISO 639-1 language code. See
    /// [Supported
    /// languages](https://cloud.google.com/resource-manager/docs/managing-notification-contacts#supported-languages)
    /// for a list of supported languages.
    #[prost(string, tag = "4")]
    pub language_tag: ::prost::alloc::string::String,
    /// The validity of the contact. A contact is considered valid if it is the
    /// correct recipient for notifications for a particular resource.
    #[prost(enumeration = "ValidationState", tag = "8")]
    pub validation_state: i32,
    /// The last time the validation_state was updated, either manually or
    /// automatically. A contact is considered stale if its validation state was
    /// updated more than 1 year ago.
    #[prost(message, optional, tag = "9")]
    pub validate_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request message for the ListContacts method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListContactsRequest {
    /// Required. The parent resource name.
    /// Format: organizations/{organization_id}, folders/{folder_id} or
    /// projects/{project_id}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of results to return from this request.
    /// Non-positive values are ignored. The presence of `next_page_token` in the
    /// response indicates that more results might be available.
    /// If not specified, the default page_size is 100.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. If present, retrieves the next batch of results from the
    /// preceding call to this method. `page_token` must be the value of
    /// `next_page_token` from the previous response. The values of other method
    /// parameters should be identical to those in the previous call.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for the ListContacts method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListContactsResponse {
    /// The contacts for the specified resource.
    #[prost(message, repeated, tag = "1")]
    pub contacts: ::prost::alloc::vec::Vec<Contact>,
    /// If there are more results than those appearing in this response, then
    /// `next_page_token` is included. To get the next set of results, call this
    /// method again using the value of `next_page_token` as `page_token` and the
    /// rest of the parameters the same as the original request.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for the GetContact method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetContactRequest {
    /// Required. The name of the contact to retrieve.
    /// Format: organizations/{organization_id}/contacts/{contact_id},
    /// folders/{folder_id}/contacts/{contact_id} or
    /// projects/{project_id}/contacts/{contact_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for the DeleteContact method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteContactRequest {
    /// Required. The name of the contact to delete.
    /// Format: organizations/{organization_id}/contacts/{contact_id},
    /// folders/{folder_id}/contacts/{contact_id} or
    /// projects/{project_id}/contacts/{contact_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for the CreateContact method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateContactRequest {
    /// Required. The resource to save this contact for.
    /// Format: organizations/{organization_id}, folders/{folder_id} or
    /// projects/{project_id}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The contact to create. Must specify an email address and language
    /// tag.
    #[prost(message, optional, tag = "2")]
    pub contact: ::core::option::Option<Contact>,
}
/// Request message for the UpdateContact method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateContactRequest {
    /// Required. The contact resource to replace the existing saved contact. Note:
    /// the email address of the contact cannot be modified.
    #[prost(message, optional, tag = "2")]
    pub contact: ::core::option::Option<Contact>,
    /// Optional. The update mask applied to the resource. For the `FieldMask`
    /// definition, see
    /// https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for the ComputeContacts method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputeContactsRequest {
    /// Required. The name of the resource to compute contacts for.
    /// Format: organizations/{organization_id},
    /// folders/{folder_id} or projects/{project_id}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The categories of notifications to compute contacts for. If ALL is included
    /// in this list, contacts subscribed to any notification category will be
    /// returned.
    #[prost(enumeration = "NotificationCategory", repeated, tag = "6")]
    pub notification_categories: ::prost::alloc::vec::Vec<i32>,
    /// Optional. The maximum number of results to return from this request.
    /// Non-positive values are ignored. The presence of `next_page_token` in the
    /// response indicates that more results might be available.
    /// If not specified, the default page_size is 100.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Optional. If present, retrieves the next batch of results from the
    /// preceding call to this method. `page_token` must be the value of
    /// `next_page_token` from the previous response. The values of other method
    /// parameters should be identical to those in the previous call.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for the ComputeContacts method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputeContactsResponse {
    /// All contacts for the resource that are subscribed to the specified
    /// notification categories, including contacts inherited from any parent
    /// resources.
    #[prost(message, repeated, tag = "1")]
    pub contacts: ::prost::alloc::vec::Vec<Contact>,
    /// If there are more results than those appearing in this response, then
    /// `next_page_token` is included. To get the next set of results, call this
    /// method again using the value of `next_page_token` as `page_token` and the
    /// rest of the parameters the same as the original request.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for the SendTestMessage method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendTestMessageRequest {
    /// Required. The list of names of the contacts to send a test message to.
    /// Format: organizations/{organization_id}/contacts/{contact_id},
    /// folders/{folder_id}/contacts/{contact_id} or
    /// projects/{project_id}/contacts/{contact_id}
    #[prost(string, repeated, tag = "1")]
    pub contacts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. The name of the resource to send the test message for. All
    /// contacts must either be set directly on this resource or inherited from
    /// another resource that is an ancestor of this one. Format:
    /// organizations/{organization_id}, folders/{folder_id} or
    /// projects/{project_id}
    #[prost(string, tag = "2")]
    pub resource: ::prost::alloc::string::String,
    /// Required. The notification category to send the test message for. All
    /// contacts must be subscribed to this category.
    #[prost(enumeration = "NotificationCategory", tag = "3")]
    pub notification_category: i32,
}
#[doc = r" Generated client implementations."]
pub mod essential_contacts_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Manages contacts for important Google Cloud notifications."]
    #[derive(Debug, Clone)]
    pub struct EssentialContactsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> EssentialContactsServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> EssentialContactsServiceClient<InterceptedService<T, F>>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            EssentialContactsServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Adds a new contact for a resource."]
        pub async fn create_contact(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateContactRequest>,
        ) -> Result<tonic::Response<super::Contact>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.essentialcontacts.v1.EssentialContactsService/CreateContact",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a contact."]
        #[doc = " Note: A contact's email address cannot be changed."]
        pub async fn update_contact(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateContactRequest>,
        ) -> Result<tonic::Response<super::Contact>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.essentialcontacts.v1.EssentialContactsService/UpdateContact",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the contacts that have been set on a resource."]
        pub async fn list_contacts(
            &mut self,
            request: impl tonic::IntoRequest<super::ListContactsRequest>,
        ) -> Result<tonic::Response<super::ListContactsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.essentialcontacts.v1.EssentialContactsService/ListContacts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a single contact."]
        pub async fn get_contact(
            &mut self,
            request: impl tonic::IntoRequest<super::GetContactRequest>,
        ) -> Result<tonic::Response<super::Contact>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.essentialcontacts.v1.EssentialContactsService/GetContact",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a contact."]
        pub async fn delete_contact(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteContactRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.essentialcontacts.v1.EssentialContactsService/DeleteContact",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all contacts for the resource that are subscribed to the"]
        #[doc = " specified notification categories, including contacts inherited from"]
        #[doc = " any parent resources."]
        pub async fn compute_contacts(
            &mut self,
            request: impl tonic::IntoRequest<super::ComputeContactsRequest>,
        ) -> Result<tonic::Response<super::ComputeContactsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.essentialcontacts.v1.EssentialContactsService/ComputeContacts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Allows a contact admin to send a test message to contact to verify that it"]
        #[doc = " has been configured correctly."]
        pub async fn send_test_message(
            &mut self,
            request: impl tonic::IntoRequest<super::SendTestMessageRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.essentialcontacts.v1.EssentialContactsService/SendTestMessage",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
