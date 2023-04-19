// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteSchedulingPolicy`](crate::operation::delete_scheduling_policy::builders::DeleteSchedulingPolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`arn(impl Into<String>)`](crate::operation::delete_scheduling_policy::builders::DeleteSchedulingPolicyFluentBuilder::arn) / [`set_arn(Option<String>)`](crate::operation::delete_scheduling_policy::builders::DeleteSchedulingPolicyFluentBuilder::set_arn): <p>The Amazon Resource Name (ARN) of the scheduling policy to delete.</p>
    /// - On success, responds with [`DeleteSchedulingPolicyOutput`](crate::operation::delete_scheduling_policy::DeleteSchedulingPolicyOutput)
    /// - On failure, responds with [`SdkError<DeleteSchedulingPolicyError>`](crate::operation::delete_scheduling_policy::DeleteSchedulingPolicyError)
    pub fn delete_scheduling_policy(
        &self,
    ) -> crate::operation::delete_scheduling_policy::builders::DeleteSchedulingPolicyFluentBuilder
    {
        crate::operation::delete_scheduling_policy::builders::DeleteSchedulingPolicyFluentBuilder::new(self.handle.clone())
    }
}