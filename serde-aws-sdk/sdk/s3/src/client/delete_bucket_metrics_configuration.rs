// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteBucketMetricsConfiguration`](crate::operation::delete_bucket_metrics_configuration::builders::DeleteBucketMetricsConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::operation::delete_bucket_metrics_configuration::builders::DeleteBucketMetricsConfigurationFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::delete_bucket_metrics_configuration::builders::DeleteBucketMetricsConfigurationFluentBuilder::set_bucket): <p>The name of the bucket containing the metrics configuration to delete.</p>
    ///   - [`id(impl Into<String>)`](crate::operation::delete_bucket_metrics_configuration::builders::DeleteBucketMetricsConfigurationFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::delete_bucket_metrics_configuration::builders::DeleteBucketMetricsConfigurationFluentBuilder::set_id): <p>The ID used to identify the metrics configuration.</p>
    ///   - [`expected_bucket_owner(impl Into<String>)`](crate::operation::delete_bucket_metrics_configuration::builders::DeleteBucketMetricsConfigurationFluentBuilder::expected_bucket_owner) / [`set_expected_bucket_owner(Option<String>)`](crate::operation::delete_bucket_metrics_configuration::builders::DeleteBucketMetricsConfigurationFluentBuilder::set_expected_bucket_owner): <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    /// - On success, responds with [`DeleteBucketMetricsConfigurationOutput`](crate::operation::delete_bucket_metrics_configuration::DeleteBucketMetricsConfigurationOutput)
    /// - On failure, responds with [`SdkError<DeleteBucketMetricsConfigurationError>`](crate::operation::delete_bucket_metrics_configuration::DeleteBucketMetricsConfigurationError)
    pub fn delete_bucket_metrics_configuration(&self) -> crate::operation::delete_bucket_metrics_configuration::builders::DeleteBucketMetricsConfigurationFluentBuilder{
        crate::operation::delete_bucket_metrics_configuration::builders::DeleteBucketMetricsConfigurationFluentBuilder::new(self.handle.clone())
    }
}