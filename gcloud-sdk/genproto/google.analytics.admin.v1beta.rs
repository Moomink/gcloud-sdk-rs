/// Dimensions are attributes of your data. For example, the dimension
/// `userEmail` indicates the email of the user that accessed reporting data.
/// Dimension values in report responses are strings.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessDimension {
    /// The API name of the dimension. See [Data Access
    /// Schema](<https://developers.google.com/analytics/devguides/config/admin/v1/access-api-schema>)
    /// for the list of dimensions supported in this API.
    ///
    /// Dimensions are referenced by name in `dimensionFilter` and `orderBys`.
    #[prost(string, tag = "1")]
    pub dimension_name: ::prost::alloc::string::String,
}
/// The quantitative measurements of a report. For example, the metric
/// `accessCount` is the total number of data access records.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessMetric {
    /// The API name of the metric. See [Data Access
    /// Schema](<https://developers.google.com/analytics/devguides/config/admin/v1/access-api-schema>)
    /// for the list of metrics supported in this API.
    ///
    /// Metrics are referenced by name in `metricFilter` & `orderBys`.
    #[prost(string, tag = "1")]
    pub metric_name: ::prost::alloc::string::String,
}
/// A contiguous range of days: startDate, startDate + 1, ..., endDate.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessDateRange {
    /// The inclusive start date for the query in the format `YYYY-MM-DD`. Cannot
    /// be after `endDate`. The format `NdaysAgo`, `yesterday`, or `today` is also
    /// accepted, and in that case, the date is inferred based on the current time
    /// in the request's time zone.
    #[prost(string, tag = "1")]
    pub start_date: ::prost::alloc::string::String,
    /// The inclusive end date for the query in the format `YYYY-MM-DD`. Cannot
    /// be before `startDate`. The format `NdaysAgo`, `yesterday`, or `today` is
    /// also accepted, and in that case, the date is inferred based on the current
    /// time in the request's time zone.
    #[prost(string, tag = "2")]
    pub end_date: ::prost::alloc::string::String,
}
/// Expresses dimension or metric filters. The fields in the same expression need
/// to be either all dimensions or all metrics.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessFilterExpression {
    /// Specify one type of filter expression for `FilterExpression`.
    #[prost(oneof = "access_filter_expression::OneExpression", tags = "1, 2, 3, 4")]
    pub one_expression: ::core::option::Option<access_filter_expression::OneExpression>,
}
/// Nested message and enum types in `AccessFilterExpression`.
pub mod access_filter_expression {
    /// Specify one type of filter expression for `FilterExpression`.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OneExpression {
        /// Each of the FilterExpressions in the and_group has an AND relationship.
        #[prost(message, tag = "1")]
        AndGroup(super::AccessFilterExpressionList),
        /// Each of the FilterExpressions in the or_group has an OR relationship.
        #[prost(message, tag = "2")]
        OrGroup(super::AccessFilterExpressionList),
        /// The FilterExpression is NOT of not_expression.
        #[prost(message, tag = "3")]
        NotExpression(::prost::alloc::boxed::Box<super::AccessFilterExpression>),
        /// A primitive filter. In the same FilterExpression, all of the filter's
        /// field names need to be either all dimensions or all metrics.
        #[prost(message, tag = "4")]
        AccessFilter(super::AccessFilter),
    }
}
/// A list of filter expressions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessFilterExpressionList {
    /// A list of filter expressions.
    #[prost(message, repeated, tag = "1")]
    pub expressions: ::prost::alloc::vec::Vec<AccessFilterExpression>,
}
/// An expression to filter dimension or metric values.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessFilter {
    /// The dimension name or metric name.
    #[prost(string, tag = "1")]
    pub field_name: ::prost::alloc::string::String,
    /// Specify one type of filter for `Filter`.
    #[prost(oneof = "access_filter::OneFilter", tags = "2, 3, 4, 5")]
    pub one_filter: ::core::option::Option<access_filter::OneFilter>,
}
/// Nested message and enum types in `AccessFilter`.
pub mod access_filter {
    /// Specify one type of filter for `Filter`.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OneFilter {
        /// Strings related filter.
        #[prost(message, tag = "2")]
        StringFilter(super::AccessStringFilter),
        /// A filter for in list values.
        #[prost(message, tag = "3")]
        InListFilter(super::AccessInListFilter),
        /// A filter for numeric or date values.
        #[prost(message, tag = "4")]
        NumericFilter(super::AccessNumericFilter),
        /// A filter for two values.
        #[prost(message, tag = "5")]
        BetweenFilter(super::AccessBetweenFilter),
    }
}
/// The filter for strings.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessStringFilter {
    /// The match type for this filter.
    #[prost(enumeration = "access_string_filter::MatchType", tag = "1")]
    pub match_type: i32,
    /// The string value used for the matching.
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
    /// If true, the string value is case sensitive.
    #[prost(bool, tag = "3")]
    pub case_sensitive: bool,
}
/// Nested message and enum types in `AccessStringFilter`.
pub mod access_string_filter {
    /// The match type of a string filter.
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
    pub enum MatchType {
        /// Unspecified
        Unspecified = 0,
        /// Exact match of the string value.
        Exact = 1,
        /// Begins with the string value.
        BeginsWith = 2,
        /// Ends with the string value.
        EndsWith = 3,
        /// Contains the string value.
        Contains = 4,
        /// Full match for the regular expression with the string value.
        FullRegexp = 5,
        /// Partial match for the regular expression with the string value.
        PartialRegexp = 6,
    }
    impl MatchType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MatchType::Unspecified => "MATCH_TYPE_UNSPECIFIED",
                MatchType::Exact => "EXACT",
                MatchType::BeginsWith => "BEGINS_WITH",
                MatchType::EndsWith => "ENDS_WITH",
                MatchType::Contains => "CONTAINS",
                MatchType::FullRegexp => "FULL_REGEXP",
                MatchType::PartialRegexp => "PARTIAL_REGEXP",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MATCH_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "EXACT" => Some(Self::Exact),
                "BEGINS_WITH" => Some(Self::BeginsWith),
                "ENDS_WITH" => Some(Self::EndsWith),
                "CONTAINS" => Some(Self::Contains),
                "FULL_REGEXP" => Some(Self::FullRegexp),
                "PARTIAL_REGEXP" => Some(Self::PartialRegexp),
                _ => None,
            }
        }
    }
}
/// The result needs to be in a list of string values.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessInListFilter {
    /// The list of string values. Must be non-empty.
    #[prost(string, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// If true, the string value is case sensitive.
    #[prost(bool, tag = "2")]
    pub case_sensitive: bool,
}
/// Filters for numeric or date values.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessNumericFilter {
    /// The operation type for this filter.
    #[prost(enumeration = "access_numeric_filter::Operation", tag = "1")]
    pub operation: i32,
    /// A numeric value or a date value.
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<NumericValue>,
}
/// Nested message and enum types in `AccessNumericFilter`.
pub mod access_numeric_filter {
    /// The operation applied to a numeric filter.
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
    pub enum Operation {
        /// Unspecified.
        Unspecified = 0,
        /// Equal
        Equal = 1,
        /// Less than
        LessThan = 2,
        /// Less than or equal
        LessThanOrEqual = 3,
        /// Greater than
        GreaterThan = 4,
        /// Greater than or equal
        GreaterThanOrEqual = 5,
    }
    impl Operation {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Operation::Unspecified => "OPERATION_UNSPECIFIED",
                Operation::Equal => "EQUAL",
                Operation::LessThan => "LESS_THAN",
                Operation::LessThanOrEqual => "LESS_THAN_OR_EQUAL",
                Operation::GreaterThan => "GREATER_THAN",
                Operation::GreaterThanOrEqual => "GREATER_THAN_OR_EQUAL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OPERATION_UNSPECIFIED" => Some(Self::Unspecified),
                "EQUAL" => Some(Self::Equal),
                "LESS_THAN" => Some(Self::LessThan),
                "LESS_THAN_OR_EQUAL" => Some(Self::LessThanOrEqual),
                "GREATER_THAN" => Some(Self::GreaterThan),
                "GREATER_THAN_OR_EQUAL" => Some(Self::GreaterThanOrEqual),
                _ => None,
            }
        }
    }
}
/// To express that the result needs to be between two numbers (inclusive).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessBetweenFilter {
    /// Begins with this number.
    #[prost(message, optional, tag = "1")]
    pub from_value: ::core::option::Option<NumericValue>,
    /// Ends with this number.
    #[prost(message, optional, tag = "2")]
    pub to_value: ::core::option::Option<NumericValue>,
}
/// To represent a number.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NumericValue {
    /// One of a numeric value
    #[prost(oneof = "numeric_value::OneValue", tags = "1, 2")]
    pub one_value: ::core::option::Option<numeric_value::OneValue>,
}
/// Nested message and enum types in `NumericValue`.
pub mod numeric_value {
    /// One of a numeric value
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OneValue {
        /// Integer value
        #[prost(int64, tag = "1")]
        Int64Value(i64),
        /// Double value
        #[prost(double, tag = "2")]
        DoubleValue(f64),
    }
}
/// Order bys define how rows will be sorted in the response. For example,
/// ordering rows by descending access count is one ordering, and ordering rows
/// by the country string is a different ordering.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessOrderBy {
    /// If true, sorts by descending order. If false or unspecified, sorts in
    /// ascending order.
    #[prost(bool, tag = "3")]
    pub desc: bool,
    /// Specify one type of order by for `OrderBy`.
    #[prost(oneof = "access_order_by::OneOrderBy", tags = "1, 2")]
    pub one_order_by: ::core::option::Option<access_order_by::OneOrderBy>,
}
/// Nested message and enum types in `AccessOrderBy`.
pub mod access_order_by {
    /// Sorts by metric values.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MetricOrderBy {
        /// A metric name in the request to order by.
        #[prost(string, tag = "1")]
        pub metric_name: ::prost::alloc::string::String,
    }
    /// Sorts by dimension values.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DimensionOrderBy {
        /// A dimension name in the request to order by.
        #[prost(string, tag = "1")]
        pub dimension_name: ::prost::alloc::string::String,
        /// Controls the rule for dimension value ordering.
        #[prost(enumeration = "dimension_order_by::OrderType", tag = "2")]
        pub order_type: i32,
    }
    /// Nested message and enum types in `DimensionOrderBy`.
    pub mod dimension_order_by {
        /// Rule to order the string dimension values by.
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
        pub enum OrderType {
            /// Unspecified.
            Unspecified = 0,
            /// Alphanumeric sort by Unicode code point. For example, "2" < "A" < "X" <
            /// "b" < "z".
            Alphanumeric = 1,
            /// Case insensitive alphanumeric sort by lower case Unicode code point.
            /// For example, "2" < "A" < "b" < "X" < "z".
            CaseInsensitiveAlphanumeric = 2,
            /// Dimension values are converted to numbers before sorting. For example
            /// in NUMERIC sort, "25" < "100", and in `ALPHANUMERIC` sort, "100" <
            /// "25". Non-numeric dimension values all have equal ordering value below
            /// all numeric values.
            Numeric = 3,
        }
        impl OrderType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    OrderType::Unspecified => "ORDER_TYPE_UNSPECIFIED",
                    OrderType::Alphanumeric => "ALPHANUMERIC",
                    OrderType::CaseInsensitiveAlphanumeric => {
                        "CASE_INSENSITIVE_ALPHANUMERIC"
                    }
                    OrderType::Numeric => "NUMERIC",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "ORDER_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                    "ALPHANUMERIC" => Some(Self::Alphanumeric),
                    "CASE_INSENSITIVE_ALPHANUMERIC" => {
                        Some(Self::CaseInsensitiveAlphanumeric)
                    }
                    "NUMERIC" => Some(Self::Numeric),
                    _ => None,
                }
            }
        }
    }
    /// Specify one type of order by for `OrderBy`.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OneOrderBy {
        /// Sorts results by a metric's values.
        #[prost(message, tag = "1")]
        Metric(MetricOrderBy),
        /// Sorts results by a dimension's values.
        #[prost(message, tag = "2")]
        Dimension(DimensionOrderBy),
    }
}
/// Describes a dimension column in the report. Dimensions requested in a report
/// produce column entries within rows and DimensionHeaders. However, dimensions
/// used exclusively within filters or expressions do not produce columns in a
/// report; correspondingly, those dimensions do not produce headers.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessDimensionHeader {
    /// The dimension's name; for example 'userEmail'.
    #[prost(string, tag = "1")]
    pub dimension_name: ::prost::alloc::string::String,
}
/// Describes a metric column in the report. Visible metrics requested in a
/// report produce column entries within rows and MetricHeaders. However,
/// metrics used exclusively within filters or expressions do not produce columns
/// in a report; correspondingly, those metrics do not produce headers.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessMetricHeader {
    /// The metric's name; for example 'accessCount'.
    #[prost(string, tag = "1")]
    pub metric_name: ::prost::alloc::string::String,
}
/// Access report data for each row.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessRow {
    /// List of dimension values. These values are in the same order as specified
    /// in the request.
    #[prost(message, repeated, tag = "1")]
    pub dimension_values: ::prost::alloc::vec::Vec<AccessDimensionValue>,
    /// List of metric values. These values are in the same order as specified
    /// in the request.
    #[prost(message, repeated, tag = "2")]
    pub metric_values: ::prost::alloc::vec::Vec<AccessMetricValue>,
}
/// The value of a dimension.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessDimensionValue {
    /// The dimension value. For example, this value may be 'France' for the
    /// 'country' dimension.
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
/// The value of a metric.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessMetricValue {
    /// The measurement value. For example, this value may be '13'.
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
/// Current state of all quotas for this Analytics property. If any quota for a
/// property is exhausted, all requests to that property will return Resource
/// Exhausted errors.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessQuota {
    /// Properties can use 250,000 tokens per day. Most requests consume fewer than
    /// 10 tokens.
    #[prost(message, optional, tag = "1")]
    pub tokens_per_day: ::core::option::Option<AccessQuotaStatus>,
    /// Properties can use 50,000 tokens per hour. An API request consumes a single
    /// number of tokens, and that number is deducted from all of the hourly,
    /// daily, and per project hourly quotas.
    #[prost(message, optional, tag = "2")]
    pub tokens_per_hour: ::core::option::Option<AccessQuotaStatus>,
    /// Properties can use up to 50 concurrent requests.
    #[prost(message, optional, tag = "3")]
    pub concurrent_requests: ::core::option::Option<AccessQuotaStatus>,
    /// Properties and cloud project pairs can have up to 50 server errors per
    /// hour.
    #[prost(message, optional, tag = "4")]
    pub server_errors_per_project_per_hour: ::core::option::Option<AccessQuotaStatus>,
    /// Properties can use up to 25% of their tokens per project per hour. This
    /// amounts to Analytics 360 Properties can use 12,500 tokens per project per
    /// hour. An API request consumes a single number of tokens, and that number is
    /// deducted from all of the hourly, daily, and per project hourly quotas.
    #[prost(message, optional, tag = "5")]
    pub tokens_per_project_per_hour: ::core::option::Option<AccessQuotaStatus>,
}
/// Current state for a particular quota group.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessQuotaStatus {
    /// Quota consumed by this request.
    #[prost(int32, tag = "1")]
    pub consumed: i32,
    /// Quota remaining after this request.
    #[prost(int32, tag = "2")]
    pub remaining: i32,
}
/// A resource message representing a Google Analytics account.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Account {
    /// Output only. Resource name of this account.
    /// Format: accounts/{account}
    /// Example: "accounts/100"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Time when this account was originally created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when account payload fields were last updated.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. Human-readable display name for this account.
    #[prost(string, tag = "4")]
    pub display_name: ::prost::alloc::string::String,
    /// Country of business. Must be a Unicode CLDR region code.
    #[prost(string, tag = "5")]
    pub region_code: ::prost::alloc::string::String,
    /// Output only. Indicates whether this Account is soft-deleted or not. Deleted
    /// accounts are excluded from List results unless specifically requested.
    #[prost(bool, tag = "6")]
    pub deleted: bool,
}
/// A resource message representing a Google Analytics GA4 property.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Property {
    /// Output only. Resource name of this property.
    /// Format: properties/{property_id}
    /// Example: "properties/1000"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. The property type for this Property resource. When creating a
    /// property, if the type is "PROPERTY_TYPE_UNSPECIFIED", then
    /// "ORDINARY_PROPERTY" will be implied. "SUBPROPERTY" and "ROLLUP_PROPERTY"
    /// types cannot yet be created with the Google Analytics Admin API.
    #[prost(enumeration = "PropertyType", tag = "14")]
    pub property_type: i32,
    /// Output only. Time when the entity was originally created.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when entity payload fields were last updated.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Immutable. Resource name of this property's logical parent.
    ///
    /// Note: The Property-Moving UI can be used to change the parent.
    /// Format: accounts/{account}, properties/{property}
    /// Example: "accounts/100", "properties/101"
    #[prost(string, tag = "2")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Human-readable display name for this property.
    ///
    /// The max allowed display name length is 100 UTF-16 code units.
    #[prost(string, tag = "5")]
    pub display_name: ::prost::alloc::string::String,
    /// Industry associated with this property
    /// Example: AUTOMOTIVE, FOOD_AND_DRINK
    #[prost(enumeration = "IndustryCategory", tag = "6")]
    pub industry_category: i32,
    /// Required. Reporting Time Zone, used as the day boundary for reports,
    /// regardless of where the data originates. If the time zone honors DST,
    /// Analytics will automatically adjust for the changes.
    ///
    /// NOTE: Changing the time zone only affects data going forward, and is not
    /// applied retroactively.
    ///
    /// Format: <https://www.iana.org/time-zones>
    /// Example: "America/Los_Angeles"
    #[prost(string, tag = "7")]
    pub time_zone: ::prost::alloc::string::String,
    /// The currency type used in reports involving monetary values.
    ///
    ///
    /// Format: <https://en.wikipedia.org/wiki/ISO_4217>
    /// Examples: "USD", "EUR", "JPY"
    #[prost(string, tag = "8")]
    pub currency_code: ::prost::alloc::string::String,
    /// Output only. The Google Analytics service level that applies to this
    /// property.
    #[prost(enumeration = "ServiceLevel", tag = "10")]
    pub service_level: i32,
    /// Output only. If set, the time at which this property was trashed. If not
    /// set, then this property is not currently in the trash can.
    #[prost(message, optional, tag = "11")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. If set, the time at which this trashed property will be
    /// permanently deleted. If not set, then this property is not currently in the
    /// trash can and is not slated to be deleted.
    #[prost(message, optional, tag = "12")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Immutable. The resource name of the parent account
    /// Format: accounts/{account_id}
    /// Example: "accounts/123"
    #[prost(string, tag = "13")]
    pub account: ::prost::alloc::string::String,
}
/// A resource message representing a data stream.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataStream {
    /// Output only. Resource name of this Data Stream.
    /// Format: properties/{property_id}/dataStreams/{stream_id}
    /// Example: "properties/1000/dataStreams/2000"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Immutable. The type of this DataStream resource.
    #[prost(enumeration = "data_stream::DataStreamType", tag = "2")]
    pub r#type: i32,
    /// Human-readable display name for the Data Stream.
    ///
    /// Required for web data streams.
    ///
    /// The max allowed display name length is 255 UTF-16 code units.
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. Time when this stream was originally created.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when stream payload fields were last updated.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Data for specific data stream types. The message that will be
    /// set corresponds to the type of this stream.
    #[prost(oneof = "data_stream::StreamData", tags = "6, 7, 8")]
    pub stream_data: ::core::option::Option<data_stream::StreamData>,
}
/// Nested message and enum types in `DataStream`.
pub mod data_stream {
    /// Data specific to web streams.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WebStreamData {
        /// Output only. Analytics "Measurement ID", without the "G-" prefix.
        /// Example: "G-1A2BCD345E" would just be "1A2BCD345E"
        #[prost(string, tag = "1")]
        pub measurement_id: ::prost::alloc::string::String,
        /// Output only. ID of the corresponding web app in Firebase, if any.
        /// This ID can change if the web app is deleted and recreated.
        #[prost(string, tag = "2")]
        pub firebase_app_id: ::prost::alloc::string::String,
        /// Immutable. Domain name of the web app being measured, or empty.
        /// Example: "<http://www.google.com",> "<https://www.google.com">
        #[prost(string, tag = "3")]
        pub default_uri: ::prost::alloc::string::String,
    }
    /// Data specific to Android app streams.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AndroidAppStreamData {
        /// Output only. ID of the corresponding Android app in Firebase, if any.
        /// This ID can change if the Android app is deleted and recreated.
        #[prost(string, tag = "1")]
        pub firebase_app_id: ::prost::alloc::string::String,
        /// Immutable. The package name for the app being measured.
        /// Example: "com.example.myandroidapp"
        #[prost(string, tag = "2")]
        pub package_name: ::prost::alloc::string::String,
    }
    /// Data specific to iOS app streams.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IosAppStreamData {
        /// Output only. ID of the corresponding iOS app in Firebase, if any.
        /// This ID can change if the iOS app is deleted and recreated.
        #[prost(string, tag = "1")]
        pub firebase_app_id: ::prost::alloc::string::String,
        /// Required. Immutable. The Apple App Store Bundle ID for the app
        /// Example: "com.example.myiosapp"
        #[prost(string, tag = "2")]
        pub bundle_id: ::prost::alloc::string::String,
    }
    /// The type of the data stream.
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
    pub enum DataStreamType {
        /// Type unknown or not specified.
        Unspecified = 0,
        /// Web data stream.
        WebDataStream = 1,
        /// Android app data stream.
        AndroidAppDataStream = 2,
        /// iOS app data stream.
        IosAppDataStream = 3,
    }
    impl DataStreamType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DataStreamType::Unspecified => "DATA_STREAM_TYPE_UNSPECIFIED",
                DataStreamType::WebDataStream => "WEB_DATA_STREAM",
                DataStreamType::AndroidAppDataStream => "ANDROID_APP_DATA_STREAM",
                DataStreamType::IosAppDataStream => "IOS_APP_DATA_STREAM",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DATA_STREAM_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "WEB_DATA_STREAM" => Some(Self::WebDataStream),
                "ANDROID_APP_DATA_STREAM" => Some(Self::AndroidAppDataStream),
                "IOS_APP_DATA_STREAM" => Some(Self::IosAppDataStream),
                _ => None,
            }
        }
    }
    /// Data for specific data stream types. The message that will be
    /// set corresponds to the type of this stream.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum StreamData {
        /// Data specific to web streams. Must be populated if type is
        /// WEB_DATA_STREAM.
        #[prost(message, tag = "6")]
        WebStreamData(WebStreamData),
        /// Data specific to Android app streams. Must be populated if type is
        /// ANDROID_APP_DATA_STREAM.
        #[prost(message, tag = "7")]
        AndroidAppStreamData(AndroidAppStreamData),
        /// Data specific to iOS app streams. Must be populated if type is
        /// IOS_APP_DATA_STREAM.
        #[prost(message, tag = "8")]
        IosAppStreamData(IosAppStreamData),
    }
}
/// A link between a GA4 property and a Firebase project.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FirebaseLink {
    /// Output only. Example format: properties/1234/firebaseLinks/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. Firebase project resource name. When creating a FirebaseLink,
    /// you may provide this resource name using either a project number or project
    /// ID. Once this resource has been created, returned FirebaseLinks will always
    /// have a project_name that contains a project number.
    ///
    /// Format: 'projects/{project number}'
    /// Example: 'projects/1234'
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    /// Output only. Time when this FirebaseLink was originally created.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// A link between a GA4 property and a Google Ads account.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoogleAdsLink {
    /// Output only. Format:
    /// properties/{propertyId}/googleAdsLinks/{googleAdsLinkId}
    ///
    /// Note: googleAdsLinkId is not the Google Ads customer ID.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. Google Ads customer ID.
    #[prost(string, tag = "3")]
    pub customer_id: ::prost::alloc::string::String,
    /// Output only. If true, this link is for a Google Ads manager account.
    #[prost(bool, tag = "4")]
    pub can_manage_clients: bool,
    /// Enable personalized advertising features with this integration.
    /// Automatically publish my Google Analytics audience lists and Google
    /// Analytics remarketing events/parameters to the linked Google Ads account.
    /// If this field is not set on create/update, it will be defaulted to true.
    #[prost(message, optional, tag = "5")]
    pub ads_personalization_enabled: ::core::option::Option<bool>,
    /// Output only. Time when this link was originally created.
    #[prost(message, optional, tag = "7")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when this link was last updated.
    #[prost(message, optional, tag = "8")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Email address of the user that created the link.
    /// An empty string will be returned if the email address can't be retrieved.
    #[prost(string, tag = "9")]
    pub creator_email_address: ::prost::alloc::string::String,
}
/// A resource message representing data sharing settings of a Google Analytics
/// account.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataSharingSettings {
    /// Output only. Resource name.
    /// Format: accounts/{account}/dataSharingSettings
    /// Example: "accounts/1000/dataSharingSettings"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Allows Google support to access the data in order to help troubleshoot
    /// issues.
    #[prost(bool, tag = "2")]
    pub sharing_with_google_support_enabled: bool,
    /// Allows Google sales teams that are assigned to the customer to access the
    /// data in order to suggest configuration changes to improve results.
    /// Sales team restrictions still apply when enabled.
    #[prost(bool, tag = "3")]
    pub sharing_with_google_assigned_sales_enabled: bool,
    /// Allows any of Google sales to access the data in order to suggest
    /// configuration changes to improve results.
    #[prost(bool, tag = "4")]
    pub sharing_with_google_any_sales_enabled: bool,
    /// Allows Google to use the data to improve other Google products or services.
    #[prost(bool, tag = "5")]
    pub sharing_with_google_products_enabled: bool,
    /// Allows Google to share the data anonymously in aggregate form with others.
    #[prost(bool, tag = "6")]
    pub sharing_with_others_enabled: bool,
}
/// A virtual resource representing an overview of an account and
/// all its child GA4 properties.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountSummary {
    /// Resource name for this account summary.
    /// Format: accountSummaries/{account_id}
    /// Example: "accountSummaries/1000"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Resource name of account referred to by this account summary
    /// Format: accounts/{account_id}
    /// Example: "accounts/1000"
    #[prost(string, tag = "2")]
    pub account: ::prost::alloc::string::String,
    /// Display name for the account referred to in this account summary.
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
    /// List of summaries for child accounts of this account.
    #[prost(message, repeated, tag = "4")]
    pub property_summaries: ::prost::alloc::vec::Vec<PropertySummary>,
}
/// A virtual resource representing metadata for a GA4 property.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PropertySummary {
    /// Resource name of property referred to by this property summary
    /// Format: properties/{property_id}
    /// Example: "properties/1000"
    #[prost(string, tag = "1")]
    pub property: ::prost::alloc::string::String,
    /// Display name for the property referred to in this property summary.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// The property's property type.
    #[prost(enumeration = "PropertyType", tag = "3")]
    pub property_type: i32,
    /// Resource name of this property's logical parent.
    ///
    /// Note: The Property-Moving UI can be used to change the parent.
    /// Format: accounts/{account}, properties/{property}
    /// Example: "accounts/100", "properties/200"
    #[prost(string, tag = "4")]
    pub parent: ::prost::alloc::string::String,
}
/// A secret value used for sending hits to Measurement Protocol.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MeasurementProtocolSecret {
    /// Output only. Resource name of this secret. This secret may be a child of
    /// any type of stream. Format:
    /// properties/{property}/dataStreams/{dataStream}/measurementProtocolSecrets/{measurementProtocolSecret}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Human-readable display name for this secret.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. The measurement protocol secret value. Pass this value to the
    /// api_secret field of the Measurement Protocol API when sending hits to this
    /// secret's parent property.
    #[prost(string, tag = "3")]
    pub secret_value: ::prost::alloc::string::String,
}
/// A set of changes within a Google Analytics account or its child properties
/// that resulted from the same cause. Common causes would be updates made in the
/// Google Analytics UI, changes from customer support, or automatic Google
/// Analytics system changes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeHistoryEvent {
    /// ID of this change history event. This ID is unique across Google Analytics.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Time when change was made.
    #[prost(message, optional, tag = "2")]
    pub change_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The type of actor that made this change.
    #[prost(enumeration = "ActorType", tag = "3")]
    pub actor_type: i32,
    /// Email address of the Google account that made the change. This will be a
    /// valid email address if the actor field is set to USER, and empty otherwise.
    /// Google accounts that have been deleted will cause an error.
    #[prost(string, tag = "4")]
    pub user_actor_email: ::prost::alloc::string::String,
    /// If true, then the list of changes returned was filtered, and does not
    /// represent all changes that occurred in this event.
    #[prost(bool, tag = "5")]
    pub changes_filtered: bool,
    /// A list of changes made in this change history event that fit the filters
    /// specified in SearchChangeHistoryEventsRequest.
    #[prost(message, repeated, tag = "6")]
    pub changes: ::prost::alloc::vec::Vec<ChangeHistoryChange>,
}
/// A description of a change to a single Google Analytics resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeHistoryChange {
    /// Resource name of the resource whose changes are described by this entry.
    #[prost(string, tag = "1")]
    pub resource: ::prost::alloc::string::String,
    /// The type of action that changed this resource.
    #[prost(enumeration = "ActionType", tag = "2")]
    pub action: i32,
    /// Resource contents from before the change was made. If this resource was
    /// created in this change, this field will be missing.
    #[prost(message, optional, tag = "3")]
    pub resource_before_change: ::core::option::Option<
        change_history_change::ChangeHistoryResource,
    >,
    /// Resource contents from after the change was made. If this resource was
    /// deleted in this change, this field will be missing.
    #[prost(message, optional, tag = "4")]
    pub resource_after_change: ::core::option::Option<
        change_history_change::ChangeHistoryResource,
    >,
}
/// Nested message and enum types in `ChangeHistoryChange`.
pub mod change_history_change {
    /// A snapshot of a resource as before or after the result of a change in
    /// change history.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ChangeHistoryResource {
        #[prost(
            oneof = "change_history_resource::Resource",
            tags = "1, 2, 6, 7, 11, 12, 15, 18"
        )]
        pub resource: ::core::option::Option<change_history_resource::Resource>,
    }
    /// Nested message and enum types in `ChangeHistoryResource`.
    pub mod change_history_resource {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Resource {
            /// A snapshot of an Account resource in change history.
            #[prost(message, tag = "1")]
            Account(super::super::Account),
            /// A snapshot of a Property resource in change history.
            #[prost(message, tag = "2")]
            Property(super::super::Property),
            /// A snapshot of a FirebaseLink resource in change history.
            #[prost(message, tag = "6")]
            FirebaseLink(super::super::FirebaseLink),
            /// A snapshot of a GoogleAdsLink resource in change history.
            #[prost(message, tag = "7")]
            GoogleAdsLink(super::super::GoogleAdsLink),
            /// A snapshot of a ConversionEvent resource in change history.
            #[prost(message, tag = "11")]
            ConversionEvent(super::super::ConversionEvent),
            /// A snapshot of a MeasurementProtocolSecret resource in change history.
            #[prost(message, tag = "12")]
            MeasurementProtocolSecret(super::super::MeasurementProtocolSecret),
            /// A snapshot of a data retention settings resource in change history.
            #[prost(message, tag = "15")]
            DataRetentionSettings(super::super::DataRetentionSettings),
            /// A snapshot of a DataStream resource in change history.
            #[prost(message, tag = "18")]
            DataStream(super::super::DataStream),
        }
    }
}
/// A conversion event in a Google Analytics property.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionEvent {
    /// Output only. Resource name of this conversion event.
    /// Format: properties/{property}/conversionEvents/{conversion_event}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. The event name for this conversion event.
    /// Examples: 'click', 'purchase'
    #[prost(string, tag = "2")]
    pub event_name: ::prost::alloc::string::String,
    /// Output only. Time when this conversion event was created in the property.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. If set, this event can currently be deleted with
    /// DeleteConversionEvent.
    #[prost(bool, tag = "4")]
    pub deletable: bool,
    /// Output only. If set to true, this conversion event refers to a custom
    /// event.  If set to false, this conversion event refers to a default event in
    /// GA. Default events typically have special meaning in GA. Default events are
    /// usually created for you by the GA system, but in some cases can be created
    /// by property admins. Custom events count towards the maximum number of
    /// custom conversion events that may be created per property.
    #[prost(bool, tag = "5")]
    pub custom: bool,
}
/// A definition for a CustomDimension.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomDimension {
    /// Output only. Resource name for this CustomDimension resource.
    /// Format: properties/{property}/customDimensions/{customDimension}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Immutable. Tagging parameter name for this custom dimension.
    ///
    /// If this is a user-scoped dimension, then this is the user property name.
    /// If this is an event-scoped dimension, then this is the event parameter
    /// name.
    ///
    /// May only contain alphanumeric and underscore characters, starting with a
    /// letter. Max length of 24 characters for user-scoped dimensions, 40
    /// characters for event-scoped dimensions.
    #[prost(string, tag = "2")]
    pub parameter_name: ::prost::alloc::string::String,
    /// Required. Display name for this custom dimension as shown in the Analytics
    /// UI. Max length of 82 characters, alphanumeric plus space and underscore
    /// starting with a letter. Legacy system-generated display names may contain
    /// square brackets, but updates to this field will never permit square
    /// brackets.
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. Description for this custom dimension. Max length of 150
    /// characters.
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    /// Required. Immutable. The scope of this dimension.
    #[prost(enumeration = "custom_dimension::DimensionScope", tag = "5")]
    pub scope: i32,
    /// Optional. If set to true, sets this dimension as NPA and excludes it from
    /// ads personalization.
    ///
    /// This is currently only supported by user-scoped custom dimensions.
    #[prost(bool, tag = "6")]
    pub disallow_ads_personalization: bool,
}
/// Nested message and enum types in `CustomDimension`.
pub mod custom_dimension {
    /// Valid values for the scope of this dimension.
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
    pub enum DimensionScope {
        /// Scope unknown or not specified.
        Unspecified = 0,
        /// Dimension scoped to an event.
        Event = 1,
        /// Dimension scoped to a user.
        User = 2,
    }
    impl DimensionScope {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DimensionScope::Unspecified => "DIMENSION_SCOPE_UNSPECIFIED",
                DimensionScope::Event => "EVENT",
                DimensionScope::User => "USER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DIMENSION_SCOPE_UNSPECIFIED" => Some(Self::Unspecified),
                "EVENT" => Some(Self::Event),
                "USER" => Some(Self::User),
                _ => None,
            }
        }
    }
}
/// A definition for a custom metric.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomMetric {
    /// Output only. Resource name for this CustomMetric resource.
    /// Format: properties/{property}/customMetrics/{customMetric}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Immutable. Tagging name for this custom metric.
    ///
    /// If this is an event-scoped metric, then this is the event parameter
    /// name.
    ///
    /// May only contain alphanumeric and underscore charactes, starting with a
    /// letter. Max length of 40 characters for event-scoped metrics.
    #[prost(string, tag = "2")]
    pub parameter_name: ::prost::alloc::string::String,
    /// Required. Display name for this custom metric as shown in the Analytics UI.
    /// Max length of 82 characters, alphanumeric plus space and underscore
    /// starting with a letter. Legacy system-generated display names may contain
    /// square brackets, but updates to this field will never permit square
    /// brackets.
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. Description for this custom dimension.
    /// Max length of 150 characters.
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    /// Required. The type for the custom metric's value.
    #[prost(enumeration = "custom_metric::MeasurementUnit", tag = "5")]
    pub measurement_unit: i32,
    /// Required. Immutable. The scope of this custom metric.
    #[prost(enumeration = "custom_metric::MetricScope", tag = "6")]
    pub scope: i32,
    /// Optional. Types of restricted data that this metric may contain. Required
    /// for metrics with CURRENCY measurement unit. Must be empty for metrics with
    /// a non-CURRENCY measurement unit.
    #[prost(
        enumeration = "custom_metric::RestrictedMetricType",
        repeated,
        packed = "false",
        tag = "8"
    )]
    pub restricted_metric_type: ::prost::alloc::vec::Vec<i32>,
}
/// Nested message and enum types in `CustomMetric`.
pub mod custom_metric {
    /// Possible types of representing the custom metric's value.
    ///
    /// Currency representation may change in the future, requiring a breaking API
    /// change.
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
    pub enum MeasurementUnit {
        /// MeasurementUnit unspecified or missing.
        Unspecified = 0,
        /// This metric uses default units.
        Standard = 1,
        /// This metric measures a currency.
        Currency = 2,
        /// This metric measures feet.
        Feet = 3,
        /// This metric measures meters.
        Meters = 4,
        /// This metric measures kilometers.
        Kilometers = 5,
        /// This metric measures miles.
        Miles = 6,
        /// This metric measures milliseconds.
        Milliseconds = 7,
        /// This metric measures seconds.
        Seconds = 8,
        /// This metric measures minutes.
        Minutes = 9,
        /// This metric measures hours.
        Hours = 10,
    }
    impl MeasurementUnit {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MeasurementUnit::Unspecified => "MEASUREMENT_UNIT_UNSPECIFIED",
                MeasurementUnit::Standard => "STANDARD",
                MeasurementUnit::Currency => "CURRENCY",
                MeasurementUnit::Feet => "FEET",
                MeasurementUnit::Meters => "METERS",
                MeasurementUnit::Kilometers => "KILOMETERS",
                MeasurementUnit::Miles => "MILES",
                MeasurementUnit::Milliseconds => "MILLISECONDS",
                MeasurementUnit::Seconds => "SECONDS",
                MeasurementUnit::Minutes => "MINUTES",
                MeasurementUnit::Hours => "HOURS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MEASUREMENT_UNIT_UNSPECIFIED" => Some(Self::Unspecified),
                "STANDARD" => Some(Self::Standard),
                "CURRENCY" => Some(Self::Currency),
                "FEET" => Some(Self::Feet),
                "METERS" => Some(Self::Meters),
                "KILOMETERS" => Some(Self::Kilometers),
                "MILES" => Some(Self::Miles),
                "MILLISECONDS" => Some(Self::Milliseconds),
                "SECONDS" => Some(Self::Seconds),
                "MINUTES" => Some(Self::Minutes),
                "HOURS" => Some(Self::Hours),
                _ => None,
            }
        }
    }
    /// The scope of this metric.
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
    pub enum MetricScope {
        /// Scope unknown or not specified.
        Unspecified = 0,
        /// Metric scoped to an event.
        Event = 1,
    }
    impl MetricScope {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MetricScope::Unspecified => "METRIC_SCOPE_UNSPECIFIED",
                MetricScope::Event => "EVENT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "METRIC_SCOPE_UNSPECIFIED" => Some(Self::Unspecified),
                "EVENT" => Some(Self::Event),
                _ => None,
            }
        }
    }
    /// Labels that mark the data in this custom metric as data that should be
    /// restricted to specific users.
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
    pub enum RestrictedMetricType {
        /// Type unknown or unspecified.
        Unspecified = 0,
        /// Metric reports cost data.
        CostData = 1,
        /// Metric reports revenue data.
        RevenueData = 2,
    }
    impl RestrictedMetricType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RestrictedMetricType::Unspecified => "RESTRICTED_METRIC_TYPE_UNSPECIFIED",
                RestrictedMetricType::CostData => "COST_DATA",
                RestrictedMetricType::RevenueData => "REVENUE_DATA",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RESTRICTED_METRIC_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "COST_DATA" => Some(Self::CostData),
                "REVENUE_DATA" => Some(Self::RevenueData),
                _ => None,
            }
        }
    }
}
/// Settings values for data retention. This is a singleton resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataRetentionSettings {
    /// Output only. Resource name for this DataRetentionSetting resource.
    /// Format: properties/{property}/dataRetentionSettings
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The length of time that event-level data is retained.
    #[prost(enumeration = "data_retention_settings::RetentionDuration", tag = "2")]
    pub event_data_retention: i32,
    /// If true, reset the retention period for the user identifier with every
    /// event from that user.
    #[prost(bool, tag = "3")]
    pub reset_user_data_on_new_activity: bool,
}
/// Nested message and enum types in `DataRetentionSettings`.
pub mod data_retention_settings {
    /// Valid values for the data retention duration.
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
    pub enum RetentionDuration {
        /// Data retention time duration is not specified.
        Unspecified = 0,
        /// The data retention time duration is 2 months.
        TwoMonths = 1,
        /// The data retention time duration is 14 months.
        FourteenMonths = 3,
        /// The data retention time duration is 26 months.
        /// Available to 360 properties only.
        TwentySixMonths = 4,
        /// The data retention time duration is 38 months.
        /// Available to 360 properties only.
        ThirtyEightMonths = 5,
        /// The data retention time duration is 50 months.
        /// Available to 360 properties only.
        FiftyMonths = 6,
    }
    impl RetentionDuration {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RetentionDuration::Unspecified => "RETENTION_DURATION_UNSPECIFIED",
                RetentionDuration::TwoMonths => "TWO_MONTHS",
                RetentionDuration::FourteenMonths => "FOURTEEN_MONTHS",
                RetentionDuration::TwentySixMonths => "TWENTY_SIX_MONTHS",
                RetentionDuration::ThirtyEightMonths => "THIRTY_EIGHT_MONTHS",
                RetentionDuration::FiftyMonths => "FIFTY_MONTHS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RETENTION_DURATION_UNSPECIFIED" => Some(Self::Unspecified),
                "TWO_MONTHS" => Some(Self::TwoMonths),
                "FOURTEEN_MONTHS" => Some(Self::FourteenMonths),
                "TWENTY_SIX_MONTHS" => Some(Self::TwentySixMonths),
                "THIRTY_EIGHT_MONTHS" => Some(Self::ThirtyEightMonths),
                "FIFTY_MONTHS" => Some(Self::FiftyMonths),
                _ => None,
            }
        }
    }
}
/// The category selected for this property, used for industry benchmarking.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IndustryCategory {
    /// Industry category unspecified
    Unspecified = 0,
    /// Automotive
    Automotive = 1,
    /// Business and industrial markets
    BusinessAndIndustrialMarkets = 2,
    /// Finance
    Finance = 3,
    /// Healthcare
    Healthcare = 4,
    /// Technology
    Technology = 5,
    /// Travel
    Travel = 6,
    /// Other
    Other = 7,
    /// Arts and entertainment
    ArtsAndEntertainment = 8,
    /// Beauty and fitness
    BeautyAndFitness = 9,
    /// Books and literature
    BooksAndLiterature = 10,
    /// Food and drink
    FoodAndDrink = 11,
    /// Games
    Games = 12,
    /// Hobbies and leisure
    HobbiesAndLeisure = 13,
    /// Home and garden
    HomeAndGarden = 14,
    /// Internet and telecom
    InternetAndTelecom = 15,
    /// Law and government
    LawAndGovernment = 16,
    /// News
    News = 17,
    /// Online communities
    OnlineCommunities = 18,
    /// People and society
    PeopleAndSociety = 19,
    /// Pets and animals
    PetsAndAnimals = 20,
    /// Real estate
    RealEstate = 21,
    /// Reference
    Reference = 22,
    /// Science
    Science = 23,
    /// Sports
    Sports = 24,
    /// Jobs and education
    JobsAndEducation = 25,
    /// Shopping
    Shopping = 26,
}
impl IndustryCategory {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            IndustryCategory::Unspecified => "INDUSTRY_CATEGORY_UNSPECIFIED",
            IndustryCategory::Automotive => "AUTOMOTIVE",
            IndustryCategory::BusinessAndIndustrialMarkets => {
                "BUSINESS_AND_INDUSTRIAL_MARKETS"
            }
            IndustryCategory::Finance => "FINANCE",
            IndustryCategory::Healthcare => "HEALTHCARE",
            IndustryCategory::Technology => "TECHNOLOGY",
            IndustryCategory::Travel => "TRAVEL",
            IndustryCategory::Other => "OTHER",
            IndustryCategory::ArtsAndEntertainment => "ARTS_AND_ENTERTAINMENT",
            IndustryCategory::BeautyAndFitness => "BEAUTY_AND_FITNESS",
            IndustryCategory::BooksAndLiterature => "BOOKS_AND_LITERATURE",
            IndustryCategory::FoodAndDrink => "FOOD_AND_DRINK",
            IndustryCategory::Games => "GAMES",
            IndustryCategory::HobbiesAndLeisure => "HOBBIES_AND_LEISURE",
            IndustryCategory::HomeAndGarden => "HOME_AND_GARDEN",
            IndustryCategory::InternetAndTelecom => "INTERNET_AND_TELECOM",
            IndustryCategory::LawAndGovernment => "LAW_AND_GOVERNMENT",
            IndustryCategory::News => "NEWS",
            IndustryCategory::OnlineCommunities => "ONLINE_COMMUNITIES",
            IndustryCategory::PeopleAndSociety => "PEOPLE_AND_SOCIETY",
            IndustryCategory::PetsAndAnimals => "PETS_AND_ANIMALS",
            IndustryCategory::RealEstate => "REAL_ESTATE",
            IndustryCategory::Reference => "REFERENCE",
            IndustryCategory::Science => "SCIENCE",
            IndustryCategory::Sports => "SPORTS",
            IndustryCategory::JobsAndEducation => "JOBS_AND_EDUCATION",
            IndustryCategory::Shopping => "SHOPPING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INDUSTRY_CATEGORY_UNSPECIFIED" => Some(Self::Unspecified),
            "AUTOMOTIVE" => Some(Self::Automotive),
            "BUSINESS_AND_INDUSTRIAL_MARKETS" => Some(Self::BusinessAndIndustrialMarkets),
            "FINANCE" => Some(Self::Finance),
            "HEALTHCARE" => Some(Self::Healthcare),
            "TECHNOLOGY" => Some(Self::Technology),
            "TRAVEL" => Some(Self::Travel),
            "OTHER" => Some(Self::Other),
            "ARTS_AND_ENTERTAINMENT" => Some(Self::ArtsAndEntertainment),
            "BEAUTY_AND_FITNESS" => Some(Self::BeautyAndFitness),
            "BOOKS_AND_LITERATURE" => Some(Self::BooksAndLiterature),
            "FOOD_AND_DRINK" => Some(Self::FoodAndDrink),
            "GAMES" => Some(Self::Games),
            "HOBBIES_AND_LEISURE" => Some(Self::HobbiesAndLeisure),
            "HOME_AND_GARDEN" => Some(Self::HomeAndGarden),
            "INTERNET_AND_TELECOM" => Some(Self::InternetAndTelecom),
            "LAW_AND_GOVERNMENT" => Some(Self::LawAndGovernment),
            "NEWS" => Some(Self::News),
            "ONLINE_COMMUNITIES" => Some(Self::OnlineCommunities),
            "PEOPLE_AND_SOCIETY" => Some(Self::PeopleAndSociety),
            "PETS_AND_ANIMALS" => Some(Self::PetsAndAnimals),
            "REAL_ESTATE" => Some(Self::RealEstate),
            "REFERENCE" => Some(Self::Reference),
            "SCIENCE" => Some(Self::Science),
            "SPORTS" => Some(Self::Sports),
            "JOBS_AND_EDUCATION" => Some(Self::JobsAndEducation),
            "SHOPPING" => Some(Self::Shopping),
            _ => None,
        }
    }
}
/// Various levels of service for Google Analytics.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ServiceLevel {
    /// Service level not specified or invalid.
    Unspecified = 0,
    /// The standard version of Google Analytics.
    GoogleAnalyticsStandard = 1,
    /// The paid, premium version of Google Analytics.
    GoogleAnalytics360 = 2,
}
impl ServiceLevel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ServiceLevel::Unspecified => "SERVICE_LEVEL_UNSPECIFIED",
            ServiceLevel::GoogleAnalyticsStandard => "GOOGLE_ANALYTICS_STANDARD",
            ServiceLevel::GoogleAnalytics360 => "GOOGLE_ANALYTICS_360",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SERVICE_LEVEL_UNSPECIFIED" => Some(Self::Unspecified),
            "GOOGLE_ANALYTICS_STANDARD" => Some(Self::GoogleAnalyticsStandard),
            "GOOGLE_ANALYTICS_360" => Some(Self::GoogleAnalytics360),
            _ => None,
        }
    }
}
/// Different kinds of actors that can make changes to Google Analytics
/// resources.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ActorType {
    /// Unknown or unspecified actor type.
    Unspecified = 0,
    /// Changes made by the user specified in actor_email.
    User = 1,
    /// Changes made by the Google Analytics system.
    System = 2,
    /// Changes made by Google Analytics support team staff.
    Support = 3,
}
impl ActorType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ActorType::Unspecified => "ACTOR_TYPE_UNSPECIFIED",
            ActorType::User => "USER",
            ActorType::System => "SYSTEM",
            ActorType::Support => "SUPPORT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ACTOR_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "USER" => Some(Self::User),
            "SYSTEM" => Some(Self::System),
            "SUPPORT" => Some(Self::Support),
            _ => None,
        }
    }
}
/// Types of actions that may change a resource.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ActionType {
    /// Action type unknown or not specified.
    Unspecified = 0,
    /// Resource was created in this change.
    Created = 1,
    /// Resource was updated in this change.
    Updated = 2,
    /// Resource was deleted in this change.
    Deleted = 3,
}
impl ActionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ActionType::Unspecified => "ACTION_TYPE_UNSPECIFIED",
            ActionType::Created => "CREATED",
            ActionType::Updated => "UPDATED",
            ActionType::Deleted => "DELETED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ACTION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "CREATED" => Some(Self::Created),
            "UPDATED" => Some(Self::Updated),
            "DELETED" => Some(Self::Deleted),
            _ => None,
        }
    }
}
/// Types of resources whose changes may be returned from change history.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChangeHistoryResourceType {
    /// Resource type unknown or not specified.
    Unspecified = 0,
    /// Account resource
    Account = 1,
    /// Property resource
    Property = 2,
    /// FirebaseLink resource
    FirebaseLink = 6,
    /// GoogleAdsLink resource
    GoogleAdsLink = 7,
    /// GoogleSignalsSettings resource
    GoogleSignalsSettings = 8,
    /// ConversionEvent resource
    ConversionEvent = 9,
    /// MeasurementProtocolSecret resource
    MeasurementProtocolSecret = 10,
    /// DataRetentionSettings resource
    DataRetentionSettings = 13,
    /// DisplayVideo360AdvertiserLink resource
    DisplayVideo360AdvertiserLink = 14,
    /// DisplayVideo360AdvertiserLinkProposal resource
    DisplayVideo360AdvertiserLinkProposal = 15,
    /// DataStream resource
    DataStream = 18,
    /// AttributionSettings resource
    AttributionSettings = 20,
}
impl ChangeHistoryResourceType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ChangeHistoryResourceType::Unspecified => {
                "CHANGE_HISTORY_RESOURCE_TYPE_UNSPECIFIED"
            }
            ChangeHistoryResourceType::Account => "ACCOUNT",
            ChangeHistoryResourceType::Property => "PROPERTY",
            ChangeHistoryResourceType::FirebaseLink => "FIREBASE_LINK",
            ChangeHistoryResourceType::GoogleAdsLink => "GOOGLE_ADS_LINK",
            ChangeHistoryResourceType::GoogleSignalsSettings => "GOOGLE_SIGNALS_SETTINGS",
            ChangeHistoryResourceType::ConversionEvent => "CONVERSION_EVENT",
            ChangeHistoryResourceType::MeasurementProtocolSecret => {
                "MEASUREMENT_PROTOCOL_SECRET"
            }
            ChangeHistoryResourceType::DataRetentionSettings => "DATA_RETENTION_SETTINGS",
            ChangeHistoryResourceType::DisplayVideo360AdvertiserLink => {
                "DISPLAY_VIDEO_360_ADVERTISER_LINK"
            }
            ChangeHistoryResourceType::DisplayVideo360AdvertiserLinkProposal => {
                "DISPLAY_VIDEO_360_ADVERTISER_LINK_PROPOSAL"
            }
            ChangeHistoryResourceType::DataStream => "DATA_STREAM",
            ChangeHistoryResourceType::AttributionSettings => "ATTRIBUTION_SETTINGS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CHANGE_HISTORY_RESOURCE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "ACCOUNT" => Some(Self::Account),
            "PROPERTY" => Some(Self::Property),
            "FIREBASE_LINK" => Some(Self::FirebaseLink),
            "GOOGLE_ADS_LINK" => Some(Self::GoogleAdsLink),
            "GOOGLE_SIGNALS_SETTINGS" => Some(Self::GoogleSignalsSettings),
            "CONVERSION_EVENT" => Some(Self::ConversionEvent),
            "MEASUREMENT_PROTOCOL_SECRET" => Some(Self::MeasurementProtocolSecret),
            "DATA_RETENTION_SETTINGS" => Some(Self::DataRetentionSettings),
            "DISPLAY_VIDEO_360_ADVERTISER_LINK" => {
                Some(Self::DisplayVideo360AdvertiserLink)
            }
            "DISPLAY_VIDEO_360_ADVERTISER_LINK_PROPOSAL" => {
                Some(Self::DisplayVideo360AdvertiserLinkProposal)
            }
            "DATA_STREAM" => Some(Self::DataStream),
            "ATTRIBUTION_SETTINGS" => Some(Self::AttributionSettings),
            _ => None,
        }
    }
}
/// Types of Property resources.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PropertyType {
    /// Unknown or unspecified property type
    Unspecified = 0,
    /// Ordinary GA4 property
    Ordinary = 1,
    /// GA4 subproperty
    Subproperty = 2,
    /// GA4 rollup property
    Rollup = 3,
}
impl PropertyType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PropertyType::Unspecified => "PROPERTY_TYPE_UNSPECIFIED",
            PropertyType::Ordinary => "PROPERTY_TYPE_ORDINARY",
            PropertyType::Subproperty => "PROPERTY_TYPE_SUBPROPERTY",
            PropertyType::Rollup => "PROPERTY_TYPE_ROLLUP",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PROPERTY_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "PROPERTY_TYPE_ORDINARY" => Some(Self::Ordinary),
            "PROPERTY_TYPE_SUBPROPERTY" => Some(Self::Subproperty),
            "PROPERTY_TYPE_ROLLUP" => Some(Self::Rollup),
            _ => None,
        }
    }
}
/// The request for a Data Access Record Report.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunAccessReportRequest {
    /// The Data Access Report supports requesting at the property level or account
    /// level. If requested at the account level, Data Access Reports include all
    /// access for all properties under that account.
    ///
    /// To request at the property level, entity should be for example
    /// 'properties/123' if "123" is your GA4 property ID. To request at the
    /// account level, entity should be for example 'accounts/1234' if "1234" is
    /// your GA4 Account ID.
    #[prost(string, tag = "1")]
    pub entity: ::prost::alloc::string::String,
    /// The dimensions requested and displayed in the response. Requests are
    /// allowed up to 9 dimensions.
    #[prost(message, repeated, tag = "2")]
    pub dimensions: ::prost::alloc::vec::Vec<AccessDimension>,
    /// The metrics requested and displayed in the response. Requests are allowed
    /// up to 10 metrics.
    #[prost(message, repeated, tag = "3")]
    pub metrics: ::prost::alloc::vec::Vec<AccessMetric>,
    /// Date ranges of access records to read. If multiple date ranges are
    /// requested, each response row will contain a zero based date range index. If
    /// two date ranges overlap, the access records for the overlapping days is
    /// included in the response rows for both date ranges. Requests are allowed up
    /// to 2 date ranges.
    #[prost(message, repeated, tag = "4")]
    pub date_ranges: ::prost::alloc::vec::Vec<AccessDateRange>,
    /// Dimension filters let you restrict report response to specific
    /// dimension values which match the filter. For example, filtering on access
    /// records of a single user. To learn more, see [Fundamentals of Dimension
    /// Filters](<https://developers.google.com/analytics/devguides/reporting/data/v1/basics#dimension_filters>)
    /// for examples. Metrics cannot be used in this filter.
    #[prost(message, optional, tag = "5")]
    pub dimension_filter: ::core::option::Option<AccessFilterExpression>,
    /// Metric filters allow you to restrict report response to specific metric
    /// values which match the filter. Metric filters are applied after aggregating
    /// the report's rows, similar to SQL having-clause. Dimensions cannot be used
    /// in this filter.
    #[prost(message, optional, tag = "6")]
    pub metric_filter: ::core::option::Option<AccessFilterExpression>,
    /// The row count of the start row. The first row is counted as row 0. If
    /// offset is unspecified, it is treated as 0. If offset is zero, then this
    /// method will return the first page of results with `limit` entries.
    ///
    /// To learn more about this pagination parameter, see
    /// \[Pagination\](<https://developers.google.com/analytics/devguides/reporting/data/v1/basics#pagination>).
    #[prost(int64, tag = "7")]
    pub offset: i64,
    /// The number of rows to return. If unspecified, 10,000 rows are returned. The
    /// API returns a maximum of 100,000 rows per request, no matter how many you
    /// ask for. `limit` must be positive.
    ///
    /// The API may return fewer rows than the requested `limit`, if there aren't
    /// as many remaining rows as the `limit`. For instance, there are fewer than
    /// 300 possible values for the dimension `country`, so when reporting on only
    /// `country`, you can't get more than 300 rows, even if you set `limit` to a
    /// higher value.
    ///
    /// To learn more about this pagination parameter, see
    /// \[Pagination\](<https://developers.google.com/analytics/devguides/reporting/data/v1/basics#pagination>).
    #[prost(int64, tag = "8")]
    pub limit: i64,
    /// This request's time zone if specified. If unspecified, the property's time
    /// zone is used. The request's time zone is used to interpret the start & end
    /// dates of the report.
    ///
    /// Formatted as strings from the IANA Time Zone database
    /// (<https://www.iana.org/time-zones>); for example "America/New_York" or
    /// "Asia/Tokyo".
    #[prost(string, tag = "9")]
    pub time_zone: ::prost::alloc::string::String,
    /// Specifies how rows are ordered in the response.
    #[prost(message, repeated, tag = "10")]
    pub order_bys: ::prost::alloc::vec::Vec<AccessOrderBy>,
    /// Toggles whether to return the current state of this Analytics Property's
    /// quota. Quota is returned in \[AccessQuota\](#AccessQuota). For account-level
    /// requests, this field must be false.
    #[prost(bool, tag = "11")]
    pub return_entity_quota: bool,
}
/// The customized Data Access Record Report response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunAccessReportResponse {
    /// The header for a column in the report that corresponds to a specific
    /// dimension. The number of DimensionHeaders and ordering of DimensionHeaders
    /// matches the dimensions present in rows.
    #[prost(message, repeated, tag = "1")]
    pub dimension_headers: ::prost::alloc::vec::Vec<AccessDimensionHeader>,
    /// The header for a column in the report that corresponds to a specific
    /// metric. The number of MetricHeaders and ordering of MetricHeaders matches
    /// the metrics present in rows.
    #[prost(message, repeated, tag = "2")]
    pub metric_headers: ::prost::alloc::vec::Vec<AccessMetricHeader>,
    /// Rows of dimension value combinations and metric values in the report.
    #[prost(message, repeated, tag = "3")]
    pub rows: ::prost::alloc::vec::Vec<AccessRow>,
    /// The total number of rows in the query result. `rowCount` is independent of
    /// the number of rows returned in the response, the `limit` request
    /// parameter, and the `offset` request parameter. For example if a query
    /// returns 175 rows and includes `limit` of 50 in the API request, the
    /// response will contain `rowCount` of 175 but only 50 rows.
    ///
    /// To learn more about this pagination parameter, see
    /// \[Pagination\](<https://developers.google.com/analytics/devguides/reporting/data/v1/basics#pagination>).
    #[prost(int32, tag = "4")]
    pub row_count: i32,
    /// The quota state for this Analytics property including this request. This
    /// field doesn't work with account-level requests.
    #[prost(message, optional, tag = "5")]
    pub quota: ::core::option::Option<AccessQuota>,
}
/// Request message for GetAccount RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountRequest {
    /// Required. The name of the account to lookup.
    /// Format: accounts/{account}
    /// Example: "accounts/100"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListAccounts RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAccountsRequest {
    /// The maximum number of resources to return. The service may return
    /// fewer than this value, even if there are additional pages.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200; (higher values will be coerced to the maximum)
    #[prost(int32, tag = "1")]
    pub page_size: i32,
    /// A page token, received from a previous `ListAccounts` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListAccounts` must
    /// match the call that provided the page token.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
    /// Whether to include soft-deleted (ie: "trashed") Accounts in the
    /// results. Accounts can be inspected to determine whether they are deleted or
    /// not.
    #[prost(bool, tag = "3")]
    pub show_deleted: bool,
}
/// Request message for ListAccounts RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAccountsResponse {
    /// Results that were accessible to the caller.
    #[prost(message, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<Account>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for DeleteAccount RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAccountRequest {
    /// Required. The name of the Account to soft-delete.
    /// Format: accounts/{account}
    /// Example: "accounts/100"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for UpdateAccount RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAccountRequest {
    /// Required. The account to update.
    /// The account's `name` field is used to identify the account.
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<Account>,
    /// Required. The list of fields to be updated. Field names must be in snake
    /// case (for example, "field_to_update"). Omitted fields will not be updated.
    /// To replace the entire entity, use one path with the string "*" to match all
    /// fields.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for ProvisionAccountTicket RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProvisionAccountTicketRequest {
    /// The account to create.
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<Account>,
    /// Redirect URI where the user will be sent after accepting Terms of Service.
    /// Must be configured in Cloud Console as a Redirect URI.
    #[prost(string, tag = "2")]
    pub redirect_uri: ::prost::alloc::string::String,
}
/// Response message for ProvisionAccountTicket RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProvisionAccountTicketResponse {
    /// The param to be passed in the ToS link.
    #[prost(string, tag = "1")]
    pub account_ticket_id: ::prost::alloc::string::String,
}
/// Request message for GetProperty RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPropertyRequest {
    /// Required. The name of the property to lookup.
    /// Format: properties/{property_id}
    /// Example: "properties/1000"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListProperties RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPropertiesRequest {
    /// Required. An expression for filtering the results of the request.
    /// Fields eligible for filtering are:
    /// `parent:`(The resource name of the parent account/property) or
    /// `ancestor:`(The resource name of the parent account) or
    /// `firebase_project:`(The id or number of the linked firebase project).
    /// Some examples of filters:
    ///
    /// ```
    /// | Filter                      | Description                               |
    /// |-----------------------------|-------------------------------------------|
    /// | parent:accounts/123         | The account with account id: 123.       |
    /// | parent:properties/123       | The property with property id: 123.       |
    /// | ancestor:accounts/123       | The account with account id: 123.         |
    /// | firebase_project:project-id | The firebase project with id: project-id. |
    /// | firebase_project:123        | The firebase project with number: 123.    |
    /// ```
    #[prost(string, tag = "1")]
    pub filter: ::prost::alloc::string::String,
    /// The maximum number of resources to return. The service may return
    /// fewer than this value, even if there are additional pages.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200; (higher values will be coerced to the maximum)
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListProperties` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListProperties` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Whether to include soft-deleted (ie: "trashed") Properties in the
    /// results. Properties can be inspected to determine whether they are deleted
    /// or not.
    #[prost(bool, tag = "4")]
    pub show_deleted: bool,
}
/// Response message for ListProperties RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPropertiesResponse {
    /// Results that matched the filter criteria and were accessible to the caller.
    #[prost(message, repeated, tag = "1")]
    pub properties: ::prost::alloc::vec::Vec<Property>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for UpdateProperty RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePropertyRequest {
    /// Required. The property to update.
    /// The property's `name` field is used to identify the property to be
    /// updated.
    #[prost(message, optional, tag = "1")]
    pub property: ::core::option::Option<Property>,
    /// Required. The list of fields to be updated. Field names must be in snake
    /// case (e.g., "field_to_update"). Omitted fields will not be updated. To
    /// replace the entire entity, use one path with the string "*" to match all
    /// fields.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for CreateProperty RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePropertyRequest {
    /// Required. The property to create.
    /// Note: the supplied property must specify its parent.
    #[prost(message, optional, tag = "1")]
    pub property: ::core::option::Option<Property>,
}
/// Request message for DeleteProperty RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePropertyRequest {
    /// Required. The name of the Property to soft-delete.
    /// Format: properties/{property_id}
    /// Example: "properties/1000"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for CreateFirebaseLink RPC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFirebaseLinkRequest {
    /// Required. Format: properties/{property_id}
    /// Example: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The Firebase link to create.
    #[prost(message, optional, tag = "2")]
    pub firebase_link: ::core::option::Option<FirebaseLink>,
}
/// Request message for DeleteFirebaseLink RPC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFirebaseLinkRequest {
    /// Required. Format: properties/{property_id}/firebaseLinks/{firebase_link_id}
    /// Example: properties/1234/firebaseLinks/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListFirebaseLinks RPC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFirebaseLinksRequest {
    /// Required. Format: properties/{property_id}
    /// Example: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to return. The service may return
    /// fewer than this value, even if there are additional pages.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200; (higher values will be coerced to the maximum)
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListFirebaseLinks` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListFirebaseLinks` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListFirebaseLinks RPC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFirebaseLinksResponse {
    /// List of FirebaseLinks. This will have at most one value.
    #[prost(message, repeated, tag = "1")]
    pub firebase_links: ::prost::alloc::vec::Vec<FirebaseLink>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    /// Currently, Google Analytics supports only one FirebaseLink per property,
    /// so this will never be populated.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for CreateGoogleAdsLink RPC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateGoogleAdsLinkRequest {
    /// Required. Example format: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The GoogleAdsLink to create.
    #[prost(message, optional, tag = "2")]
    pub google_ads_link: ::core::option::Option<GoogleAdsLink>,
}
/// Request message for UpdateGoogleAdsLink RPC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGoogleAdsLinkRequest {
    /// The GoogleAdsLink to update
    #[prost(message, optional, tag = "1")]
    pub google_ads_link: ::core::option::Option<GoogleAdsLink>,
    /// Required. The list of fields to be updated. Field names must be in snake
    /// case (e.g., "field_to_update"). Omitted fields will not be updated. To
    /// replace the entire entity, use one path with the string "*" to match all
    /// fields.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for DeleteGoogleAdsLink RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteGoogleAdsLinkRequest {
    /// Required. Example format: properties/1234/googleAdsLinks/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListGoogleAdsLinks RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGoogleAdsLinksRequest {
    /// Required. Example format: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to return.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200 (higher values will be coerced to the maximum).
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListGoogleAdsLinks` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListGoogleAdsLinks` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListGoogleAdsLinks RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGoogleAdsLinksResponse {
    /// List of GoogleAdsLinks.
    #[prost(message, repeated, tag = "1")]
    pub google_ads_links: ::prost::alloc::vec::Vec<GoogleAdsLink>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for GetDataSharingSettings RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataSharingSettingsRequest {
    /// Required. The name of the settings to lookup.
    /// Format: accounts/{account}/dataSharingSettings
    /// Example: "accounts/1000/dataSharingSettings"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListAccountSummaries RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAccountSummariesRequest {
    /// The maximum number of AccountSummary resources to return. The service may
    /// return fewer than this value, even if there are additional pages.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200; (higher values will be coerced to the maximum)
    #[prost(int32, tag = "1")]
    pub page_size: i32,
    /// A page token, received from a previous `ListAccountSummaries` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListAccountSummaries`
    /// must match the call that provided the page token.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListAccountSummaries RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAccountSummariesResponse {
    /// Account summaries of all accounts the caller has access to.
    #[prost(message, repeated, tag = "1")]
    pub account_summaries: ::prost::alloc::vec::Vec<AccountSummary>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for AcknowledgeUserDataCollection RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcknowledgeUserDataCollectionRequest {
    /// Required. The property for which to acknowledge user data collection.
    #[prost(string, tag = "1")]
    pub property: ::prost::alloc::string::String,
    /// Required. An acknowledgement that the caller of this method understands the
    /// terms of user data collection.
    ///
    /// This field must contain the exact value:
    /// "I acknowledge that I have the necessary privacy disclosures and rights
    /// from my end users for the collection and processing of their data,
    /// including the association of such data with the visitation information
    /// Google Analytics collects from my site and/or app property."
    #[prost(string, tag = "2")]
    pub acknowledgement: ::prost::alloc::string::String,
}
/// Response message for AcknowledgeUserDataCollection RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcknowledgeUserDataCollectionResponse {}
/// Request message for SearchChangeHistoryEvents RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchChangeHistoryEventsRequest {
    /// Required. The account resource for which to return change history
    /// resources.
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    /// Optional. Resource name for a child property. If set, only return changes
    /// made to this property or its child resources.
    #[prost(string, tag = "2")]
    pub property: ::prost::alloc::string::String,
    /// Optional. If set, only return changes if they are for a resource that
    /// matches at least one of these types.
    #[prost(
        enumeration = "ChangeHistoryResourceType",
        repeated,
        packed = "false",
        tag = "3"
    )]
    pub resource_type: ::prost::alloc::vec::Vec<i32>,
    /// Optional. If set, only return changes that match one or more of these types
    /// of actions.
    #[prost(enumeration = "ActionType", repeated, packed = "false", tag = "4")]
    pub action: ::prost::alloc::vec::Vec<i32>,
    /// Optional. If set, only return changes if they are made by a user in this
    /// list.
    #[prost(string, repeated, tag = "5")]
    pub actor_email: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. If set, only return changes made after this time (inclusive).
    #[prost(message, optional, tag = "6")]
    pub earliest_change_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. If set, only return changes made before this time (inclusive).
    #[prost(message, optional, tag = "7")]
    pub latest_change_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. The maximum number of ChangeHistoryEvent items to return.
    /// The service may return fewer than this value, even if there are additional
    /// pages. If unspecified, at most 50 items will be returned.
    /// The maximum value is 200 (higher values will be coerced to the maximum).
    #[prost(int32, tag = "8")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous
    /// `SearchChangeHistoryEvents` call. Provide this to retrieve the subsequent
    /// page. When paginating, all other parameters provided to
    /// `SearchChangeHistoryEvents` must match the call that provided the page
    /// token.
    #[prost(string, tag = "9")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for SearchAccounts RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchChangeHistoryEventsResponse {
    /// Results that were accessible to the caller.
    #[prost(message, repeated, tag = "1")]
    pub change_history_events: ::prost::alloc::vec::Vec<ChangeHistoryEvent>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for GetMeasurementProtocolSecret RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMeasurementProtocolSecretRequest {
    /// Required. The name of the measurement protocol secret to lookup.
    /// Format:
    /// properties/{property}/dataStreams/{dataStream}/measurementProtocolSecrets/{measurementProtocolSecret}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for CreateMeasurementProtocolSecret RPC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMeasurementProtocolSecretRequest {
    /// Required. The parent resource where this secret will be created.
    /// Format: properties/{property}/dataStreams/{dataStream}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The measurement protocol secret to create.
    #[prost(message, optional, tag = "2")]
    pub measurement_protocol_secret: ::core::option::Option<MeasurementProtocolSecret>,
}
/// Request message for DeleteMeasurementProtocolSecret RPC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteMeasurementProtocolSecretRequest {
    /// Required. The name of the MeasurementProtocolSecret to delete.
    /// Format:
    /// properties/{property}/dataStreams/{dataStream}/measurementProtocolSecrets/{measurementProtocolSecret}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for UpdateMeasurementProtocolSecret RPC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMeasurementProtocolSecretRequest {
    /// Required. The measurement protocol secret to update.
    #[prost(message, optional, tag = "1")]
    pub measurement_protocol_secret: ::core::option::Option<MeasurementProtocolSecret>,
    /// The list of fields to be updated. Omitted fields will not be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for ListMeasurementProtocolSecret RPC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMeasurementProtocolSecretsRequest {
    /// Required. The resource name of the parent stream.
    /// Format:
    /// properties/{property}/dataStreams/{dataStream}/measurementProtocolSecrets
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to return.
    /// If unspecified, at most 10 resources will be returned.
    /// The maximum value is 10. Higher values will be coerced to the maximum.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListMeasurementProtocolSecrets`
    /// call. Provide this to retrieve the subsequent page. When paginating, all
    /// other parameters provided to `ListMeasurementProtocolSecrets` must match
    /// the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListMeasurementProtocolSecret RPC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMeasurementProtocolSecretsResponse {
    /// A list of secrets for the parent stream specified in the request.
    #[prost(message, repeated, tag = "1")]
    pub measurement_protocol_secrets: ::prost::alloc::vec::Vec<
        MeasurementProtocolSecret,
    >,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for CreateConversionEvent RPC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateConversionEventRequest {
    /// Required. The conversion event to create.
    #[prost(message, optional, tag = "1")]
    pub conversion_event: ::core::option::Option<ConversionEvent>,
    /// Required. The resource name of the parent property where this conversion
    /// event will be created. Format: properties/123
    #[prost(string, tag = "2")]
    pub parent: ::prost::alloc::string::String,
}
/// Request message for GetConversionEvent RPC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConversionEventRequest {
    /// Required. The resource name of the conversion event to retrieve.
    /// Format: properties/{property}/conversionEvents/{conversion_event}
    /// Example: "properties/123/conversionEvents/456"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for DeleteConversionEvent RPC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteConversionEventRequest {
    /// Required. The resource name of the conversion event to delete.
    /// Format: properties/{property}/conversionEvents/{conversion_event}
    /// Example: "properties/123/conversionEvents/456"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListConversionEvents RPC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConversionEventsRequest {
    /// Required. The resource name of the parent property.
    /// Example: 'properties/123'
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to return.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200; (higher values will be coerced to the maximum)
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListConversionEvents` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListConversionEvents`
    /// must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListConversionEvents RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConversionEventsResponse {
    /// The requested conversion events
    #[prost(message, repeated, tag = "1")]
    pub conversion_events: ::prost::alloc::vec::Vec<ConversionEvent>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for CreateCustomDimension RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCustomDimensionRequest {
    /// Required. Example format: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The CustomDimension to create.
    #[prost(message, optional, tag = "2")]
    pub custom_dimension: ::core::option::Option<CustomDimension>,
}
/// Request message for UpdateCustomDimension RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCustomDimensionRequest {
    /// The CustomDimension to update
    #[prost(message, optional, tag = "1")]
    pub custom_dimension: ::core::option::Option<CustomDimension>,
    /// Required. The list of fields to be updated. Omitted fields will not be
    /// updated. To replace the entire entity, use one path with the string "*" to
    /// match all fields.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for ListCustomDimensions RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCustomDimensionsRequest {
    /// Required. Example format: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to return.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200 (higher values will be coerced to the maximum).
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListCustomDimensions` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListCustomDimensions`
    /// must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListCustomDimensions RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCustomDimensionsResponse {
    /// List of CustomDimensions.
    #[prost(message, repeated, tag = "1")]
    pub custom_dimensions: ::prost::alloc::vec::Vec<CustomDimension>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for ArchiveCustomDimension RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArchiveCustomDimensionRequest {
    /// Required. The name of the CustomDimension to archive.
    /// Example format: properties/1234/customDimensions/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for GetCustomDimension RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomDimensionRequest {
    /// Required. The name of the CustomDimension to get.
    /// Example format: properties/1234/customDimensions/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for CreateCustomMetric RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCustomMetricRequest {
    /// Required. Example format: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The CustomMetric to create.
    #[prost(message, optional, tag = "2")]
    pub custom_metric: ::core::option::Option<CustomMetric>,
}
/// Request message for UpdateCustomMetric RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCustomMetricRequest {
    /// The CustomMetric to update
    #[prost(message, optional, tag = "1")]
    pub custom_metric: ::core::option::Option<CustomMetric>,
    /// Required. The list of fields to be updated. Omitted fields will not be
    /// updated. To replace the entire entity, use one path with the string "*" to
    /// match all fields.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for ListCustomMetrics RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCustomMetricsRequest {
    /// Required. Example format: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to return.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200 (higher values will be coerced to the maximum).
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListCustomMetrics` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListCustomMetrics` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListCustomMetrics RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCustomMetricsResponse {
    /// List of CustomMetrics.
    #[prost(message, repeated, tag = "1")]
    pub custom_metrics: ::prost::alloc::vec::Vec<CustomMetric>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for ArchiveCustomMetric RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArchiveCustomMetricRequest {
    /// Required. The name of the CustomMetric to archive.
    /// Example format: properties/1234/customMetrics/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for GetCustomMetric RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomMetricRequest {
    /// Required. The name of the CustomMetric to get.
    /// Example format: properties/1234/customMetrics/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for GetDataRetentionSettings RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataRetentionSettingsRequest {
    /// Required. The name of the settings to lookup.
    /// Format:
    /// properties/{property}/dataRetentionSettings
    /// Example: "properties/1000/dataRetentionSettings"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for UpdateDataRetentionSettings RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDataRetentionSettingsRequest {
    /// Required. The settings to update.
    /// The `name` field is used to identify the settings to be updated.
    #[prost(message, optional, tag = "1")]
    pub data_retention_settings: ::core::option::Option<DataRetentionSettings>,
    /// Required. The list of fields to be updated. Field names must be in snake
    /// case (e.g., "field_to_update"). Omitted fields will not be updated. To
    /// replace the entire entity, use one path with the string "*" to match all
    /// fields.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for CreateDataStream RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDataStreamRequest {
    /// Required. Example format: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The DataStream to create.
    #[prost(message, optional, tag = "2")]
    pub data_stream: ::core::option::Option<DataStream>,
}
/// Request message for DeleteDataStream RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDataStreamRequest {
    /// Required. The name of the DataStream to delete.
    /// Example format: properties/1234/dataStreams/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for UpdateDataStream RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDataStreamRequest {
    /// The DataStream to update
    #[prost(message, optional, tag = "1")]
    pub data_stream: ::core::option::Option<DataStream>,
    /// Required. The list of fields to be updated. Omitted fields will not be
    /// updated. To replace the entire entity, use one path with the string "*" to
    /// match all fields.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for ListDataStreams RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDataStreamsRequest {
    /// Required. Example format: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to return.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200 (higher values will be coerced to the maximum).
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListDataStreams` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListDataStreams` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListDataStreams RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDataStreamsResponse {
    /// List of DataStreams.
    #[prost(message, repeated, tag = "1")]
    pub data_streams: ::prost::alloc::vec::Vec<DataStream>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for GetDataStream RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataStreamRequest {
    /// Required. The name of the DataStream to get.
    /// Example format: properties/1234/dataStreams/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod analytics_admin_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service Interface for the Analytics Admin API (GA4).
    #[derive(Debug, Clone)]
    pub struct AnalyticsAdminServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AnalyticsAdminServiceClient<tonic::transport::Channel> {
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
    impl<T> AnalyticsAdminServiceClient<T>
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
        ) -> AnalyticsAdminServiceClient<InterceptedService<T, F>>
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
            AnalyticsAdminServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lookup for a single Account.
        pub async fn get_account(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAccountRequest>,
        ) -> std::result::Result<tonic::Response<super::Account>, tonic::Status> {
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/GetAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "GetAccount",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns all accounts accessible by the caller.
        ///
        /// Note that these accounts might not currently have GA4 properties.
        /// Soft-deleted (ie: "trashed") accounts are excluded by default.
        /// Returns an empty list if no relevant accounts are found.
        pub async fn list_accounts(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAccountsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAccountsResponse>,
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/ListAccounts",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "ListAccounts",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Marks target Account as soft-deleted (ie: "trashed") and returns it.
        ///
        /// This API does not have a method to restore soft-deleted accounts.
        /// However, they can be restored using the Trash Can UI.
        ///
        /// If the accounts are not restored before the expiration time, the account
        /// and all child resources (eg: Properties, GoogleAdsLinks, Streams,
        /// UserLinks) will be permanently purged.
        /// https://support.google.com/analytics/answer/6154772
        ///
        /// Returns an error if the target is not found.
        pub async fn delete_account(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAccountRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/DeleteAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "DeleteAccount",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates an account.
        pub async fn update_account(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAccountRequest>,
        ) -> std::result::Result<tonic::Response<super::Account>, tonic::Status> {
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/UpdateAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "UpdateAccount",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Requests a ticket for creating an account.
        pub async fn provision_account_ticket(
            &mut self,
            request: impl tonic::IntoRequest<super::ProvisionAccountTicketRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ProvisionAccountTicketResponse>,
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/ProvisionAccountTicket",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "ProvisionAccountTicket",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns summaries of all accounts accessible by the caller.
        pub async fn list_account_summaries(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAccountSummariesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAccountSummariesResponse>,
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/ListAccountSummaries",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "ListAccountSummaries",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lookup for a single "GA4" Property.
        pub async fn get_property(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPropertyRequest>,
        ) -> std::result::Result<tonic::Response<super::Property>, tonic::Status> {
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/GetProperty",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "GetProperty",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns child Properties under the specified parent Account.
        ///
        /// Only "GA4" properties will be returned.
        /// Properties will be excluded if the caller does not have access.
        /// Soft-deleted (ie: "trashed") properties are excluded by default.
        /// Returns an empty list if no relevant properties are found.
        pub async fn list_properties(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPropertiesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPropertiesResponse>,
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/ListProperties",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "ListProperties",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates an "GA4" property with the specified location and attributes.
        pub async fn create_property(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePropertyRequest>,
        ) -> std::result::Result<tonic::Response<super::Property>, tonic::Status> {
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/CreateProperty",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "CreateProperty",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Marks target Property as soft-deleted (ie: "trashed") and returns it.
        ///
        /// This API does not have a method to restore soft-deleted properties.
        /// However, they can be restored using the Trash Can UI.
        ///
        /// If the properties are not restored before the expiration time, the Property
        /// and all child resources (eg: GoogleAdsLinks, Streams, UserLinks)
        /// will be permanently purged.
        /// https://support.google.com/analytics/answer/6154772
        ///
        /// Returns an error if the target is not found, or is not a GA4 Property.
        pub async fn delete_property(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePropertyRequest>,
        ) -> std::result::Result<tonic::Response<super::Property>, tonic::Status> {
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/DeleteProperty",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "DeleteProperty",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a property.
        pub async fn update_property(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePropertyRequest>,
        ) -> std::result::Result<tonic::Response<super::Property>, tonic::Status> {
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/UpdateProperty",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "UpdateProperty",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a FirebaseLink.
        ///
        /// Properties can have at most one FirebaseLink.
        pub async fn create_firebase_link(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateFirebaseLinkRequest>,
        ) -> std::result::Result<tonic::Response<super::FirebaseLink>, tonic::Status> {
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/CreateFirebaseLink",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "CreateFirebaseLink",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a FirebaseLink on a property
        pub async fn delete_firebase_link(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteFirebaseLinkRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/DeleteFirebaseLink",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "DeleteFirebaseLink",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists FirebaseLinks on a property.
        /// Properties can have at most one FirebaseLink.
        pub async fn list_firebase_links(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFirebaseLinksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListFirebaseLinksResponse>,
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/ListFirebaseLinks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "ListFirebaseLinks",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a GoogleAdsLink.
        pub async fn create_google_ads_link(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateGoogleAdsLinkRequest>,
        ) -> std::result::Result<tonic::Response<super::GoogleAdsLink>, tonic::Status> {
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/CreateGoogleAdsLink",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "CreateGoogleAdsLink",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a GoogleAdsLink on a property
        pub async fn update_google_ads_link(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateGoogleAdsLinkRequest>,
        ) -> std::result::Result<tonic::Response<super::GoogleAdsLink>, tonic::Status> {
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/UpdateGoogleAdsLink",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "UpdateGoogleAdsLink",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a GoogleAdsLink on a property
        pub async fn delete_google_ads_link(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteGoogleAdsLinkRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/DeleteGoogleAdsLink",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "DeleteGoogleAdsLink",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists GoogleAdsLinks on a property.
        pub async fn list_google_ads_links(
            &mut self,
            request: impl tonic::IntoRequest<super::ListGoogleAdsLinksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListGoogleAdsLinksResponse>,
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/ListGoogleAdsLinks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "ListGoogleAdsLinks",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get data sharing settings on an account.
        /// Data sharing settings are singletons.
        pub async fn get_data_sharing_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDataSharingSettingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DataSharingSettings>,
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/GetDataSharingSettings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "GetDataSharingSettings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lookup for a single "GA4" MeasurementProtocolSecret.
        pub async fn get_measurement_protocol_secret(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMeasurementProtocolSecretRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MeasurementProtocolSecret>,
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/GetMeasurementProtocolSecret",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "GetMeasurementProtocolSecret",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns child MeasurementProtocolSecrets under the specified parent
        /// Property.
        pub async fn list_measurement_protocol_secrets(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ListMeasurementProtocolSecretsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ListMeasurementProtocolSecretsResponse>,
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/ListMeasurementProtocolSecrets",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "ListMeasurementProtocolSecrets",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a measurement protocol secret.
        pub async fn create_measurement_protocol_secret(
            &mut self,
            request: impl tonic::IntoRequest<
                super::CreateMeasurementProtocolSecretRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::MeasurementProtocolSecret>,
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/CreateMeasurementProtocolSecret",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "CreateMeasurementProtocolSecret",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes target MeasurementProtocolSecret.
        pub async fn delete_measurement_protocol_secret(
            &mut self,
            request: impl tonic::IntoRequest<
                super::DeleteMeasurementProtocolSecretRequest,
            >,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/DeleteMeasurementProtocolSecret",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "DeleteMeasurementProtocolSecret",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a measurement protocol secret.
        pub async fn update_measurement_protocol_secret(
            &mut self,
            request: impl tonic::IntoRequest<
                super::UpdateMeasurementProtocolSecretRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::MeasurementProtocolSecret>,
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/UpdateMeasurementProtocolSecret",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "UpdateMeasurementProtocolSecret",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Acknowledges the terms of user data collection for the specified property.
        ///
        /// This acknowledgement must be completed (either in the Google Analytics UI
        /// or through this API) before MeasurementProtocolSecret resources may be
        /// created.
        pub async fn acknowledge_user_data_collection(
            &mut self,
            request: impl tonic::IntoRequest<super::AcknowledgeUserDataCollectionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AcknowledgeUserDataCollectionResponse>,
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/AcknowledgeUserDataCollection",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "AcknowledgeUserDataCollection",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Searches through all changes to an account or its children given the
        /// specified set of filters.
        pub async fn search_change_history_events(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchChangeHistoryEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SearchChangeHistoryEventsResponse>,
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/SearchChangeHistoryEvents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "SearchChangeHistoryEvents",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a conversion event with the specified attributes.
        pub async fn create_conversion_event(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateConversionEventRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ConversionEvent>,
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/CreateConversionEvent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "CreateConversionEvent",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieve a single conversion event.
        pub async fn get_conversion_event(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConversionEventRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ConversionEvent>,
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/GetConversionEvent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "GetConversionEvent",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a conversion event in a property.
        pub async fn delete_conversion_event(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteConversionEventRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/DeleteConversionEvent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "DeleteConversionEvent",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns a list of conversion events in the specified parent property.
        ///
        /// Returns an empty list if no conversion events are found.
        pub async fn list_conversion_events(
            &mut self,
            request: impl tonic::IntoRequest<super::ListConversionEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListConversionEventsResponse>,
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/ListConversionEvents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "ListConversionEvents",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a CustomDimension.
        pub async fn create_custom_dimension(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCustomDimensionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CustomDimension>,
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/CreateCustomDimension",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "CreateCustomDimension",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a CustomDimension on a property.
        pub async fn update_custom_dimension(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCustomDimensionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CustomDimension>,
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/UpdateCustomDimension",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "UpdateCustomDimension",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists CustomDimensions on a property.
        pub async fn list_custom_dimensions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCustomDimensionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCustomDimensionsResponse>,
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/ListCustomDimensions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "ListCustomDimensions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Archives a CustomDimension on a property.
        pub async fn archive_custom_dimension(
            &mut self,
            request: impl tonic::IntoRequest<super::ArchiveCustomDimensionRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/ArchiveCustomDimension",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "ArchiveCustomDimension",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lookup for a single CustomDimension.
        pub async fn get_custom_dimension(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCustomDimensionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CustomDimension>,
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/GetCustomDimension",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "GetCustomDimension",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a CustomMetric.
        pub async fn create_custom_metric(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCustomMetricRequest>,
        ) -> std::result::Result<tonic::Response<super::CustomMetric>, tonic::Status> {
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/CreateCustomMetric",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "CreateCustomMetric",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a CustomMetric on a property.
        pub async fn update_custom_metric(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCustomMetricRequest>,
        ) -> std::result::Result<tonic::Response<super::CustomMetric>, tonic::Status> {
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/UpdateCustomMetric",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "UpdateCustomMetric",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists CustomMetrics on a property.
        pub async fn list_custom_metrics(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCustomMetricsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCustomMetricsResponse>,
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/ListCustomMetrics",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "ListCustomMetrics",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Archives a CustomMetric on a property.
        pub async fn archive_custom_metric(
            &mut self,
            request: impl tonic::IntoRequest<super::ArchiveCustomMetricRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/ArchiveCustomMetric",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "ArchiveCustomMetric",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lookup for a single CustomMetric.
        pub async fn get_custom_metric(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCustomMetricRequest>,
        ) -> std::result::Result<tonic::Response<super::CustomMetric>, tonic::Status> {
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/GetCustomMetric",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "GetCustomMetric",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the singleton data retention settings for this property.
        pub async fn get_data_retention_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDataRetentionSettingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DataRetentionSettings>,
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/GetDataRetentionSettings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "GetDataRetentionSettings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the singleton data retention settings for this property.
        pub async fn update_data_retention_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDataRetentionSettingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DataRetentionSettings>,
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/UpdateDataRetentionSettings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "UpdateDataRetentionSettings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a DataStream.
        pub async fn create_data_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDataStreamRequest>,
        ) -> std::result::Result<tonic::Response<super::DataStream>, tonic::Status> {
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/CreateDataStream",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "CreateDataStream",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a DataStream on a property.
        pub async fn delete_data_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDataStreamRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/DeleteDataStream",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "DeleteDataStream",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a DataStream on a property.
        pub async fn update_data_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDataStreamRequest>,
        ) -> std::result::Result<tonic::Response<super::DataStream>, tonic::Status> {
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/UpdateDataStream",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "UpdateDataStream",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists DataStreams on a property.
        pub async fn list_data_streams(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDataStreamsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDataStreamsResponse>,
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/ListDataStreams",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "ListDataStreams",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lookup for a single DataStream.
        pub async fn get_data_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDataStreamRequest>,
        ) -> std::result::Result<tonic::Response<super::DataStream>, tonic::Status> {
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/GetDataStream",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "GetDataStream",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns a customized report of data access records. The report provides
        /// records of each time a user reads Google Analytics reporting data. Access
        /// records are retained for up to 2 years.
        ///
        /// Data Access Reports can be requested for a property. The property must be
        /// in Google Analytics 360. This method is only available to Administrators.
        ///
        /// These data access records include GA4 UI Reporting, GA4 UI Explorations,
        /// GA4 Data API, and other products like Firebase & Admob that can retrieve
        /// data from Google Analytics through a linkage. These records don't include
        /// property configuration changes like adding a stream or changing a
        /// property's time zone. For configuration change history, see
        /// [searchChangeHistoryEvents](https://developers.google.com/analytics/devguides/config/admin/v1/rest/v1alpha/accounts/searchChangeHistoryEvents).
        pub async fn run_access_report(
            &mut self,
            request: impl tonic::IntoRequest<super::RunAccessReportRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RunAccessReportResponse>,
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
                "/google.analytics.admin.v1beta.AnalyticsAdminService/RunAccessReport",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.admin.v1beta.AnalyticsAdminService",
                        "RunAccessReport",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
