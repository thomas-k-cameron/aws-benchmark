// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_query_logging_config::_create_query_logging_config_output::CreateQueryLoggingConfigOutputBuilder;

pub use crate::operation::create_query_logging_config::_create_query_logging_config_input::CreateQueryLoggingConfigInputBuilder;

/// Fluent builder constructing a request to `CreateQueryLoggingConfig`.
///
/// <p>Creates a configuration for DNS query logging. After you create a query logging configuration, Amazon Route 53 begins to publish log data to an Amazon CloudWatch Logs log group.</p>
/// <p>DNS query logs contain information about the queries that Route 53 receives for a specified public hosted zone, such as the following:</p>
/// <ul>
/// <li> <p>Route 53 edge location that responded to the DNS query</p> </li>
/// <li> <p>Domain or subdomain that was requested</p> </li>
/// <li> <p>DNS record type, such as A or AAAA</p> </li>
/// <li> <p>DNS response code, such as <code>NoError</code> or <code>ServFail</code> </p> </li>
/// </ul>
/// <dl>
/// <dt>
/// Log Group and Resource Policy
/// </dt>
/// <dd>
/// <p>Before you create a query logging configuration, perform the following operations.</p> <note>
/// <p>If you create a query logging configuration using the Route 53 console, Route 53 performs these operations automatically.</p>
/// </note>
/// <ol>
/// <li> <p>Create a CloudWatch Logs log group, and make note of the ARN, which you specify when you create a query logging configuration. Note the following:</p>
/// <ul>
/// <li> <p>You must create the log group in the us-east-1 region.</p> </li>
/// <li> <p>You must use the same Amazon Web Services account to create the log group and the hosted zone that you want to configure query logging for.</p> </li>
/// <li> <p>When you create log groups for query logging, we recommend that you use a consistent prefix, for example:</p> <p> <code>/aws/route53/<i>hosted zone name</i> </code> </p> <p>In the next step, you'll create a resource policy, which controls access to one or more log groups and the associated Amazon Web Services resources, such as Route 53 hosted zones. There's a limit on the number of resource policies that you can create, so we recommend that you use a consistent prefix so you can use the same resource policy for all the log groups that you create for query logging.</p> </li>
/// </ul> </li>
/// <li> <p>Create a CloudWatch Logs resource policy, and give it the permissions that Route 53 needs to create log streams and to send query logs to log streams. For the value of <code>Resource</code>, specify the ARN for the log group that you created in the previous step. To use the same resource policy for all the CloudWatch Logs log groups that you created for query logging configurations, replace the hosted zone name with <code>*</code>, for example:</p> <p> <code>arn:aws:logs:us-east-1:123412341234:log-group:/aws/route53/*</code> </p> <p>To avoid the confused deputy problem, a security issue where an entity without a permission for an action can coerce a more-privileged entity to perform it, you can optionally limit the permissions that a service has to a resource in a resource-based policy by supplying the following values:</p>
/// <ul>
/// <li> <p>For <code>aws:SourceArn</code>, supply the hosted zone ARN used in creating the query logging configuration. For example, <code>aws:SourceArn: arn:aws:route53:::hostedzone/hosted zone ID</code>.</p> </li>
/// <li> <p>For <code>aws:SourceAccount</code>, supply the account ID for the account that creates the query logging configuration. For example, <code>aws:SourceAccount:111111111111</code>.</p> </li>
/// </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/confused-deputy.html">The confused deputy problem</a> in the <i>Amazon Web Services IAM User Guide</i>.</p> <note>
/// <p>You can't use the CloudWatch console to create or edit a resource policy. You must use the CloudWatch API, one of the Amazon Web Services SDKs, or the CLI.</p>
/// </note> </li>
/// </ol>
/// </dd>
/// <dt>
/// Log Streams and Edge Locations
/// </dt>
/// <dd>
/// <p>When Route 53 finishes creating the configuration for DNS query logging, it does the following:</p>
/// <ul>
/// <li> <p>Creates a log stream for an edge location the first time that the edge location responds to DNS queries for the specified hosted zone. That log stream is used to log all queries that Route 53 responds to for that edge location.</p> </li>
/// <li> <p>Begins to send query logs to the applicable log stream.</p> </li>
/// </ul>
/// <p>The name of each log stream is in the following format:</p>
/// <p> <code> <i>hosted zone ID</i>/<i>edge location code</i> </code> </p>
/// <p>The edge location code is a three-letter code and an arbitrarily assigned number, for example, DFW3. The three-letter code typically corresponds with the International Air Transport Association airport code for an airport near the edge location. (These abbreviations might change in the future.) For a list of edge locations, see "The Route 53 Global Network" on the <a href="http://aws.amazon.com/route53/details/">Route 53 Product Details</a> page.</p>
/// </dd>
/// <dt>
/// Queries That Are Logged
/// </dt>
/// <dd>
/// <p>Query logs contain only the queries that DNS resolvers forward to Route 53. If a DNS resolver has already cached the response to a query (such as the IP address for a load balancer for example.com), the resolver will continue to return the cached response. It doesn't forward another query to Route 53 until the TTL for the corresponding resource record set expires. Depending on how many DNS queries are submitted for a resource record set, and depending on the TTL for that resource record set, query logs might contain information about only one query out of every several thousand queries that are submitted to DNS. For more information about how DNS works, see <a href="https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/welcome-dns-service.html">Routing Internet Traffic to Your Website or Web Application</a> in the <i>Amazon Route 53 Developer Guide</i>.</p>
/// </dd>
/// <dt>
/// Log File Format
/// </dt>
/// <dd>
/// <p>For a list of the values in each query log and the format of each value, see <a href="https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/query-logs.html">Logging DNS Queries</a> in the <i>Amazon Route 53 Developer Guide</i>.</p>
/// </dd>
/// <dt>
/// Pricing
/// </dt>
/// <dd>
/// <p>For information about charges for query logs, see <a href="http://aws.amazon.com/cloudwatch/pricing/">Amazon CloudWatch Pricing</a>.</p>
/// </dd>
/// <dt>
/// How to Stop Logging
/// </dt>
/// <dd>
/// <p>If you want Route 53 to stop sending query logs to CloudWatch Logs, delete the query logging configuration. For more information, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_DeleteQueryLoggingConfig.html">DeleteQueryLoggingConfig</a>.</p>
/// </dd>
/// </dl>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateQueryLoggingConfigFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::create_query_logging_config::builders::CreateQueryLoggingConfigInputBuilder
            }
