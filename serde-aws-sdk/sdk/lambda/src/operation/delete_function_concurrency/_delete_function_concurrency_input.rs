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
pub struct DeleteFunctionConcurrencyInput {
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
}
impl DeleteFunctionConcurrencyInput {
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
}
impl DeleteFunctionConcurrencyInput {
    /// Creates a new builder-style object to manufacture [`DeleteFunctionConcurrencyInput`](crate::operation::delete_function_concurrency::DeleteFunctionConcurrencyInput).
    pub fn builder() -> crate::operation::delete_function_concurrency::builders::DeleteFunctionConcurrencyInputBuilder{
        crate::operation::delete_function_concurrency::builders::DeleteFunctionConcurrencyInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::delete_function_concurrency::DeleteFunctionConcurrencyInput;
/// A builder for [`DeleteFunctionConcurrencyInput`](crate::operation::delete_function_concurrency::DeleteFunctionConcurrencyInput).
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
pub struct DeleteFunctionConcurrencyInputBuilder {
    pub(crate) function_name: std::option::Option<std::string::String>,
}
impl DeleteFunctionConcurrencyInputBuilder {
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
    /// Consumes the builder and constructs a [`DeleteFunctionConcurrencyInput`](crate::operation::delete_function_concurrency::DeleteFunctionConcurrencyInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::delete_function_concurrency::DeleteFunctionConcurrencyInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::delete_function_concurrency::DeleteFunctionConcurrencyInput {
                function_name: self.function_name,
            },
        )
    }
}
