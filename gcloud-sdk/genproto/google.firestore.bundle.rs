/// Encodes a query saved in the bundle.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BundledQuery {
    /// The parent resource name.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    #[prost(enumeration="bundled_query::LimitType", tag="3")]
    pub limit_type: i32,
    /// The query to run.
    #[prost(oneof="bundled_query::QueryType", tags="2")]
    pub query_type: ::core::option::Option<bundled_query::QueryType>,
}
/// Nested message and enum types in `BundledQuery`.
pub mod bundled_query {
    /// If the query is a limit query, should the limit be applied to the beginning or
    /// the end of results.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum LimitType {
        First = 0,
        Last = 1,
    }
    /// The query to run.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum QueryType {
        /// A structured query.
        #[prost(message, tag="2")]
        StructuredQuery(super::super::v1::StructuredQuery),
    }
}
/// A Query associated with a name, created as part of the bundle file, and can be read
/// by client SDKs once the bundle containing them is loaded.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NamedQuery {
    /// Name of the query, such that client can use the name to load this query
    /// from bundle, and resume from when the query results are materialized
    /// into this bundle.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The query saved in the bundle.
    #[prost(message, optional, tag="2")]
    pub bundled_query: ::core::option::Option<BundledQuery>,
    /// The read time of the query, when it is used to build the bundle. This is useful to
    /// resume the query from the bundle, once it is loaded by client SDKs.
    #[prost(message, optional, tag="3")]
    pub read_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Metadata describing a Firestore document saved in the bundle.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BundledDocumentMetadata {
    /// The document key of a bundled document.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The snapshot version of the document data bundled.
    #[prost(message, optional, tag="2")]
    pub read_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Whether the document exists.
    #[prost(bool, tag="3")]
    pub exists: bool,
    /// The names of the queries in this bundle that this document matches to.
    #[prost(string, repeated, tag="4")]
    pub queries: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Metadata describing the bundle file/stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BundleMetadata {
    /// The ID of the bundle.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// Time at which the documents snapshot is taken for this bundle.
    #[prost(message, optional, tag="2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The schema version of the bundle.
    #[prost(uint32, tag="3")]
    pub version: u32,
    /// The number of documents in the bundle.
    #[prost(uint32, tag="4")]
    pub total_documents: u32,
    /// The size of the bundle in bytes, excluding this `BundleMetadata`.
    #[prost(uint64, tag="5")]
    pub total_bytes: u64,
}
/// A Firestore bundle is a length-prefixed stream of JSON representations of
/// `BundleElement`.
/// Only one `BundleMetadata` is expected, and it should be the first element.
/// The named queries follow after `metadata`. Every `document_metadata` is
/// immediately followed by a `document`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BundleElement {
    #[prost(oneof="bundle_element::ElementType", tags="1, 2, 3, 4")]
    pub element_type: ::core::option::Option<bundle_element::ElementType>,
}
/// Nested message and enum types in `BundleElement`.
pub mod bundle_element {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ElementType {
        #[prost(message, tag="1")]
        Metadata(super::BundleMetadata),
        #[prost(message, tag="2")]
        NamedQuery(super::NamedQuery),
        #[prost(message, tag="3")]
        DocumentMetadata(super::BundledDocumentMetadata),
        #[prost(message, tag="4")]
        Document(super::super::v1::Document),
    }
}
