// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
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
pub struct GetFunctionUrlConfigOutput {
    /// <p>The HTTP URL endpoint for your function.</p>
    #[doc(hidden)]
    pub function_url: std::option::Option<std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of your function.</p>
    #[doc(hidden)]
    pub function_arn: std::option::Option<std::string::String>,
    /// <p>The type of authentication that your function URL uses. Set to <code>AWS_IAM</code> if you want to restrict access to authenticated users only. Set to <code>NONE</code> if you want to bypass IAM authentication to create a public endpoint. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/urls-auth.html">Security and auth model for Lambda function URLs</a>.</p>
    #[doc(hidden)]
    pub auth_type: std::option::Option<crate::types::FunctionUrlAuthType>,
    /// <p>The <a href="https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS">cross-origin resource sharing (CORS)</a> settings for your function URL.</p>
    #[doc(hidden)]
    pub cors: std::option::Option<crate::types::Cors>,
    /// <p>When the function URL was created, in <a href="https://www.w3.org/TR/NOTE-datetime">ISO-8601 format</a> (YYYY-MM-DDThh:mm:ss.sTZD).</p>
    #[doc(hidden)]
    pub creation_time: std::option::Option<std::string::String>,
    /// <p>When the function URL configuration was last updated, in <a href="https://www.w3.org/TR/NOTE-datetime">ISO-8601 format</a> (YYYY-MM-DDThh:mm:ss.sTZD).</p>
    #[doc(hidden)]
    pub last_modified_time: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl GetFunctionUrlConfigOutput {
    /// <p>The HTTP URL endpoint for your function.</p>
    pub fn function_url(&self) -> std::option::Option<&str> {
        self.function_url.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of your function.</p>
    pub fn function_arn(&self) -> std::option::Option<&str> {
        self.function_arn.as_deref()
    }
    /// <p>The type of authentication that your function URL uses. Set to <code>AWS_IAM</code> if you want to restrict access to authenticated users only. Set to <code>NONE</code> if you want to bypass IAM authentication to create a public endpoint. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/urls-auth.html">Security and auth model for Lambda function URLs</a>.</p>
    pub fn auth_type(&self) -> std::option::Option<&crate::types::FunctionUrlAuthType> {
        self.auth_type.as_ref()
    }
    /// <p>The <a href="https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS">cross-origin resource sharing (CORS)</a> settings for your function URL.</p>
    pub fn cors(&self) -> std::option::Option<&crate::types::Cors> {
        self.cors.as_ref()
    }
    /// <p>When the function URL was created, in <a href="https://www.w3.org/TR/NOTE-datetime">ISO-8601 format</a> (YYYY-MM-DDThh:mm:ss.sTZD).</p>
    pub fn creation_time(&self) -> std::option::Option<&str> {
        self.creation_time.as_deref()
    }
    /// <p>When the function URL configuration was last updated, in <a href="https://www.w3.org/TR/NOTE-datetime">ISO-8601 format</a> (YYYY-MM-DDThh:mm:ss.sTZD).</p>
    pub fn last_modified_time(&self) -> std::option::Option<&str> {
        self.last_modified_time.as_deref()
    }
}
impl aws_http::request_id::RequestId for GetFunctionUrlConfigOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetFunctionUrlConfigOutput {
    /// Creates a new builder-style object to manufacture [`GetFunctionUrlConfigOutput`](crate::operation::get_function_url_config::GetFunctionUrlConfigOutput).
    pub fn builder(
    ) -> crate::operation::get_function_url_config::builders::GetFunctionUrlConfigOutputBuilder
    {
        crate::operation::get_function_url_config::builders::GetFunctionUrlConfigOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_function_url_config::GetFunctionUrlConfigOutput;
/// A builder for [`GetFunctionUrlConfigOutput`](crate::operation::get_function_url_config::GetFunctionUrlConfigOutput).
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
pub struct GetFunctionUrlConfigOutputBuilder {
    pub(crate) function_url: std::option::Option<std::string::String>,
    pub(crate) function_arn: std::option::Option<std::string::String>,
    pub(crate) auth_type: std::option::Option<crate::types::FunctionUrlAuthType>,
    pub(crate) cors: std::option::Option<crate::types::Cors>,
    pub(crate) creation_time: std::option::Option<std::string::String>,
    pub(crate) last_modified_time: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl GetFunctionUrlConfigOutputBuilder {
    /// <p>The HTTP URL endpoint for your function.</p>
    pub fn function_url(mut self, input: impl Into<std::string::String>) -> Self {
        self.function_url = Some(input.into());
        self
    }
    /// <p>The HTTP URL endpoint for your function.</p>
    pub fn set_function_url(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.function_url = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of your function.</p>
    pub fn function_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.function_arn = Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of your function.</p>
    pub fn set_function_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.function_arn = input;
        self
    }
    /// <p>The type of authentication that your function URL uses. Set to <code>AWS_IAM</code> if you want to restrict access to authenticated users only. Set to <code>NONE</code> if you want to bypass IAM authentication to create a public endpoint. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/urls-auth.html">Security and auth model for Lambda function URLs</a>.</p>
    pub fn auth_type(mut self, input: crate::types::FunctionUrlAuthType) -> Self {
        self.auth_type = Some(input);
        self
    }
    /// <p>The type of authentication that your function URL uses. Set to <code>AWS_IAM</code> if you want to restrict access to authenticated users only. Set to <code>NONE</code> if you want to bypass IAM authentication to create a public endpoint. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/urls-auth.html">Security and auth model for Lambda function URLs</a>.</p>
    pub fn set_auth_type(
        mut self,
        input: std::option::Option<crate::types::FunctionUrlAuthType>,
    ) -> Self {
        self.auth_type = input;
        self
    }
    /// <p>The <a href="https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS">cross-origin resource sharing (CORS)</a> settings for your function URL.</p>
    pub fn cors(mut self, input: crate::types::Cors) -> Self {
        self.cors = Some(input);
        self
    }
    /// <p>The <a href="https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS">cross-origin resource sharing (CORS)</a> settings for your function URL.</p>
    pub fn set_cors(mut self, input: std::option::Option<crate::types::Cors>) -> Self {
        self.cors = input;
        self
    }
    /// <p>When the function URL was created, in <a href="https://www.w3.org/TR/NOTE-datetime">ISO-8601 format</a> (YYYY-MM-DDThh:mm:ss.sTZD).</p>
    pub fn creation_time(mut self, input: impl Into<std::string::String>) -> Self {
        self.creation_time = Some(input.into());
        self
    }
    /// <p>When the function URL was created, in <a href="https://www.w3.org/TR/NOTE-datetime">ISO-8601 format</a> (YYYY-MM-DDThh:mm:ss.sTZD).</p>
    pub fn set_creation_time(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.creation_time = input;
        self
    }
    /// <p>When the function URL configuration was last updated, in <a href="https://www.w3.org/TR/NOTE-datetime">ISO-8601 format</a> (YYYY-MM-DDThh:mm:ss.sTZD).</p>
    pub fn last_modified_time(mut self, input: impl Into<std::string::String>) -> Self {
        self.last_modified_time = Some(input.into());
        self
    }
    /// <p>When the function URL configuration was last updated, in <a href="https://www.w3.org/TR/NOTE-datetime">ISO-8601 format</a> (YYYY-MM-DDThh:mm:ss.sTZD).</p>
    pub fn set_last_modified_time(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.last_modified_time = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GetFunctionUrlConfigOutput`](crate::operation::get_function_url_config::GetFunctionUrlConfigOutput).
    pub fn build(self) -> crate::operation::get_function_url_config::GetFunctionUrlConfigOutput {
        crate::operation::get_function_url_config::GetFunctionUrlConfigOutput {
            function_url: self.function_url,
            function_arn: self.function_arn,
            auth_type: self.auth_type,
            cors: self.cors,
            creation_time: self.creation_time,
            last_modified_time: self.last_modified_time,
            _request_id: self._request_id,
        }
    }
}