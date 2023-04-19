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
pub struct DeleteIpamOutput {
    /// <p>Information about the results of the deletion.</p>
    #[doc(hidden)]
    pub ipam: std::option::Option<crate::types::Ipam>,
    _request_id: Option<String>,
}
impl DeleteIpamOutput {
    /// <p>Information about the results of the deletion.</p>
    pub fn ipam(&self) -> std::option::Option<&crate::types::Ipam> {
        self.ipam.as_ref()
    }
}
impl aws_http::request_id::RequestId for DeleteIpamOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteIpamOutput {
    /// Creates a new builder-style object to manufacture [`DeleteIpamOutput`](crate::operation::delete_ipam::DeleteIpamOutput).
    pub fn builder() -> crate::operation::delete_ipam::builders::DeleteIpamOutputBuilder {
        crate::operation::delete_ipam::builders::DeleteIpamOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::delete_ipam::DeleteIpamOutput;
/// A builder for [`DeleteIpamOutput`](crate::operation::delete_ipam::DeleteIpamOutput).
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
pub struct DeleteIpamOutputBuilder {
    pub(crate) ipam: std::option::Option<crate::types::Ipam>,
    _request_id: Option<String>,
}
impl DeleteIpamOutputBuilder {
    /// <p>Information about the results of the deletion.</p>
    pub fn ipam(mut self, input: crate::types::Ipam) -> Self {
        self.ipam = Some(input);
        self
    }
    /// <p>Information about the results of the deletion.</p>
    pub fn set_ipam(mut self, input: std::option::Option<crate::types::Ipam>) -> Self {
        self.ipam = input;
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
    /// Consumes the builder and constructs a [`DeleteIpamOutput`](crate::operation::delete_ipam::DeleteIpamOutput).
    pub fn build(self) -> crate::operation::delete_ipam::DeleteIpamOutput {
        crate::operation::delete_ipam::DeleteIpamOutput {
            ipam: self.ipam,
            _request_id: self._request_id,
        }
    }
}