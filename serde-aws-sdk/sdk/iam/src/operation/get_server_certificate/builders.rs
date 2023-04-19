// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_server_certificate::_get_server_certificate_output::GetServerCertificateOutputBuilder;

pub use crate::operation::get_server_certificate::_get_server_certificate_input::GetServerCertificateInputBuilder;

/// Fluent builder constructing a request to `GetServerCertificate`.
///
/// <p>Retrieves information about the specified server certificate stored in IAM.</p>
/// <p>For more information about working with server certificates, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_server-certs.html">Working with server certificates</a> in the <i>IAM User Guide</i>. This topic includes a list of Amazon Web Services services that can use the server certificates that you manage with IAM.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetServerCertificateFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_server_certificate::builders::GetServerCertificateInputBuilder,
}
impl GetServerCertificateFluentBuilder {
    /// Creates a new `GetServerCertificate`.
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
            crate::operation::get_server_certificate::GetServerCertificate,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::get_server_certificate::GetServerCertificateError,
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
        crate::operation::get_server_certificate::GetServerCertificateOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::get_server_certificate::GetServerCertificateError,
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
    ///     let deserialized_parameters: crate::operation::get_server_certificate::builders::GetServerCertificateInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.get_server_certificate().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::get_server_certificate::builders::GetServerCertificateInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The name of the server certificate you want to retrieve information about.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn server_certificate_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.server_certificate_name(input.into());
        self
    }
    /// <p>The name of the server certificate you want to retrieve information about.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn set_server_certificate_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_server_certificate_name(input);
        self
    }
}