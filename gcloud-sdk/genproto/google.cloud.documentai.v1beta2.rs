/// Encodes the detailed information of a barcode.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Barcode {
    /// Format of a barcode.
    /// The supported formats are:
    ///
    /// - `CODE_128`: Code 128 type.
    /// - `CODE_39`: Code 39 type.
    /// - `CODE_93`: Code 93 type.
    /// - `CODABAR`: Codabar type.
    /// - `DATA_MATRIX`: 2D Data Matrix type.
    /// - `ITF`: ITF type.
    /// - `EAN_13`: EAN-13 type.
    /// - `EAN_8`: EAN-8 type.
    /// - `QR_CODE`: 2D QR code type.
    /// - `UPC_A`: UPC-A type.
    /// - `UPC_E`: UPC-E type.
    /// - `PDF417`: PDF417 type.
    /// - `AZTEC`: 2D Aztec code type.
    /// - `DATABAR`: GS1 DataBar code type.
    #[prost(string, tag = "1")]
    pub format: ::prost::alloc::string::String,
    /// Value format describes the format of the value that a barcode
    /// encodes.
    /// The supported formats are:
    ///
    /// - `CONTACT_INFO`: Contact information.
    /// - `EMAIL`: Email address.
    /// - `ISBN`: ISBN identifier.
    /// - `PHONE`: Phone number.
    /// - `PRODUCT`: Product.
    /// - `SMS`: SMS message.
    /// - `TEXT`: Text string.
    /// - `URL`: URL address.
    /// - `WIFI`: Wifi information.
    /// - `GEO`: Geo-localization.
    /// - `CALENDAR_EVENT`: Calendar event.
    /// - `DRIVER_LICENSE`: Driver's license.
    #[prost(string, tag = "2")]
    pub value_format: ::prost::alloc::string::String,
    /// Raw value encoded in the barcode.
    /// For example: `'MEBKM:TITLE:Google;URL:<https://www.google.com;;'`.>
    #[prost(string, tag = "3")]
    pub raw_value: ::prost::alloc::string::String,
}
/// A vertex represents a 2D point in the image.
/// NOTE: the vertex coordinates are in the same scale as the original image.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vertex {
    /// X coordinate.
    #[prost(int32, tag = "1")]
    pub x: i32,
    /// Y coordinate (starts from the top of the image).
    #[prost(int32, tag = "2")]
    pub y: i32,
}
/// A vertex represents a 2D point in the image.
/// NOTE: the normalized vertex coordinates are relative to the original image
/// and range from 0 to 1.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NormalizedVertex {
    /// X coordinate.
    #[prost(float, tag = "1")]
    pub x: f32,
    /// Y coordinate (starts from the top of the image).
    #[prost(float, tag = "2")]
    pub y: f32,
}
/// A bounding polygon for the detected image annotation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoundingPoly {
    /// The bounding polygon vertices.
    #[prost(message, repeated, tag = "1")]
    pub vertices: ::prost::alloc::vec::Vec<Vertex>,
    /// The bounding polygon normalized vertices.
    #[prost(message, repeated, tag = "2")]
    pub normalized_vertices: ::prost::alloc::vec::Vec<NormalizedVertex>,
}
/// Document represents the canonical document resource in Document AI. It is an
/// interchange format that provides insights into documents and allows for
/// collaboration between users and Document AI to iterate and optimize for
/// quality.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Document {
    /// An IANA published [media type (MIME
    /// type)](<https://www.iana.org/assignments/media-types/media-types.xhtml>).
    #[prost(string, tag = "3")]
    pub mime_type: ::prost::alloc::string::String,
    /// Optional. UTF-8 encoded text in reading order from the document.
    #[prost(string, tag = "4")]
    pub text: ::prost::alloc::string::String,
    /// Styles for the
    /// [Document.text][google.cloud.documentai.v1beta2.Document.text].
    #[deprecated]
    #[prost(message, repeated, tag = "5")]
    pub text_styles: ::prost::alloc::vec::Vec<document::Style>,
    /// Visual page layout for the
    /// [Document][google.cloud.documentai.v1beta2.Document].
    #[prost(message, repeated, tag = "6")]
    pub pages: ::prost::alloc::vec::Vec<document::Page>,
    /// A list of entities detected on
    /// [Document.text][google.cloud.documentai.v1beta2.Document.text]. For
    /// document shards, entities in this list may cross shard boundaries.
    #[prost(message, repeated, tag = "7")]
    pub entities: ::prost::alloc::vec::Vec<document::Entity>,
    /// Placeholder.  Relationship among
    /// [Document.entities][google.cloud.documentai.v1beta2.Document.entities].
    #[prost(message, repeated, tag = "8")]
    pub entity_relations: ::prost::alloc::vec::Vec<document::EntityRelation>,
    /// Placeholder.  A list of text corrections made to
    /// [Document.text][google.cloud.documentai.v1beta2.Document.text].  This is
    /// usually used for annotating corrections to OCR mistakes.  Text changes for
    /// a given revision may not overlap with each other.
    #[prost(message, repeated, tag = "14")]
    pub text_changes: ::prost::alloc::vec::Vec<document::TextChange>,
    /// Information about the sharding if this document is sharded part of a larger
    /// document. If the document is not sharded, this message is not specified.
    #[prost(message, optional, tag = "9")]
    pub shard_info: ::core::option::Option<document::ShardInfo>,
    /// [Label][google.cloud.documentai.v1beta2.Document.Label]s for this document.
    #[prost(message, repeated, tag = "11")]
    pub labels: ::prost::alloc::vec::Vec<document::Label>,
    /// Any error that occurred while processing this document.
    #[prost(message, optional, tag = "10")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
    /// Placeholder. Revision history of this document.
    #[prost(message, repeated, tag = "13")]
    pub revisions: ::prost::alloc::vec::Vec<document::Revision>,
    /// Original source document from the user.
    #[prost(oneof = "document::Source", tags = "1, 2")]
    pub source: ::core::option::Option<document::Source>,
}
/// Nested message and enum types in `Document`.
pub mod document {
    /// For a large document, sharding may be performed to produce several
    /// document shards. Each document shard contains this field to detail which
    /// shard it is.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ShardInfo {
        /// The 0-based index of this shard.
        #[prost(int64, tag = "1")]
        pub shard_index: i64,
        /// Total number of shards.
        #[prost(int64, tag = "2")]
        pub shard_count: i64,
        /// The index of the first character in
        /// [Document.text][google.cloud.documentai.v1beta2.Document.text] in the
        /// overall document global text.
        #[prost(int64, tag = "3")]
        pub text_offset: i64,
    }
    /// Label attaches schema information and/or other metadata to segments within
    /// a [Document][google.cloud.documentai.v1beta2.Document]. Multiple
    /// [Label][google.cloud.documentai.v1beta2.Document.Label]s on a single field
    /// can denote either different labels, different instances of the same label
    /// created at different times, or some combination of both.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Label {
        /// Name of the label.
        ///
        /// When the label is generated from AutoML Text Classification model, this
        /// field represents the name of the category.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// Confidence score between 0 and 1 for label assignment.
        #[prost(float, tag = "3")]
        pub confidence: f32,
        /// Provenance of the label.
        #[prost(oneof = "label::Source", tags = "2")]
        pub source: ::core::option::Option<label::Source>,
    }
    /// Nested message and enum types in `Label`.
    pub mod label {
        /// Provenance of the label.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Source {
            /// Label is generated AutoML model. This field stores the full resource
            /// name of the AutoML model.
            ///
            /// Format:
            /// `projects/{project-id}/locations/{location-id}/models/{model-id}`
            #[prost(string, tag = "2")]
            AutomlModel(::prost::alloc::string::String),
        }
    }
    /// Annotation for common text style attributes. This adheres to CSS
    /// conventions as much as possible.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Style {
        /// Text anchor indexing into the
        /// [Document.text][google.cloud.documentai.v1beta2.Document.text].
        #[prost(message, optional, tag = "1")]
        pub text_anchor: ::core::option::Option<TextAnchor>,
        /// Text color.
        #[prost(message, optional, tag = "2")]
        pub color: ::core::option::Option<super::super::super::super::r#type::Color>,
        /// Text background color.
        #[prost(message, optional, tag = "3")]
        pub background_color: ::core::option::Option<
            super::super::super::super::r#type::Color,
        >,
        /// [Font weight](<https://www.w3schools.com/cssref/pr_font_weight.asp>).
        /// Possible values are `normal`, `bold`, `bolder`, and `lighter`.
        #[prost(string, tag = "4")]
        pub font_weight: ::prost::alloc::string::String,
        /// [Text style](<https://www.w3schools.com/cssref/pr_font_font-style.asp>).
        /// Possible values are `normal`, `italic`, and `oblique`.
        #[prost(string, tag = "5")]
        pub text_style: ::prost::alloc::string::String,
        /// [Text
        /// decoration](<https://www.w3schools.com/cssref/pr_text_text-decoration.asp>).
        /// Follows CSS standard. <text-decoration-line> <text-decoration-color>
        /// <text-decoration-style>
        #[prost(string, tag = "6")]
        pub text_decoration: ::prost::alloc::string::String,
        /// Font size.
        #[prost(message, optional, tag = "7")]
        pub font_size: ::core::option::Option<style::FontSize>,
        /// Font family such as `Arial`, `Times New Roman`.
        /// <https://www.w3schools.com/cssref/pr_font_font-family.asp>
        #[prost(string, tag = "8")]
        pub font_family: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `Style`.
    pub mod style {
        /// Font size with unit.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct FontSize {
            /// Font size for the text.
            #[prost(float, tag = "1")]
            pub size: f32,
            /// Unit for the font size. Follows CSS naming (such as `in`, `px`, and
            /// `pt`).
            #[prost(string, tag = "2")]
            pub unit: ::prost::alloc::string::String,
        }
    }
    /// A page in a [Document][google.cloud.documentai.v1beta2.Document].
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Page {
        /// 1-based index for current
        /// [Page][google.cloud.documentai.v1beta2.Document.Page] in a parent
        /// [Document][google.cloud.documentai.v1beta2.Document]. Useful when a page
        /// is taken out of a [Document][google.cloud.documentai.v1beta2.Document]
        /// for individual processing.
        #[prost(int32, tag = "1")]
        pub page_number: i32,
        /// Rendered image for this page. This image is preprocessed to remove any
        /// skew, rotation, and distortions such that the annotation bounding boxes
        /// can be upright and axis-aligned.
        #[prost(message, optional, tag = "13")]
        pub image: ::core::option::Option<page::Image>,
        /// Transformation matrices that were applied to the original document image
        /// to produce
        /// [Page.image][google.cloud.documentai.v1beta2.Document.Page.image].
        #[prost(message, repeated, tag = "14")]
        pub transforms: ::prost::alloc::vec::Vec<page::Matrix>,
        /// Physical dimension of the page.
        #[prost(message, optional, tag = "2")]
        pub dimension: ::core::option::Option<page::Dimension>,
        /// [Layout][google.cloud.documentai.v1beta2.Document.Page.Layout] for the
        /// page.
        #[prost(message, optional, tag = "3")]
        pub layout: ::core::option::Option<page::Layout>,
        /// A list of detected languages together with confidence.
        #[prost(message, repeated, tag = "4")]
        pub detected_languages: ::prost::alloc::vec::Vec<page::DetectedLanguage>,
        /// A list of visually detected text blocks on the page.
        /// A block has a set of lines (collected into paragraphs) that have a common
        /// line-spacing and orientation.
        #[prost(message, repeated, tag = "5")]
        pub blocks: ::prost::alloc::vec::Vec<page::Block>,
        /// A list of visually detected text paragraphs on the page.
        /// A collection of lines that a human would perceive as a paragraph.
        #[prost(message, repeated, tag = "6")]
        pub paragraphs: ::prost::alloc::vec::Vec<page::Paragraph>,
        /// A list of visually detected text lines on the page.
        /// A collection of tokens that a human would perceive as a line.
        #[prost(message, repeated, tag = "7")]
        pub lines: ::prost::alloc::vec::Vec<page::Line>,
        /// A list of visually detected tokens on the page.
        #[prost(message, repeated, tag = "8")]
        pub tokens: ::prost::alloc::vec::Vec<page::Token>,
        /// A list of detected non-text visual elements e.g. checkbox,
        /// signature etc. on the page.
        #[prost(message, repeated, tag = "9")]
        pub visual_elements: ::prost::alloc::vec::Vec<page::VisualElement>,
        /// A list of visually detected tables on the page.
        #[prost(message, repeated, tag = "10")]
        pub tables: ::prost::alloc::vec::Vec<page::Table>,
        /// A list of visually detected form fields on the page.
        #[prost(message, repeated, tag = "11")]
        pub form_fields: ::prost::alloc::vec::Vec<page::FormField>,
        /// A list of visually detected symbols on the page.
        #[prost(message, repeated, tag = "12")]
        pub symbols: ::prost::alloc::vec::Vec<page::Symbol>,
        /// A list of detected barcodes.
        #[prost(message, repeated, tag = "15")]
        pub detected_barcodes: ::prost::alloc::vec::Vec<page::DetectedBarcode>,
        /// Image quality scores.
        #[prost(message, optional, tag = "17")]
        pub image_quality_scores: ::core::option::Option<page::ImageQualityScores>,
        /// The history of this page.
        #[deprecated]
        #[prost(message, optional, tag = "16")]
        pub provenance: ::core::option::Option<Provenance>,
    }
    /// Nested message and enum types in `Page`.
    pub mod page {
        /// Dimension for the page.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Dimension {
            /// Page width.
            #[prost(float, tag = "1")]
            pub width: f32,
            /// Page height.
            #[prost(float, tag = "2")]
            pub height: f32,
            /// Dimension unit.
            #[prost(string, tag = "3")]
            pub unit: ::prost::alloc::string::String,
        }
        /// Rendered image contents for this page.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Image {
            /// Raw byte content of the image.
            #[prost(bytes = "vec", tag = "1")]
            pub content: ::prost::alloc::vec::Vec<u8>,
            /// Encoding [media type (MIME
            /// type)](<https://www.iana.org/assignments/media-types/media-types.xhtml>)
            /// for the image.
            #[prost(string, tag = "2")]
            pub mime_type: ::prost::alloc::string::String,
            /// Width of the image in pixels.
            #[prost(int32, tag = "3")]
            pub width: i32,
            /// Height of the image in pixels.
            #[prost(int32, tag = "4")]
            pub height: i32,
        }
        /// Representation for transformation matrix, intended to be compatible and
        /// used with OpenCV format for image manipulation.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Matrix {
            /// Number of rows in the matrix.
            #[prost(int32, tag = "1")]
            pub rows: i32,
            /// Number of columns in the matrix.
            #[prost(int32, tag = "2")]
            pub cols: i32,
            /// This encodes information about what data type the matrix uses.
            /// For example, 0 (CV_8U) is an unsigned 8-bit image. For the full list
            /// of OpenCV primitive data types, please refer to
            /// <https://docs.opencv.org/4.3.0/d1/d1b/group__core__hal__interface.html>
            #[prost(int32, tag = "3")]
            pub r#type: i32,
            /// The matrix data.
            #[prost(bytes = "vec", tag = "4")]
            pub data: ::prost::alloc::vec::Vec<u8>,
        }
        /// Visual element describing a layout unit on a page.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Layout {
            /// Text anchor indexing into the
            /// [Document.text][google.cloud.documentai.v1beta2.Document.text].
            #[prost(message, optional, tag = "1")]
            pub text_anchor: ::core::option::Option<super::TextAnchor>,
            /// Confidence of the current
            /// [Layout][google.cloud.documentai.v1beta2.Document.Page.Layout] within
            /// context of the object this layout is for. e.g. confidence can be for a
            /// single token, a table, a visual element, etc. depending on context.
            /// Range `\[0, 1\]`.
            #[prost(float, tag = "2")]
            pub confidence: f32,
            /// The bounding polygon for the
            /// [Layout][google.cloud.documentai.v1beta2.Document.Page.Layout].
            #[prost(message, optional, tag = "3")]
            pub bounding_poly: ::core::option::Option<super::super::BoundingPoly>,
            /// Detected orientation for the
            /// [Layout][google.cloud.documentai.v1beta2.Document.Page.Layout].
            #[prost(enumeration = "layout::Orientation", tag = "4")]
            pub orientation: i32,
        }
        /// Nested message and enum types in `Layout`.
        pub mod layout {
            /// Detected human reading orientation.
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
            pub enum Orientation {
                /// Unspecified orientation.
                Unspecified = 0,
                /// Orientation is aligned with page up.
                PageUp = 1,
                /// Orientation is aligned with page right.
                /// Turn the head 90 degrees clockwise from upright to read.
                PageRight = 2,
                /// Orientation is aligned with page down.
                /// Turn the head 180 degrees from upright to read.
                PageDown = 3,
                /// Orientation is aligned with page left.
                /// Turn the head 90 degrees counterclockwise from upright to read.
                PageLeft = 4,
            }
            impl Orientation {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        Orientation::Unspecified => "ORIENTATION_UNSPECIFIED",
                        Orientation::PageUp => "PAGE_UP",
                        Orientation::PageRight => "PAGE_RIGHT",
                        Orientation::PageDown => "PAGE_DOWN",
                        Orientation::PageLeft => "PAGE_LEFT",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "ORIENTATION_UNSPECIFIED" => Some(Self::Unspecified),
                        "PAGE_UP" => Some(Self::PageUp),
                        "PAGE_RIGHT" => Some(Self::PageRight),
                        "PAGE_DOWN" => Some(Self::PageDown),
                        "PAGE_LEFT" => Some(Self::PageLeft),
                        _ => None,
                    }
                }
            }
        }
        /// A block has a set of lines (collected into paragraphs) that have a
        /// common line-spacing and orientation.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Block {
            /// [Layout][google.cloud.documentai.v1beta2.Document.Page.Layout] for
            /// [Block][google.cloud.documentai.v1beta2.Document.Page.Block].
            #[prost(message, optional, tag = "1")]
            pub layout: ::core::option::Option<Layout>,
            /// A list of detected languages together with confidence.
            #[prost(message, repeated, tag = "2")]
            pub detected_languages: ::prost::alloc::vec::Vec<DetectedLanguage>,
            /// The history of this annotation.
            #[deprecated]
            #[prost(message, optional, tag = "3")]
            pub provenance: ::core::option::Option<super::Provenance>,
        }
        /// A collection of lines that a human would perceive as a paragraph.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Paragraph {
            /// [Layout][google.cloud.documentai.v1beta2.Document.Page.Layout] for
            /// [Paragraph][google.cloud.documentai.v1beta2.Document.Page.Paragraph].
            #[prost(message, optional, tag = "1")]
            pub layout: ::core::option::Option<Layout>,
            /// A list of detected languages together with confidence.
            #[prost(message, repeated, tag = "2")]
            pub detected_languages: ::prost::alloc::vec::Vec<DetectedLanguage>,
            /// The  history of this annotation.
            #[deprecated]
            #[prost(message, optional, tag = "3")]
            pub provenance: ::core::option::Option<super::Provenance>,
        }
        /// A collection of tokens that a human would perceive as a line.
        /// Does not cross column boundaries, can be horizontal, vertical, etc.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Line {
            /// [Layout][google.cloud.documentai.v1beta2.Document.Page.Layout] for
            /// [Line][google.cloud.documentai.v1beta2.Document.Page.Line].
            #[prost(message, optional, tag = "1")]
            pub layout: ::core::option::Option<Layout>,
            /// A list of detected languages together with confidence.
            #[prost(message, repeated, tag = "2")]
            pub detected_languages: ::prost::alloc::vec::Vec<DetectedLanguage>,
            /// The  history of this annotation.
            #[deprecated]
            #[prost(message, optional, tag = "3")]
            pub provenance: ::core::option::Option<super::Provenance>,
        }
        /// A detected token.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Token {
            /// [Layout][google.cloud.documentai.v1beta2.Document.Page.Layout] for
            /// [Token][google.cloud.documentai.v1beta2.Document.Page.Token].
            #[prost(message, optional, tag = "1")]
            pub layout: ::core::option::Option<Layout>,
            /// Detected break at the end of a
            /// [Token][google.cloud.documentai.v1beta2.Document.Page.Token].
            #[prost(message, optional, tag = "2")]
            pub detected_break: ::core::option::Option<token::DetectedBreak>,
            /// A list of detected languages together with confidence.
            #[prost(message, repeated, tag = "3")]
            pub detected_languages: ::prost::alloc::vec::Vec<DetectedLanguage>,
            /// The history of this annotation.
            #[deprecated]
            #[prost(message, optional, tag = "4")]
            pub provenance: ::core::option::Option<super::Provenance>,
            /// Text style attributes.
            #[prost(message, optional, tag = "5")]
            pub style_info: ::core::option::Option<token::StyleInfo>,
        }
        /// Nested message and enum types in `Token`.
        pub mod token {
            /// Detected break at the end of a
            /// [Token][google.cloud.documentai.v1beta2.Document.Page.Token].
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct DetectedBreak {
                /// Detected break type.
                #[prost(enumeration = "detected_break::Type", tag = "1")]
                pub r#type: i32,
            }
            /// Nested message and enum types in `DetectedBreak`.
            pub mod detected_break {
                /// Enum to denote the type of break found.
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
                    /// Unspecified break type.
                    Unspecified = 0,
                    /// A single whitespace.
                    Space = 1,
                    /// A wider whitespace.
                    WideSpace = 2,
                    /// A hyphen that indicates that a token has been split across lines.
                    Hyphen = 3,
                }
                impl Type {
                    /// String value of the enum field names used in the ProtoBuf definition.
                    ///
                    /// The values are not transformed in any way and thus are considered stable
                    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                    pub fn as_str_name(&self) -> &'static str {
                        match self {
                            Type::Unspecified => "TYPE_UNSPECIFIED",
                            Type::Space => "SPACE",
                            Type::WideSpace => "WIDE_SPACE",
                            Type::Hyphen => "HYPHEN",
                        }
                    }
                    /// Creates an enum from field names used in the ProtoBuf definition.
                    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                        match value {
                            "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                            "SPACE" => Some(Self::Space),
                            "WIDE_SPACE" => Some(Self::WideSpace),
                            "HYPHEN" => Some(Self::Hyphen),
                            _ => None,
                        }
                    }
                }
            }
            /// Font and other text style attributes.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct StyleInfo {
                /// Font size in points (`1` point is `¹⁄₇₂` inches).
                #[prost(int32, tag = "1")]
                pub font_size: i32,
                /// Font size in pixels, equal to _unrounded
                /// [font_size][google.cloud.documentai.v1beta2.Document.Page.Token.StyleInfo.font_size]_
                /// * _resolution_ ÷ `72.0`.
                #[prost(double, tag = "2")]
                pub pixel_font_size: f64,
                /// Letter spacing in points.
                #[prost(double, tag = "3")]
                pub letter_spacing: f64,
                /// Name or style of the font.
                #[prost(string, tag = "4")]
                pub font_type: ::prost::alloc::string::String,
                /// Whether the text is bold (equivalent to
                /// [font_weight][google.cloud.documentai.v1beta2.Document.Page.Token.StyleInfo.font_weight]
                /// is at least `700`).
                #[prost(bool, tag = "5")]
                pub bold: bool,
                /// Whether the text is italic.
                #[prost(bool, tag = "6")]
                pub italic: bool,
                /// Whether the text is underlined.
                #[prost(bool, tag = "7")]
                pub underlined: bool,
                /// Whether the text is strikethrough.
                #[prost(bool, tag = "8")]
                pub strikeout: bool,
                /// Whether the text is a subscript.
                #[prost(bool, tag = "9")]
                pub subscript: bool,
                /// Whether the text is a superscript.
                #[prost(bool, tag = "10")]
                pub superscript: bool,
                /// Whether the text is in small caps.
                #[prost(bool, tag = "11")]
                pub smallcaps: bool,
                /// TrueType weight on a scale `100` (thin) to `1000` (ultra-heavy).
                /// Normal is `400`, bold is `700`.
                #[prost(int32, tag = "12")]
                pub font_weight: i32,
                /// Whether the text is handwritten.
                #[prost(bool, tag = "13")]
                pub handwritten: bool,
                /// Color of the text.
                #[prost(message, optional, tag = "14")]
                pub text_color: ::core::option::Option<
                    super::super::super::super::super::super::r#type::Color,
                >,
                /// Color of the background.
                #[prost(message, optional, tag = "15")]
                pub background_color: ::core::option::Option<
                    super::super::super::super::super::super::r#type::Color,
                >,
            }
        }
        /// A detected symbol.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Symbol {
            /// [Layout][google.cloud.documentai.v1beta2.Document.Page.Layout] for
            /// [Symbol][google.cloud.documentai.v1beta2.Document.Page.Symbol].
            #[prost(message, optional, tag = "1")]
            pub layout: ::core::option::Option<Layout>,
            /// A list of detected languages together with confidence.
            #[prost(message, repeated, tag = "2")]
            pub detected_languages: ::prost::alloc::vec::Vec<DetectedLanguage>,
        }
        /// Detected non-text visual elements e.g. checkbox, signature etc. on the
        /// page.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct VisualElement {
            /// [Layout][google.cloud.documentai.v1beta2.Document.Page.Layout] for
            /// [VisualElement][google.cloud.documentai.v1beta2.Document.Page.VisualElement].
            #[prost(message, optional, tag = "1")]
            pub layout: ::core::option::Option<Layout>,
            /// Type of the
            /// [VisualElement][google.cloud.documentai.v1beta2.Document.Page.VisualElement].
            #[prost(string, tag = "2")]
            pub r#type: ::prost::alloc::string::String,
            /// A list of detected languages together with confidence.
            #[prost(message, repeated, tag = "3")]
            pub detected_languages: ::prost::alloc::vec::Vec<DetectedLanguage>,
        }
        /// A table representation similar to HTML table structure.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Table {
            /// [Layout][google.cloud.documentai.v1beta2.Document.Page.Layout] for
            /// [Table][google.cloud.documentai.v1beta2.Document.Page.Table].
            #[prost(message, optional, tag = "1")]
            pub layout: ::core::option::Option<Layout>,
            /// Header rows of the table.
            #[prost(message, repeated, tag = "2")]
            pub header_rows: ::prost::alloc::vec::Vec<table::TableRow>,
            /// Body rows of the table.
            #[prost(message, repeated, tag = "3")]
            pub body_rows: ::prost::alloc::vec::Vec<table::TableRow>,
            /// A list of detected languages together with confidence.
            #[prost(message, repeated, tag = "4")]
            pub detected_languages: ::prost::alloc::vec::Vec<DetectedLanguage>,
            /// The history of this table.
            #[deprecated]
            #[prost(message, optional, tag = "5")]
            pub provenance: ::core::option::Option<super::Provenance>,
        }
        /// Nested message and enum types in `Table`.
        pub mod table {
            /// A row of table cells.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct TableRow {
                /// Cells that make up this row.
                #[prost(message, repeated, tag = "1")]
                pub cells: ::prost::alloc::vec::Vec<TableCell>,
            }
            /// A cell representation inside the table.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct TableCell {
                /// [Layout][google.cloud.documentai.v1beta2.Document.Page.Layout] for
                /// [TableCell][google.cloud.documentai.v1beta2.Document.Page.Table.TableCell].
                #[prost(message, optional, tag = "1")]
                pub layout: ::core::option::Option<super::Layout>,
                /// How many rows this cell spans.
                #[prost(int32, tag = "2")]
                pub row_span: i32,
                /// How many columns this cell spans.
                #[prost(int32, tag = "3")]
                pub col_span: i32,
                /// A list of detected languages together with confidence.
                #[prost(message, repeated, tag = "4")]
                pub detected_languages: ::prost::alloc::vec::Vec<
                    super::DetectedLanguage,
                >,
            }
        }
        /// A form field detected on the page.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct FormField {
            /// [Layout][google.cloud.documentai.v1beta2.Document.Page.Layout] for the
            /// [FormField][google.cloud.documentai.v1beta2.Document.Page.FormField]
            /// name. e.g. `Address`, `Email`, `Grand total`, `Phone number`, etc.
            #[prost(message, optional, tag = "1")]
            pub field_name: ::core::option::Option<Layout>,
            /// [Layout][google.cloud.documentai.v1beta2.Document.Page.Layout] for the
            /// [FormField][google.cloud.documentai.v1beta2.Document.Page.FormField]
            /// value.
            #[prost(message, optional, tag = "2")]
            pub field_value: ::core::option::Option<Layout>,
            /// A list of detected languages for name together with confidence.
            #[prost(message, repeated, tag = "3")]
            pub name_detected_languages: ::prost::alloc::vec::Vec<DetectedLanguage>,
            /// A list of detected languages for value together with confidence.
            #[prost(message, repeated, tag = "4")]
            pub value_detected_languages: ::prost::alloc::vec::Vec<DetectedLanguage>,
            /// If the value is non-textual, this field represents the type. Current
            /// valid values are:
            ///
            /// - blank (this indicates the `field_value` is normal text)
            /// - `unfilled_checkbox`
            /// - `filled_checkbox`
            #[prost(string, tag = "5")]
            pub value_type: ::prost::alloc::string::String,
            /// Created for Labeling UI to export key text.
            /// If corrections were made to the text identified by the
            /// `field_name.text_anchor`, this field will contain the correction.
            #[prost(string, tag = "6")]
            pub corrected_key_text: ::prost::alloc::string::String,
            /// Created for Labeling UI to export value text.
            /// If corrections were made to the text identified by the
            /// `field_value.text_anchor`, this field will contain the correction.
            #[prost(string, tag = "7")]
            pub corrected_value_text: ::prost::alloc::string::String,
            /// The history of this annotation.
            #[prost(message, optional, tag = "8")]
            pub provenance: ::core::option::Option<super::Provenance>,
        }
        /// A detected barcode.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct DetectedBarcode {
            /// [Layout][google.cloud.documentai.v1beta2.Document.Page.Layout] for
            /// [DetectedBarcode][google.cloud.documentai.v1beta2.Document.Page.DetectedBarcode].
            #[prost(message, optional, tag = "1")]
            pub layout: ::core::option::Option<Layout>,
            /// Detailed barcode information of the
            /// [DetectedBarcode][google.cloud.documentai.v1beta2.Document.Page.DetectedBarcode].
            #[prost(message, optional, tag = "2")]
            pub barcode: ::core::option::Option<super::super::Barcode>,
        }
        /// Detected language for a structural component.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct DetectedLanguage {
            /// The [BCP-47 language
            /// code](<https://www.unicode.org/reports/tr35/#Unicode_locale_identifier>),
            /// such as `en-US` or `sr-Latn`.
            #[prost(string, tag = "1")]
            pub language_code: ::prost::alloc::string::String,
            /// Confidence of detected language. Range `\[0, 1\]`.
            #[prost(float, tag = "2")]
            pub confidence: f32,
        }
        /// Image quality scores for the page image.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ImageQualityScores {
            /// The overall quality score. Range `\[0, 1\]` where `1` is perfect quality.
            #[prost(float, tag = "1")]
            pub quality_score: f32,
            /// A list of detected defects.
            #[prost(message, repeated, tag = "2")]
            pub detected_defects: ::prost::alloc::vec::Vec<
                image_quality_scores::DetectedDefect,
            >,
        }
        /// Nested message and enum types in `ImageQualityScores`.
        pub mod image_quality_scores {
            /// Image Quality Defects
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct DetectedDefect {
                /// Name of the defect type. Supported values are:
                ///
                /// - `quality/defect_blurry`
                /// - `quality/defect_noisy`
                /// - `quality/defect_dark`
                /// - `quality/defect_faint`
                /// - `quality/defect_text_too_small`
                /// - `quality/defect_document_cutoff`
                /// - `quality/defect_text_cutoff`
                /// - `quality/defect_glare`
                #[prost(string, tag = "1")]
                pub r#type: ::prost::alloc::string::String,
                /// Confidence of detected defect. Range `\[0, 1\]` where `1` indicates
                /// strong confidence that the defect exists.
                #[prost(float, tag = "2")]
                pub confidence: f32,
            }
        }
    }
    /// An entity that could be a phrase in the text or a property that belongs to
    /// the document. It is a known entity type, such as a person, an organization,
    /// or location.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Entity {
        /// Optional. Provenance of the entity.
        /// Text anchor indexing into the
        /// [Document.text][google.cloud.documentai.v1beta2.Document.text].
        #[prost(message, optional, tag = "1")]
        pub text_anchor: ::core::option::Option<TextAnchor>,
        /// Required. Entity type from a schema e.g. `Address`.
        #[prost(string, tag = "2")]
        pub r#type: ::prost::alloc::string::String,
        /// Optional. Text value of the entity e.g. `1600 Amphitheatre Pkwy`.
        #[prost(string, tag = "3")]
        pub mention_text: ::prost::alloc::string::String,
        /// Optional. Deprecated.  Use `id` field instead.
        #[prost(string, tag = "4")]
        pub mention_id: ::prost::alloc::string::String,
        /// Optional. Confidence of detected Schema entity. Range `\[0, 1\]`.
        #[prost(float, tag = "5")]
        pub confidence: f32,
        /// Optional. Represents the provenance of this entity wrt. the location on
        /// the page where it was found.
        #[prost(message, optional, tag = "6")]
        pub page_anchor: ::core::option::Option<PageAnchor>,
        /// Optional. Canonical id. This will be a unique value in the entity list
        /// for this document.
        #[prost(string, tag = "7")]
        pub id: ::prost::alloc::string::String,
        /// Optional. Normalized entity value. Absent if the extracted value could
        /// not be converted or the type (e.g. address) is not supported for certain
        /// parsers. This field is also only populated for certain supported document
        /// types.
        #[prost(message, optional, tag = "9")]
        pub normalized_value: ::core::option::Option<entity::NormalizedValue>,
        /// Optional. Entities can be nested to form a hierarchical data structure
        /// representing the content in the document.
        #[prost(message, repeated, tag = "10")]
        pub properties: ::prost::alloc::vec::Vec<Entity>,
        /// Optional. The history of this annotation.
        #[prost(message, optional, tag = "11")]
        pub provenance: ::core::option::Option<Provenance>,
        /// Optional. Whether the entity will be redacted for de-identification
        /// purposes.
        #[prost(bool, tag = "12")]
        pub redacted: bool,
    }
    /// Nested message and enum types in `Entity`.
    pub mod entity {
        /// Parsed and normalized entity value.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct NormalizedValue {
            /// Optional. An optional field to store a normalized string.
            /// For some entity types, one of respective `structured_value` fields may
            /// also be populated. Also not all the types of `structured_value` will be
            /// normalized. For example, some processors may not generate `float`
            /// or `integer` normalized text by default.
            ///
            /// Below are sample formats mapped to structured values.
            ///
            /// - Money/Currency type (`money_value`) is in the ISO 4217 text format.
            /// - Date type (`date_value`) is in the ISO 8601 text format.
            /// - Datetime type (`datetime_value`) is in the ISO 8601 text format.
            #[prost(string, tag = "1")]
            pub text: ::prost::alloc::string::String,
            /// An optional structured entity value.
            /// Must match entity type defined in schema if
            /// known. If this field is present, the `text` field could also be
            /// populated.
            #[prost(
                oneof = "normalized_value::StructuredValue",
                tags = "2, 3, 4, 5, 6, 7, 8"
            )]
            pub structured_value: ::core::option::Option<
                normalized_value::StructuredValue,
            >,
        }
        /// Nested message and enum types in `NormalizedValue`.
        pub mod normalized_value {
            /// An optional structured entity value.
            /// Must match entity type defined in schema if
            /// known. If this field is present, the `text` field could also be
            /// populated.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum StructuredValue {
                /// Money value. See also:
                /// <https://github.com/googleapis/googleapis/blob/master/google/type/money.proto>
                #[prost(message, tag = "2")]
                MoneyValue(super::super::super::super::super::super::r#type::Money),
                /// Date value. Includes year, month, day. See also:
                /// <https://github.com/googleapis/googleapis/blob/master/google/type/date.proto>
                #[prost(message, tag = "3")]
                DateValue(super::super::super::super::super::super::r#type::Date),
                /// DateTime value. Includes date, time, and timezone. See also:
                /// <https://github.com/googleapis/googleapis/blob/master/google/type/datetime.proto>
                #[prost(message, tag = "4")]
                DatetimeValue(
                    super::super::super::super::super::super::r#type::DateTime,
                ),
                /// Postal address. See also:
                /// <https://github.com/googleapis/googleapis/blob/master/google/type/postal_address.proto>
                #[prost(message, tag = "5")]
                AddressValue(
                    super::super::super::super::super::super::r#type::PostalAddress,
                ),
                /// Boolean value. Can be used for entities with binary values, or for
                /// checkboxes.
                #[prost(bool, tag = "6")]
                BooleanValue(bool),
                /// Integer value.
                #[prost(int32, tag = "7")]
                IntegerValue(i32),
                /// Float value.
                #[prost(float, tag = "8")]
                FloatValue(f32),
            }
        }
    }
    /// Relationship between
    /// [Entities][google.cloud.documentai.v1beta2.Document.Entity].
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EntityRelation {
        /// Subject entity id.
        #[prost(string, tag = "1")]
        pub subject_id: ::prost::alloc::string::String,
        /// Object entity id.
        #[prost(string, tag = "2")]
        pub object_id: ::prost::alloc::string::String,
        /// Relationship description.
        #[prost(string, tag = "3")]
        pub relation: ::prost::alloc::string::String,
    }
    /// Text reference indexing into the
    /// [Document.text][google.cloud.documentai.v1beta2.Document.text].
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TextAnchor {
        /// The text segments from the
        /// [Document.text][google.cloud.documentai.v1beta2.Document.text].
        #[prost(message, repeated, tag = "1")]
        pub text_segments: ::prost::alloc::vec::Vec<text_anchor::TextSegment>,
        /// Contains the content of the text span so that users do
        /// not have to look it up in the text_segments.  It is always
        /// populated for formFields.
        #[prost(string, tag = "2")]
        pub content: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `TextAnchor`.
    pub mod text_anchor {
        /// A text segment in the
        /// [Document.text][google.cloud.documentai.v1beta2.Document.text]. The
        /// indices may be out of bounds which indicate that the text extends into
        /// another document shard for large sharded documents. See
        /// [ShardInfo.text_offset][google.cloud.documentai.v1beta2.Document.ShardInfo.text_offset]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct TextSegment {
            /// [TextSegment][google.cloud.documentai.v1beta2.Document.TextAnchor.TextSegment]
            /// start UTF-8 char index in the
            /// [Document.text][google.cloud.documentai.v1beta2.Document.text].
            #[prost(int64, tag = "1")]
            pub start_index: i64,
            /// [TextSegment][google.cloud.documentai.v1beta2.Document.TextAnchor.TextSegment]
            /// half open end UTF-8 char index in the
            /// [Document.text][google.cloud.documentai.v1beta2.Document.text].
            #[prost(int64, tag = "2")]
            pub end_index: i64,
        }
    }
    /// Referencing the visual context of the entity in the
    /// [Document.pages][google.cloud.documentai.v1beta2.Document.pages]. Page
    /// anchors can be cross-page, consist of multiple bounding polygons and
    /// optionally reference specific layout element types.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PageAnchor {
        /// One or more references to visual page elements
        #[prost(message, repeated, tag = "1")]
        pub page_refs: ::prost::alloc::vec::Vec<page_anchor::PageRef>,
    }
    /// Nested message and enum types in `PageAnchor`.
    pub mod page_anchor {
        /// Represents a weak reference to a page element within a document.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct PageRef {
            /// Required. Index into the
            /// [Document.pages][google.cloud.documentai.v1beta2.Document.pages]
            /// element, for example using
            /// `[Document.pages][page_refs.page]` to locate the related page element.
            /// This field is skipped when its value is the default `0`. See
            /// <https://developers.google.com/protocol-buffers/docs/proto3#json.>
            #[prost(int64, tag = "1")]
            pub page: i64,
            /// Optional. The type of the layout element that is being referenced if
            /// any.
            #[prost(enumeration = "page_ref::LayoutType", tag = "2")]
            pub layout_type: i32,
            /// Optional. Deprecated.  Use
            /// [PageRef.bounding_poly][google.cloud.documentai.v1beta2.Document.PageAnchor.PageRef.bounding_poly]
            /// instead.
            #[deprecated]
            #[prost(string, tag = "3")]
            pub layout_id: ::prost::alloc::string::String,
            /// Optional. Identifies the bounding polygon of a layout element on the
            /// page. If `layout_type` is set, the bounding polygon must be exactly the
            /// same to the layout element it's referring to.
            #[prost(message, optional, tag = "4")]
            pub bounding_poly: ::core::option::Option<super::super::BoundingPoly>,
            /// Optional. Confidence of detected page element, if applicable. Range
            /// `\[0, 1\]`.
            #[prost(float, tag = "5")]
            pub confidence: f32,
        }
        /// Nested message and enum types in `PageRef`.
        pub mod page_ref {
            /// The type of layout that is being referenced.
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
            pub enum LayoutType {
                /// Layout Unspecified.
                Unspecified = 0,
                /// References a
                /// [Page.blocks][google.cloud.documentai.v1beta2.Document.Page.blocks]
                /// element.
                Block = 1,
                /// References a
                /// [Page.paragraphs][google.cloud.documentai.v1beta2.Document.Page.paragraphs]
                /// element.
                Paragraph = 2,
                /// References a
                /// [Page.lines][google.cloud.documentai.v1beta2.Document.Page.lines]
                /// element.
                Line = 3,
                /// References a
                /// [Page.tokens][google.cloud.documentai.v1beta2.Document.Page.tokens]
                /// element.
                Token = 4,
                /// References a
                /// [Page.visual_elements][google.cloud.documentai.v1beta2.Document.Page.visual_elements]
                /// element.
                VisualElement = 5,
                /// Refrrences a
                /// [Page.tables][google.cloud.documentai.v1beta2.Document.Page.tables]
                /// element.
                Table = 6,
                /// References a
                /// [Page.form_fields][google.cloud.documentai.v1beta2.Document.Page.form_fields]
                /// element.
                FormField = 7,
            }
            impl LayoutType {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        LayoutType::Unspecified => "LAYOUT_TYPE_UNSPECIFIED",
                        LayoutType::Block => "BLOCK",
                        LayoutType::Paragraph => "PARAGRAPH",
                        LayoutType::Line => "LINE",
                        LayoutType::Token => "TOKEN",
                        LayoutType::VisualElement => "VISUAL_ELEMENT",
                        LayoutType::Table => "TABLE",
                        LayoutType::FormField => "FORM_FIELD",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "LAYOUT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                        "BLOCK" => Some(Self::Block),
                        "PARAGRAPH" => Some(Self::Paragraph),
                        "LINE" => Some(Self::Line),
                        "TOKEN" => Some(Self::Token),
                        "VISUAL_ELEMENT" => Some(Self::VisualElement),
                        "TABLE" => Some(Self::Table),
                        "FORM_FIELD" => Some(Self::FormField),
                        _ => None,
                    }
                }
            }
        }
    }
    /// Structure to identify provenance relationships between annotations in
    /// different revisions.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Provenance {
        /// The index of the revision that produced this element.
        #[deprecated]
        #[prost(int32, tag = "1")]
        pub revision: i32,
        /// The Id of this operation.  Needs to be unique within the scope of the
        /// revision.
        #[deprecated]
        #[prost(int32, tag = "2")]
        pub id: i32,
        /// References to the original elements that are replaced.
        #[prost(message, repeated, tag = "3")]
        pub parents: ::prost::alloc::vec::Vec<provenance::Parent>,
        /// The type of provenance operation.
        #[prost(enumeration = "provenance::OperationType", tag = "4")]
        pub r#type: i32,
    }
    /// Nested message and enum types in `Provenance`.
    pub mod provenance {
        /// The parent element the current element is based on. Used for
        /// referencing/aligning, removal and replacement operations.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Parent {
            /// The index of the index into current revision's parent_ids list.
            #[prost(int32, tag = "1")]
            pub revision: i32,
            /// The index of the parent item in the corresponding item list (eg. list
            /// of entities, properties within entities, etc.) in the parent revision.
            #[prost(int32, tag = "3")]
            pub index: i32,
            /// The id of the parent provenance.
            #[deprecated]
            #[prost(int32, tag = "2")]
            pub id: i32,
        }
        /// If a processor or agent does an explicit operation on existing elements.
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
        pub enum OperationType {
            /// Operation type unspecified. If no operation is specified a provenance
            /// entry is simply used to match against a `parent`.
            Unspecified = 0,
            /// Add an element.
            Add = 1,
            /// Remove an element identified by `parent`.
            Remove = 2,
            /// Updates any fields within the given provenance scope of the message. It
            /// overwrites the fields rather than replacing them.  Use this when you
            /// want to update a field value of an entity without also updating all the
            /// child properties.
            Update = 7,
            /// Currently unused. Replace an element identified by `parent`.
            Replace = 3,
            /// Deprecated. Request human review for the element identified by
            /// `parent`.
            EvalRequested = 4,
            /// Deprecated. Element is reviewed and approved at human review,
            /// confidence will be set to 1.0.
            EvalApproved = 5,
            /// Deprecated. Element is skipped in the validation process.
            EvalSkipped = 6,
        }
        impl OperationType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    OperationType::Unspecified => "OPERATION_TYPE_UNSPECIFIED",
                    OperationType::Add => "ADD",
                    OperationType::Remove => "REMOVE",
                    OperationType::Update => "UPDATE",
                    OperationType::Replace => "REPLACE",
                    OperationType::EvalRequested => "EVAL_REQUESTED",
                    OperationType::EvalApproved => "EVAL_APPROVED",
                    OperationType::EvalSkipped => "EVAL_SKIPPED",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "OPERATION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                    "ADD" => Some(Self::Add),
                    "REMOVE" => Some(Self::Remove),
                    "UPDATE" => Some(Self::Update),
                    "REPLACE" => Some(Self::Replace),
                    "EVAL_REQUESTED" => Some(Self::EvalRequested),
                    "EVAL_APPROVED" => Some(Self::EvalApproved),
                    "EVAL_SKIPPED" => Some(Self::EvalSkipped),
                    _ => None,
                }
            }
        }
    }
    /// Contains past or forward revisions of this document.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Revision {
        /// Id of the revision, internally generated by doc proto storage.
        /// Unique within the context of the document.
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
        /// The revisions that this revision is based on.  This can include one or
        /// more parent (when documents are merged.)  This field represents the
        /// index into the `revisions` field.
        #[deprecated]
        #[prost(int32, repeated, packed = "false", tag = "2")]
        pub parent: ::prost::alloc::vec::Vec<i32>,
        /// The revisions that this revision is based on. Must include all the ids
        /// that have anything to do with this revision - eg. there are
        /// `provenance.parent.revision` fields that index into this field.
        #[prost(string, repeated, tag = "7")]
        pub parent_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// The time that the revision was created, internally generated by
        /// doc proto storage at the time of create.
        #[prost(message, optional, tag = "3")]
        pub create_time: ::core::option::Option<::prost_types::Timestamp>,
        /// Human Review information of this revision.
        #[prost(message, optional, tag = "6")]
        pub human_review: ::core::option::Option<revision::HumanReview>,
        /// Who/what made the change
        #[prost(oneof = "revision::Source", tags = "4, 5")]
        pub source: ::core::option::Option<revision::Source>,
    }
    /// Nested message and enum types in `Revision`.
    pub mod revision {
        /// Human Review information of the document.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct HumanReview {
            /// Human review state. e.g. `requested`, `succeeded`, `rejected`.
            #[prost(string, tag = "1")]
            pub state: ::prost::alloc::string::String,
            /// A message providing more details about the current state of processing.
            /// For example, the rejection reason when the state is `rejected`.
            #[prost(string, tag = "2")]
            pub state_message: ::prost::alloc::string::String,
        }
        /// Who/what made the change
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Source {
            /// If the change was made by a person specify the name or id of that
            /// person.
            #[prost(string, tag = "4")]
            Agent(::prost::alloc::string::String),
            /// If the annotation was made by processor identify the processor by its
            /// resource name.
            #[prost(string, tag = "5")]
            Processor(::prost::alloc::string::String),
        }
    }
    /// This message is used for text changes aka. OCR corrections.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TextChange {
        /// Provenance of the correction.
        /// Text anchor indexing into the
        /// [Document.text][google.cloud.documentai.v1beta2.Document.text].  There
        /// can only be a single `TextAnchor.text_segments` element.  If the start
        /// and end index of the text segment are the same, the text change is
        /// inserted before that index.
        #[prost(message, optional, tag = "1")]
        pub text_anchor: ::core::option::Option<TextAnchor>,
        /// The text that replaces the text identified in the `text_anchor`.
        #[prost(string, tag = "2")]
        pub changed_text: ::prost::alloc::string::String,
        /// The history of this annotation.
        #[deprecated]
        #[prost(message, repeated, tag = "3")]
        pub provenance: ::prost::alloc::vec::Vec<Provenance>,
    }
    /// Original source document from the user.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// Optional. Currently supports Google Cloud Storage URI of the form
        /// `gs://bucket_name/object_name`. Object versioning is not supported.
        /// For more information, refer to [Google Cloud Storage Request
        /// URIs](<https://cloud.google.com/storage/docs/reference-uris>).
        #[prost(string, tag = "1")]
        Uri(::prost::alloc::string::String),
        /// Optional. Inline document content, represented as a stream of bytes.
        /// Note: As with all `bytes` fields, protobuffers use a pure binary
        /// representation, whereas JSON representations use base64.
        #[prost(bytes, tag = "2")]
        Content(::prost::alloc::vec::Vec<u8>),
    }
}
/// Request to batch process documents as an asynchronous operation. The output
/// is written to Cloud Storage as JSON in the \[Document\] format.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchProcessDocumentsRequest {
    /// Required. Individual requests for each document.
    #[prost(message, repeated, tag = "1")]
    pub requests: ::prost::alloc::vec::Vec<ProcessDocumentRequest>,
    /// Target project and location to make a call.
    ///
    /// Format: `projects/{project-id}/locations/{location-id}`.
    ///
    /// If no location is specified, a region will be chosen automatically.
    #[prost(string, tag = "2")]
    pub parent: ::prost::alloc::string::String,
}
/// Request to process one document.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessDocumentRequest {
    /// Target project and location to make a call.
    ///
    /// Format: `projects/{project-id}/locations/{location-id}`.
    ///
    /// If no location is specified, a region will be chosen automatically.
    /// This field is only populated when used in ProcessDocument method.
    #[prost(string, tag = "9")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Information about the input file.
    #[prost(message, optional, tag = "1")]
    pub input_config: ::core::option::Option<InputConfig>,
    /// The desired output location. This field is only needed in
    /// BatchProcessDocumentsRequest.
    #[prost(message, optional, tag = "2")]
    pub output_config: ::core::option::Option<OutputConfig>,
    /// Specifies a known document type for deeper structure detection. Valid
    /// values are currently "general" and "invoice". If not provided, "general"\
    /// is used as default. If any other value is given, the request is rejected.
    #[prost(string, tag = "3")]
    pub document_type: ::prost::alloc::string::String,
    /// Controls table extraction behavior. If not specified, the system will
    /// decide reasonable defaults.
    #[prost(message, optional, tag = "4")]
    pub table_extraction_params: ::core::option::Option<TableExtractionParams>,
    /// Controls form extraction behavior. If not specified, the system will
    /// decide reasonable defaults.
    #[prost(message, optional, tag = "5")]
    pub form_extraction_params: ::core::option::Option<FormExtractionParams>,
    /// Controls entity extraction behavior. If not specified, the system will
    /// decide reasonable defaults.
    #[prost(message, optional, tag = "6")]
    pub entity_extraction_params: ::core::option::Option<EntityExtractionParams>,
    /// Controls OCR behavior. If not specified, the system will decide reasonable
    /// defaults.
    #[prost(message, optional, tag = "7")]
    pub ocr_params: ::core::option::Option<OcrParams>,
    /// Controls AutoML model prediction behavior. AutoMlParams cannot be used
    /// together with other Params.
    #[prost(message, optional, tag = "8")]
    pub automl_params: ::core::option::Option<AutoMlParams>,
}
/// Response to an batch document processing request. This is returned in
/// the LRO Operation after the operation is complete.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchProcessDocumentsResponse {
    /// Responses for each individual document.
    #[prost(message, repeated, tag = "1")]
    pub responses: ::prost::alloc::vec::Vec<ProcessDocumentResponse>,
}
/// Response to a single document processing request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessDocumentResponse {
    /// Information about the input file. This is the same as the corresponding
    /// input config in the request.
    #[prost(message, optional, tag = "1")]
    pub input_config: ::core::option::Option<InputConfig>,
    /// The output location of the parsed responses. The responses are written to
    /// this location as JSON-serialized `Document` objects.
    #[prost(message, optional, tag = "2")]
    pub output_config: ::core::option::Option<OutputConfig>,
}
/// Parameters to control Optical Character Recognition (OCR) behavior.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OcrParams {
    /// List of languages to use for OCR. In most cases, an empty value
    /// yields the best results since it enables automatic language detection. For
    /// languages based on the Latin alphabet, setting `language_hints` is not
    /// needed. In rare cases, when the language of the text in the image is known,
    /// setting a hint will help get better results (although it will be a
    /// significant hindrance if the hint is wrong). Document processing returns an
    /// error if one or more of the specified languages is not one of the
    /// supported languages.
    #[prost(string, repeated, tag = "1")]
    pub language_hints: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Parameters to control table extraction behavior.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableExtractionParams {
    /// Whether to enable table extraction.
    #[prost(bool, tag = "1")]
    pub enabled: bool,
    /// Optional. Table bounding box hints that can be provided to complex cases
    /// which our algorithm cannot locate the table(s) in.
    #[prost(message, repeated, tag = "2")]
    pub table_bound_hints: ::prost::alloc::vec::Vec<TableBoundHint>,
    /// Optional. Reserved for future use.
    #[prost(string, repeated, tag = "3")]
    pub header_hints: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Model version of the table extraction system. Default is "builtin/stable".
    /// Specify "builtin/latest" for the latest model.
    #[prost(string, tag = "4")]
    pub model_version: ::prost::alloc::string::String,
}
/// A hint for a table bounding box on the page for table parsing.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableBoundHint {
    /// Optional. Page number for multi-paged inputs this hint applies to. If not
    /// provided, this hint will apply to all pages by default. This value is
    /// 1-based.
    #[prost(int32, tag = "1")]
    pub page_number: i32,
    /// Bounding box hint for a table on this page. The coordinates must be
    /// normalized to \[0,1\] and the bounding box must be an axis-aligned rectangle.
    #[prost(message, optional, tag = "2")]
    pub bounding_box: ::core::option::Option<BoundingPoly>,
}
/// Parameters to control form extraction behavior.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FormExtractionParams {
    /// Whether to enable form extraction.
    #[prost(bool, tag = "1")]
    pub enabled: bool,
    /// Reserved for future use.
    #[prost(message, repeated, tag = "2")]
    pub key_value_pair_hints: ::prost::alloc::vec::Vec<KeyValuePairHint>,
    /// Model version of the form extraction system. Default is
    /// "builtin/stable". Specify "builtin/latest" for the latest model.
    /// For custom form models, specify: "custom/{model_name}". Model name
    /// format is "bucket_name/path/to/modeldir" corresponding to
    /// "gs://bucket_name/path/to/modeldir" where annotated examples are stored.
    #[prost(string, tag = "3")]
    pub model_version: ::prost::alloc::string::String,
}
/// Reserved for future use.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyValuePairHint {
    /// The key text for the hint.
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    /// Type of the value. This is case-insensitive, and could be one of:
    /// ADDRESS, LOCATION, ORGANIZATION, PERSON, PHONE_NUMBER,
    /// ID, NUMBER, EMAIL, PRICE, TERMS, DATE, NAME. Types not in this list will
    /// be ignored.
    #[prost(string, repeated, tag = "2")]
    pub value_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Parameters to control entity extraction behavior.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityExtractionParams {
    /// Whether to enable entity extraction.
    #[prost(bool, tag = "1")]
    pub enabled: bool,
    /// Model version of the entity extraction. Default is
    /// "builtin/stable". Specify "builtin/latest" for the latest model.
    #[prost(string, tag = "2")]
    pub model_version: ::prost::alloc::string::String,
}
/// Parameters to control AutoML model prediction behavior.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoMlParams {
    /// Resource name of the AutoML model.
    ///
    /// Format: `projects/{project-id}/locations/{location-id}/models/{model-id}`.
    #[prost(string, tag = "1")]
    pub model: ::prost::alloc::string::String,
}
/// The desired input location and metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputConfig {
    /// Required. Mimetype of the input. Current supported mimetypes are
    /// application/pdf, image/tiff, and image/gif. In addition, application/json
    /// type is supported for requests with
    /// [ProcessDocumentRequest.automl_params][google.cloud.documentai.v1beta2.ProcessDocumentRequest.automl_params]
    /// field set. The JSON file needs to be in
    /// [Document][google.cloud.documentai.v1beta2.Document] format.
    #[prost(string, tag = "2")]
    pub mime_type: ::prost::alloc::string::String,
    /// Required.
    #[prost(oneof = "input_config::Source", tags = "1, 3")]
    pub source: ::core::option::Option<input_config::Source>,
}
/// Nested message and enum types in `InputConfig`.
pub mod input_config {
    /// Required.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// The Google Cloud Storage location to read the input from. This must be a
        /// single file.
        #[prost(message, tag = "1")]
        GcsSource(super::GcsSource),
        /// Content in bytes, represented as a stream of bytes.
        /// Note: As with all `bytes` fields, proto buffer messages use a pure binary
        /// representation, whereas JSON representations use base64.
        ///
        /// This field only works for synchronous ProcessDocument method.
        #[prost(bytes, tag = "3")]
        Contents(::prost::alloc::vec::Vec<u8>),
    }
}
/// The desired output location and metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutputConfig {
    /// The max number of pages to include into each output Document shard JSON on
    /// Google Cloud Storage.
    ///
    /// The valid range is \[1, 100\]. If not specified, the default value is 20.
    ///
    /// For example, for one pdf file with 100 pages, 100 parsed pages will be
    /// produced. If `pages_per_shard` = 20, then 5 Document shard JSON files each
    /// containing 20 parsed pages will be written under the prefix
    /// [OutputConfig.gcs_destination.uri][] and suffix pages-x-to-y.json where
    /// x and y are 1-indexed page numbers.
    ///
    /// Example GCS outputs with 157 pages and pages_per_shard = 50:
    ///
    /// <prefix>pages-001-to-050.json
    /// <prefix>pages-051-to-100.json
    /// <prefix>pages-101-to-150.json
    /// <prefix>pages-151-to-157.json
    #[prost(int32, tag = "2")]
    pub pages_per_shard: i32,
    /// Required.
    #[prost(oneof = "output_config::Destination", tags = "1")]
    pub destination: ::core::option::Option<output_config::Destination>,
}
/// Nested message and enum types in `OutputConfig`.
pub mod output_config {
    /// Required.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// The Google Cloud Storage location to write the output to.
        #[prost(message, tag = "1")]
        GcsDestination(super::GcsDestination),
    }
}
/// The Google Cloud Storage location where the input file will be read from.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsSource {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
}
/// The Google Cloud Storage location where the output file will be written to.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsDestination {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
}
/// Contains metadata for the BatchProcessDocuments operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// The state of the current batch processing.
    #[prost(enumeration = "operation_metadata::State", tag = "1")]
    pub state: i32,
    /// A message providing more details about the current state of processing.
    #[prost(string, tag = "2")]
    pub state_message: ::prost::alloc::string::String,
    /// The creation time of the operation.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The last update time of the operation.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `OperationMetadata`.
