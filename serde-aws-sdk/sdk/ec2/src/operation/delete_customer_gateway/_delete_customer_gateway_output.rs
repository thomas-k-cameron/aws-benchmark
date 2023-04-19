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
pub struct DeleteCustomerGatewayOutput {
    _request_id: Option<String>,
}
impl aws_http::request_id::RequestId for DeleteCustomerGatewayOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteCustomerGatewayOutput {
    /// Creates a new builder-style object to manufacture [`DeleteCustomerGatewayOutput`](crate::operation::delete_customer_gateway::DeleteCustomerGatewayOutput).
    pub fn builder(
    ) -> crate::operation::delete_customer_gateway::builders::DeleteCustomerGatewayOutputBuilder
    {
        crate::operation::delete_customer_gateway::builders::DeleteCustomerGatewayOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::delete_customer_gateway::DeleteCustomerGatewayOutput;
/// A builder for [`DeleteCustomerGatewayOutput`](crate::operation::delete_customer_gateway::DeleteCustomerGatewayOutput).
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
pub struct DeleteCustomerGatewayOutputBuilder {
    _request_id: Option<String>,
}
impl DeleteCustomerGatewayOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DeleteCustomerGatewayOutput`](crate::operation::delete_customer_gateway::DeleteCustomerGatewayOutput).
    pub fn build(self) -> crate::operation::delete_customer_gateway::DeleteCustomerGatewayOutput {
        crate::operation::delete_customer_gateway::DeleteCustomerGatewayOutput {
            _request_id: self._request_id,
        }
    }
}