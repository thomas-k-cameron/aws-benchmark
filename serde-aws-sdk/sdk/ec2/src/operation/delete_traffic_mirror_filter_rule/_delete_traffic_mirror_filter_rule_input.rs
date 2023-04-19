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
pub struct DeleteTrafficMirrorFilterRuleInput {
    /// <p>The ID of the Traffic Mirror rule.</p>
    #[doc(hidden)]
    pub traffic_mirror_filter_rule_id: std::option::Option<std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
}
impl DeleteTrafficMirrorFilterRuleInput {
    /// <p>The ID of the Traffic Mirror rule.</p>
    pub fn traffic_mirror_filter_rule_id(&self) -> std::option::Option<&str> {
        self.traffic_mirror_filter_rule_id.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
}
impl DeleteTrafficMirrorFilterRuleInput {
    /// Creates a new builder-style object to manufacture [`DeleteTrafficMirrorFilterRuleInput`](crate::operation::delete_traffic_mirror_filter_rule::DeleteTrafficMirrorFilterRuleInput).
    pub fn builder() -> crate::operation::delete_traffic_mirror_filter_rule::builders::DeleteTrafficMirrorFilterRuleInputBuilder{
        crate::operation::delete_traffic_mirror_filter_rule::builders::DeleteTrafficMirrorFilterRuleInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::delete_traffic_mirror_filter_rule::DeleteTrafficMirrorFilterRuleInput;
/// A builder for [`DeleteTrafficMirrorFilterRuleInput`](crate::operation::delete_traffic_mirror_filter_rule::DeleteTrafficMirrorFilterRuleInput).
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
pub struct DeleteTrafficMirrorFilterRuleInputBuilder {
    pub(crate) traffic_mirror_filter_rule_id: std::option::Option<std::string::String>,
    pub(crate) dry_run: std::option::Option<bool>,
}
impl DeleteTrafficMirrorFilterRuleInputBuilder {
    /// <p>The ID of the Traffic Mirror rule.</p>
    pub fn traffic_mirror_filter_rule_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.traffic_mirror_filter_rule_id = Some(input.into());
        self
    }
    /// <p>The ID of the Traffic Mirror rule.</p>
    pub fn set_traffic_mirror_filter_rule_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.traffic_mirror_filter_rule_id = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteTrafficMirrorFilterRuleInput`](crate::operation::delete_traffic_mirror_filter_rule::DeleteTrafficMirrorFilterRuleInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::delete_traffic_mirror_filter_rule::DeleteTrafficMirrorFilterRuleInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::delete_traffic_mirror_filter_rule::DeleteTrafficMirrorFilterRuleInput {
                traffic_mirror_filter_rule_id: self.traffic_mirror_filter_rule_id
                ,
                dry_run: self.dry_run
                ,
            }
        )
    }
}