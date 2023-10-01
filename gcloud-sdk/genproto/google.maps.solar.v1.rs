/// Request message for `Solar.FindClosestBuildingInsights`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindClosestBuildingInsightsRequest {
    /// The longitude and latitude from which the API looks for the nearest known
    /// building.
    #[prost(message, optional, tag = "1")]
    pub location: ::core::option::Option<super::super::super::r#type::LatLng>,
    /// The minimum quality level allowed in the results. No result with
    /// lower quality than this will be returned. Not specifying this is
    /// equivalent to restricting to HIGH quality only.
    #[prost(enumeration = "ImageryQuality", tag = "3")]
    pub required_quality: i32,
}
/// A bounding box in lat/lng coordinates.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LatLngBox {
    /// The southwest corner of the box.
    #[prost(message, optional, tag = "1")]
    pub sw: ::core::option::Option<super::super::super::r#type::LatLng>,
    /// The northeast corner of the box.
    #[prost(message, optional, tag = "2")]
    pub ne: ::core::option::Option<super::super::super::r#type::LatLng>,
}
/// Response message for `Solar.FindClosestBuildingInsights`.
/// Information about the location, dimensions, and solar potential of a
/// building.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuildingInsights {
    /// The resource name for the building, of the format `building/<place ID>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A point near the center of the building.
    #[prost(message, optional, tag = "2")]
    pub center: ::core::option::Option<super::super::super::r#type::LatLng>,
    /// The bounding box of the building.
    #[prost(message, optional, tag = "9")]
    pub bounding_box: ::core::option::Option<LatLngBox>,
    /// Date that the underlying imagery was acquired. This is approximate.
    #[prost(message, optional, tag = "3")]
    pub imagery_date: ::core::option::Option<super::super::super::r#type::Date>,
    /// When processing was completed on this imagery.
    #[prost(message, optional, tag = "11")]
    pub imagery_processed_date: ::core::option::Option<
        super::super::super::r#type::Date,
    >,
    /// Postal code (e.g., US zip code) this building is contained by.
    #[prost(string, tag = "4")]
    pub postal_code: ::prost::alloc::string::String,
    /// Administrative area 1 (e.g., in the US, the state) that contains this
    /// building. For example, in the US, the abbreviation might be "MA" or "CA."
    #[prost(string, tag = "5")]
    pub administrative_area: ::prost::alloc::string::String,
    /// Statistical area (e.g., US census tract) this building is in.
    #[prost(string, tag = "6")]
    pub statistical_area: ::prost::alloc::string::String,
    /// Region code for the country (or region) this building is in.
    #[prost(string, tag = "7")]
    pub region_code: ::prost::alloc::string::String,
    /// Solar potential of the building.
    #[prost(message, optional, tag = "8")]
    pub solar_potential: ::core::option::Option<SolarPotential>,
    /// The quality of the imagery used to compute the data for this building.
    #[prost(enumeration = "ImageryQuality", tag = "10")]
    pub imagery_quality: i32,
}
/// Information about the solar potential of a building. A number of
/// fields in this are defined in terms of "panels". The fields
/// \[panel_capacity_watts\]
/// \[google.maps.solar.v1.SolarPotential.panel_capacity_watts\],
/// \[panel_height_meters\]
/// \[google.maps.solar.v1.SolarPotential.panel_height_meters\],
/// and \[panel_width_meters\]
/// \[google.maps.solar.v1.SolarPotential.panel_width_meters\]
/// describe the parameters of the model of panel used in these
/// calculations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SolarPotential {
    /// Size of the maximum array - that is, the maximum number of panels that
    /// can fit on the roof.
    #[prost(int32, tag = "1")]
    pub max_array_panels_count: i32,
    /// Capacity, in watts, of the panel used in the calculations.
    #[prost(float, tag = "9")]
    pub panel_capacity_watts: f32,
    /// Height, in meters in portrait orientation, of the panel used in
    /// the calculations.
    #[prost(float, tag = "10")]
    pub panel_height_meters: f32,
    /// Width, in meters in portrait orientation, of the panel used in
    /// the calculations.
    #[prost(float, tag = "11")]
    pub panel_width_meters: f32,
    /// The expected lifetime, in years, of the solar panels. This is
    /// used in the financial calculations.
    #[prost(int32, tag = "12")]
    pub panel_lifetime_years: i32,
    /// Size, in square meters, of the maximum array.
    #[prost(float, tag = "2")]
    pub max_array_area_meters2: f32,
    /// Maximum number of sunshine hours received per year, by any point
    /// on the roof. Sunshine hours are a measure of the total amount of
    /// insolation (energy) received per year. 1 sunshine hour = 1 kWh per kW
    /// (where kW refers to kW of capacity under Standard Testing Conditions).
    #[prost(float, tag = "3")]
    pub max_sunshine_hours_per_year: f32,
    /// Equivalent amount of CO2 produced per MWh of grid electricity. This
    /// is a measure of the carbon intensity of grid electricity displaced
    /// by solar electricity.
    #[prost(float, tag = "4")]
    pub carbon_offset_factor_kg_per_mwh: f32,
    /// Total size and sunlight quantiles for the part of the roof that
    /// was assigned to some roof segment. Despite the name, this may not
    /// include the entire building. See \[building_stats\]
    /// \[google.maps.solar.v1.SolarPotential.building_stats\].
    #[prost(message, optional, tag = "5")]
    pub whole_roof_stats: ::core::option::Option<SizeAndSunshineStats>,
    /// Size and sunlight quantiles for the entire building, including
    /// parts of the roof that were not assigned to some roof segment.
    /// Because the orientations of these parts are not well
    /// characterised, the roof area estimate is unreliable, but the
    /// ground area estimate is reliable. It may be that a more reliable
    /// whole building roof area can be obtained by scaling the roof area
    /// from \[whole_roof_stats\]
    /// \[google.maps.solar.v1.SolarPotential.whole_roof_stats\] by
    /// the ratio of the ground areas of `building_stats` and
    /// `whole_roof_stats`.
    #[prost(message, optional, tag = "13")]
    pub building_stats: ::core::option::Option<SizeAndSunshineStats>,
    /// Size and sunlight quantiles for each roof segment.
    #[prost(message, repeated, tag = "6")]
    pub roof_segment_stats: ::prost::alloc::vec::Vec<RoofSegmentSizeAndSunshineStats>,
    /// Each \[SolarPanel\] [google.maps.solar.v1.SolarPanel]
    /// describes a single solar panel. They are listed in the order that
    /// the panel layout algorithm placed this. This is usually, though
    /// not always, in decreasing order of annual energy production.
    #[prost(message, repeated, tag = "14")]
    pub solar_panels: ::prost::alloc::vec::Vec<SolarPanel>,
    /// Each \[SolarPanelConfig\]
    /// \[google.maps.solar.v1.SolarPanelConfig\] describes a
    /// different arrangement of solar panels on the roof. They are in
    /// order of increasing number of panels. The `SolarPanelConfig` with
    /// \[panels_count\]
    /// \[google.maps.solar.v1.SolarPanelConfig.panels_count\]=N is
    /// based on the first N panels in the `solar_panels` list.
    #[prost(message, repeated, tag = "7")]
    pub solar_panel_configs: ::prost::alloc::vec::Vec<SolarPanelConfig>,
    /// A \[FinancialAnalysis\]
    /// \[google.maps.solar.v1.FinancialAnalysis\] gives the savings
    /// from going solar assuming a given monthly bill and a given
    /// electricity provider. They are in order of increasing order of
    /// monthly bill amount. This field will be empty for buildings in
    /// areas for which the Solar API does not have enough information to
    /// perform financial computations.
    #[prost(message, repeated, tag = "8")]
    pub financial_analyses: ::prost::alloc::vec::Vec<FinancialAnalysis>,
}
/// Information about the size and sunniness quantiles of a roof segment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoofSegmentSizeAndSunshineStats {
    /// Angle of the roof segment relative to the theoretical ground plane.
    /// 0 = parallel to the ground, 90 = perpendicular to the ground.
    #[prost(float, optional, tag = "1")]
    pub pitch_degrees: ::core::option::Option<f32>,
    /// Compass direction the roof segment is pointing in. 0 = North, 90 =
    /// East, 180 = South. For a "flat" roof segment (`pitch_degrees` very
    /// near 0), azimuth is not well defined, so for consistency, we define it
    /// arbitrarily to be 0 (North).
    #[prost(float, optional, tag = "2")]
    pub azimuth_degrees: ::core::option::Option<f32>,
    /// Total size and sunlight quantiles for the roof segment.
    #[prost(message, optional, tag = "3")]
    pub stats: ::core::option::Option<SizeAndSunshineStats>,
    /// A point near the center of the roof segment.
    #[prost(message, optional, tag = "4")]
    pub center: ::core::option::Option<super::super::super::r#type::LatLng>,
    /// The bounding box of the roof segment.
    #[prost(message, optional, tag = "5")]
    pub bounding_box: ::core::option::Option<LatLngBox>,
    /// The height of the roof segment plane, in meters above sea level,
    /// at the point designated by `center`. Together with the pitch,
    /// azimuth, and center location, this fully defines the roof segment
    /// plane.
    #[prost(float, optional, tag = "6")]
    pub plane_height_at_center_meters: ::core::option::Option<f32>,
}
/// Size and sunniness quantiles of a roof, or part of a roof.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SizeAndSunshineStats {
    /// The area of the roof or roof segment, in m^2. This is the roof area
    /// (accounting for tilt), not the ground footprint area.
    #[prost(float, tag = "1")]
    pub area_meters2: f32,
    /// Quantiles of the pointwise sunniness across the area. If there
    /// are N values here, this represents the (N-1)-iles. For example,
    /// if there are 5 values, then they would be the quartiles (min,
    /// 25%, 50%, 75%, max). Values are in annual kWh/kW like
    /// \[max_sunshine_hours_per_year\]
    /// \[google.maps.solar.v1.SolarPotential.max_sunshine_hours_per_year\].
    #[prost(float, repeated, tag = "2")]
    pub sunshine_quantiles: ::prost::alloc::vec::Vec<f32>,
    /// The ground footprint area covered by the roof or roof segment, in m^2.
    #[prost(float, tag = "3")]
    pub ground_area_meters2: f32,
}
/// SolarPanel describes the position, orientation, and production of a
/// single solar panel. See the \[panel_height_meters\]
/// \[google.maps.solar.v1.SolarPotential.panel_height_meters\],
/// \[panel_width_meters\]
/// \[google.maps.solar.v1.SolarPotential.panel_width_meters\],
/// and \[panel_capacity_watts\]
/// \[google.maps.solar.v1.SolarPotential.panel_capacity_watts\]
/// fields in \[SolarPotential\]
/// \[google.maps.solar.v1.SolarPotential\] for information on the
/// parameters of the panel.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SolarPanel {
    /// The centre of the panel.
    #[prost(message, optional, tag = "1")]
    pub center: ::core::option::Option<super::super::super::r#type::LatLng>,
    /// The orientation of the panel.
    #[prost(enumeration = "SolarPanelOrientation", tag = "2")]
    pub orientation: i32,
    /// How much sunlight energy this layout captures over the course of a
    /// year, in DC kWh.
    #[prost(float, tag = "3")]
    pub yearly_energy_dc_kwh: f32,
    /// Index in \[roof_segment_stats\]
    /// \[google.maps.solar.v1.SolarPotential.roof_segment_stats\]
    /// of the `RoofSegmentSizeAndSunshineStats` which corresponds to the
    /// roof segment that this panel is placed on.
    #[prost(int32, optional, tag = "4")]
    pub segment_index: ::core::option::Option<i32>,
}
/// SolarPanelConfig describes a particular placement of solar panels
/// on the roof.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SolarPanelConfig {
    /// Total number of panels. Note that this is redundant to (the sum
    /// of) the corresponding fields in \[roof_segment_summaries\]
    /// \[google.maps.solar.v1.SolarPanelConfig.roof_segment_summaries\].
    #[prost(int32, tag = "1")]
    pub panels_count: i32,
    /// How much sunlight energy this layout captures over the course of a
    /// year, in DC kWh, assuming the panels described above.
    #[prost(float, tag = "2")]
    pub yearly_energy_dc_kwh: f32,
    /// Information about the production of each roof segment that is carrying
    /// at least one panel in this layout. `roof_segment_summaries\[i\]` describes
    /// the i-th roof segment, including its size, expected production and
    /// orientation.
    #[prost(message, repeated, tag = "4")]
    pub roof_segment_summaries: ::prost::alloc::vec::Vec<RoofSegmentSummary>,
}
/// Information about a roof segment on the building, with some number of
/// panels placed on it.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoofSegmentSummary {
    /// Angle of the roof segment relative to the theoretical ground plane.
    /// 0 = parallel to the ground, 90 = perpendicular to the ground.
    #[prost(float, optional, tag = "2")]
    pub pitch_degrees: ::core::option::Option<f32>,
    /// Compass direction the roof segment is pointing in. 0 = North, 90 =
    /// East, 180 = South. For a "flat" roof segment (`pitch_degrees` very
    /// near 0), azimuth is not well defined, so for consistency, we define it
    /// arbitrarily to be 0 (North).
    #[prost(float, optional, tag = "3")]
    pub azimuth_degrees: ::core::option::Option<f32>,
    /// The total number of panels on this segment.
    #[prost(int32, tag = "7")]
    pub panels_count: i32,
    /// How much sunlight energy this part of the layout captures over the
    /// course of a year, in DC kWh, assuming the panels described above.
    #[prost(float, tag = "8")]
    pub yearly_energy_dc_kwh: f32,
    /// Index in \[roof_segment_stats\]
    /// \[google.maps.solar.v1.SolarPotential.roof_segment_stats\]
    /// of the corresponding `RoofSegmentSizeAndSunshineStats`.
    #[prost(int32, optional, tag = "9")]
    pub segment_index: ::core::option::Option<i32>,
}
/// Analysis of the cost and benefits of the optimum solar layout for a
/// particular electric bill size.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinancialAnalysis {
    /// The monthly electric bill this analysis assumes.
    #[prost(message, optional, tag = "3")]
    pub monthly_bill: ::core::option::Option<super::super::super::r#type::Money>,
    /// Whether this is the bill size selected to be the default bill for the
    /// area this building is in. Exactly one `FinancialAnalysis` in
    /// `BuildingSolarPotential` should have `default_bill` set.
    #[prost(bool, tag = "4")]
    pub default_bill: bool,
    /// How much electricity the house uses in an average month, based on the
    /// bill size and the local electricity rates.
    #[prost(float, tag = "5")]
    pub average_kwh_per_month: f32,
    /// Index in \[solar_panel_configs\]
    /// \[google.maps.solar.v1.SolarPotential.solar_panel_configs\]
    /// of the optimum solar layout for this bill size. This can be -1
    /// indicating that there is no layout. In this case, the remaining
    /// submessages will be omitted.
    #[prost(int32, optional, tag = "6")]
    pub panel_config_index: ::core::option::Option<i32>,
    /// Financial information that applies regardless of the financing method
    /// used.
    #[prost(message, optional, tag = "7")]
    pub financial_details: ::core::option::Option<FinancialDetails>,
    /// Cost and benefit of leasing the solar panels.
    #[prost(message, optional, tag = "8")]
    pub leasing_savings: ::core::option::Option<LeasingSavings>,
    /// Cost and benefit of buying the solar panels with cash.
    #[prost(message, optional, tag = "9")]
    pub cash_purchase_savings: ::core::option::Option<CashPurchaseSavings>,
    /// Cost and benefit of buying the solar panels by financing the purchase.
    #[prost(message, optional, tag = "10")]
    pub financed_purchase_savings: ::core::option::Option<FinancedPurchaseSavings>,
}
/// Details of a financial analysis. Some of these details are already
/// stored at higher levels (e.g., out of pocket cost). Total money
/// amounts are over a lifetime period defined by the
/// \[panel_lifetime_years\]
/// \[google.maps.solar.v1.SolarPotential.panel_lifetime_years\]
/// field in \[SolarPotential\]
/// \[google.maps.solar.v1.SolarPotential\]. Note: The out of
/// pocket cost of purchasing the panels is given in the
/// \[out_of_pocket_cost\]
/// \[google.maps.solar.v1.CashPurchaseSavings.out_of_pocket_cost\]
/// field in \[CashPurchaseSavings\]
/// \[google.maps.solar.v1.CashPurchaseSavings\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinancialDetails {
    /// How many AC kWh we think the solar panels will generate in their first
    /// year.
    #[prost(float, tag = "1")]
    pub initial_ac_kwh_per_year: f32,
    /// Utility bill for electricity not produced by solar, for the
    /// lifetime of the panels.
    #[prost(message, optional, tag = "2")]
    pub remaining_lifetime_utility_bill: ::core::option::Option<
        super::super::super::r#type::Money,
    >,
    /// Amount of money available from federal incentives; this applies if the
    /// user buys (with or without a loan) the panels.
    #[prost(message, optional, tag = "3")]
    pub federal_incentive: ::core::option::Option<super::super::super::r#type::Money>,
    /// Amount of money available from state incentives; this applies if the
    /// user buys (with or without a loan) the panels.
    #[prost(message, optional, tag = "4")]
    pub state_incentive: ::core::option::Option<super::super::super::r#type::Money>,
    /// Amount of money available from utility incentives; this applies if the
    /// user buys (with or without a loan) the panels.
    #[prost(message, optional, tag = "5")]
    pub utility_incentive: ::core::option::Option<super::super::super::r#type::Money>,
    /// Amount of money the user will receive from Solar Renewable Energy
    /// Credits over the panel lifetime; this applies if the user buys
    /// (with or without a loan) the panels.
    #[prost(message, optional, tag = "6")]
    pub lifetime_srec_total: ::core::option::Option<super::super::super::r#type::Money>,
    /// Total cost of electricity the user would have paid over the
    /// lifetime period if they didn't install solar.
    #[prost(message, optional, tag = "7")]
    pub cost_of_electricity_without_solar: ::core::option::Option<
        super::super::super::r#type::Money,
    >,
    /// Whether net metering is allowed.
    #[prost(bool, tag = "8")]
    pub net_metering_allowed: bool,
    /// Percentage (0-100) of the user's power supplied by solar.
    /// Valid for the first year but approximately correct for future years.
    #[prost(float, optional, tag = "9")]
    pub solar_percentage: ::core::option::Option<f32>,
    /// The percentage (0-100) of solar electricity production we assumed was
    /// exported to the grid, based on the first quarter of production. This
    /// affects the calculations if net metering is not allowed.
    #[prost(float, optional, tag = "10")]
    pub percentage_exported_to_grid: ::core::option::Option<f32>,
}
/// Financial information that's shared between different financing methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SavingsOverTime {
    /// Savings in the first year after panel installation.
    #[prost(message, optional, tag = "1")]
    pub savings_year1: ::core::option::Option<super::super::super::r#type::Money>,
    /// Savings in the first twenty years after panel installation.
    #[prost(message, optional, tag = "2")]
    pub savings_year20: ::core::option::Option<super::super::super::r#type::Money>,
    /// Using the assumed discount rate, what is the present value of the
    /// cumulative 20-year savings?
    #[prost(message, optional, tag = "3")]
    pub present_value_of_savings_year20: ::core::option::Option<
        super::super::super::r#type::Money,
    >,
    /// Savings in the entire panel lifetime.
    #[prost(message, optional, tag = "5")]
    pub savings_lifetime: ::core::option::Option<super::super::super::r#type::Money>,
    /// Using the assumed discount rate, what is the present value of the
    /// cumulative lifetime savings?
    #[prost(message, optional, tag = "6")]
    pub present_value_of_savings_lifetime: ::core::option::Option<
        super::super::super::r#type::Money,
    >,
    /// Indicates whether this scenario is financially viable.  Will be false for
    /// scenarios with poor financial viability (e.g., money-losing).
    #[prost(bool, tag = "4")]
    pub financially_viable: bool,
}
/// Cost and benefit of leasing a particular configuration of solar panels
/// with a particular electricity usage.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeasingSavings {
    /// Whether leases are allowed in this juristiction (leases are not
    /// allowed in some states). If this field is false, then the values in
    /// this message should probably be ignored.
    #[prost(bool, tag = "1")]
    pub leases_allowed: bool,
    /// Whether leases are supported in this juristiction by the financial
    /// calculation engine. If this field is false, then the values in this
    /// message should probably be ignored. This is independent of
    /// `leases_allowed`: in some areas leases are allowed, but under conditions
    /// that aren't handled by the financial models.
    #[prost(bool, tag = "2")]
    pub leases_supported: bool,
    /// Estimated annual leasing cost.
    #[prost(message, optional, tag = "3")]
    pub annual_leasing_cost: ::core::option::Option<super::super::super::r#type::Money>,
    /// How much is saved (or not) over the lifetime period.
    #[prost(message, optional, tag = "4")]
    pub savings: ::core::option::Option<SavingsOverTime>,
}
/// Cost and benefit of an outright purchase of a particular configuration
/// of solar panels with a particular electricity usage.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CashPurchaseSavings {
    /// Initial cost before tax incentives: the amount that must be paid
    /// out-of-pocket. Contrast with `upfront_cost`, which is after tax incentives.
    #[prost(message, optional, tag = "1")]
    pub out_of_pocket_cost: ::core::option::Option<super::super::super::r#type::Money>,
    /// Initial cost after tax incentives: it's the amount that must be paid
    /// during first year. Contrast with `out_of_pocket_cost`, which is before tax
    /// incentives.
    #[prost(message, optional, tag = "2")]
    pub upfront_cost: ::core::option::Option<super::super::super::r#type::Money>,
    /// The value of all tax rebates.
    #[prost(message, optional, tag = "3")]
    pub rebate_value: ::core::option::Option<super::super::super::r#type::Money>,
    /// Number of years until payback occurs. A negative value means payback
    /// never occurs within the lifetime period.
    #[prost(float, optional, tag = "4")]
    pub payback_years: ::core::option::Option<f32>,
    /// How much is saved (or not) over the lifetime period.
    #[prost(message, optional, tag = "5")]
    pub savings: ::core::option::Option<SavingsOverTime>,
}
/// Cost and benefit of using a loan to buy a particular configuration
/// of solar panels with a particular electricity usage.
/// Initial out of pocket cost is zero in our model: the loan covers
/// everything.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinancedPurchaseSavings {
    /// Annual loan payments.
    #[prost(message, optional, tag = "1")]
    pub annual_loan_payment: ::core::option::Option<super::super::super::r#type::Money>,
    /// The value of all tax rebates (including Federal Investment Tax Credit
    /// (ITC)).
    #[prost(message, optional, tag = "2")]
    pub rebate_value: ::core::option::Option<super::super::super::r#type::Money>,
    /// The interest rate on loans assumed in this set of calculations.
    #[prost(float, tag = "3")]
    pub loan_interest_rate: f32,
    /// How much is saved (or not) over the lifetime period.
    #[prost(message, optional, tag = "4")]
    pub savings: ::core::option::Option<SavingsOverTime>,
}
/// Request message for `Solar.GetDataLayers`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataLayersRequest {
    /// The longitude and latitude for the center of the region to get data for.
    #[prost(message, optional, tag = "1")]
    pub location: ::core::option::Option<super::super::super::r#type::LatLng>,
    /// The radius, in meters, defining the region surrounding that
    /// centre point for which data should be returned. The limitations
    /// on this value are:
    ///
    /// * Any value up to 100m can always be specified.
    /// * Values over 100m can be specified, as long as
    ///    `radius_meters` <= `pixel_size_meters * 1000`.
    /// * However, for values over 175m, the `DataLayerView` in the
    ///    request must not include monthly flux or hourly shade.
    #[prost(float, tag = "2")]
    pub radius_meters: f32,
    /// The desired subset of the data to return.
    #[prost(enumeration = "DataLayerView", tag = "3")]
    pub view: i32,
    /// The minimum quality level allowed in the results. No result with
    /// lower quality than this will be returned. Not specifying this is
    /// equivalent to restricting to HIGH quality only.
    #[prost(enumeration = "ImageryQuality", tag = "5")]
    pub required_quality: i32,
    /// The minimum scale, in meters per pixel, of the data to return.
    /// Values of 0.1 (the default, if this field is not set explicitly),
    /// 0.25, 0.5, and 1.0 are supported. Imagery components whose normal
    /// resolution is less than `pixel_size_meters` will be returned at
    /// the resolution specified by `pixel_size_meters`; imagery
    /// components whose normal resolution is equal to or greater than
    /// `pixel_size_meters` will be returned at that normal resolution.
    #[prost(float, tag = "6")]
    pub pixel_size_meters: f32,
}
/// Information about the solar potential of a region. The actual data
/// are contained in a number of GeoTIFF files covering the requested
/// region, for which this message contains URLs: Each string in the
/// `DataLayers` message contains a URL from which the
/// corresponding GeoTIFF can be fetched. These URLs are valid for a
/// few hours after they've been generated. Most of the GeoTIFF files
/// are at a resolution of 0.1m/pixel, but the monthly flux file is at
/// 0.5m/pixel, and the hourly shade files are at 1m/pixel. If a
/// `pixel_size_meters` value was specified in the
/// `GetDataLayersRequest`, then the minimum resolution in the GeoTIFF
/// files will be that value.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataLayers {
    /// When the source imagery (from which all the other data are derived) in this
    /// region was taken. It is necessarily somewhat approximate, as the images may
    /// have been taken over more than one day.
    #[prost(message, optional, tag = "1")]
    pub imagery_date: ::core::option::Option<super::super::super::r#type::Date>,
    /// When processing was completed on this imagery.
    #[prost(message, optional, tag = "2")]
    pub imagery_processed_date: ::core::option::Option<
        super::super::super::r#type::Date,
    >,
    /// The URL for an image of the DSM (Digital Surface Model) of the region.
    /// Values are in meters above EGM96 geoid (i.e., sea level). Invalid locations
    /// (where we don't have data) are stored as -9999.
    #[prost(string, tag = "3")]
    pub dsm_url: ::prost::alloc::string::String,
    /// The URL for an image of RGB data (aerial photo) of the region.
    #[prost(string, tag = "4")]
    pub rgb_url: ::prost::alloc::string::String,
    /// The URL for the building mask image: one bit per pixel saying whether that
    /// pixel is considered to be part of a rooftop or not.
    #[prost(string, tag = "5")]
    pub mask_url: ::prost::alloc::string::String,
    /// The URL for the annual flux map (annual sunlight on roofs) of the region.
    /// Values are kWh/kW/year. This is *unmasked flux*: flux is computed for every
    /// location, not just building rooftops. Invalid locations are stored as
    /// -9999: locations outside our coverage area will be invalid, and a few
    /// locations inside the coverage area, where we were unable to calculate flux,
    /// will also be invalid.
    #[prost(string, tag = "6")]
    pub annual_flux_url: ::prost::alloc::string::String,
    /// The URL for the monthly flux map (sunlight on roofs, broken down by month)
    /// of the region. Values are kWh/kW/year. The GeoTIFF pointed to by this URL
    /// will contain twelve bands, corresponding to January...December, in order.
    #[prost(string, tag = "7")]
    pub monthly_flux_url: ::prost::alloc::string::String,
    /// Twelve URLs for hourly shade, corresponding to January...December, in
    /// order. Each GeoTIFF will contain 24 bands, corresponding to the 24 hours of
    /// the day. Each pixel is a 32 bit integer, corresponding to the (up to) 31
    /// days of that month; a 1 bit means that the corresponding location is able
    /// to see the sun at that day, of that hour, of that month. Invalid locations
    /// are stored as -9999 (since this is negative, it has bit 31 set, and no
    /// valid value could have bit 31 set as that would correspond to the 32nd day
    /// of the month).
    ///
    /// An example may be useful. If you want to know whether a point (at
    /// pixel location (x, y)) saw sun at 4pm on the 22nd of June you
    /// would:
    ///
    /// 1. fetch the sixth URL in this list (corresponding to June).
    /// 1. look up the 17th channel (corresponding to 4pm).
    /// 1. read the 32-bit value at (x, y).
    /// 1. read bit 21 of the value (corresponding to the 22nd of the month).
    /// 1. if that bit is a 1, then that spot saw the sun at 4pm 22 June.
    ///
    /// More formally:
    /// Given `month` (1-12), `day` (1...month max; February has 28 days)
    /// and `hour` (0-23), the shade/sun for that month/day/hour at a
    /// position `(x, y)` is the bit
    /// ```
    /// (hourly_shade\[month - 1\])(x, y)\[hour\] & (1 << (day - 1))
    /// ```
    /// where `(x, y)` is spatial indexing, `\[month - 1\]` refers to
    /// fetching the `month - 1`st URL (indexing from zero), `\[hour\]` is
    /// indexing into the channels, and a final non-zero result means
    /// "sunny". There are no leap days, and DST doesn't exist (all days
    /// are 24 hours long; noon is always "standard time" noon).
    #[prost(string, repeated, tag = "8")]
    pub hourly_shade_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The quality of the result's imagery.
    #[prost(enumeration = "ImageryQuality", tag = "9")]
    pub imagery_quality: i32,
}
/// Request message for `Solar.GetGeoTiff`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGeoTiffRequest {
    /// The ID of the asset being requested.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// What subset of the solar information to return.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DataLayerView {
    /// Equivalent to FULL.
    Unspecified = 0,
    /// Get the DSM only.
    DsmLayer = 1,
    /// Get the DSM, RGB, and mask.
    ImageryLayers = 2,
    /// Get the DSM, RGB, mask, and annual flux.
    ImageryAndAnnualFluxLayers = 3,
    /// Get the DSM, RGB, mask, annual flux, and monthly flux.
    ImageryAndAllFluxLayers = 4,
    /// Get all data.
    FullLayers = 5,
}
impl DataLayerView {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DataLayerView::Unspecified => "DATA_LAYER_VIEW_UNSPECIFIED",
            DataLayerView::DsmLayer => "DSM_LAYER",
            DataLayerView::ImageryLayers => "IMAGERY_LAYERS",
            DataLayerView::ImageryAndAnnualFluxLayers => "IMAGERY_AND_ANNUAL_FLUX_LAYERS",
            DataLayerView::ImageryAndAllFluxLayers => "IMAGERY_AND_ALL_FLUX_LAYERS",
            DataLayerView::FullLayers => "FULL_LAYERS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DATA_LAYER_VIEW_UNSPECIFIED" => Some(Self::Unspecified),
            "DSM_LAYER" => Some(Self::DsmLayer),
            "IMAGERY_LAYERS" => Some(Self::ImageryLayers),
            "IMAGERY_AND_ANNUAL_FLUX_LAYERS" => Some(Self::ImageryAndAnnualFluxLayers),
            "IMAGERY_AND_ALL_FLUX_LAYERS" => Some(Self::ImageryAndAllFluxLayers),
            "FULL_LAYERS" => Some(Self::FullLayers),
            _ => None,
        }
    }
}
/// The quality of the imagery used to compute some API result.
///
/// Note: Regardless of imagery quality level, DSM outputs always have a
/// resolution of 0.1 m/pixel, monthly flux outputs always have a resolution of
/// 0.5 m/pixel, and hourly shade outputs always have a resolution of 1 m/pixel.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ImageryQuality {
    /// No quality is known.
    Unspecified = 0,
    /// The underlying imagery and DSM data were processed at 0.1 m/pixel.
    High = 1,
    /// The underlying imagery and DSM data were processed at 0.25 m/pixel.
    Medium = 2,
    /// The underlying imagery and DSM data were processed at 0.5 m/pixel.
    Low = 3,
}
impl ImageryQuality {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ImageryQuality::Unspecified => "IMAGERY_QUALITY_UNSPECIFIED",
            ImageryQuality::High => "HIGH",
            ImageryQuality::Medium => "MEDIUM",
            ImageryQuality::Low => "LOW",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "IMAGERY_QUALITY_UNSPECIFIED" => Some(Self::Unspecified),
            "HIGH" => Some(Self::High),
            "MEDIUM" => Some(Self::Medium),
            "LOW" => Some(Self::Low),
            _ => None,
        }
    }
}
/// The orientation of a solar panel. This must be interpreted relative to the
/// azimuth of the roof segment that the panel is placed on.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SolarPanelOrientation {
    /// No panel orientation is known.
    Unspecified = 0,
    /// A `LANDSCAPE` panel has its long edge perpendicular to the
    /// azimuth direction of the roof segment that it is placed on.
    Landscape = 1,
    /// A `PORTRAIT` panel has its long edge parallel to the azimuth
    /// direction of the roof segment that it is placed on.
    Portrait = 2,
}
impl SolarPanelOrientation {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SolarPanelOrientation::Unspecified => "SOLAR_PANEL_ORIENTATION_UNSPECIFIED",
            SolarPanelOrientation::Landscape => "LANDSCAPE",
            SolarPanelOrientation::Portrait => "PORTRAIT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SOLAR_PANEL_ORIENTATION_UNSPECIFIED" => Some(Self::Unspecified),
            "LANDSCAPE" => Some(Self::Landscape),
            "PORTRAIT" => Some(Self::Portrait),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod solar_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service definition for the Solar API.
    #[derive(Debug, Clone)]
    pub struct SolarClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SolarClient<tonic::transport::Channel> {
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
    impl<T> SolarClient<T>
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
        ) -> SolarClient<InterceptedService<T, F>>
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
            SolarClient::new(InterceptedService::new(inner, interceptor))
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
        /// Locates the closest building to a query point. Returns an error with
        /// code `NOT_FOUND` if there are no buildings within approximately 50m of the
        /// query point.
        pub async fn find_closest_building_insights(
            &mut self,
            request: impl tonic::IntoRequest<super::FindClosestBuildingInsightsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BuildingInsights>,
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
                "/google.maps.solar.v1.Solar/FindClosestBuildingInsights",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.maps.solar.v1.Solar",
                        "FindClosestBuildingInsights",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets solar information for a region surrounding a location.
        /// Returns an error with code `NOT_FOUND` if the location is outside
        /// the coverage area.
        pub async fn get_data_layers(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDataLayersRequest>,
        ) -> std::result::Result<tonic::Response<super::DataLayers>, tonic::Status> {
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
                "/google.maps.solar.v1.Solar/GetDataLayers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.maps.solar.v1.Solar", "GetDataLayers"));
            self.inner.unary(req, path, codec).await
        }
        /// Returns an image by its ID.
        pub async fn get_geo_tiff(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGeoTiffRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::api::HttpBody>,
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
                "/google.maps.solar.v1.Solar/GetGeoTiff",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.maps.solar.v1.Solar", "GetGeoTiff"));
            self.inner.unary(req, path, codec).await
        }
    }
}
