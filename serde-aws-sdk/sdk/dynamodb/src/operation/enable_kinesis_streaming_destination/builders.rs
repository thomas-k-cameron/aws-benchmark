// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::enable_kinesis_streaming_destination::_enable_kinesis_streaming_destination_output::EnableKinesisStreamingDestinationOutputBuilder;

pub use crate::operation::enable_kinesis_streaming_destination::_enable_kinesis_streaming_destination_input::EnableKinesisStreamingDestinationInputBuilder;

/// Fluent builder constructing a request to `EnableKinesisStreamingDestination`.
///
/// <p>Starts table data replication to the specified Kinesis data stream at a timestamp chosen during the enable workflow. If this operation doesn't return results immediately, use DescribeKinesisStreamingDestination to check if streaming to the Kinesis data stream is ACTIVE.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct EnableKinesisStreamingDestinationFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationInputBuilder
            }
impl EnableKinesisStreamingDestinationFluentBuilder {
    /// Creates a new `EnableKinesisStreamingDestination`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestination, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationError>
    >{
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
                    pub async fn send(self) -> std::result::Result<crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationOutput, aws_smithy_http::result::SdkError<crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationError>>
                     {
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
    ///     let deserialized_parameters: crate::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.enable_kinesis_streaming_destination().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The name of the DynamoDB table.</p>
    pub fn table_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.table_name(input.into());
        self
    }
    /// <p>The name of the DynamoDB table.</p>
    pub fn set_table_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_table_name(input);
        self
    }
    /// <p>The ARN for a Kinesis data stream.</p>
    pub fn stream_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.stream_arn(input.into());
        self
    }
    /// <p>The ARN for a Kinesis data stream.</p>
    pub fn set_stream_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_stream_arn(input);
        self
    }
}