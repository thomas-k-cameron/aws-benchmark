// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutBucketAnalyticsConfiguration`](crate::operation::put_bucket_analytics_configuration::builders::PutBucketAnalyticsConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::operation::put_bucket_analytics_configuration::builders::PutBucketAnalyticsConfigurationFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::put_bucket_analytics_configuration::builders::PutBucketAnalyticsConfigurationFluentBuilder::set_bucket): <p>The name of the bucket to which an analytics configuration is stored.</p>
    ///   - [`id(impl Into<String>)`](crate::operation::put_bucket_analytics_configuration::builders::PutBucketAnalyticsConfigurationFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::put_bucket_analytics_configuration::builders::PutBucketAnalyticsConfigurationFluentBuilder::set_id): <p>The ID that identifies the analytics configuration.</p>
    ///   - [`analytics_configuration(AnalyticsConfiguration)`](crate::operation::put_bucket_analytics_configuration::builders::PutBucketAnalyticsConfigurationFluentBuilder::analytics_configuration) / [`set_analytics_configuration(Option<AnalyticsConfiguration>)`](crate::operation::put_bucket_analytics_configuration::builders::PutBucketAnalyticsConfigurationFluentBuilder::set_analytics_configuration): <p>The configuration and any analyses for the analytics filter.</p>
    ///   - [`expected_bucket_owner(impl Into<String>)`](crate::operation::put_bucket_analytics_configuration::builders::PutBucketAnalyticsConfigurationFluentBuilder::expected_bucket_owner) / [`set_expected_bucket_owner(Option<String>)`](crate::operation::put_bucket_analytics_configuration::builders::PutBucketAnalyticsConfigurationFluentBuilder::set_expected_bucket_owner): <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    /// - On success, responds with [`PutBucketAnalyticsConfigurationOutput`](crate::operation::put_bucket_analytics_configuration::PutBucketAnalyticsConfigurationOutput)
    /// - On failure, responds with [`SdkError<PutBucketAnalyticsConfigurationError>`](crate::operation::put_bucket_analytics_configuration::PutBucketAnalyticsConfigurationError)
    pub fn put_bucket_analytics_configuration(&self) -> crate::operation::put_bucket_analytics_configuration::builders::PutBucketAnalyticsConfigurationFluentBuilder{
        crate::operation::put_bucket_analytics_configuration::builders::PutBucketAnalyticsConfigurationFluentBuilder::new(self.handle.clone())
    }
}