impl CreateQueryLoggingConfigFluentBuilder {
    /// Creates a new `CreateQueryLoggingConfig`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::create_query_logging_config::CreateQueryLoggingConfig,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::create_query_logging_config::CreateQueryLoggingConfigError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        Ok(crate::client::customize::CustomizableOperation { handle, operation })
    }

    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> std::result::Result<
        crate::operation::create_query_logging_config::CreateQueryLoggingConfigOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::create_query_logging_config::CreateQueryLoggingConfigError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    #[cfg(aws_sdk_unstable)]
    /// This function replaces the parameter with new one.
    /// It is useful when you want to replace the existing data with de-serialized data.
    /// ```compile_fail
    /// let result_future = async {
    ///     let deserialized_parameters: crate::operation::create_query_logging_config::builders::CreateQueryLoggingConfigInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.create_query_logging_config().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::create_query_logging_config::builders::CreateQueryLoggingConfigInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The ID of the hosted zone that you want to log queries for. You can log queries only for public hosted zones.</p>
    pub fn hosted_zone_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.hosted_zone_id(input.into());
        self
    }
    /// <p>The ID of the hosted zone that you want to log queries for. You can log queries only for public hosted zones.</p>
    pub fn set_hosted_zone_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_hosted_zone_id(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) for the log group that you want to Amazon Route 53 to send query logs to. This is the format of the ARN:</p>
    /// <p>arn:aws:logs:<i>region</i>:<i>account-id</i>:log-group:<i>log_group_name</i> </p>
    /// <p>To get the ARN for a log group, you can use the CloudWatch console, the <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_DescribeLogGroups.html">DescribeLogGroups</a> API action, the <a href="https://docs.aws.amazon.com/cli/latest/reference/logs/describe-log-groups.html">describe-log-groups</a> command, or the applicable command in one of the Amazon Web Services SDKs.</p>
    pub fn cloud_watch_logs_log_group_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.cloud_watch_logs_log_group_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) for the log group that you want to Amazon Route 53 to send query logs to. This is the format of the ARN:</p>
    /// <p>arn:aws:logs:<i>region</i>:<i>account-id</i>:log-group:<i>log_group_name</i> </p>
    /// <p>To get the ARN for a log group, you can use the CloudWatch console, the <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_DescribeLogGroups.html">DescribeLogGroups</a> API action, the <a href="https://docs.aws.amazon.com/cli/latest/reference/logs/describe-log-groups.html">describe-log-groups</a> command, or the applicable command in one of the Amazon Web Services SDKs.</p>
    pub fn set_cloud_watch_logs_log_group_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_cloud_watch_logs_log_group_arn(input);
        self
    }
}