pub mod operation_metadata {
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
        /// The default value. This value is used if the state is omitted.
        Unspecified = 0,
        /// Request is received.
        Accepted = 1,
        /// Request operation is waiting for scheduling.
        Waiting = 2,
        /// Request is being processed.
        Running = 3,
        /// The batch processing completed successfully.
        Succeeded = 4,
        /// The batch processing was cancelled.
        Cancelled = 5,
        /// The batch processing has failed.
        Failed = 6,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Accepted => "ACCEPTED",
                State::Waiting => "WAITING",
                State::Running => "RUNNING",
                State::Succeeded => "SUCCEEDED",
                State::Cancelled => "CANCELLED",
                State::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ACCEPTED" => Some(Self::Accepted),
                "WAITING" => Some(Self::Waiting),
                "RUNNING" => Some(Self::Running),
                "SUCCEEDED" => Some(Self::Succeeded),
                "CANCELLED" => Some(Self::Cancelled),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
}
/// Generated client implementations.
pub mod document_understanding_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service to parse structured information from unstructured or semi-structured
    /// documents using state-of-the-art Google AI such as natural language,
    /// computer vision, and translation.
    #[derive(Debug, Clone)]
    pub struct DocumentUnderstandingServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DocumentUnderstandingServiceClient<tonic::transport::Channel> {
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
    impl<T> DocumentUnderstandingServiceClient<T>
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
        ) -> DocumentUnderstandingServiceClient<InterceptedService<T, F>>
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
            DocumentUnderstandingServiceClient::new(
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
        /// LRO endpoint to batch process many documents. The output is written
        /// to Cloud Storage as JSON in the [Document] format.
        pub async fn batch_process_documents(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchProcessDocumentsRequest>,
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
                "/google.cloud.documentai.v1beta2.DocumentUnderstandingService/BatchProcessDocuments",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.documentai.v1beta2.DocumentUnderstandingService",
                        "BatchProcessDocuments",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Processes a single document.
        pub async fn process_document(
            &mut self,
            request: impl tonic::IntoRequest<super::ProcessDocumentRequest>,
        ) -> std::result::Result<tonic::Response<super::Document>, tonic::Status> {
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
                "/google.cloud.documentai.v1beta2.DocumentUnderstandingService/ProcessDocument",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.documentai.v1beta2.DocumentUnderstandingService",
                        "ProcessDocument",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
