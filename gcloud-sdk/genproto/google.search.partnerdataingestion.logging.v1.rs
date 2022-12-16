/// Log message used to send to Platform Logging.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngestDetailsLog {
    /// Identification of the successfully accepted request.
    #[prost(string, tag = "1")]
    pub ingestion_tracking_id: ::prost::alloc::string::String,
    /// The message content will be sent to Platform Logging.
    #[prost(string, tag = "2")]
    pub content: ::prost::alloc::string::String,
}
