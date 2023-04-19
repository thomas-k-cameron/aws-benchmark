// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetHealthCheckLastFailureReason`](crate::operation::get_health_check_last_failure_reason::builders::GetHealthCheckLastFailureReasonFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`health_check_id(impl Into<String>)`](crate::operation::get_health_check_last_failure_reason::builders::GetHealthCheckLastFailureReasonFluentBuilder::health_check_id) / [`set_health_check_id(Option<String>)`](crate::operation::get_health_check_last_failure_reason::builders::GetHealthCheckLastFailureReasonFluentBuilder::set_health_check_id): <p>The ID for the health check for which you want the last failure reason. When you created the health check, <code>CreateHealthCheck</code> returned the ID in the response, in the <code>HealthCheckId</code> element.</p> <note>   <p>If you want to get the last failure reason for a calculated health check, you must use the Amazon Route 53 console or the CloudWatch console. You can't use <code>GetHealthCheckLastFailureReason</code> for a calculated health check.</p>  </note>
    /// - On success, responds with [`GetHealthCheckLastFailureReasonOutput`](crate::operation::get_health_check_last_failure_reason::GetHealthCheckLastFailureReasonOutput) with field(s):
    ///   - [`health_check_observations(Option<Vec<HealthCheckObservation>>)`](crate::operation::get_health_check_last_failure_reason::GetHealthCheckLastFailureReasonOutput::health_check_observations): <p>A list that contains one <code>Observation</code> element for each Amazon Route 53 health checker that is reporting a last failure reason. </p>
    /// - On failure, responds with [`SdkError<GetHealthCheckLastFailureReasonError>`](crate::operation::get_health_check_last_failure_reason::GetHealthCheckLastFailureReasonError)
    pub fn get_health_check_last_failure_reason(&self) -> crate::operation::get_health_check_last_failure_reason::builders::GetHealthCheckLastFailureReasonFluentBuilder{
        crate::operation::get_health_check_last_failure_reason::builders::GetHealthCheckLastFailureReasonFluentBuilder::new(self.handle.clone())
    }
}