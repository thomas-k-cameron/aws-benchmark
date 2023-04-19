// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The retry strategy that's associated with a job. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/job_retries.html">Automated job retries</a> in the <i>Batch User Guide</i>.</p>
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
pub struct RetryStrategy {
    /// <p>The number of times to move a job to the <code>RUNNABLE</code> status. You can specify between 1 and 10 attempts. If the value of <code>attempts</code> is greater than one, the job is retried on failure the same number of attempts as the value.</p>
    #[doc(hidden)]
    pub attempts: std::option::Option<i32>,
    /// <p>Array of up to 5 objects that specify the conditions where jobs are retried or failed. If this parameter is specified, then the <code>attempts</code> parameter must also be specified. If none of the listed conditions match, then the job is retried.</p>
    #[doc(hidden)]
    pub evaluate_on_exit: std::option::Option<std::vec::Vec<crate::types::EvaluateOnExit>>,
}
impl RetryStrategy {
    /// <p>The number of times to move a job to the <code>RUNNABLE</code> status. You can specify between 1 and 10 attempts. If the value of <code>attempts</code> is greater than one, the job is retried on failure the same number of attempts as the value.</p>
    pub fn attempts(&self) -> std::option::Option<i32> {
        self.attempts
    }
    /// <p>Array of up to 5 objects that specify the conditions where jobs are retried or failed. If this parameter is specified, then the <code>attempts</code> parameter must also be specified. If none of the listed conditions match, then the job is retried.</p>
    pub fn evaluate_on_exit(&self) -> std::option::Option<&[crate::types::EvaluateOnExit]> {
        self.evaluate_on_exit.as_deref()
    }
}
impl RetryStrategy {
    /// Creates a new builder-style object to manufacture [`RetryStrategy`](crate::types::RetryStrategy).
    pub fn builder() -> crate::types::builders::RetryStrategyBuilder {
        crate::types::builders::RetryStrategyBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::RetryStrategy;
/// A builder for [`RetryStrategy`](crate::types::RetryStrategy).
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
pub struct RetryStrategyBuilder {
    pub(crate) attempts: std::option::Option<i32>,
    pub(crate) evaluate_on_exit: std::option::Option<std::vec::Vec<crate::types::EvaluateOnExit>>,
}
impl RetryStrategyBuilder {
    /// <p>The number of times to move a job to the <code>RUNNABLE</code> status. You can specify between 1 and 10 attempts. If the value of <code>attempts</code> is greater than one, the job is retried on failure the same number of attempts as the value.</p>
    pub fn attempts(mut self, input: i32) -> Self {
        self.attempts = Some(input);
        self
    }
    /// <p>The number of times to move a job to the <code>RUNNABLE</code> status. You can specify between 1 and 10 attempts. If the value of <code>attempts</code> is greater than one, the job is retried on failure the same number of attempts as the value.</p>
    pub fn set_attempts(mut self, input: std::option::Option<i32>) -> Self {
        self.attempts = input;
        self
    }
    /// Appends an item to `evaluate_on_exit`.
    ///
    /// To override the contents of this collection use [`set_evaluate_on_exit`](Self::set_evaluate_on_exit).
    ///
    /// <p>Array of up to 5 objects that specify the conditions where jobs are retried or failed. If this parameter is specified, then the <code>attempts</code> parameter must also be specified. If none of the listed conditions match, then the job is retried.</p>
    pub fn evaluate_on_exit(mut self, input: crate::types::EvaluateOnExit) -> Self {
        let mut v = self.evaluate_on_exit.unwrap_or_default();
        v.push(input);
        self.evaluate_on_exit = Some(v);
        self
    }
    /// <p>Array of up to 5 objects that specify the conditions where jobs are retried or failed. If this parameter is specified, then the <code>attempts</code> parameter must also be specified. If none of the listed conditions match, then the job is retried.</p>
    pub fn set_evaluate_on_exit(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::EvaluateOnExit>>,
    ) -> Self {
        self.evaluate_on_exit = input;
        self
    }
    /// Consumes the builder and constructs a [`RetryStrategy`](crate::types::RetryStrategy).
    pub fn build(self) -> crate::types::RetryStrategy {
        crate::types::RetryStrategy {
            attempts: self.attempts,
            evaluate_on_exit: self.evaluate_on_exit,
        }
    }
}