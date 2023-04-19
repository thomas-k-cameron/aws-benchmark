// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information about an attached policy.</p>
/// <p>An attached policy is a managed policy that has been attached to a user, group, or role. This data type is used as a response element in the <code>ListAttachedGroupPolicies</code>, <code>ListAttachedRolePolicies</code>, <code>ListAttachedUserPolicies</code>, and <code>GetAccountAuthorizationDetails</code> operations. </p>
/// <p>For more information about managed policies, refer to <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/policies-managed-vs-inline.html">Managed policies and inline policies</a> in the <i>IAM User Guide</i>. </p>
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
pub struct AttachedPolicy {
    /// <p>The friendly name of the attached policy.</p>
    #[doc(hidden)]
    pub policy_name: std::option::Option<std::string::String>,
    /// <p>The Amazon Resource Name (ARN). ARNs are unique identifiers for Amazon Web Services resources.</p>
    /// <p>For more information about ARNs, go to <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>. </p>
    #[doc(hidden)]
    pub policy_arn: std::option::Option<std::string::String>,
}
impl AttachedPolicy {
    /// <p>The friendly name of the attached policy.</p>
    pub fn policy_name(&self) -> std::option::Option<&str> {
        self.policy_name.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN). ARNs are unique identifiers for Amazon Web Services resources.</p>
    /// <p>For more information about ARNs, go to <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>. </p>
    pub fn policy_arn(&self) -> std::option::Option<&str> {
        self.policy_arn.as_deref()
    }
}
impl AttachedPolicy {
    /// Creates a new builder-style object to manufacture [`AttachedPolicy`](crate::types::AttachedPolicy).
    pub fn builder() -> crate::types::builders::AttachedPolicyBuilder {
        crate::types::builders::AttachedPolicyBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::AttachedPolicy;
/// A builder for [`AttachedPolicy`](crate::types::AttachedPolicy).
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
pub struct AttachedPolicyBuilder {
    pub(crate) policy_name: std::option::Option<std::string::String>,
    pub(crate) policy_arn: std::option::Option<std::string::String>,
}
impl AttachedPolicyBuilder {
    /// <p>The friendly name of the attached policy.</p>
    pub fn policy_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.policy_name = Some(input.into());
        self
    }
    /// <p>The friendly name of the attached policy.</p>
    pub fn set_policy_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.policy_name = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN). ARNs are unique identifiers for Amazon Web Services resources.</p>
    /// <p>For more information about ARNs, go to <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>. </p>
    pub fn policy_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.policy_arn = Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN). ARNs are unique identifiers for Amazon Web Services resources.</p>
    /// <p>For more information about ARNs, go to <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>. </p>
    pub fn set_policy_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.policy_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`AttachedPolicy`](crate::types::AttachedPolicy).
    pub fn build(self) -> crate::types::AttachedPolicy {
        crate::types::AttachedPolicy {
            policy_name: self.policy_name,
            policy_arn: self.policy_arn,
        }
    }
}