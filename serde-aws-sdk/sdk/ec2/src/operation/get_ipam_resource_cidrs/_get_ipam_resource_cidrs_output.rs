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
pub struct GetIpamResourceCidrsOutput {
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    /// <p>The resource CIDRs.</p>
    #[doc(hidden)]
    pub ipam_resource_cidrs: std::option::Option<std::vec::Vec<crate::types::IpamResourceCidr>>,
    _request_id: Option<String>,
}
impl GetIpamResourceCidrsOutput {
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The resource CIDRs.</p>
    pub fn ipam_resource_cidrs(&self) -> std::option::Option<&[crate::types::IpamResourceCidr]> {
        self.ipam_resource_cidrs.as_deref()
    }
}
impl aws_http::request_id::RequestId for GetIpamResourceCidrsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetIpamResourceCidrsOutput {
    /// Creates a new builder-style object to manufacture [`GetIpamResourceCidrsOutput`](crate::operation::get_ipam_resource_cidrs::GetIpamResourceCidrsOutput).
    pub fn builder(
    ) -> crate::operation::get_ipam_resource_cidrs::builders::GetIpamResourceCidrsOutputBuilder
    {
        crate::operation::get_ipam_resource_cidrs::builders::GetIpamResourceCidrsOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_ipam_resource_cidrs::GetIpamResourceCidrsOutput;
/// A builder for [`GetIpamResourceCidrsOutput`](crate::operation::get_ipam_resource_cidrs::GetIpamResourceCidrsOutput).
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
pub struct GetIpamResourceCidrsOutputBuilder {
    pub(crate) next_token: std::option::Option<std::string::String>,
    pub(crate) ipam_resource_cidrs:
        std::option::Option<std::vec::Vec<crate::types::IpamResourceCidr>>,
    _request_id: Option<String>,
}
impl GetIpamResourceCidrsOutputBuilder {
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Appends an item to `ipam_resource_cidrs`.
    ///
    /// To override the contents of this collection use [`set_ipam_resource_cidrs`](Self::set_ipam_resource_cidrs).
    ///
    /// <p>The resource CIDRs.</p>
    pub fn ipam_resource_cidrs(mut self, input: crate::types::IpamResourceCidr) -> Self {
        let mut v = self.ipam_resource_cidrs.unwrap_or_default();
        v.push(input);
        self.ipam_resource_cidrs = Some(v);
        self
    }
    /// <p>The resource CIDRs.</p>
    pub fn set_ipam_resource_cidrs(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::IpamResourceCidr>>,
    ) -> Self {
        self.ipam_resource_cidrs = input;
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
    /// Consumes the builder and constructs a [`GetIpamResourceCidrsOutput`](crate::operation::get_ipam_resource_cidrs::GetIpamResourceCidrsOutput).
    pub fn build(self) -> crate::operation::get_ipam_resource_cidrs::GetIpamResourceCidrsOutput {
        crate::operation::get_ipam_resource_cidrs::GetIpamResourceCidrsOutput {
            next_token: self.next_token,
            ipam_resource_cidrs: self.ipam_resource_cidrs,
            _request_id: self._request_id,
        }
    }
}