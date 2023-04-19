// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_code_signing_config::_create_code_signing_config_output::CreateCodeSigningConfigOutputBuilder;

pub use crate::operation::create_code_signing_config::_create_code_signing_config_input::CreateCodeSigningConfigInputBuilder;

/// Fluent builder constructing a request to `CreateCodeSigningConfig`.
///
/// <p>Creates a code signing configuration. A <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-codesigning.html">code signing configuration</a> defines a list of allowed signing profiles and defines the code-signing validation policy (action to be taken if deployment validation checks fail). </p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateCodeSigningConfigFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::create_code_signing_config::builders::CreateCodeSigningConfigInputBuilder,
}
impl CreateCodeSigningConfigFluentBuilder {
    /// Creates a new `CreateCodeSigningConfig`.
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
            crate::operation::create_code_signing_config::CreateCodeSigningConfig,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::create_code_signing_config::CreateCodeSigningConfigError,
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
        crate::operation::create_code_signing_config::CreateCodeSigningConfigOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::create_code_signing_config::CreateCodeSigningConfigError,
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
    ///     let deserialized_parameters: crate::operation::create_code_signing_config::builders::CreateCodeSigningConfigInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.create_code_signing_config().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::create_code_signing_config::builders::CreateCodeSigningConfigInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>Descriptive name for this code signing configuration.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>Descriptive name for this code signing configuration.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>Signing profiles for this code signing configuration.</p>
    pub fn allowed_publishers(mut self, input: crate::types::AllowedPublishers) -> Self {
        self.inner = self.inner.allowed_publishers(input);
        self
    }
    /// <p>Signing profiles for this code signing configuration.</p>
    pub fn set_allowed_publishers(
        mut self,
        input: std::option::Option<crate::types::AllowedPublishers>,
    ) -> Self {
        self.inner = self.inner.set_allowed_publishers(input);
        self
    }
    /// <p>The code signing policies define the actions to take if the validation checks fail. </p>
    pub fn code_signing_policies(mut self, input: crate::types::CodeSigningPolicies) -> Self {
        self.inner = self.inner.code_signing_policies(input);
        self
    }
    /// <p>The code signing policies define the actions to take if the validation checks fail. </p>
    pub fn set_code_signing_policies(
        mut self,
        input: std::option::Option<crate::types::CodeSigningPolicies>,
    ) -> Self {
        self.inner = self.inner.set_code_signing_policies(input);
        self
    }
}