// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_provisioned_concurrency_config::_put_provisioned_concurrency_config_output::PutProvisionedConcurrencyConfigOutputBuilder;

pub use crate::operation::put_provisioned_concurrency_config::_put_provisioned_concurrency_config_input::PutProvisionedConcurrencyConfigInputBuilder;

/// Fluent builder constructing a request to `PutProvisionedConcurrencyConfig`.
///
/// <p>Adds a provisioned concurrency configuration to a function's alias or version.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct PutProvisionedConcurrencyConfigFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::put_provisioned_concurrency_config::builders::PutProvisionedConcurrencyConfigInputBuilder
            }
impl PutProvisionedConcurrencyConfigFluentBuilder {
    /// Creates a new `PutProvisionedConcurrencyConfig`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::put_provisioned_concurrency_config::PutProvisionedConcurrencyConfig, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::put_provisioned_concurrency_config::PutProvisionedConcurrencyConfigError>
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
                    pub async fn send(self) -> std::result::Result<crate::operation::put_provisioned_concurrency_config::PutProvisionedConcurrencyConfigOutput, aws_smithy_http::result::SdkError<crate::operation::put_provisioned_concurrency_config::PutProvisionedConcurrencyConfigError>>
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
    ///     let deserialized_parameters: crate::operation::put_provisioned_concurrency_config::builders::PutProvisionedConcurrencyConfigInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.put_provisioned_concurrency_config().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::put_provisioned_concurrency_config::builders::PutProvisionedConcurrencyConfigInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The name of the Lambda function.</p>
    /// <p class="title"> <b>Name formats</b> </p>
    /// <ul>
    /// <li> <p> <b>Function name</b> – <code>my-function</code>.</p> </li>
    /// <li> <p> <b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li>
    /// <li> <p> <b>Partial ARN</b> – <code>123456789012:function:my-function</code>.</p> </li>
    /// </ul>
    /// <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    pub fn function_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.function_name(input.into());
        self
    }
    /// <p>The name of the Lambda function.</p>
    /// <p class="title"> <b>Name formats</b> </p>
    /// <ul>
    /// <li> <p> <b>Function name</b> – <code>my-function</code>.</p> </li>
    /// <li> <p> <b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li>
    /// <li> <p> <b>Partial ARN</b> – <code>123456789012:function:my-function</code>.</p> </li>
    /// </ul>
    /// <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    pub fn set_function_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_function_name(input);
        self
    }
    /// <p>The version number or alias name.</p>
    pub fn qualifier(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.qualifier(input.into());
        self
    }
    /// <p>The version number or alias name.</p>
    pub fn set_qualifier(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_qualifier(input);
        self
    }
    /// <p>The amount of provisioned concurrency to allocate for the version or alias.</p>
    pub fn provisioned_concurrent_executions(mut self, input: i32) -> Self {
        self.inner = self.inner.provisioned_concurrent_executions(input);
        self
    }
    /// <p>The amount of provisioned concurrency to allocate for the version or alias.</p>
    pub fn set_provisioned_concurrent_executions(
        mut self,
        input: std::option::Option<i32>,
    ) -> Self {
        self.inner = self.inner.set_provisioned_concurrent_executions(input);
        self
    }
}