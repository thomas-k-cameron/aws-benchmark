// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::tag_role::_tag_role_output::TagRoleOutputBuilder;

pub use crate::operation::tag_role::_tag_role_input::TagRoleInputBuilder;

/// Fluent builder constructing a request to `TagRole`.
///
/// <p>Adds one or more tags to an IAM role. The role can be a regular role or a service-linked role. If a tag with the same key name already exists, then that tag is overwritten with the new value.</p>
/// <p>A tag consists of a key name and an associated value. By assigning tags to your resources, you can do the following:</p>
/// <ul>
/// <li> <p> <b>Administrative grouping and discovery</b> - Attach tags to resources to aid in organization and search. For example, you could search for all resources with the key name <i>Project</i> and the value <i>MyImportantProject</i>. Or search for all resources with the key name <i>Cost Center</i> and the value <i>41200</i>. </p> </li>
/// <li> <p> <b>Access control</b> - Include tags in IAM user-based and resource-based policies. You can use tags to restrict access to only an IAM role that has a specified tag attached. You can also restrict access to only those resources that have a certain tag attached. For examples of policies that show how to use tags to control access, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_tags.html">Control access using IAM tags</a> in the <i>IAM User Guide</i>.</p> </li>
/// <li> <p> <b>Cost allocation</b> - Use tags to help track which individuals and teams are using which Amazon Web Services resources.</p> </li>
/// </ul> <note>
/// <ul>
/// <li> <p>If any one of the tags is invalid or if you exceed the allowed maximum number of tags, then the entire request fails and the resource is not created. For more information about tagging, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_tags.html">Tagging IAM resources</a> in the <i>IAM User Guide</i>.</p> </li>
/// <li> <p>Amazon Web Services always interprets the tag <code>Value</code> as a single string. If you need to store an array, you can store comma-separated values in the string. However, you must interpret the value in your code.</p> </li>
/// </ul>
/// </note>
/// <p>For more information about tagging, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_tags.html">Tagging IAM identities</a> in the <i>IAM User Guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct TagRoleFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::tag_role::builders::TagRoleInputBuilder,
}
impl TagRoleFluentBuilder {
    /// Creates a new `TagRole`.
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
            crate::operation::tag_role::TagRole,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::tag_role::TagRoleError>,
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
        crate::operation::tag_role::TagRoleOutput,
        aws_smithy_http::result::SdkError<crate::operation::tag_role::TagRoleError>,
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
    ///     let deserialized_parameters: crate::operation::tag_role::builders::TagRoleInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.tag_role().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::tag_role::builders::TagRoleInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The name of the IAM role to which you want to add tags.</p>
    /// <p>This parameter accepts (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters that consist of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn role_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.role_name(input.into());
        self
    }
    /// <p>The name of the IAM role to which you want to add tags.</p>
    /// <p>This parameter accepts (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters that consist of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn set_role_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_role_name(input);
        self
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The list of tags that you want to attach to the IAM role. Each tag consists of a key name and an associated value.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>The list of tags that you want to attach to the IAM role. Each tag consists of a key name and an associated value.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}