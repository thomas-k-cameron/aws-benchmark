// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the output of DescribeVpnGateways.</p>
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
pub struct DescribeVpnGatewaysOutput {
    /// <p>Information about one or more virtual private gateways.</p>
    #[doc(hidden)]
    pub vpn_gateways: std::option::Option<std::vec::Vec<crate::types::VpnGateway>>,
    _request_id: Option<String>,
}
impl DescribeVpnGatewaysOutput {
    /// <p>Information about one or more virtual private gateways.</p>
    pub fn vpn_gateways(&self) -> std::option::Option<&[crate::types::VpnGateway]> {
        self.vpn_gateways.as_deref()
    }
}
impl aws_http::request_id::RequestId for DescribeVpnGatewaysOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeVpnGatewaysOutput {
    /// Creates a new builder-style object to manufacture [`DescribeVpnGatewaysOutput`](crate::operation::describe_vpn_gateways::DescribeVpnGatewaysOutput).
    pub fn builder(
    ) -> crate::operation::describe_vpn_gateways::builders::DescribeVpnGatewaysOutputBuilder {
        crate::operation::describe_vpn_gateways::builders::DescribeVpnGatewaysOutputBuilder::default(
        )
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::describe_vpn_gateways::DescribeVpnGatewaysOutput;
/// A builder for [`DescribeVpnGatewaysOutput`](crate::operation::describe_vpn_gateways::DescribeVpnGatewaysOutput).
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
pub struct DescribeVpnGatewaysOutputBuilder {
    pub(crate) vpn_gateways: std::option::Option<std::vec::Vec<crate::types::VpnGateway>>,
    _request_id: Option<String>,
}
impl DescribeVpnGatewaysOutputBuilder {
    /// Appends an item to `vpn_gateways`.
    ///
    /// To override the contents of this collection use [`set_vpn_gateways`](Self::set_vpn_gateways).
    ///
    /// <p>Information about one or more virtual private gateways.</p>
    pub fn vpn_gateways(mut self, input: crate::types::VpnGateway) -> Self {
        let mut v = self.vpn_gateways.unwrap_or_default();
        v.push(input);
        self.vpn_gateways = Some(v);
        self
    }
    /// <p>Information about one or more virtual private gateways.</p>
    pub fn set_vpn_gateways(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::VpnGateway>>,
    ) -> Self {
        self.vpn_gateways = input;
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
    /// Consumes the builder and constructs a [`DescribeVpnGatewaysOutput`](crate::operation::describe_vpn_gateways::DescribeVpnGatewaysOutput).
    pub fn build(self) -> crate::operation::describe_vpn_gateways::DescribeVpnGatewaysOutput {
        crate::operation::describe_vpn_gateways::DescribeVpnGatewaysOutput {
            vpn_gateways: self.vpn_gateways,
            _request_id: self._request_id,
        }
    }
}
