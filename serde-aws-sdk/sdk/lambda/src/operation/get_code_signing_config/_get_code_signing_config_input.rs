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
pub struct GetCodeSigningConfigInput {
    /// <p>The The Amazon Resource Name (ARN) of the code signing configuration. </p>
    #[doc(hidden)]
    pub code_signing_config_arn: std::option::Option<std::string::String>,
}
impl GetCodeSigningConfigInput {
    /// <p>The The Amazon Resource Name (ARN) of the code signing configuration. </p>
    pub fn code_signing_config_arn(&self) -> std::option::Option<&str> {
        self.code_signing_config_arn.as_deref()
    }
}
impl GetCodeSigningConfigInput {
    /// Creates a new builder-style object to manufacture [`GetCodeSigningConfigInput`](crate::operation::get_code_signing_config::GetCodeSigningConfigInput).
    pub fn builder(
    ) -> crate::operation::get_code_signing_config::builders::GetCodeSigningConfigInputBuilder {
        crate::operation::get_code_signing_config::builders::GetCodeSigningConfigInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_code_signing_config::GetCodeSigningConfigInput;
/// A builder for [`GetCodeSigningConfigInput`](crate::operation::get_code_signing_config::GetCodeSigningConfigInput).
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
pub struct GetCodeSigningConfigInputBuilder {
    pub(crate) code_signing_config_arn: std::option::Option<std::string::String>,
}
impl GetCodeSigningConfigInputBuilder {
    /// <p>The The Amazon Resource Name (ARN) of the code signing configuration. </p>
    pub fn code_signing_config_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.code_signing_config_arn = Some(input.into());
        self
    }
    /// <p>The The Amazon Resource Name (ARN) of the code signing configuration. </p>
    pub fn set_code_signing_config_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.code_signing_config_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`GetCodeSigningConfigInput`](crate::operation::get_code_signing_config::GetCodeSigningConfigInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::get_code_signing_config::GetCodeSigningConfigInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::get_code_signing_config::GetCodeSigningConfigInput {
                code_signing_config_arn: self.code_signing_config_arn,
            },
        )
    }
}
