// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Indicates whether the instance is enabled for Amazon Web Services Nitro Enclaves. For more information, see <a href="https://docs.aws.amazon.com/enclaves/latest/user/nitro-enclave.html">What is Amazon Web Services Nitro Enclaves?</a> in the <i>Amazon Web Services Nitro Enclaves User Guide</i>.</p>
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
pub struct LaunchTemplateEnclaveOptionsRequest {
    /// <p>To enable the instance for Amazon Web Services Nitro Enclaves, set this parameter to <code>true</code>.</p>
    #[doc(hidden)]
    pub enabled: std::option::Option<bool>,
}
impl LaunchTemplateEnclaveOptionsRequest {
    /// <p>To enable the instance for Amazon Web Services Nitro Enclaves, set this parameter to <code>true</code>.</p>
    pub fn enabled(&self) -> std::option::Option<bool> {
        self.enabled
    }
}
impl LaunchTemplateEnclaveOptionsRequest {
    /// Creates a new builder-style object to manufacture [`LaunchTemplateEnclaveOptionsRequest`](crate::types::LaunchTemplateEnclaveOptionsRequest).
    pub fn builder() -> crate::types::builders::LaunchTemplateEnclaveOptionsRequestBuilder {
        crate::types::builders::LaunchTemplateEnclaveOptionsRequestBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::LaunchTemplateEnclaveOptionsRequest;
/// A builder for [`LaunchTemplateEnclaveOptionsRequest`](crate::types::LaunchTemplateEnclaveOptionsRequest).
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
pub struct LaunchTemplateEnclaveOptionsRequestBuilder {
    pub(crate) enabled: std::option::Option<bool>,
}
impl LaunchTemplateEnclaveOptionsRequestBuilder {
    /// <p>To enable the instance for Amazon Web Services Nitro Enclaves, set this parameter to <code>true</code>.</p>
    pub fn enabled(mut self, input: bool) -> Self {
        self.enabled = Some(input);
        self
    }
    /// <p>To enable the instance for Amazon Web Services Nitro Enclaves, set this parameter to <code>true</code>.</p>
    pub fn set_enabled(mut self, input: std::option::Option<bool>) -> Self {
        self.enabled = input;
        self
    }
    /// Consumes the builder and constructs a [`LaunchTemplateEnclaveOptionsRequest`](crate::types::LaunchTemplateEnclaveOptionsRequest).
    pub fn build(self) -> crate::types::LaunchTemplateEnclaveOptionsRequest {
        crate::types::LaunchTemplateEnclaveOptionsRequest {
            enabled: self.enabled,
        }
    }
}