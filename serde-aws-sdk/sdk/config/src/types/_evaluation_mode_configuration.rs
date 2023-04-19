// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The configuration object for Config rule evaluation mode. The Supported valid values are Detective or Proactive.</p>
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
pub struct EvaluationModeConfiguration {
    /// <p>The mode of an evaluation. The valid values are Detective or Proactive.</p>
    #[doc(hidden)]
    pub mode: std::option::Option<crate::types::EvaluationMode>,
}
impl EvaluationModeConfiguration {
    /// <p>The mode of an evaluation. The valid values are Detective or Proactive.</p>
    pub fn mode(&self) -> std::option::Option<&crate::types::EvaluationMode> {
        self.mode.as_ref()
    }
}
impl EvaluationModeConfiguration {
    /// Creates a new builder-style object to manufacture [`EvaluationModeConfiguration`](crate::types::EvaluationModeConfiguration).
    pub fn builder() -> crate::types::builders::EvaluationModeConfigurationBuilder {
        crate::types::builders::EvaluationModeConfigurationBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::EvaluationModeConfiguration;
/// A builder for [`EvaluationModeConfiguration`](crate::types::EvaluationModeConfiguration).
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
pub struct EvaluationModeConfigurationBuilder {
    pub(crate) mode: std::option::Option<crate::types::EvaluationMode>,
}
impl EvaluationModeConfigurationBuilder {
    /// <p>The mode of an evaluation. The valid values are Detective or Proactive.</p>
    pub fn mode(mut self, input: crate::types::EvaluationMode) -> Self {
        self.mode = Some(input);
        self
    }
    /// <p>The mode of an evaluation. The valid values are Detective or Proactive.</p>
    pub fn set_mode(mut self, input: std::option::Option<crate::types::EvaluationMode>) -> Self {
        self.mode = input;
        self
    }
    /// Consumes the builder and constructs a [`EvaluationModeConfiguration`](crate::types::EvaluationModeConfiguration).
    pub fn build(self) -> crate::types::EvaluationModeConfiguration {
        crate::types::EvaluationModeConfiguration { mode: self.mode }
    }
}