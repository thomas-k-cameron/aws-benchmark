// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The last established access control policy for a Multi-Region Access Point.</p>
/// <p>When you update the policy, the update is first listed as the proposed policy. After the update is finished and all Regions have been updated, the proposed policy is listed as the established policy. If both policies have the same version number, the proposed policy is the established policy.</p>
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
pub struct EstablishedMultiRegionAccessPointPolicy {
    /// <p>The details of the last established policy.</p>
    #[doc(hidden)]
    pub policy: std::option::Option<std::string::String>,
}
impl EstablishedMultiRegionAccessPointPolicy {
    /// <p>The details of the last established policy.</p>
    pub fn policy(&self) -> std::option::Option<&str> {
        self.policy.as_deref()
    }
}
impl EstablishedMultiRegionAccessPointPolicy {
    /// Creates a new builder-style object to manufacture [`EstablishedMultiRegionAccessPointPolicy`](crate::types::EstablishedMultiRegionAccessPointPolicy).
    pub fn builder() -> crate::types::builders::EstablishedMultiRegionAccessPointPolicyBuilder {
        crate::types::builders::EstablishedMultiRegionAccessPointPolicyBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::EstablishedMultiRegionAccessPointPolicy;
/// A builder for [`EstablishedMultiRegionAccessPointPolicy`](crate::types::EstablishedMultiRegionAccessPointPolicy).
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
pub struct EstablishedMultiRegionAccessPointPolicyBuilder {
    pub(crate) policy: std::option::Option<std::string::String>,
}
impl EstablishedMultiRegionAccessPointPolicyBuilder {
    /// <p>The details of the last established policy.</p>
    pub fn policy(mut self, input: impl Into<std::string::String>) -> Self {
        self.policy = Some(input.into());
        self
    }
    /// <p>The details of the last established policy.</p>
    pub fn set_policy(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.policy = input;
        self
    }
    /// Consumes the builder and constructs a [`EstablishedMultiRegionAccessPointPolicy`](crate::types::EstablishedMultiRegionAccessPointPolicy).
    pub fn build(self) -> crate::types::EstablishedMultiRegionAccessPointPolicy {
        crate::types::EstablishedMultiRegionAccessPointPolicy {
            policy: self.policy,
        }
    }
}
