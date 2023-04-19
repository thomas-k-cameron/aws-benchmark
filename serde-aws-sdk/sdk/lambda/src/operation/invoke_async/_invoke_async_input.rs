// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[deprecated]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct InvokeAsyncInput {
    /// <p>The name of the Lambda function.</p>
    /// <p class="title"> <b>Name formats</b> </p>
    /// <ul>
    /// <li> <p> <b>Function name</b> – <code>my-function</code>.</p> </li>
    /// <li> <p> <b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li>
    /// <li> <p> <b>Partial ARN</b> – <code>123456789012:function:my-function</code>.</p> </li>
    /// </ul>
    /// <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[doc(hidden)]
    pub function_name: std::option::Option<std::string::String>,
    /// <p>The JSON that you want to provide to your Lambda function as input.</p>
    pub invoke_args: aws_smithy_http::byte_stream::ByteStream,
}
impl InvokeAsyncInput {
    /// <p>The name of the Lambda function.</p>
    /// <p class="title"> <b>Name formats</b> </p>
    /// <ul>
    /// <li> <p> <b>Function name</b> – <code>my-function</code>.</p> </li>
    /// <li> <p> <b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li>
    /// <li> <p> <b>Partial ARN</b> – <code>123456789012:function:my-function</code>.</p> </li>
    /// </ul>
    /// <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    pub fn function_name(&self) -> std::option::Option<&str> {
        self.function_name.as_deref()
    }
    /// <p>The JSON that you want to provide to your Lambda function as input.</p>
    pub fn invoke_args(&self) -> &aws_smithy_http::byte_stream::ByteStream {
        &self.invoke_args
    }
}
impl InvokeAsyncInput {
    /// Creates a new builder-style object to manufacture [`InvokeAsyncInput`](crate::operation::invoke_async::InvokeAsyncInput).
    pub fn builder() -> crate::operation::invoke_async::builders::InvokeAsyncInputBuilder {
        crate::operation::invoke_async::builders::InvokeAsyncInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::invoke_async::InvokeAsyncInput;
/// A builder for [`InvokeAsyncInput`](crate::operation::invoke_async::InvokeAsyncInput).
#[non_exhaustive]
#[derive(std::default::Default, std::fmt::Debug)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
pub struct InvokeAsyncInputBuilder {
    pub(crate) function_name: std::option::Option<std::string::String>,
    pub(crate) invoke_args: std::option::Option<aws_smithy_http::byte_stream::ByteStream>,
}
impl InvokeAsyncInputBuilder {
    /// <p>The name of the Lambda function.</p>
    /// <p class="title"> <b>Name formats</b> </p>
    /// <ul>
    /// <li> <p> <b>Function name</b> – <code>my-function</code>.</p> </li>
    /// <li> <p> <b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li>
    /// <li> <p> <b>Partial ARN</b> – <code>123456789012:function:my-function</code>.</p> </li>
    /// </ul>
    /// <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    pub fn function_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.function_name = Some(input.into());
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
        self.function_name = input;
        self
    }
    /// <p>The JSON that you want to provide to your Lambda function as input.</p>
    pub fn invoke_args(mut self, input: aws_smithy_http::byte_stream::ByteStream) -> Self {
        self.invoke_args = Some(input);
        self
    }
    /// <p>The JSON that you want to provide to your Lambda function as input.</p>
    pub fn set_invoke_args(
        mut self,
        input: std::option::Option<aws_smithy_http::byte_stream::ByteStream>,
    ) -> Self {
        self.invoke_args = input;
        self
    }
    /// Consumes the builder and constructs a [`InvokeAsyncInput`](crate::operation::invoke_async::InvokeAsyncInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::invoke_async::InvokeAsyncInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::invoke_async::InvokeAsyncInput {
            function_name: self.function_name,
            invoke_args: self.invoke_args.unwrap_or_default(),
        })
    }
}