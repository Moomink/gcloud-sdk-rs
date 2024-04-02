/// Information about the author of the UGC data. Used in
/// [Photo][google.maps.places.v1.Photo], and
/// [Review][google.maps.places.v1.Review].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorAttribution {
    /// Name of the author of the [Photo][google.maps.places.v1.Photo] or
    /// [Review][google.maps.places.v1.Review].
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// URI of the author of the [Photo][google.maps.places.v1.Photo] or
    /// [Review][google.maps.places.v1.Review].
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// Profile photo URI of the author of the
    /// [Photo][google.maps.places.v1.Photo] or
    /// [Review][google.maps.places.v1.Review].
    #[prost(string, tag = "3")]
    pub photo_uri: ::prost::alloc::string::String,
}
/// Information about the EV Charge Station hosted in Place.
/// Terminology follows
/// <https://afdc.energy.gov/fuels/electricity_infrastructure.html> One port
/// could charge one car at a time. One port has one or more connectors. One
/// station has one or more ports.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvChargeOptions {
    /// Number of connectors at this station. However, because some ports can have
    /// multiple connectors but only be able to charge one car at a time (e.g.) the
    /// number of connectors may be greater than the total number of cars which can
    /// charge simultaneously.
    #[prost(int32, tag = "1")]
    pub connector_count: i32,
    /// A list of EV charging connector aggregations that contain connectors of the
    /// same type and same charge rate.
    #[prost(message, repeated, tag = "2")]
    pub connector_aggregation: ::prost::alloc::vec::Vec<
        ev_charge_options::ConnectorAggregation,
    >,
}
/// Nested message and enum types in `EVChargeOptions`.
pub mod ev_charge_options {
    /// EV charging information grouped by \[type, max_charge_rate_kw\].
    /// Shows EV charge aggregation of connectors that have the same type and max
    /// charge rate in kw.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConnectorAggregation {
        /// The connector type of this aggregation.
        #[prost(enumeration = "super::EvConnectorType", tag = "1")]
        pub r#type: i32,
        /// The static max charging rate in kw of each connector in the aggregation.
        #[prost(double, tag = "2")]
        pub max_charge_rate_kw: f64,
        /// Number of connectors in this aggregation.
        #[prost(int32, tag = "3")]
        pub count: i32,
        /// Number of connectors in this aggregation that are currently available.
        #[prost(int32, optional, tag = "4")]
        pub available_count: ::core::option::Option<i32>,
        /// Number of connectors in this aggregation that are currently out of
        /// service.
        #[prost(int32, optional, tag = "5")]
        pub out_of_service_count: ::core::option::Option<i32>,
        /// The timestamp when the connector availability information in this
        /// aggregation was last updated.
        #[prost(message, optional, tag = "6")]
        pub availability_last_update_time: ::core::option::Option<
            ::prost_types::Timestamp,
        >,
    }
}
/// See <http://ieeexplore.ieee.org/stamp/stamp.jsp?arnumber=6872107> for
/// additional information/context on EV charging connector types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EvConnectorType {
    /// Unspecified connector.
    Unspecified = 0,
    /// Other connector types.
    Other = 1,
    /// J1772 type 1 connector.
    J1772 = 2,
    /// IEC 62196 type 2 connector. Often referred to as MENNEKES.
    Type2 = 3,
    /// CHAdeMO type connector.
    Chademo = 4,
    /// Combined Charging System (AC and DC). Based on SAE.
    ///   Type-1 J-1772 connector
    CcsCombo1 = 5,
    /// Combined Charging System (AC and DC). Based on Type-2
    /// Mennekes connector
    CcsCombo2 = 6,
    /// The generic TESLA connector. This is NACS in the North America but can be
    /// non-NACS in other parts of the world (e.g. CCS Combo 2 (CCS2) or GB/T).
    /// This value is less representative of an actual connector type, and more
    /// represents the ability to charge a Tesla brand vehicle at a Tesla owned
    /// charging station.
    Tesla = 7,
    /// GB/T type corresponds to the GB/T standard in China. This type covers all
    /// GB_T types.
    UnspecifiedGbT = 8,
    /// Unspecified wall outlet.
    UnspecifiedWallOutlet = 9,
}
impl EvConnectorType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EvConnectorType::Unspecified => "EV_CONNECTOR_TYPE_UNSPECIFIED",
            EvConnectorType::Other => "EV_CONNECTOR_TYPE_OTHER",
            EvConnectorType::J1772 => "EV_CONNECTOR_TYPE_J1772",
            EvConnectorType::Type2 => "EV_CONNECTOR_TYPE_TYPE_2",
            EvConnectorType::Chademo => "EV_CONNECTOR_TYPE_CHADEMO",
            EvConnectorType::CcsCombo1 => "EV_CONNECTOR_TYPE_CCS_COMBO_1",
            EvConnectorType::CcsCombo2 => "EV_CONNECTOR_TYPE_CCS_COMBO_2",
            EvConnectorType::Tesla => "EV_CONNECTOR_TYPE_TESLA",
            EvConnectorType::UnspecifiedGbT => "EV_CONNECTOR_TYPE_UNSPECIFIED_GB_T",
            EvConnectorType::UnspecifiedWallOutlet => {
                "EV_CONNECTOR_TYPE_UNSPECIFIED_WALL_OUTLET"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EV_CONNECTOR_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "EV_CONNECTOR_TYPE_OTHER" => Some(Self::Other),
            "EV_CONNECTOR_TYPE_J1772" => Some(Self::J1772),
            "EV_CONNECTOR_TYPE_TYPE_2" => Some(Self::Type2),
            "EV_CONNECTOR_TYPE_CHADEMO" => Some(Self::Chademo),
            "EV_CONNECTOR_TYPE_CCS_COMBO_1" => Some(Self::CcsCombo1),
            "EV_CONNECTOR_TYPE_CCS_COMBO_2" => Some(Self::CcsCombo2),
            "EV_CONNECTOR_TYPE_TESLA" => Some(Self::Tesla),
            "EV_CONNECTOR_TYPE_UNSPECIFIED_GB_T" => Some(Self::UnspecifiedGbT),
            "EV_CONNECTOR_TYPE_UNSPECIFIED_WALL_OUTLET" => {
                Some(Self::UnspecifiedWallOutlet)
            }
            _ => None,
        }
    }
}
/// The most recent information about fuel options in a gas station. This
/// information is updated regularly.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FuelOptions {
    /// The last known fuel price for each type of fuel this station has. There is
    /// one entry per fuel type this station has. Order is not important.
    #[prost(message, repeated, tag = "1")]
    pub fuel_prices: ::prost::alloc::vec::Vec<fuel_options::FuelPrice>,
}
/// Nested message and enum types in `FuelOptions`.
pub mod fuel_options {
    /// Fuel price information for a given type.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FuelPrice {
        /// The type of fuel.
        #[prost(enumeration = "fuel_price::FuelType", tag = "1")]
        pub r#type: i32,
        /// The price of the fuel.
        #[prost(message, optional, tag = "2")]
        pub price: ::core::option::Option<super::super::super::super::r#type::Money>,
        /// The time the fuel price was last updated.
        #[prost(message, optional, tag = "3")]
        pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    }
    /// Nested message and enum types in `FuelPrice`.
    pub mod fuel_price {
        /// Types of fuel.
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
        pub enum FuelType {
            /// Unspecified fuel type.
            Unspecified = 0,
            /// Diesel fuel.
            Diesel = 1,
            /// Regular unleaded.
            RegularUnleaded = 2,
            /// Midgrade.
            Midgrade = 3,
            /// Premium.
            Premium = 4,
            /// SP 91.
            Sp91 = 5,
            /// SP 91 E10.
            Sp91E10 = 6,
            /// SP 92.
            Sp92 = 7,
            /// SP 95.
            Sp95 = 8,
            /// SP95 E10.
            Sp95E10 = 9,
            /// SP 98.
            Sp98 = 10,
            /// SP 99.
            Sp99 = 11,
            /// SP 100.
            Sp100 = 12,
            /// LPG.
            Lpg = 13,
            /// E 80.
            E80 = 14,
            /// E 85.
            E85 = 15,
            /// Methane.
            Methane = 16,
            /// Bio-diesel.
            BioDiesel = 17,
            /// Truck diesel.
            TruckDiesel = 18,
        }
        impl FuelType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    FuelType::Unspecified => "FUEL_TYPE_UNSPECIFIED",
                    FuelType::Diesel => "DIESEL",
                    FuelType::RegularUnleaded => "REGULAR_UNLEADED",
                    FuelType::Midgrade => "MIDGRADE",
                    FuelType::Premium => "PREMIUM",
                    FuelType::Sp91 => "SP91",
                    FuelType::Sp91E10 => "SP91_E10",
                    FuelType::Sp92 => "SP92",
                    FuelType::Sp95 => "SP95",
                    FuelType::Sp95E10 => "SP95_E10",
                    FuelType::Sp98 => "SP98",
                    FuelType::Sp99 => "SP99",
                    FuelType::Sp100 => "SP100",
                    FuelType::Lpg => "LPG",
                    FuelType::E80 => "E80",
                    FuelType::E85 => "E85",
                    FuelType::Methane => "METHANE",
                    FuelType::BioDiesel => "BIO_DIESEL",
                    FuelType::TruckDiesel => "TRUCK_DIESEL",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "FUEL_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                    "DIESEL" => Some(Self::Diesel),
                    "REGULAR_UNLEADED" => Some(Self::RegularUnleaded),
                    "MIDGRADE" => Some(Self::Midgrade),
                    "PREMIUM" => Some(Self::Premium),
                    "SP91" => Some(Self::Sp91),
                    "SP91_E10" => Some(Self::Sp91E10),
                    "SP92" => Some(Self::Sp92),
                    "SP95" => Some(Self::Sp95),
                    "SP95_E10" => Some(Self::Sp95E10),
                    "SP98" => Some(Self::Sp98),
                    "SP99" => Some(Self::Sp99),
                    "SP100" => Some(Self::Sp100),
                    "LPG" => Some(Self::Lpg),
                    "E80" => Some(Self::E80),
                    "E85" => Some(Self::E85),
                    "METHANE" => Some(Self::Methane),
                    "BIO_DIESEL" => Some(Self::BioDiesel),
                    "TRUCK_DIESEL" => Some(Self::TruckDiesel),
                    _ => None,
                }
            }
        }
    }
}
/// Circle with a LatLng as center and radius.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Circle {
    /// Required. Center latitude and longitude.
    ///
    /// The range of latitude must be within \[-90.0, 90.0\]. The range of the
    /// longitude must be within \[-180.0, 180.0\].
    #[prost(message, optional, tag = "1")]
    pub center: ::core::option::Option<super::super::super::r#type::LatLng>,
    /// Required. Radius measured in meters. The radius must be within [0.0,
    /// 50000.0].
    #[prost(double, tag = "2")]
    pub radius: f64,
}
/// Information about a photo of a place.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Photo {
    /// Identifier. A reference representing this place photo which may be used to
    /// look up this place photo again (also called the API "resource" name:
    /// `places/{place_id}/photos/{photo}`).
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The maximum available width, in pixels.
    #[prost(int32, tag = "2")]
    pub width_px: i32,
    /// The maximum available height, in pixels.
    #[prost(int32, tag = "3")]
    pub height_px: i32,
    /// This photo's authors.
    #[prost(message, repeated, tag = "4")]
    pub author_attributions: ::prost::alloc::vec::Vec<AuthorAttribution>,
}
/// Information about a review of a place.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Review {
    /// A reference representing this place review which may be used to look up
    /// this place review again (also called the API "resource" name:
    /// `places/{place_id}/reviews/{review}`).
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A string of formatted recent time, expressing the review time relative
    /// to the current time in a form appropriate for the language and country.
    #[prost(string, tag = "2")]
    pub relative_publish_time_description: ::prost::alloc::string::String,
    /// The localized text of the review.
    #[prost(message, optional, tag = "9")]
    pub text: ::core::option::Option<super::super::super::r#type::LocalizedText>,
    /// The review text in its original language.
    #[prost(message, optional, tag = "12")]
    pub original_text: ::core::option::Option<
        super::super::super::r#type::LocalizedText,
    >,
    /// A number between 1.0 and 5.0, also called the number of stars.
    #[prost(double, tag = "7")]
    pub rating: f64,
    /// This review's author.
    #[prost(message, optional, tag = "13")]
    pub author_attribution: ::core::option::Option<AuthorAttribution>,
    /// Timestamp for the review.
    #[prost(message, optional, tag = "14")]
    pub publish_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// All the information representing a Place.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Place {
    /// This Place's resource name, in `places/{place_id}` format.  Can be used to
    /// look up the Place.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The unique identifier of a place.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// The localized name of the place, suitable as a short human-readable
    /// description. For example, "Google Sydney", "Starbucks", "Pyrmont", etc.
    #[prost(message, optional, tag = "31")]
    pub display_name: ::core::option::Option<super::super::super::r#type::LocalizedText>,
    /// A set of type tags for this result. For example, "political" and
    /// "locality".  For the complete list of possible values, see Table A and
    /// Table B at
    /// <https://developers.google.com/maps/documentation/places/web-service/place-types>
    #[prost(string, repeated, tag = "5")]
    pub types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The primary type of the given result. This type must one of the Places API
    /// supported types. For example, "restaurant", "cafe", "airport", etc.  A
    /// place can only have a single primary type.  For the complete list of
    /// possible values, see Table A and Table B at
    /// <https://developers.google.com/maps/documentation/places/web-service/place-types>
    #[prost(string, tag = "50")]
    pub primary_type: ::prost::alloc::string::String,
    /// The display name of the primary type, localized to the request language if
    /// applicable.  For the complete list of possible values, see Table A and
    /// Table B at
    /// <https://developers.google.com/maps/documentation/places/web-service/place-types>
    #[prost(message, optional, tag = "32")]
    pub primary_type_display_name: ::core::option::Option<
        super::super::super::r#type::LocalizedText,
    >,
    /// A human-readable phone number for the place, in national format.
    #[prost(string, tag = "7")]
    pub national_phone_number: ::prost::alloc::string::String,
    /// A human-readable phone number for the place, in international format.
    #[prost(string, tag = "8")]
    pub international_phone_number: ::prost::alloc::string::String,
    /// A full, human-readable address for this place.
    #[prost(string, tag = "9")]
    pub formatted_address: ::prost::alloc::string::String,
    /// A short, human-readable address for this place.
    #[prost(string, tag = "51")]
    pub short_formatted_address: ::prost::alloc::string::String,
    /// Repeated components for each locality level.
    /// Note the following facts about the address_components\[\] array:
    /// - The array of address components may contain more components than the
    /// formatted_address.
    /// - The array does not necessarily include all the political entities that
    /// contain an address, apart from those included in the formatted_address. To
    /// retrieve all the political entities that contain a specific address, you
    /// should use reverse geocoding, passing the latitude/longitude of the address
    /// as a parameter to the request.
    /// - The format of the response is not guaranteed to remain the same between
    /// requests. In particular, the number of address_components varies based on
    /// the address requested and can change over time for the same address. A
    /// component can change position in the array. The type of the component can
    /// change. A particular component may be missing in a later response.
    #[prost(message, repeated, tag = "10")]
    pub address_components: ::prost::alloc::vec::Vec<place::AddressComponent>,
    /// Plus code of the place location lat/long.
    #[prost(message, optional, tag = "11")]
    pub plus_code: ::core::option::Option<place::PlusCode>,
    /// The position of this place.
    #[prost(message, optional, tag = "12")]
    pub location: ::core::option::Option<super::super::super::r#type::LatLng>,
    /// A viewport suitable for displaying the place on an average-sized map.
    #[prost(message, optional, tag = "13")]
    pub viewport: ::core::option::Option<super::super::super::geo::r#type::Viewport>,
    /// A rating between 1.0 and 5.0, based on user reviews of this place.
    #[prost(double, tag = "14")]
    pub rating: f64,
    /// A URL providing more information about this place.
    #[prost(string, tag = "15")]
    pub google_maps_uri: ::prost::alloc::string::String,
    /// The authoritative website for this place, e.g. a business' homepage.
    /// Note that for places that are part of a chain (e.g. an IKEA store), this
    /// will usually be the website for the individual store, not the overall
    /// chain.
    #[prost(string, tag = "16")]
    pub website_uri: ::prost::alloc::string::String,
    /// List of reviews about this place, sorted by relevance. A maximum of 5
    /// reviews can be returned.
    #[prost(message, repeated, tag = "53")]
    pub reviews: ::prost::alloc::vec::Vec<Review>,
    /// The regular hours of operation.
    #[prost(message, optional, tag = "21")]
    pub regular_opening_hours: ::core::option::Option<place::OpeningHours>,
    /// Number of minutes this place's timezone is currently offset from UTC.
    /// This is expressed in minutes to support timezones that are offset by
    /// fractions of an hour, e.g. X hours and 15 minutes.
    #[prost(int32, optional, tag = "22")]
    pub utc_offset_minutes: ::core::option::Option<i32>,
    /// Information (including references) about photos of this place. A maximum of
    /// 10 photos can be returned.
    #[prost(message, repeated, tag = "54")]
    pub photos: ::prost::alloc::vec::Vec<Photo>,
    /// The place's address in adr microformat: <http://microformats.org/wiki/adr.>
    #[prost(string, tag = "24")]
    pub adr_format_address: ::prost::alloc::string::String,
    /// The business status for the place.
    #[prost(enumeration = "place::BusinessStatus", tag = "25")]
    pub business_status: i32,
    /// Price level of the place.
    #[prost(enumeration = "PriceLevel", tag = "26")]
    pub price_level: i32,
    /// A set of data provider that must be shown with this result.
    #[prost(message, repeated, tag = "27")]
    pub attributions: ::prost::alloc::vec::Vec<place::Attribution>,
    /// The total number of reviews (with or without text) for this place.
    #[prost(int32, optional, tag = "28")]
    pub user_rating_count: ::core::option::Option<i32>,
    /// A truncated URL to an icon mask. User can access different icon type by
    /// appending type suffix to the end (eg, ".svg" or ".png").
    #[prost(string, tag = "29")]
    pub icon_mask_base_uri: ::prost::alloc::string::String,
    /// Background color for icon_mask in hex format, e.g. #909CE1.
    #[prost(string, tag = "30")]
    pub icon_background_color: ::prost::alloc::string::String,
    /// Specifies if the business supports takeout.
    #[prost(bool, optional, tag = "33")]
    pub takeout: ::core::option::Option<bool>,
    /// Specifies if the business supports delivery.
    #[prost(bool, optional, tag = "34")]
    pub delivery: ::core::option::Option<bool>,
    /// Specifies if the business supports indoor or outdoor seating options.
    #[prost(bool, optional, tag = "35")]
    pub dine_in: ::core::option::Option<bool>,
    /// Specifies if the business supports curbside pickup.
    #[prost(bool, optional, tag = "36")]
    pub curbside_pickup: ::core::option::Option<bool>,
    /// Specifies if the place supports reservations.
    #[prost(bool, optional, tag = "38")]
    pub reservable: ::core::option::Option<bool>,
    /// Specifies if the place serves breakfast.
    #[prost(bool, optional, tag = "39")]
    pub serves_breakfast: ::core::option::Option<bool>,
    /// Specifies if the place serves lunch.
    #[prost(bool, optional, tag = "40")]
    pub serves_lunch: ::core::option::Option<bool>,
    /// Specifies if the place serves dinner.
    #[prost(bool, optional, tag = "41")]
    pub serves_dinner: ::core::option::Option<bool>,
    /// Specifies if the place serves beer.
    #[prost(bool, optional, tag = "42")]
    pub serves_beer: ::core::option::Option<bool>,
    /// Specifies if the place serves wine.
    #[prost(bool, optional, tag = "43")]
    pub serves_wine: ::core::option::Option<bool>,
    /// Specifies if the place serves brunch.
    #[prost(bool, optional, tag = "44")]
    pub serves_brunch: ::core::option::Option<bool>,
    /// Specifies if the place serves vegetarian food.
    #[prost(bool, optional, tag = "45")]
    pub serves_vegetarian_food: ::core::option::Option<bool>,
    /// The hours of operation for the next seven days (including today). The time
    /// period starts at midnight on the date of the request and ends at 11:59 pm
    /// six days later. This field includes the special_days subfield of all hours,
    /// set for dates that have exceptional hours.
    #[prost(message, optional, tag = "46")]
    pub current_opening_hours: ::core::option::Option<place::OpeningHours>,
    /// Contains an array of entries for the next seven days including information
    /// about secondary hours of a business. Secondary hours are different from a
    /// business's main hours. For example, a restaurant can specify drive through
    /// hours or delivery hours as its secondary hours. This field populates the
    /// type subfield, which draws from a predefined list of opening hours types
    /// (such as DRIVE_THROUGH, PICKUP, or TAKEOUT) based on the types of the
    /// place. This field includes the special_days subfield of all hours, set for
    /// dates that have exceptional hours.
    #[prost(message, repeated, tag = "47")]
    pub current_secondary_opening_hours: ::prost::alloc::vec::Vec<place::OpeningHours>,
    /// Contains an array of entries for information about regular secondary hours
    /// of a business. Secondary hours are different from a business's main hours.
    /// For example, a restaurant can specify drive through hours or delivery hours
    /// as its secondary hours. This field populates the type subfield, which draws
    /// from a predefined list of opening hours types (such as DRIVE_THROUGH,
    /// PICKUP, or TAKEOUT) based on the types of the place.
    #[prost(message, repeated, tag = "49")]
    pub regular_secondary_opening_hours: ::prost::alloc::vec::Vec<place::OpeningHours>,
    /// Contains a summary of the place. A summary is comprised of a textual
    /// overview, and also includes the language code for these if applicable.
    /// Summary text must be presented as-is and can not be modified or altered.
    #[prost(message, optional, tag = "52")]
    pub editorial_summary: ::core::option::Option<
        super::super::super::r#type::LocalizedText,
    >,
    /// Place provides outdoor seating.
    #[prost(bool, optional, tag = "55")]
    pub outdoor_seating: ::core::option::Option<bool>,
    /// Place provides live music.
    #[prost(bool, optional, tag = "56")]
    pub live_music: ::core::option::Option<bool>,
    /// Place has a children's menu.
    #[prost(bool, optional, tag = "57")]
    pub menu_for_children: ::core::option::Option<bool>,
    /// Place serves cocktails.
    #[prost(bool, optional, tag = "58")]
    pub serves_cocktails: ::core::option::Option<bool>,
    /// Place serves dessert.
    #[prost(bool, optional, tag = "59")]
    pub serves_dessert: ::core::option::Option<bool>,
    /// Place serves coffee.
    #[prost(bool, optional, tag = "60")]
    pub serves_coffee: ::core::option::Option<bool>,
    /// Place is good for children.
    #[prost(bool, optional, tag = "62")]
    pub good_for_children: ::core::option::Option<bool>,
    /// Place allows dogs.
    #[prost(bool, optional, tag = "63")]
    pub allows_dogs: ::core::option::Option<bool>,
    /// Place has restroom.
    #[prost(bool, optional, tag = "64")]
    pub restroom: ::core::option::Option<bool>,
    /// Place accommodates groups.
    #[prost(bool, optional, tag = "65")]
    pub good_for_groups: ::core::option::Option<bool>,
    /// Place is suitable for watching sports.
    #[prost(bool, optional, tag = "66")]
    pub good_for_watching_sports: ::core::option::Option<bool>,
    /// Payment options the place accepts. If a payment option data is not
    /// available, the payment option field will be unset.
    #[prost(message, optional, tag = "67")]
    pub payment_options: ::core::option::Option<place::PaymentOptions>,
    /// Options of parking provided by the place.
    #[prost(message, optional, tag = "70")]
    pub parking_options: ::core::option::Option<place::ParkingOptions>,
    /// A list of sub destinations related to the place.
    #[prost(message, repeated, tag = "71")]
    pub sub_destinations: ::prost::alloc::vec::Vec<place::SubDestination>,
    /// Information about the accessibility options a place offers.
    #[prost(message, optional, tag = "72")]
    pub accessibility_options: ::core::option::Option<place::AccessibilityOptions>,
    /// The most recent information about fuel options in a gas station. This
    /// information is updated regularly.
    #[prost(message, optional, tag = "78")]
    pub fuel_options: ::core::option::Option<FuelOptions>,
    /// Information of ev charging options.
    #[prost(message, optional, tag = "79")]
    pub ev_charge_options: ::core::option::Option<EvChargeOptions>,
}
/// Nested message and enum types in `Place`.
pub mod place {
    /// The structured components that form the formatted address, if this
    /// information is available.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AddressComponent {
        /// The full text description or name of the address component. For example,
        /// an address component for the country Australia may have a long_name of
        /// "Australia".
        #[prost(string, tag = "1")]
        pub long_text: ::prost::alloc::string::String,
        /// An abbreviated textual name for the address component, if available. For
        /// example, an address component for the country of Australia may have a
        /// short_name of "AU".
        #[prost(string, tag = "2")]
        pub short_text: ::prost::alloc::string::String,
        /// An array indicating the type(s) of the address component.
        #[prost(string, repeated, tag = "3")]
        pub types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// The language used to format this components, in CLDR notation.
        #[prost(string, tag = "4")]
        pub language_code: ::prost::alloc::string::String,
    }
    /// Plus code (<http://plus.codes>) is a location reference with two formats:
    /// global code defining a 14mx14m (1/8000th of a degree) or smaller rectangle,
    /// and compound code, replacing the prefix with a reference location.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PlusCode {
        /// Place's global (full) code, such as "9FWM33GV+HQ", representing an
        /// 1/8000 by 1/8000 degree area (~14 by 14 meters).
        #[prost(string, tag = "1")]
        pub global_code: ::prost::alloc::string::String,
        /// Place's compound code, such as "33GV+HQ, Ramberg, Norway", containing
        /// the suffix of the global code and replacing the prefix with a formatted
        /// name of a reference entity.
        #[prost(string, tag = "2")]
        pub compound_code: ::prost::alloc::string::String,
    }
    /// Information about business hour of the place.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OpeningHours {
        /// Is this place open right now?  Always present unless we lack time-of-day
        /// or timezone data for these opening hours.
        #[prost(bool, optional, tag = "1")]
        pub open_now: ::core::option::Option<bool>,
        /// The periods that this place is open during the week. The periods are in
        /// chronological order, starting with Sunday in the place-local timezone. An
        /// empty (but not absent) value indicates a place that is never open, e.g.
        /// because it is closed temporarily for renovations.
        #[prost(message, repeated, tag = "2")]
        pub periods: ::prost::alloc::vec::Vec<opening_hours::Period>,
        /// Localized strings describing the opening hours of this place, one string
        /// for each day of the week.  Will be empty if the hours are unknown or
        /// could not be converted to localized text. Example: "Sun: 18:00–06:00"
        #[prost(string, repeated, tag = "3")]
        pub weekday_descriptions: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
        /// A type string used to identify the type of secondary hours.
        #[prost(enumeration = "opening_hours::SecondaryHoursType", tag = "4")]
        pub secondary_hours_type: i32,
        /// Structured information for special days that fall within the period that
        /// the returned opening hours cover. Special days are days that could impact
        /// the business hours of a place, e.g. Christmas day. Set for
        /// current_opening_hours and current_secondary_opening_hours if there are
        /// exceptional hours.
        #[prost(message, repeated, tag = "5")]
        pub special_days: ::prost::alloc::vec::Vec<opening_hours::SpecialDay>,
    }
    /// Nested message and enum types in `OpeningHours`.
    pub mod opening_hours {
        /// A period the place remains in open_now status.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Period {
            /// The time that the place starts to be open.
            #[prost(message, optional, tag = "1")]
            pub open: ::core::option::Option<period::Point>,
            /// The time that the place starts to be closed.
            #[prost(message, optional, tag = "2")]
            pub close: ::core::option::Option<period::Point>,
        }
        /// Nested message and enum types in `Period`.
        pub mod period {
            /// Status changing points.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Point {
                /// A day of the week, as an integer in the range 0-6.  0 is Sunday, 1 is
                /// Monday, etc.
                #[prost(int32, optional, tag = "1")]
                pub day: ::core::option::Option<i32>,
                /// The hour in 2 digits. Ranges from 00 to 23.
                #[prost(int32, optional, tag = "2")]
                pub hour: ::core::option::Option<i32>,
                /// The minute in 2 digits. Ranges from 00 to 59.
                #[prost(int32, optional, tag = "3")]
                pub minute: ::core::option::Option<i32>,
                /// Date in the local timezone for the place.
                #[prost(message, optional, tag = "6")]
                pub date: ::core::option::Option<
                    super::super::super::super::super::super::r#type::Date,
                >,
                /// Whether or not this endpoint was truncated. Truncation occurs when
                /// the real hours are outside the times we are willing to return hours
                /// between, so we truncate the hours back to these boundaries. This
                /// ensures that at most 24 * 7 hours from midnight of the day of the
                /// request are returned.
                #[prost(bool, tag = "5")]
                pub truncated: bool,
            }
        }
        /// Structured information for special days that fall within the period that
        /// the returned opening hours cover. Special days are days that could impact
        /// the business hours of a place, e.g. Christmas day.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct SpecialDay {
            /// The date of this special day.
            #[prost(message, optional, tag = "1")]
            pub date: ::core::option::Option<
                super::super::super::super::super::r#type::Date,
            >,
        }
        /// A type used to identify the type of secondary hours.
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
        pub enum SecondaryHoursType {
            /// Default value when secondary hour type is not specified.
            Unspecified = 0,
            /// The drive-through hour for banks, restaurants, or pharmacies.
            DriveThrough = 1,
            /// The happy hour.
            HappyHour = 2,
            /// The delivery hour.
            Delivery = 3,
            /// The takeout hour.
            Takeout = 4,
            /// The kitchen hour.
            Kitchen = 5,
            /// The breakfast hour.
            Breakfast = 6,
            /// The lunch hour.
            Lunch = 7,
            /// The dinner hour.
            Dinner = 8,
            /// The brunch hour.
            Brunch = 9,
            /// The pickup hour.
            Pickup = 10,
            /// The access hours for storage places.
            Access = 11,
            /// The special hours for seniors.
            SeniorHours = 12,
            /// The online service hours.
            OnlineServiceHours = 13,
        }
        impl SecondaryHoursType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    SecondaryHoursType::Unspecified => "SECONDARY_HOURS_TYPE_UNSPECIFIED",
                    SecondaryHoursType::DriveThrough => "DRIVE_THROUGH",
                    SecondaryHoursType::HappyHour => "HAPPY_HOUR",
                    SecondaryHoursType::Delivery => "DELIVERY",
                    SecondaryHoursType::Takeout => "TAKEOUT",
                    SecondaryHoursType::Kitchen => "KITCHEN",
                    SecondaryHoursType::Breakfast => "BREAKFAST",
                    SecondaryHoursType::Lunch => "LUNCH",
                    SecondaryHoursType::Dinner => "DINNER",
                    SecondaryHoursType::Brunch => "BRUNCH",
                    SecondaryHoursType::Pickup => "PICKUP",
                    SecondaryHoursType::Access => "ACCESS",
                    SecondaryHoursType::SeniorHours => "SENIOR_HOURS",
                    SecondaryHoursType::OnlineServiceHours => "ONLINE_SERVICE_HOURS",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "SECONDARY_HOURS_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                    "DRIVE_THROUGH" => Some(Self::DriveThrough),
                    "HAPPY_HOUR" => Some(Self::HappyHour),
                    "DELIVERY" => Some(Self::Delivery),
                    "TAKEOUT" => Some(Self::Takeout),
                    "KITCHEN" => Some(Self::Kitchen),
                    "BREAKFAST" => Some(Self::Breakfast),
                    "LUNCH" => Some(Self::Lunch),
                    "DINNER" => Some(Self::Dinner),
                    "BRUNCH" => Some(Self::Brunch),
                    "PICKUP" => Some(Self::Pickup),
                    "ACCESS" => Some(Self::Access),
                    "SENIOR_HOURS" => Some(Self::SeniorHours),
                    "ONLINE_SERVICE_HOURS" => Some(Self::OnlineServiceHours),
                    _ => None,
                }
            }
        }
    }
    /// Information about data providers of this place.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Attribution {
        /// Name of the Place's data provider.
        #[prost(string, tag = "1")]
        pub provider: ::prost::alloc::string::String,
        /// URI to the Place's data provider.
        #[prost(string, tag = "2")]
        pub provider_uri: ::prost::alloc::string::String,
    }
    /// Payment options the place accepts.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PaymentOptions {
        /// Place accepts credit cards as payment.
        #[prost(bool, optional, tag = "1")]
        pub accepts_credit_cards: ::core::option::Option<bool>,
        /// Place accepts debit cards as payment.
        #[prost(bool, optional, tag = "2")]
        pub accepts_debit_cards: ::core::option::Option<bool>,
        /// Place accepts cash only as payment. Places with this attribute may still
        /// accept other payment methods.
        #[prost(bool, optional, tag = "3")]
        pub accepts_cash_only: ::core::option::Option<bool>,
        /// Place accepts NFC payments.
        #[prost(bool, optional, tag = "4")]
        pub accepts_nfc: ::core::option::Option<bool>,
    }
    /// Information about parking options for the place. A parking lot could
    /// support more than one option at the same time.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ParkingOptions {
        /// Place offers free parking lots.
        #[prost(bool, optional, tag = "1")]
        pub free_parking_lot: ::core::option::Option<bool>,
        /// Place offers paid parking lots.
        #[prost(bool, optional, tag = "2")]
        pub paid_parking_lot: ::core::option::Option<bool>,
        /// Place offers free street parking.
        #[prost(bool, optional, tag = "3")]
        pub free_street_parking: ::core::option::Option<bool>,
        /// Place offers paid street parking.
        #[prost(bool, optional, tag = "4")]
        pub paid_street_parking: ::core::option::Option<bool>,
        /// Place offers valet parking.
        #[prost(bool, optional, tag = "5")]
        pub valet_parking: ::core::option::Option<bool>,
        /// Place offers free garage parking.
        #[prost(bool, optional, tag = "6")]
        pub free_garage_parking: ::core::option::Option<bool>,
        /// Place offers paid garage parking.
        #[prost(bool, optional, tag = "7")]
        pub paid_garage_parking: ::core::option::Option<bool>,
    }
    /// Place resource name and id of sub destinations that relate to the place.
    /// For example, different terminals are different destinations of an airport.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SubDestination {
        /// The resource name of the sub destination.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// The place id of the sub destination.
        #[prost(string, tag = "2")]
        pub id: ::prost::alloc::string::String,
    }
    /// Information about the accessibility options a place offers.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AccessibilityOptions {
        /// Place offers wheelchair accessible parking.
        #[prost(bool, optional, tag = "1")]
        pub wheelchair_accessible_parking: ::core::option::Option<bool>,
        /// Places has wheelchair accessible entrance.
        #[prost(bool, optional, tag = "2")]
        pub wheelchair_accessible_entrance: ::core::option::Option<bool>,
        /// Place has wheelchair accessible restroom.
        #[prost(bool, optional, tag = "3")]
        pub wheelchair_accessible_restroom: ::core::option::Option<bool>,
        /// Place has wheelchair accessible seating.
        #[prost(bool, optional, tag = "4")]
        pub wheelchair_accessible_seating: ::core::option::Option<bool>,
    }
    /// Business status for the place.
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
    pub enum BusinessStatus {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// The establishment is operational, not necessarily open now.
        Operational = 1,
        /// The establishment is temporarily closed.
        ClosedTemporarily = 2,
        /// The establishment is permanently closed.
        ClosedPermanently = 3,
    }
    impl BusinessStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                BusinessStatus::Unspecified => "BUSINESS_STATUS_UNSPECIFIED",
                BusinessStatus::Operational => "OPERATIONAL",
                BusinessStatus::ClosedTemporarily => "CLOSED_TEMPORARILY",
                BusinessStatus::ClosedPermanently => "CLOSED_PERMANENTLY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BUSINESS_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
                "OPERATIONAL" => Some(Self::Operational),
                "CLOSED_TEMPORARILY" => Some(Self::ClosedTemporarily),
                "CLOSED_PERMANENTLY" => Some(Self::ClosedPermanently),
                _ => None,
            }
        }
    }
}
/// Price level of the place.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PriceLevel {
    /// Place price level is unspecified or unknown.
    Unspecified = 0,
    /// Place provides free services.
    Free = 1,
    /// Place provides inexpensive services.
    Inexpensive = 2,
    /// Place provides moderately priced services.
    Moderate = 3,
    /// Place provides expensive services.
    Expensive = 4,
    /// Place provides very expensive services.
    VeryExpensive = 5,
}
impl PriceLevel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PriceLevel::Unspecified => "PRICE_LEVEL_UNSPECIFIED",
            PriceLevel::Free => "PRICE_LEVEL_FREE",
            PriceLevel::Inexpensive => "PRICE_LEVEL_INEXPENSIVE",
            PriceLevel::Moderate => "PRICE_LEVEL_MODERATE",
            PriceLevel::Expensive => "PRICE_LEVEL_EXPENSIVE",
            PriceLevel::VeryExpensive => "PRICE_LEVEL_VERY_EXPENSIVE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PRICE_LEVEL_UNSPECIFIED" => Some(Self::Unspecified),
            "PRICE_LEVEL_FREE" => Some(Self::Free),
            "PRICE_LEVEL_INEXPENSIVE" => Some(Self::Inexpensive),
            "PRICE_LEVEL_MODERATE" => Some(Self::Moderate),
            "PRICE_LEVEL_EXPENSIVE" => Some(Self::Expensive),
            "PRICE_LEVEL_VERY_EXPENSIVE" => Some(Self::VeryExpensive),
            _ => None,
        }
    }
}
/// Request proto for Search Nearby.
///
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchNearbyRequest {
    /// Place details will be displayed with the preferred language if available.
    /// If the language code is unspecified or unrecognized, place details of any
    /// language may be returned, with a preference for English if such details
    /// exist.
    ///
    /// Current list of supported languages:
    /// <https://developers.google.com/maps/faq#languagesupport.>
    #[prost(string, tag = "1")]
    pub language_code: ::prost::alloc::string::String,
    /// The Unicode country/region code (CLDR) of the location where the
    /// request is coming from. This parameter is used to display the place
    /// details, like region-specific place name, if available. The parameter can
    /// affect results based on applicable law.
    ///
    /// For more information, see
    /// <https://www.unicode.org/cldr/charts/latest/supplemental/territory_language_information.html.>
    ///
    ///
    /// Note that 3-digit region codes are not currently supported.
    #[prost(string, tag = "2")]
    pub region_code: ::prost::alloc::string::String,
    /// Included Place type (eg, "restaurant" or "gas_station") from
    /// <https://developers.google.com/maps/documentation/places/web-service/place-types.>
    ///
    /// Up to 50 types from [Table
    /// A](<https://developers.google.com/maps/documentation/places/web-service/place-types#table-a>)
    /// may be specified.
    ///
    /// If there are any conflicting types, i.e. a type appears in both
    /// included_types and excluded_types, an INVALID_ARGUMENT error is
    /// returned.
    ///
    /// If a Place type is specified with multiple type restrictions, only places
    /// that satisfy all of the restrictions are returned. For example, if we
    /// have {included_types = \["restaurant"\], excluded_primary_types =
    /// \["restaurant"\]}, the returned places provide "restaurant"
    /// related services but do not operate primarily as "restaurants".
    #[prost(string, repeated, tag = "3")]
    pub included_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Excluded Place type (eg, "restaurant" or "gas_station") from
    /// <https://developers.google.com/maps/documentation/places/web-service/place-types.>
    ///
    /// Up to 50 types from [Table
    /// A](<https://developers.google.com/maps/documentation/places/web-service/place-types#table-a>)
    /// may be specified.
    ///
    /// If the client provides both included_types (e.g. restaurant) and
    /// excluded_types (e.g. cafe), then the response should include places that
    /// are restaurant but not cafe. The response includes places that match at
    /// least one of the included_types and none of the excluded_types.
    ///
    /// If there are any conflicting types, i.e. a type appears in both
    /// included_types and excluded_types, an INVALID_ARGUMENT error is returned.
    ///
    /// If a Place type is specified with multiple type restrictions, only places
    /// that satisfy all of the restrictions are returned. For example, if we
    /// have {included_types = \["restaurant"\], excluded_primary_types =
    /// \["restaurant"\]}, the returned places provide "restaurant"
    /// related services but do not operate primarily as "restaurants".
    #[prost(string, repeated, tag = "4")]
    pub excluded_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Included primary Place type (e.g. "restaurant" or "gas_station") from
    /// <https://developers.google.com/maps/documentation/places/web-service/place-types.>
    /// A place can only have a single primary type from the supported types table
    /// associated with it.
    ///
    /// Up to 50 types from [Table
    /// A](<https://developers.google.com/maps/documentation/places/web-service/place-types#table-a>)
    /// may be specified.
    ///
    /// If there are any conflicting primary types, i.e. a type appears in both
    /// included_primary_types and excluded_primary_types, an INVALID_ARGUMENT
    /// error is returned.
    ///
    /// If a Place type is specified with multiple type restrictions, only places
    /// that satisfy all of the restrictions are returned. For example, if we
    /// have {included_types = \["restaurant"\], excluded_primary_types =
    /// \["restaurant"\]}, the returned places provide "restaurant"
    /// related services but do not operate primarily as "restaurants".
    #[prost(string, repeated, tag = "5")]
    pub included_primary_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Excluded primary Place type (e.g. "restaurant" or "gas_station") from
    /// <https://developers.google.com/maps/documentation/places/web-service/place-types.>
    ///
    /// Up to 50 types from [Table
    /// A](<https://developers.google.com/maps/documentation/places/web-service/place-types#table-a>)
    /// may be specified.
    ///
    /// If there are any conflicting primary types, i.e. a type appears in both
    /// included_primary_types and excluded_primary_types, an INVALID_ARGUMENT
    /// error is returned.
    ///
    /// If a Place type is specified with multiple type restrictions, only places
    /// that satisfy all of the restrictions are returned. For example, if we
    /// have {included_types = \["restaurant"\], excluded_primary_types =
    /// \["restaurant"\]}, the returned places provide "restaurant"
    /// related services but do not operate primarily as "restaurants".
    #[prost(string, repeated, tag = "6")]
    pub excluded_primary_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Maximum number of results to return. It must be between 1 and 20 (default),
    /// inclusively. If the number is unset, it falls back to the upper limit. If
    /// the number is set to negative or exceeds the upper limit, an
    /// INVALID_ARGUMENT error is returned.
    #[prost(int32, tag = "7")]
    pub max_result_count: i32,
    /// Required. The region to search.
    #[prost(message, optional, tag = "8")]
    pub location_restriction: ::core::option::Option<
        search_nearby_request::LocationRestriction,
    >,
    /// How results will be ranked in the response.
    #[prost(enumeration = "search_nearby_request::RankPreference", tag = "9")]
    pub rank_preference: i32,
}
/// Nested message and enum types in `SearchNearbyRequest`.
pub mod search_nearby_request {
    /// The region to search.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LocationRestriction {
        #[prost(oneof = "location_restriction::Type", tags = "2")]
        pub r#type: ::core::option::Option<location_restriction::Type>,
    }
    /// Nested message and enum types in `LocationRestriction`.
    pub mod location_restriction {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Type {
            /// A circle defined by center point and radius.
            #[prost(message, tag = "2")]
            Circle(super::super::Circle),
        }
    }
    /// How results will be ranked in the response.
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
    pub enum RankPreference {
        /// RankPreference value not set. Will use rank by POPULARITY by default.
        Unspecified = 0,
        /// Ranks results by distance.
        Distance = 1,
        /// Ranks results by popularity.
        Popularity = 2,
    }
    impl RankPreference {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RankPreference::Unspecified => "RANK_PREFERENCE_UNSPECIFIED",
                RankPreference::Distance => "DISTANCE",
                RankPreference::Popularity => "POPULARITY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RANK_PREFERENCE_UNSPECIFIED" => Some(Self::Unspecified),
                "DISTANCE" => Some(Self::Distance),
                "POPULARITY" => Some(Self::Popularity),
                _ => None,
            }
        }
    }
}
/// Response proto for Search Nearby.
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchNearbyResponse {
    /// A list of places that meets user's requirements like places
    /// types, number of places and specific location restriction.
    #[prost(message, repeated, tag = "1")]
    pub places: ::prost::alloc::vec::Vec<Place>,
}
/// Request proto for SearchText.
///
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchTextRequest {
    /// Required. The text query for textual search.
    #[prost(string, tag = "1")]
    pub text_query: ::prost::alloc::string::String,
    /// Place details will be displayed with the preferred language if available.
    /// If the language code is unspecified or unrecognized, place details of any
    /// language may be returned, with a preference for English if such details
    /// exist.
    ///
    /// Current list of supported languages:
    /// <https://developers.google.com/maps/faq#languagesupport.>
    #[prost(string, tag = "2")]
    pub language_code: ::prost::alloc::string::String,
    /// The Unicode country/region code (CLDR) of the location where the
    /// request is coming from. This parameter is used to display the place
    /// details, like region-specific place name, if available. The parameter can
    /// affect results based on applicable law.
    ///
    /// For more information, see
    /// <https://www.unicode.org/cldr/charts/latest/supplemental/territory_language_information.html.>
    ///
    ///
    /// Note that 3-digit region codes are not currently supported.
    #[prost(string, tag = "3")]
    pub region_code: ::prost::alloc::string::String,
    /// How results will be ranked in the response.
    #[prost(enumeration = "search_text_request::RankPreference", tag = "4")]
    pub rank_preference: i32,
    /// The requested place type. Full list of types supported:
    /// <https://developers.google.com/maps/documentation/places/web-service/place-types.>
    /// Only support one included type.
    #[prost(string, tag = "6")]
    pub included_type: ::prost::alloc::string::String,
    /// Used to restrict the search to places that are currently open.  The default
    /// is false.
    #[prost(bool, tag = "7")]
    pub open_now: bool,
    /// Filter out results whose average user rating is strictly less than this
    /// limit. A valid value must be a float between 0 and 5 (inclusively) at a
    /// 0.5 cadence i.e. \[0, 0.5, 1.0, ... , 5.0\] inclusively. The input rating
    /// will round up to the nearest 0.5(ceiling). For instance, a rating of 0.6
    /// will eliminate all results with a less than 1.0 rating.
    #[prost(double, tag = "9")]
    pub min_rating: f64,
    /// Maximum number of results to return. It must be between 1 and 20,
    /// inclusively. The default is 20.  If the number is unset, it falls back to
    /// the upper limit. If the number is set to negative or exceeds the upper
    /// limit, an INVALID_ARGUMENT error is returned.
    #[prost(int32, tag = "10")]
    pub max_result_count: i32,
    /// Used to restrict the search to places that are marked as certain price
    /// levels. Users can choose any combinations of price levels. Default to
    /// select all price levels.
    #[prost(enumeration = "PriceLevel", repeated, tag = "11")]
    pub price_levels: ::prost::alloc::vec::Vec<i32>,
    /// Used to set strict type filtering for included_type. If set to true, only
    /// results of the same type will be returned. Default to false.
    #[prost(bool, tag = "12")]
    pub strict_type_filtering: bool,
    /// The region to search. This location serves as a bias which means results
    /// around given location might be returned. Cannot be set along with
    /// location_restriction.
    #[prost(message, optional, tag = "13")]
    pub location_bias: ::core::option::Option<search_text_request::LocationBias>,
    /// The region to search. This location serves as a restriction which means
    /// results outside given location will not be returned. Cannot be set along
    /// with location_bias.
    #[prost(message, optional, tag = "14")]
    pub location_restriction: ::core::option::Option<
        search_text_request::LocationRestriction,
    >,
    /// Optional. Set the searchable EV options of a place search request.
    #[prost(message, optional, tag = "15")]
    pub ev_options: ::core::option::Option<search_text_request::EvOptions>,
}
/// Nested message and enum types in `SearchTextRequest`.
pub mod search_text_request {
    /// The region to search. This location serves as a bias which means results
    /// around given location might be returned.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LocationBias {
        #[prost(oneof = "location_bias::Type", tags = "1, 2")]
        pub r#type: ::core::option::Option<location_bias::Type>,
    }
    /// Nested message and enum types in `LocationBias`.
    pub mod location_bias {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Type {
            /// A rectangle box defined by northeast and southwest corner.
            /// `rectangle.high()` must be the northeast point of the rectangle
            /// viewport. `rectangle.low()` must be the southwest point of the
            /// rectangle viewport. `rectangle.low().latitude()` cannot be greater than
            /// `rectangle.high().latitude()`. This will result in an empty latitude
            /// range. A rectangle viewport cannot be wider than 180 degrees.
            #[prost(message, tag = "1")]
            Rectangle(super::super::super::super::super::geo::r#type::Viewport),
            /// A circle defined by center point and radius.
            #[prost(message, tag = "2")]
            Circle(super::super::Circle),
        }
    }
    /// The region to search. This location serves as a restriction which means
    /// results outside given location will not be returned.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LocationRestriction {
        #[prost(oneof = "location_restriction::Type", tags = "1")]
        pub r#type: ::core::option::Option<location_restriction::Type>,
    }
    /// Nested message and enum types in `LocationRestriction`.
    pub mod location_restriction {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Type {
            /// A rectangle box defined by northeast and southwest corner.
            /// `rectangle.high()` must be the northeast point of the rectangle
            /// viewport. `rectangle.low()` must be the southwest point of the
            /// rectangle viewport. `rectangle.low().latitude()` cannot be greater than
            /// `rectangle.high().latitude()`. This will result in an empty latitude
            /// range. A rectangle viewport cannot be wider than 180 degrees.
            #[prost(message, tag = "1")]
            Rectangle(super::super::super::super::super::geo::r#type::Viewport),
        }
    }
    /// Searchable EV options of a place search request.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EvOptions {
        /// Optional. Filtering places by minimum charging rate. Any places with
        /// charging a rate less than the minimum charging rate are filtered out.
        #[prost(double, tag = "1")]
        pub minimum_charging_rate_kw: f64,
        /// Optional. The list of preferred EV connector types. A place that does not
        /// support any of the listed connector types are filter out.
        #[prost(
            enumeration = "super::EvConnectorType",
            repeated,
            packed = "false",
            tag = "2"
        )]
        pub connector_types: ::prost::alloc::vec::Vec<i32>,
    }
    /// How results will be ranked in the response.
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
    pub enum RankPreference {
        /// RankPreference value not set. Will default to DISTANCE.
        Unspecified = 0,
        /// Ranks results by distance.
        Distance = 1,
        /// Ranks results by relevance. Sort order determined by normal ranking
        /// stack.
        Relevance = 2,
    }
    impl RankPreference {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RankPreference::Unspecified => "RANK_PREFERENCE_UNSPECIFIED",
                RankPreference::Distance => "DISTANCE",
                RankPreference::Relevance => "RELEVANCE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RANK_PREFERENCE_UNSPECIFIED" => Some(Self::Unspecified),
                "DISTANCE" => Some(Self::Distance),
                "RELEVANCE" => Some(Self::Relevance),
                _ => None,
            }
        }
    }
}
/// Response proto for SearchText.
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchTextResponse {
    /// A list of places that meet the user's text search criteria.
    #[prost(message, repeated, tag = "1")]
    pub places: ::prost::alloc::vec::Vec<Place>,
}
/// Request for fetching a photo of a place using a photo resource name.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPhotoMediaRequest {
    /// Required. The resource name of a photo media in the format:
    /// `places/{place_id}/photos/{photo_reference}/media`.
    ///
    /// The resource name of a photo as returned in a Place object's `photos.name`
    /// field comes with the format
    /// `places/{place_id}/photos/{photo_reference}`. You need to append `/media`
    /// at the end of the photo resource to get the photo media resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Specifies the maximum desired width, in pixels, of the image. If
    /// the image is smaller than the values specified, the original image will be
    /// returned. If the image is larger in either dimension, it will be scaled to
    /// match the smaller of the two dimensions, restricted to its original aspect
    /// ratio. Both the max_height_px and max_width_px properties accept an integer
    /// between 1 and 4800, inclusively. If the value is not within the allowed
    /// range, an INVALID_ARGUMENT error will be returned.
    ///
    /// At least one of max_height_px or max_width_px needs to be specified. If
    /// neither max_height_px nor max_width_px is specified, an INVALID_ARGUMENT
    /// error will be returned.
    #[prost(int32, tag = "2")]
    pub max_width_px: i32,
    /// Optional. Specifies the maximum desired height, in pixels, of the image. If
    /// the image is smaller than the values specified, the original image will be
    /// returned. If the image is larger in either dimension, it will be scaled to
    /// match the smaller of the two dimensions, restricted to its original aspect
    /// ratio. Both the max_height_px and max_width_px properties accept an integer
    /// between 1 and 4800, inclusively. If the value is not within the allowed
    /// range, an INVALID_ARGUMENT error will be returned.
    ///
    /// At least one of max_height_px or max_width_px needs to be specified. If
    /// neither max_height_px nor max_width_px is specified, an INVALID_ARGUMENT
    /// error will be returned.
    #[prost(int32, tag = "3")]
    pub max_height_px: i32,
    /// Optional. If set, skip the default HTTP redirect behavior and render a text
    /// format (for example, in JSON format for HTTP use case) response. If not
    /// set, an HTTP redirect will be issued to redirect the call to the image
    /// media. This option is ignored for non-HTTP requests.
    #[prost(bool, tag = "4")]
    pub skip_http_redirect: bool,
}
/// A photo media from Places API.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhotoMedia {
    /// The resource name of a photo media in the format:
    /// `places/{place_id}/photos/{photo_reference}/media`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A short-lived uri that can be used to render the photo.
    #[prost(string, tag = "2")]
    pub photo_uri: ::prost::alloc::string::String,
}
/// Request for fetching a Place based on its resource name, which is a string in
/// the `places/{place_id}` format.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPlaceRequest {
    /// Required. The resource name of a place, in the `places/{place_id}` format.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Place details will be displayed with the preferred language if
    /// available.
    ///
    /// Current list of supported languages:
    /// <https://developers.google.com/maps/faq#languagesupport.>
    #[prost(string, tag = "2")]
    pub language_code: ::prost::alloc::string::String,
    /// Optional. The Unicode country/region code (CLDR) of the location where the
    /// request is coming from. This parameter is used to display the place
    /// details, like region-specific place name, if available. The parameter can
    /// affect results based on applicable law.
    /// For more information, see
    /// <https://www.unicode.org/cldr/charts/latest/supplemental/territory_language_information.html.>
    ///
    ///
    /// Note that 3-digit region codes are not currently supported.
    #[prost(string, tag = "3")]
    pub region_code: ::prost::alloc::string::String,
    /// Optional. A string which identifies an Autocomplete session for billing
    /// purposes. Must be a URL and filename safe base64 string with at most 36
    /// ASCII characters in length. Otherwise an INVALID_ARGUMENT error is
    /// returned.
    ///
    /// The session begins when the user starts typing a query, and concludes when
    /// they select a place and a call to Place Details or Address Validation is
    /// made. Each session can have multiple queries, followed by one Place Details
    /// or Address Validation request. The credentials used for each request within
    /// a session must belong to the same Google Cloud Console project. Once a
    /// session has concluded, the token is no longer valid; your app must generate
    /// a fresh token for each session. If the `session_token` parameter is
    /// omitted, or if you reuse a session token, the session is charged as if no
    /// session token was provided (each request is billed separately).
    ///
    /// We recommend the following guidelines:
    ///
    /// * Use session tokens for all Place Autocomplete calls.
    /// * Generate a fresh token for each session. Using a version 4 UUID is
    ///    recommended.
    /// * Ensure that the credentials used for all Place Autocomplete, Place
    ///    Details, and Address Validation requests within a session belong to the
    ///    same Cloud Console project.
    /// * Be sure to pass a unique session token for each new session. Using the
    ///    same token for more than one session will result in each request being
    ///    billed individually.
    #[prost(string, tag = "4")]
    pub session_token: ::prost::alloc::string::String,
}
/// Request proto for AutocompletePlaces.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutocompletePlacesRequest {
    /// Required. The text string on which to search.
    #[prost(string, tag = "1")]
    pub input: ::prost::alloc::string::String,
    /// Optional. Bias results to a specified location.
    ///
    /// At most one of `location_bias` or `location_restriction` should be set. If
    /// neither are set, the results will be biased by IP address, meaning the IP
    /// address will be mapped to an imprecise location and used as a biasing
    /// signal.
    #[prost(message, optional, tag = "2")]
    pub location_bias: ::core::option::Option<autocomplete_places_request::LocationBias>,
    /// Optional. Restrict results to a specified location.
    ///
    /// At most one of `location_bias` or `location_restriction` should be set. If
    /// neither are set, the results will be biased by IP address, meaning the IP
    /// address will be mapped to an imprecise location and used as a biasing
    /// signal.
    #[prost(message, optional, tag = "3")]
    pub location_restriction: ::core::option::Option<
        autocomplete_places_request::LocationRestriction,
    >,
    /// Optional. Included primary Place type (for example, "restaurant" or
    /// "gas_station") from
    /// <https://developers.google.com/maps/documentation/places/web-service/place-types.>
    /// A Place is only returned if its primary type is included in this list. Up
    /// to 5 values can be specified. If no types are specified, all Place types
    /// are returned.
    #[prost(string, repeated, tag = "4")]
    pub included_primary_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. Only include results in the specified regions, specified as up to
    /// 15 CLDR two-character region codes. An empty set will not restrict the
    /// results. If both `location_restriction` and `included_region_codes` are
    /// set, the results will be located in the area of intersection.
    #[prost(string, repeated, tag = "5")]
    pub included_region_codes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. The language in which to return results. Defaults to en-US. The
    /// results may be in mixed languages if the language used in `input` is
    /// different from `language_code` or if the returned Place does not have a
    /// translation from the local language to `language_code`.
    #[prost(string, tag = "6")]
    pub language_code: ::prost::alloc::string::String,
    /// Optional. The region code, specified as a CLDR two-character region code.
    /// This affects address formatting, result ranking, and may influence what
    /// results are returned. This does not restrict results to the specified
    /// region. To restrict results to a region, use `region_code_restriction`.
    #[prost(string, tag = "7")]
    pub region_code: ::prost::alloc::string::String,
    /// Optional. The origin point from which to calculate geodesic distance to the
    /// destination (returned as `distance_meters`). If this value is omitted,
    /// geodesic distance will not be returned.
    #[prost(message, optional, tag = "8")]
    pub origin: ::core::option::Option<super::super::super::r#type::LatLng>,
    /// Optional. A zero-based Unicode character offset of `input` indicating the
    /// cursor position in `input`. The cursor position may influence what
    /// predictions are returned.
    ///
    /// If empty, defaults to the length of `input`.
    #[prost(int32, tag = "9")]
    pub input_offset: i32,
    /// Optional. If true, the response will include both Place and query
    /// predictions. Otherwise the response will only return Place predictions.
    #[prost(bool, tag = "10")]
    pub include_query_predictions: bool,
    /// Optional. A string which identifies an Autocomplete session for billing
    /// purposes. Must be a URL and filename safe base64 string with at most 36
    /// ASCII characters in length. Otherwise an INVALID_ARGUMENT error is
    /// returned.
    ///
    /// The session begins when the user starts typing a query, and concludes when
    /// they select a place and a call to Place Details or Address Validation is
    /// made. Each session can have multiple queries, followed by one Place Details
    /// or Address Validation request. The credentials used for each request within
    /// a session must belong to the same Google Cloud Console project. Once a
    /// session has concluded, the token is no longer valid; your app must generate
    /// a fresh token for each session. If the `session_token` parameter is
    /// omitted, or if you reuse a session token, the session is charged as if no
    /// session token was provided (each request is billed separately).
    ///
    /// We recommend the following guidelines:
    ///
    /// * Use session tokens for all Place Autocomplete calls.
    /// * Generate a fresh token for each session. Using a version 4 UUID is
    ///    recommended.
    /// * Ensure that the credentials used for all Place Autocomplete, Place
    ///    Details, and Address Validation requests within a session belong to the
    ///    same Cloud Console project.
    /// * Be sure to pass a unique session token for each new session. Using the
    ///    same token for more than one session will result in each request being
    ///    billed individually.
    #[prost(string, tag = "11")]
    pub session_token: ::prost::alloc::string::String,
}
/// Nested message and enum types in `AutocompletePlacesRequest`.
pub mod autocomplete_places_request {
    /// The region to search. The results may be biased around the specified
    /// region.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LocationBias {
        #[prost(oneof = "location_bias::Type", tags = "1, 2")]
        pub r#type: ::core::option::Option<location_bias::Type>,
    }
    /// Nested message and enum types in `LocationBias`.
    pub mod location_bias {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Type {
            /// A viewport defined by a northeast and a southwest corner.
            #[prost(message, tag = "1")]
            Rectangle(super::super::super::super::super::geo::r#type::Viewport),
            /// A circle defined by a center point and radius.
            #[prost(message, tag = "2")]
            Circle(super::super::Circle),
        }
    }
    /// The region to search. The results will be restricted to the specified
    /// region.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LocationRestriction {
        #[prost(oneof = "location_restriction::Type", tags = "1, 2")]
        pub r#type: ::core::option::Option<location_restriction::Type>,
    }
    /// Nested message and enum types in `LocationRestriction`.
    pub mod location_restriction {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Type {
            /// A viewport defined by a northeast and a southwest corner.
            #[prost(message, tag = "1")]
            Rectangle(super::super::super::super::super::geo::r#type::Viewport),
            /// A circle defined by a center point and radius.
            #[prost(message, tag = "2")]
            Circle(super::super::Circle),
        }
    }
}
/// Response proto for AutocompletePlaces.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutocompletePlacesResponse {
    /// Contains a list of suggestions, ordered in descending order of relevance.
    #[prost(message, repeated, tag = "1")]
    pub suggestions: ::prost::alloc::vec::Vec<autocomplete_places_response::Suggestion>,
}
/// Nested message and enum types in `AutocompletePlacesResponse`.
pub mod autocomplete_places_response {
    /// An Autocomplete suggestion result.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Suggestion {
        #[prost(oneof = "suggestion::Kind", tags = "1, 2")]
        pub kind: ::core::option::Option<suggestion::Kind>,
    }
    /// Nested message and enum types in `Suggestion`.
    pub mod suggestion {
        /// Identifies a substring within a given text.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct StringRange {
            /// Zero-based offset of the first Unicode character of the string
            /// (inclusive).
            #[prost(int32, tag = "1")]
            pub start_offset: i32,
            /// Zero-based offset of the last Unicode character (exclusive).
            #[prost(int32, tag = "2")]
            pub end_offset: i32,
        }
        /// Text representing a Place or query prediction. The text may be used as is
        /// or formatted.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct FormattableText {
            /// Text that may be used as is or formatted with `matches`.
            #[prost(string, tag = "1")]
            pub text: ::prost::alloc::string::String,
            /// A list of string ranges identifying where the input request matched in
            /// `text`. The ranges can be used to format specific parts of `text`. The
            /// substrings may not be exact matches of `input` if the matching was
            /// determined by criteria other than string matching (for example, spell
            /// corrections or transliterations).
            ///
            /// These values are Unicode character offsets of `text`. The ranges are
            /// guaranteed to be ordered in increasing offset values.
            #[prost(message, repeated, tag = "2")]
            pub matches: ::prost::alloc::vec::Vec<StringRange>,
        }
        /// Contains a breakdown of a Place or query prediction into main text
        /// and secondary text.
        ///
        /// For Place predictions, the main text contains the specific name of the
        /// Place. For query predictions, the main text contains the query.
        ///
        /// The secondary text contains additional disambiguating features (such as a
        /// city or region) to further identify the Place or refine the query.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct StructuredFormat {
            /// Represents the name of the Place or query.
            #[prost(message, optional, tag = "1")]
            pub main_text: ::core::option::Option<FormattableText>,
            /// Represents additional disambiguating features (such as a city or
            /// region) to further identify the Place or refine the query.
            #[prost(message, optional, tag = "2")]
            pub secondary_text: ::core::option::Option<FormattableText>,
        }
        /// Prediction results for a Place Autocomplete prediction.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct PlacePrediction {
            /// The resource name of the suggested Place. This name can be used in
            /// other APIs that accept Place names.
            #[prost(string, tag = "1")]
            pub place: ::prost::alloc::string::String,
            /// The unique identifier of the suggested Place. This identifier can be
            /// used in other APIs that accept Place IDs.
            #[prost(string, tag = "2")]
            pub place_id: ::prost::alloc::string::String,
            /// Contains the human-readable name for the returned result. For
            /// establishment results, this is usually the business name and address.
            ///
            /// `text` is recommended for developers who wish to show a single UI
            /// element. Developers who wish to show two separate, but related, UI
            /// elements may want to use `structured_format` instead. They are two
            /// different ways to represent a Place prediction. Users should not try to
            /// parse `structured_format` into `text` or vice versa.
            ///
            /// This text may be different from the `display_name` returned by
            /// GetPlace.
            ///
            /// May be in mixed languages if the request `input` and `language_code`
            /// are in different languages or if the Place does not have a translation
            /// from the local language to `language_code`.
            #[prost(message, optional, tag = "3")]
            pub text: ::core::option::Option<FormattableText>,
            /// A breakdown of the Place prediction into main text containing the name
            /// of the Place and secondary text containing additional disambiguating
            /// features (such as a city or region).
            ///
            /// `structured_format` is recommended for developers who wish to show two
            /// separate, but related, UI elements. Developers who wish to show a
            /// single UI element may want to use `text` instead. They are two
            /// different ways to represent a Place prediction. Users should not try to
            /// parse `structured_format` into `text` or vice versa.
            #[prost(message, optional, tag = "4")]
            pub structured_format: ::core::option::Option<StructuredFormat>,
            /// List of types that apply to this Place from Table A or Table B in
            /// <https://developers.google.com/maps/documentation/places/web-service/place-types.>
            ///
            /// A type is a categorization of a Place. Places with shared types will
            /// share similar characteristics.
            #[prost(string, repeated, tag = "5")]
            pub types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            /// The length of the geodesic in meters from `origin` if `origin` is
            /// specified. Certain predictions such as routes may not populate this
            /// field.
            #[prost(int32, tag = "6")]
            pub distance_meters: i32,
        }
        /// Prediction results for a Query Autocomplete prediction.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct QueryPrediction {
            /// The predicted text. This text does not represent a Place, but rather a
            /// text query that could be used in a search endpoint (for example,
            /// TextSearch).
            ///
            /// `text` is recommended for developers who wish to show a single UI
            /// element. Developers who wish to show two separate, but related, UI
            /// elements may want to use `structured_format` instead. They are two
            /// different ways to represent a query prediction. Users should not try to
            /// parse `structured_format` into `text` or vice versa.
            ///
            /// May be in mixed languages if the request `input` and `language_code`
            /// are in different languages or if part of the query does not have a
            /// translation from the local language to `language_code`.
            #[prost(message, optional, tag = "1")]
            pub text: ::core::option::Option<FormattableText>,
            /// A breakdown of the query prediction into main text containing the query
            /// and secondary text containing additional disambiguating features (such
            /// as a city or region).
            ///
            /// `structured_format` is recommended for developers who wish to show two
            /// separate, but related, UI elements. Developers who wish to show a
            /// single UI element may want to use `text` instead. They are two
            /// different ways to represent a query prediction. Users should not try to
            /// parse `structured_format` into `text` or vice versa.
            #[prost(message, optional, tag = "2")]
            pub structured_format: ::core::option::Option<StructuredFormat>,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Kind {
            /// A prediction for a Place.
            #[prost(message, tag = "1")]
            PlacePrediction(PlacePrediction),
            /// A prediction for a query.
            #[prost(message, tag = "2")]
            QueryPrediction(QueryPrediction),
        }
    }
}
/// Generated client implementations.
pub mod places_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service definition for the Places API.
    /// Note: every request actually requires a field mask set outside of
    /// the request proto (all/'*', is not assumed).  That can be set via either a
    /// side channel (SystemParameterContext) over RPC, or a header
    /// (X-Goog-FieldMask) over HTTP. See:
    /// https://cloud.google.com/apis/docs/system-parameters
    #[derive(Debug, Clone)]
    pub struct PlacesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PlacesClient<tonic::transport::Channel> {
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
    impl<T> PlacesClient<T>
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
        ) -> PlacesClient<InterceptedService<T, F>>
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
            PlacesClient::new(InterceptedService::new(inner, interceptor))
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
        /// Search for places near locations.
        pub async fn search_nearby(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchNearbyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SearchNearbyResponse>,
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
                "/google.maps.places.v1.Places/SearchNearby",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.maps.places.v1.Places", "SearchNearby"));
            self.inner.unary(req, path, codec).await
        }
        /// Text query based place search.
        pub async fn search_text(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchTextRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SearchTextResponse>,
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
                "/google.maps.places.v1.Places/SearchText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.maps.places.v1.Places", "SearchText"));
            self.inner.unary(req, path, codec).await
        }
        /// Get a photo media with a photo reference string.
        pub async fn get_photo_media(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPhotoMediaRequest>,
        ) -> std::result::Result<tonic::Response<super::PhotoMedia>, tonic::Status> {
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
                "/google.maps.places.v1.Places/GetPhotoMedia",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.maps.places.v1.Places", "GetPhotoMedia"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get the details of a place based on its resource name, which is a string
        /// in the `places/{place_id}` format.
        pub async fn get_place(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPlaceRequest>,
        ) -> std::result::Result<tonic::Response<super::Place>, tonic::Status> {
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
                "/google.maps.places.v1.Places/GetPlace",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.maps.places.v1.Places", "GetPlace"));
            self.inner.unary(req, path, codec).await
        }
        /// Returns predictions for the given input.
        pub async fn autocomplete_places(
            &mut self,
            request: impl tonic::IntoRequest<super::AutocompletePlacesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AutocompletePlacesResponse>,
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
                "/google.maps.places.v1.Places/AutocompletePlaces",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.maps.places.v1.Places", "AutocompletePlaces"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
