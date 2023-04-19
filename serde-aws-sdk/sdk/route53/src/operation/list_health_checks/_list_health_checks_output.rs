// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A complex type that contains the response to a <code>ListHealthChecks</code> request.</p>
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
pub struct ListHealthChecksOutput {
    /// <p>A complex type that contains one <code>HealthCheck</code> element for each health check that is associated with the current Amazon Web Services account.</p>
    #[doc(hidden)]
    pub health_checks: std::option::Option<std::vec::Vec<crate::types::HealthCheck>>,
    /// <p>For the second and subsequent calls to <code>ListHealthChecks</code>, <code>Marker</code> is the value that you specified for the <code>marker</code> parameter in the previous request.</p>
    #[doc(hidden)]
    pub marker: std::option::Option<std::string::String>,
    /// <p>A flag that indicates whether there are more health checks to be listed. If the response was truncated, you can get the next group of health checks by submitting another <code>ListHealthChecks</code> request and specifying the value of <code>NextMarker</code> in the <code>marker</code> parameter.</p>
    #[doc(hidden)]
    pub is_truncated: bool,
    /// <p>If <code>IsTruncated</code> is <code>true</code>, the value of <code>NextMarker</code> identifies the first health check that Amazon Route 53 returns if you submit another <code>ListHealthChecks</code> request and specify the value of <code>NextMarker</code> in the <code>marker</code> parameter.</p>
    #[doc(hidden)]
    pub next_marker: std::option::Option<std::string::String>,
    /// <p>The value that you specified for the <code>maxitems</code> parameter in the call to <code>ListHealthChecks</code> that produced the current response.</p>
    #[doc(hidden)]
    pub max_items: std::option::Option<i32>,
    _request_id: Option<String>,
}
impl ListHealthChecksOutput {
    /// <p>A complex type that contains one <code>HealthCheck</code> element for each health check that is associated with the current Amazon Web Services account.</p>
    pub fn health_checks(&self) -> std::option::Option<&[crate::types::HealthCheck]> {
        self.health_checks.as_deref()
    }
    /// <p>For the second and subsequent calls to <code>ListHealthChecks</code>, <code>Marker</code> is the value that you specified for the <code>marker</code> parameter in the previous request.</p>
    pub fn marker(&self) -> std::option::Option<&str> {
        self.marker.as_deref()
    }
    /// <p>A flag that indicates whether there are more health checks to be listed. If the response was truncated, you can get the next group of health checks by submitting another <code>ListHealthChecks</code> request and specifying the value of <code>NextMarker</code> in the <code>marker</code> parameter.</p>
    pub fn is_truncated(&self) -> bool {
        self.is_truncated
    }
    /// <p>If <code>IsTruncated</code> is <code>true</code>, the value of <code>NextMarker</code> identifies the first health check that Amazon Route 53 returns if you submit another <code>ListHealthChecks</code> request and specify the value of <code>NextMarker</code> in the <code>marker</code> parameter.</p>
    pub fn next_marker(&self) -> std::option::Option<&str> {
        self.next_marker.as_deref()
    }
    /// <p>The value that you specified for the <code>maxitems</code> parameter in the call to <code>ListHealthChecks</code> that produced the current response.</p>
    pub fn max_items(&self) -> std::option::Option<i32> {
        self.max_items
    }
}
impl aws_http::request_id::RequestId for ListHealthChecksOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListHealthChecksOutput {
    /// Creates a new builder-style object to manufacture [`ListHealthChecksOutput`](crate::operation::list_health_checks::ListHealthChecksOutput).
    pub fn builder() -> crate::operation::list_health_checks::builders::ListHealthChecksOutputBuilder
    {
        crate::operation::list_health_checks::builders::ListHealthChecksOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::list_health_checks::ListHealthChecksOutput;
/// A builder for [`ListHealthChecksOutput`](crate::operation::list_health_checks::ListHealthChecksOutput).
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
pub struct ListHealthChecksOutputBuilder {
    pub(crate) health_checks: std::option::Option<std::vec::Vec<crate::types::HealthCheck>>,
    pub(crate) marker: std::option::Option<std::string::String>,
    pub(crate) is_truncated: std::option::Option<bool>,
    pub(crate) next_marker: std::option::Option<std::string::String>,
    pub(crate) max_items: std::option::Option<i32>,
    _request_id: Option<String>,
}
impl ListHealthChecksOutputBuilder {
    /// Appends an item to `health_checks`.
    ///
    /// To override the contents of this collection use [`set_health_checks`](Self::set_health_checks).
    ///
    /// <p>A complex type that contains one <code>HealthCheck</code> element for each health check that is associated with the current Amazon Web Services account.</p>
    pub fn health_checks(mut self, input: crate::types::HealthCheck) -> Self {
        let mut v = self.health_checks.unwrap_or_default();
        v.push(input);
        self.health_checks = Some(v);
        self
    }
    /// <p>A complex type that contains one <code>HealthCheck</code> element for each health check that is associated with the current Amazon Web Services account.</p>
    pub fn set_health_checks(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::HealthCheck>>,
    ) -> Self {
        self.health_checks = input;
        self
    }
    /// <p>For the second and subsequent calls to <code>ListHealthChecks</code>, <code>Marker</code> is the value that you specified for the <code>marker</code> parameter in the previous request.</p>
    pub fn marker(mut self, input: impl Into<std::string::String>) -> Self {
        self.marker = Some(input.into());
        self
    }
    /// <p>For the second and subsequent calls to <code>ListHealthChecks</code>, <code>Marker</code> is the value that you specified for the <code>marker</code> parameter in the previous request.</p>
    pub fn set_marker(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.marker = input;
        self
    }
    /// <p>A flag that indicates whether there are more health checks to be listed. If the response was truncated, you can get the next group of health checks by submitting another <code>ListHealthChecks</code> request and specifying the value of <code>NextMarker</code> in the <code>marker</code> parameter.</p>
    pub fn is_truncated(mut self, input: bool) -> Self {
        self.is_truncated = Some(input);
        self
    }
    /// <p>A flag that indicates whether there are more health checks to be listed. If the response was truncated, you can get the next group of health checks by submitting another <code>ListHealthChecks</code> request and specifying the value of <code>NextMarker</code> in the <code>marker</code> parameter.</p>
    pub fn set_is_truncated(mut self, input: std::option::Option<bool>) -> Self {
        self.is_truncated = input;
        self
    }
    /// <p>If <code>IsTruncated</code> is <code>true</code>, the value of <code>NextMarker</code> identifies the first health check that Amazon Route 53 returns if you submit another <code>ListHealthChecks</code> request and specify the value of <code>NextMarker</code> in the <code>marker</code> parameter.</p>
    pub fn next_marker(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_marker = Some(input.into());
        self
    }
    /// <p>If <code>IsTruncated</code> is <code>true</code>, the value of <code>NextMarker</code> identifies the first health check that Amazon Route 53 returns if you submit another <code>ListHealthChecks</code> request and specify the value of <code>NextMarker</code> in the <code>marker</code> parameter.</p>
    pub fn set_next_marker(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_marker = input;
        self
    }
    /// <p>The value that you specified for the <code>maxitems</code> parameter in the call to <code>ListHealthChecks</code> that produced the current response.</p>
    pub fn max_items(mut self, input: i32) -> Self {
        self.max_items = Some(input);
        self
    }
    /// <p>The value that you specified for the <code>maxitems</code> parameter in the call to <code>ListHealthChecks</code> that produced the current response.</p>
    pub fn set_max_items(mut self, input: std::option::Option<i32>) -> Self {
        self.max_items = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ListHealthChecksOutput`](crate::operation::list_health_checks::ListHealthChecksOutput).
    pub fn build(self) -> crate::operation::list_health_checks::ListHealthChecksOutput {
        crate::operation::list_health_checks::ListHealthChecksOutput {
            health_checks: self.health_checks,
            marker: self.marker,
            is_truncated: self.is_truncated.unwrap_or_default(),
            next_marker: self.next_marker,
            max_items: self.max_items,
            _request_id: self._request_id,
        }
    }
}