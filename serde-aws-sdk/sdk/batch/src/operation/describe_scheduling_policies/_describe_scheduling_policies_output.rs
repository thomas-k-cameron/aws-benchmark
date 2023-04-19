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
pub struct DescribeSchedulingPoliciesOutput {
    /// <p>The list of scheduling policies.</p>
    #[doc(hidden)]
    pub scheduling_policies:
        std::option::Option<std::vec::Vec<crate::types::SchedulingPolicyDetail>>,
    _request_id: Option<String>,
}
impl DescribeSchedulingPoliciesOutput {
    /// <p>The list of scheduling policies.</p>
    pub fn scheduling_policies(
        &self,
    ) -> std::option::Option<&[crate::types::SchedulingPolicyDetail]> {
        self.scheduling_policies.as_deref()
    }
}
impl aws_http::request_id::RequestId for DescribeSchedulingPoliciesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeSchedulingPoliciesOutput {
    /// Creates a new builder-style object to manufacture [`DescribeSchedulingPoliciesOutput`](crate::operation::describe_scheduling_policies::DescribeSchedulingPoliciesOutput).
    pub fn builder() -> crate::operation::describe_scheduling_policies::builders::DescribeSchedulingPoliciesOutputBuilder{
        crate::operation::describe_scheduling_policies::builders::DescribeSchedulingPoliciesOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::describe_scheduling_policies::DescribeSchedulingPoliciesOutput;
/// A builder for [`DescribeSchedulingPoliciesOutput`](crate::operation::describe_scheduling_policies::DescribeSchedulingPoliciesOutput).
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
pub struct DescribeSchedulingPoliciesOutputBuilder {
    pub(crate) scheduling_policies:
        std::option::Option<std::vec::Vec<crate::types::SchedulingPolicyDetail>>,
    _request_id: Option<String>,
}
impl DescribeSchedulingPoliciesOutputBuilder {
    /// Appends an item to `scheduling_policies`.
    ///
    /// To override the contents of this collection use [`set_scheduling_policies`](Self::set_scheduling_policies).
    ///
    /// <p>The list of scheduling policies.</p>
    pub fn scheduling_policies(mut self, input: crate::types::SchedulingPolicyDetail) -> Self {
        let mut v = self.scheduling_policies.unwrap_or_default();
        v.push(input);
        self.scheduling_policies = Some(v);
        self
    }
    /// <p>The list of scheduling policies.</p>
    pub fn set_scheduling_policies(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::SchedulingPolicyDetail>>,
    ) -> Self {
        self.scheduling_policies = input;
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
    /// Consumes the builder and constructs a [`DescribeSchedulingPoliciesOutput`](crate::operation::describe_scheduling_policies::DescribeSchedulingPoliciesOutput).
    pub fn build(
        self,
    ) -> crate::operation::describe_scheduling_policies::DescribeSchedulingPoliciesOutput {
        crate::operation::describe_scheduling_policies::DescribeSchedulingPoliciesOutput {
            scheduling_policies: self.scheduling_policies,
            _request_id: self._request_id,
        }
    }
}