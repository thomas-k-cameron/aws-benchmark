// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The details of a conformance pack evaluation. Provides Config rule and Amazon Web Services resource type that was evaluated, the compliance of the conformance pack, related time stamps, and supplementary information. </p>
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
pub struct ConformancePackEvaluationResult {
    /// <p>The compliance type. The allowed values are <code>COMPLIANT</code> and <code>NON_COMPLIANT</code>. <code>INSUFFICIENT_DATA</code> is not supported.</p>
    #[doc(hidden)]
    pub compliance_type: std::option::Option<crate::types::ConformancePackComplianceType>,
    /// <p>Uniquely identifies an evaluation result.</p>
    #[doc(hidden)]
    pub evaluation_result_identifier: std::option::Option<crate::types::EvaluationResultIdentifier>,
    /// <p>The time when Config rule evaluated Amazon Web Services resource.</p>
    #[doc(hidden)]
    pub config_rule_invoked_time: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The time when Config recorded the evaluation result. </p>
    #[doc(hidden)]
    pub result_recorded_time: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>Supplementary information about how the evaluation determined the compliance. </p>
    #[doc(hidden)]
    pub annotation: std::option::Option<std::string::String>,
}
impl ConformancePackEvaluationResult {
    /// <p>The compliance type. The allowed values are <code>COMPLIANT</code> and <code>NON_COMPLIANT</code>. <code>INSUFFICIENT_DATA</code> is not supported.</p>
    pub fn compliance_type(
        &self,
    ) -> std::option::Option<&crate::types::ConformancePackComplianceType> {
        self.compliance_type.as_ref()
    }
    /// <p>Uniquely identifies an evaluation result.</p>
    pub fn evaluation_result_identifier(
        &self,
    ) -> std::option::Option<&crate::types::EvaluationResultIdentifier> {
        self.evaluation_result_identifier.as_ref()
    }
    /// <p>The time when Config rule evaluated Amazon Web Services resource.</p>
    pub fn config_rule_invoked_time(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.config_rule_invoked_time.as_ref()
    }
    /// <p>The time when Config recorded the evaluation result. </p>
    pub fn result_recorded_time(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.result_recorded_time.as_ref()
    }
    /// <p>Supplementary information about how the evaluation determined the compliance. </p>
    pub fn annotation(&self) -> std::option::Option<&str> {
        self.annotation.as_deref()
    }
}
impl ConformancePackEvaluationResult {
    /// Creates a new builder-style object to manufacture [`ConformancePackEvaluationResult`](crate::types::ConformancePackEvaluationResult).
    pub fn builder() -> crate::types::builders::ConformancePackEvaluationResultBuilder {
        crate::types::builders::ConformancePackEvaluationResultBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::ConformancePackEvaluationResult;
/// A builder for [`ConformancePackEvaluationResult`](crate::types::ConformancePackEvaluationResult).
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
pub struct ConformancePackEvaluationResultBuilder {
    pub(crate) compliance_type: std::option::Option<crate::types::ConformancePackComplianceType>,
    pub(crate) evaluation_result_identifier:
        std::option::Option<crate::types::EvaluationResultIdentifier>,
    pub(crate) config_rule_invoked_time: std::option::Option<aws_smithy_types::DateTime>,
    pub(crate) result_recorded_time: std::option::Option<aws_smithy_types::DateTime>,
    pub(crate) annotation: std::option::Option<std::string::String>,
}
impl ConformancePackEvaluationResultBuilder {
    /// <p>The compliance type. The allowed values are <code>COMPLIANT</code> and <code>NON_COMPLIANT</code>. <code>INSUFFICIENT_DATA</code> is not supported.</p>
    pub fn compliance_type(mut self, input: crate::types::ConformancePackComplianceType) -> Self {
        self.compliance_type = Some(input);
        self
    }
    /// <p>The compliance type. The allowed values are <code>COMPLIANT</code> and <code>NON_COMPLIANT</code>. <code>INSUFFICIENT_DATA</code> is not supported.</p>
    pub fn set_compliance_type(
        mut self,
        input: std::option::Option<crate::types::ConformancePackComplianceType>,
    ) -> Self {
        self.compliance_type = input;
        self
    }
    /// <p>Uniquely identifies an evaluation result.</p>
    pub fn evaluation_result_identifier(
        mut self,
        input: crate::types::EvaluationResultIdentifier,
    ) -> Self {
        self.evaluation_result_identifier = Some(input);
        self
    }
    /// <p>Uniquely identifies an evaluation result.</p>
    pub fn set_evaluation_result_identifier(
        mut self,
        input: std::option::Option<crate::types::EvaluationResultIdentifier>,
    ) -> Self {
        self.evaluation_result_identifier = input;
        self
    }
    /// <p>The time when Config rule evaluated Amazon Web Services resource.</p>
    pub fn config_rule_invoked_time(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.config_rule_invoked_time = Some(input);
        self
    }
    /// <p>The time when Config rule evaluated Amazon Web Services resource.</p>
    pub fn set_config_rule_invoked_time(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.config_rule_invoked_time = input;
        self
    }
    /// <p>The time when Config recorded the evaluation result. </p>
    pub fn result_recorded_time(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.result_recorded_time = Some(input);
        self
    }
    /// <p>The time when Config recorded the evaluation result. </p>
    pub fn set_result_recorded_time(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.result_recorded_time = input;
        self
    }
    /// <p>Supplementary information about how the evaluation determined the compliance. </p>
    pub fn annotation(mut self, input: impl Into<std::string::String>) -> Self {
        self.annotation = Some(input.into());
        self
    }
    /// <p>Supplementary information about how the evaluation determined the compliance. </p>
    pub fn set_annotation(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.annotation = input;
        self
    }
    /// Consumes the builder and constructs a [`ConformancePackEvaluationResult`](crate::types::ConformancePackEvaluationResult).
    pub fn build(self) -> crate::types::ConformancePackEvaluationResult {
        crate::types::ConformancePackEvaluationResult {
            compliance_type: self.compliance_type,
            evaluation_result_identifier: self.evaluation_result_identifier,
            config_rule_invoked_time: self.config_rule_invoked_time,
            result_recorded_time: self.result_recorded_time,
            annotation: self.annotation,
        }
    }
}