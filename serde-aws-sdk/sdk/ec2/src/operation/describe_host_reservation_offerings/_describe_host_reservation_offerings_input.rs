// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DescribeHostReservationOfferingsInput {
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>instance-family</code> - The instance family of the offering (for example, <code>m4</code>).</p> </li>
    /// <li> <p> <code>payment-option</code> - The payment option (<code>NoUpfront</code> | <code>PartialUpfront</code> | <code>AllUpfront</code>).</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub filter: std::option::Option<std::vec::Vec<crate::types::Filter>>,
    /// <p>This is the maximum duration of the reservation to purchase, specified in seconds. Reservations are available in one-year and three-year terms. The number of seconds specified must be the number of seconds in a year (365x24x60x60) times one of the supported durations (1 or 3). For example, specify 94608000 for three years.</p>
    #[doc(hidden)]
    pub max_duration: std::option::Option<i32>,
    /// <p>The maximum number of results to return for the request in a single page. The remaining results can be seen by sending another request with the returned <code>nextToken</code> value. This value can be between 5 and 500. If <code>maxResults</code> is given a larger value than 500, you receive an error.</p>
    #[doc(hidden)]
    pub max_results: std::option::Option<i32>,
    /// <p>This is the minimum duration of the reservation you'd like to purchase, specified in seconds. Reservations are available in one-year and three-year terms. The number of seconds specified must be the number of seconds in a year (365x24x60x60) times one of the supported durations (1 or 3). For example, specify 31536000 for one year.</p>
    #[doc(hidden)]
    pub min_duration: std::option::Option<i32>,
    /// <p>The token to use to retrieve the next page of results.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    /// <p>The ID of the reservation offering.</p>
    #[doc(hidden)]
    pub offering_id: std::option::Option<std::string::String>,
}
impl DescribeHostReservationOfferingsInput {
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>instance-family</code> - The instance family of the offering (for example, <code>m4</code>).</p> </li>
    /// <li> <p> <code>payment-option</code> - The payment option (<code>NoUpfront</code> | <code>PartialUpfront</code> | <code>AllUpfront</code>).</p> </li>
    /// </ul>
    pub fn filter(&self) -> std::option::Option<&[crate::types::Filter]> {
        self.filter.as_deref()
    }
    /// <p>This is the maximum duration of the reservation to purchase, specified in seconds. Reservations are available in one-year and three-year terms. The number of seconds specified must be the number of seconds in a year (365x24x60x60) times one of the supported durations (1 or 3). For example, specify 94608000 for three years.</p>
    pub fn max_duration(&self) -> std::option::Option<i32> {
        self.max_duration
    }
    /// <p>The maximum number of results to return for the request in a single page. The remaining results can be seen by sending another request with the returned <code>nextToken</code> value. This value can be between 5 and 500. If <code>maxResults</code> is given a larger value than 500, you receive an error.</p>
    pub fn max_results(&self) -> std::option::Option<i32> {
        self.max_results
    }
    /// <p>This is the minimum duration of the reservation you'd like to purchase, specified in seconds. Reservations are available in one-year and three-year terms. The number of seconds specified must be the number of seconds in a year (365x24x60x60) times one of the supported durations (1 or 3). For example, specify 31536000 for one year.</p>
    pub fn min_duration(&self) -> std::option::Option<i32> {
        self.min_duration
    }
    /// <p>The token to use to retrieve the next page of results.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The ID of the reservation offering.</p>
    pub fn offering_id(&self) -> std::option::Option<&str> {
        self.offering_id.as_deref()
    }
}
impl DescribeHostReservationOfferingsInput {
    /// Creates a new builder-style object to manufacture [`DescribeHostReservationOfferingsInput`](crate::operation::describe_host_reservation_offerings::DescribeHostReservationOfferingsInput).
    pub fn builder() -> crate::operation::describe_host_reservation_offerings::builders::DescribeHostReservationOfferingsInputBuilder{
        crate::operation::describe_host_reservation_offerings::builders::DescribeHostReservationOfferingsInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::describe_host_reservation_offerings::DescribeHostReservationOfferingsInput;
/// A builder for [`DescribeHostReservationOfferingsInput`](crate::operation::describe_host_reservation_offerings::DescribeHostReservationOfferingsInput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
pub struct DescribeHostReservationOfferingsInputBuilder {
    pub(crate) filter: std::option::Option<std::vec::Vec<crate::types::Filter>>,
    pub(crate) max_duration: std::option::Option<i32>,
    pub(crate) max_results: std::option::Option<i32>,
    pub(crate) min_duration: std::option::Option<i32>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    pub(crate) offering_id: std::option::Option<std::string::String>,
}
impl DescribeHostReservationOfferingsInputBuilder {
    /// Appends an item to `filter`.
    ///
    /// To override the contents of this collection use [`set_filter`](Self::set_filter).
    ///
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>instance-family</code> - The instance family of the offering (for example, <code>m4</code>).</p> </li>
    /// <li> <p> <code>payment-option</code> - The payment option (<code>NoUpfront</code> | <code>PartialUpfront</code> | <code>AllUpfront</code>).</p> </li>
    /// </ul>
    pub fn filter(mut self, input: crate::types::Filter) -> Self {
        let mut v = self.filter.unwrap_or_default();
        v.push(input);
        self.filter = Some(v);
        self
    }
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>instance-family</code> - The instance family of the offering (for example, <code>m4</code>).</p> </li>
    /// <li> <p> <code>payment-option</code> - The payment option (<code>NoUpfront</code> | <code>PartialUpfront</code> | <code>AllUpfront</code>).</p> </li>
    /// </ul>
    pub fn set_filter(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Filter>>,
    ) -> Self {
        self.filter = input;
        self
    }
    /// <p>This is the maximum duration of the reservation to purchase, specified in seconds. Reservations are available in one-year and three-year terms. The number of seconds specified must be the number of seconds in a year (365x24x60x60) times one of the supported durations (1 or 3). For example, specify 94608000 for three years.</p>
    pub fn max_duration(mut self, input: i32) -> Self {
        self.max_duration = Some(input);
        self
    }
    /// <p>This is the maximum duration of the reservation to purchase, specified in seconds. Reservations are available in one-year and three-year terms. The number of seconds specified must be the number of seconds in a year (365x24x60x60) times one of the supported durations (1 or 3). For example, specify 94608000 for three years.</p>
    pub fn set_max_duration(mut self, input: std::option::Option<i32>) -> Self {
        self.max_duration = input;
        self
    }
    /// <p>The maximum number of results to return for the request in a single page. The remaining results can be seen by sending another request with the returned <code>nextToken</code> value. This value can be between 5 and 500. If <code>maxResults</code> is given a larger value than 500, you receive an error.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = Some(input);
        self
    }
    /// <p>The maximum number of results to return for the request in a single page. The remaining results can be seen by sending another request with the returned <code>nextToken</code> value. This value can be between 5 and 500. If <code>maxResults</code> is given a larger value than 500, you receive an error.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>This is the minimum duration of the reservation you'd like to purchase, specified in seconds. Reservations are available in one-year and three-year terms. The number of seconds specified must be the number of seconds in a year (365x24x60x60) times one of the supported durations (1 or 3). For example, specify 31536000 for one year.</p>
    pub fn min_duration(mut self, input: i32) -> Self {
        self.min_duration = Some(input);
        self
    }
    /// <p>This is the minimum duration of the reservation you'd like to purchase, specified in seconds. Reservations are available in one-year and three-year terms. The number of seconds specified must be the number of seconds in a year (365x24x60x60) times one of the supported durations (1 or 3). For example, specify 31536000 for one year.</p>
    pub fn set_min_duration(mut self, input: std::option::Option<i32>) -> Self {
        self.min_duration = input;
        self
    }
    /// <p>The token to use to retrieve the next page of results.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>The token to use to retrieve the next page of results.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The ID of the reservation offering.</p>
    pub fn offering_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.offering_id = Some(input.into());
        self
    }
    /// <p>The ID of the reservation offering.</p>
    pub fn set_offering_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.offering_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeHostReservationOfferingsInput`](crate::operation::describe_host_reservation_offerings::DescribeHostReservationOfferingsInput).
    pub fn build(self) -> Result<crate::operation::describe_host_reservation_offerings::DescribeHostReservationOfferingsInput, aws_smithy_http::operation::error::BuildError>{
        Ok(
            crate::operation::describe_host_reservation_offerings::DescribeHostReservationOfferingsInput {
                filter: self.filter
                ,
                max_duration: self.max_duration
                ,
                max_results: self.max_results
                ,
                min_duration: self.min_duration
                ,
                next_token: self.next_token
                ,
                offering_id: self.offering_id
                ,
            }
        )
    }
}