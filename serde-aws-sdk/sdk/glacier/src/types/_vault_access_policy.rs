// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the vault access policy.</p>
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
pub struct VaultAccessPolicy {
    /// <p>The vault access policy.</p>
    #[doc(hidden)]
    pub policy: std::option::Option<std::string::String>,
}
impl VaultAccessPolicy {
    /// <p>The vault access policy.</p>
    pub fn policy(&self) -> std::option::Option<&str> {
        self.policy.as_deref()
    }
}
impl VaultAccessPolicy {
    /// Creates a new builder-style object to manufacture [`VaultAccessPolicy`](crate::types::VaultAccessPolicy).
    pub fn builder() -> crate::types::builders::VaultAccessPolicyBuilder {
        crate::types::builders::VaultAccessPolicyBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::VaultAccessPolicy;
/// A builder for [`VaultAccessPolicy`](crate::types::VaultAccessPolicy).
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
pub struct VaultAccessPolicyBuilder {
    pub(crate) policy: std::option::Option<std::string::String>,
}
impl VaultAccessPolicyBuilder {
    /// <p>The vault access policy.</p>
    pub fn policy(mut self, input: impl Into<std::string::String>) -> Self {
        self.policy = Some(input.into());
        self
    }
    /// <p>The vault access policy.</p>
    pub fn set_policy(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.policy = input;
        self
    }
    /// Consumes the builder and constructs a [`VaultAccessPolicy`](crate::types::VaultAccessPolicy).
    pub fn build(self) -> crate::types::VaultAccessPolicy {
        crate::types::VaultAccessPolicy {
            policy: self.policy,
        }
    }
}
