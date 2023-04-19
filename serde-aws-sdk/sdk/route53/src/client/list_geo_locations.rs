// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListGeoLocations`](crate::operation::list_geo_locations::builders::ListGeoLocationsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`start_continent_code(impl Into<String>)`](crate::operation::list_geo_locations::builders::ListGeoLocationsFluentBuilder::start_continent_code) / [`set_start_continent_code(Option<String>)`](crate::operation::list_geo_locations::builders::ListGeoLocationsFluentBuilder::set_start_continent_code): <p>The code for the continent with which you want to start listing locations that Amazon Route 53 supports for geolocation. If Route 53 has already returned a page or more of results, if <code>IsTruncated</code> is true, and if <code>NextContinentCode</code> from the previous response has a value, enter that value in <code>startcontinentcode</code> to return the next page of results.</p>  <p>Include <code>startcontinentcode</code> only if you want to list continents. Don't include <code>startcontinentcode</code> when you're listing countries or countries with their subdivisions.</p>
    ///   - [`start_country_code(impl Into<String>)`](crate::operation::list_geo_locations::builders::ListGeoLocationsFluentBuilder::start_country_code) / [`set_start_country_code(Option<String>)`](crate::operation::list_geo_locations::builders::ListGeoLocationsFluentBuilder::set_start_country_code): <p>The code for the country with which you want to start listing locations that Amazon Route 53 supports for geolocation. If Route 53 has already returned a page or more of results, if <code>IsTruncated</code> is <code>true</code>, and if <code>NextCountryCode</code> from the previous response has a value, enter that value in <code>startcountrycode</code> to return the next page of results.</p>
    ///   - [`start_subdivision_code(impl Into<String>)`](crate::operation::list_geo_locations::builders::ListGeoLocationsFluentBuilder::start_subdivision_code) / [`set_start_subdivision_code(Option<String>)`](crate::operation::list_geo_locations::builders::ListGeoLocationsFluentBuilder::set_start_subdivision_code): <p>The code for the state of the United States with which you want to start listing locations that Amazon Route 53 supports for geolocation. If Route 53 has already returned a page or more of results, if <code>IsTruncated</code> is <code>true</code>, and if <code>NextSubdivisionCode</code> from the previous response has a value, enter that value in <code>startsubdivisioncode</code> to return the next page of results.</p>  <p>To list subdivisions (U.S. states), you must include both <code>startcountrycode</code> and <code>startsubdivisioncode</code>.</p>
    ///   - [`max_items(i32)`](crate::operation::list_geo_locations::builders::ListGeoLocationsFluentBuilder::max_items) / [`set_max_items(Option<i32>)`](crate::operation::list_geo_locations::builders::ListGeoLocationsFluentBuilder::set_max_items): <p>(Optional) The maximum number of geolocations to be included in the response body for this request. If more than <code>maxitems</code> geolocations remain to be listed, then the value of the <code>IsTruncated</code> element in the response is <code>true</code>.</p>
    /// - On success, responds with [`ListGeoLocationsOutput`](crate::operation::list_geo_locations::ListGeoLocationsOutput) with field(s):
    ///   - [`geo_location_details_list(Option<Vec<GeoLocationDetails>>)`](crate::operation::list_geo_locations::ListGeoLocationsOutput::geo_location_details_list): <p>A complex type that contains one <code>GeoLocationDetails</code> element for each location that Amazon Route 53 supports for geolocation.</p>
    ///   - [`is_truncated(bool)`](crate::operation::list_geo_locations::ListGeoLocationsOutput::is_truncated): <p>A value that indicates whether more locations remain to be listed after the last location in this response. If so, the value of <code>IsTruncated</code> is <code>true</code>. To get more values, submit another request and include the values of <code>NextContinentCode</code>, <code>NextCountryCode</code>, and <code>NextSubdivisionCode</code> in the <code>startcontinentcode</code>, <code>startcountrycode</code>, and <code>startsubdivisioncode</code>, as applicable.</p>
    ///   - [`next_continent_code(Option<String>)`](crate::operation::list_geo_locations::ListGeoLocationsOutput::next_continent_code): <p>If <code>IsTruncated</code> is <code>true</code>, you can make a follow-up request to display more locations. Enter the value of <code>NextContinentCode</code> in the <code>startcontinentcode</code> parameter in another <code>ListGeoLocations</code> request.</p>
    ///   - [`next_country_code(Option<String>)`](crate::operation::list_geo_locations::ListGeoLocationsOutput::next_country_code): <p>If <code>IsTruncated</code> is <code>true</code>, you can make a follow-up request to display more locations. Enter the value of <code>NextCountryCode</code> in the <code>startcountrycode</code> parameter in another <code>ListGeoLocations</code> request.</p>
    ///   - [`next_subdivision_code(Option<String>)`](crate::operation::list_geo_locations::ListGeoLocationsOutput::next_subdivision_code): <p>If <code>IsTruncated</code> is <code>true</code>, you can make a follow-up request to display more locations. Enter the value of <code>NextSubdivisionCode</code> in the <code>startsubdivisioncode</code> parameter in another <code>ListGeoLocations</code> request.</p>
    ///   - [`max_items(Option<i32>)`](crate::operation::list_geo_locations::ListGeoLocationsOutput::max_items): <p>The value that you specified for <code>MaxItems</code> in the request.</p>
    /// - On failure, responds with [`SdkError<ListGeoLocationsError>`](crate::operation::list_geo_locations::ListGeoLocationsError)
    pub fn list_geo_locations(
        &self,
    ) -> crate::operation::list_geo_locations::builders::ListGeoLocationsFluentBuilder {
        crate::operation::list_geo_locations::builders::ListGeoLocationsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}