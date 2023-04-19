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
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct CreateResourceInput {
    /// <p>The name of the resource type.</p>
    #[doc(hidden)]
    pub type_name: std::option::Option<std::string::String>,
    /// <p>For private resource types, the type version to use in this resource operation. If you do not specify a resource version, CloudFormation uses the default version.</p>
    #[doc(hidden)]
    pub type_version_id: std::option::Option<std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the Identity and Access Management (IAM) role for Cloud Control API to use when performing this resource operation. The role specified must have the permissions required for this operation. The necessary permissions for each event handler are defined in the <code> <a href="https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/resource-type-schema.html#schema-properties-handlers">handlers</a> </code> section of the <a href="https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/resource-type-schema.html">resource type definition schema</a>.</p>
    /// <p>If you do not specify a role, Cloud Control API uses a temporary session created using your Amazon Web Services user credentials.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/userguide/resource-operations.html#resource-operations-permissions">Specifying credentials</a> in the <i>Amazon Web Services Cloud Control API User Guide</i>.</p>
    #[doc(hidden)]
    pub role_arn: std::option::Option<std::string::String>,
    /// <p>A unique identifier to ensure the idempotency of the resource request. As a best practice, specify this token to ensure idempotency, so that Amazon Web Services Cloud Control API can accurately distinguish between request retries and new resource requests. You might retry a resource request to ensure that it was successfully received.</p>
    /// <p>A client token is valid for 36 hours once used. After that, a resource request with the same client token is treated as a new request.</p>
    /// <p>If you do not specify a client token, one is generated for inclusion in the request.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/userguide/resource-operations.html#resource-operations-idempotency">Ensuring resource operation requests are unique</a> in the <i>Amazon Web Services Cloud Control API User Guide</i>.</p>
    #[doc(hidden)]
    pub client_token: std::option::Option<std::string::String>,
    /// <p>Structured data format representing the desired state of the resource, consisting of that resource's properties and their desired values.</p> <note>
    /// <p>Cloud Control API currently supports JSON as a structured data format.</p>
    /// </note>
    /// <p>Specify the desired state as one of the following:</p>
    /// <ul>
    /// <li> <p>A JSON blob</p> </li>
    /// <li> <p>A local path containing the desired state in JSON data format</p> </li>
    /// </ul>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/userguide/resource-operations-create.html#resource-operations-create-desiredstate">Composing the desired state of the resource</a> in the <i>Amazon Web Services Cloud Control API User Guide</i>.</p>
    /// <p>For more information about the properties of a specific resource, refer to the related topic for the resource in the <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html">Resource and property types reference</a> in the <i>CloudFormation Users Guide</i>.</p>
    #[doc(hidden)]
    pub desired_state: std::option::Option<std::string::String>,
}
impl CreateResourceInput {
    /// <p>The name of the resource type.</p>
    pub fn type_name(&self) -> std::option::Option<&str> {
        self.type_name.as_deref()
    }
    /// <p>For private resource types, the type version to use in this resource operation. If you do not specify a resource version, CloudFormation uses the default version.</p>
    pub fn type_version_id(&self) -> std::option::Option<&str> {
        self.type_version_id.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the Identity and Access Management (IAM) role for Cloud Control API to use when performing this resource operation. The role specified must have the permissions required for this operation. The necessary permissions for each event handler are defined in the <code> <a href="https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/resource-type-schema.html#schema-properties-handlers">handlers</a> </code> section of the <a href="https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/resource-type-schema.html">resource type definition schema</a>.</p>
    /// <p>If you do not specify a role, Cloud Control API uses a temporary session created using your Amazon Web Services user credentials.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/userguide/resource-operations.html#resource-operations-permissions">Specifying credentials</a> in the <i>Amazon Web Services Cloud Control API User Guide</i>.</p>
    pub fn role_arn(&self) -> std::option::Option<&str> {
        self.role_arn.as_deref()
    }
    /// <p>A unique identifier to ensure the idempotency of the resource request. As a best practice, specify this token to ensure idempotency, so that Amazon Web Services Cloud Control API can accurately distinguish between request retries and new resource requests. You might retry a resource request to ensure that it was successfully received.</p>
    /// <p>A client token is valid for 36 hours once used. After that, a resource request with the same client token is treated as a new request.</p>
    /// <p>If you do not specify a client token, one is generated for inclusion in the request.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/userguide/resource-operations.html#resource-operations-idempotency">Ensuring resource operation requests are unique</a> in the <i>Amazon Web Services Cloud Control API User Guide</i>.</p>
    pub fn client_token(&self) -> std::option::Option<&str> {
        self.client_token.as_deref()
    }
    /// <p>Structured data format representing the desired state of the resource, consisting of that resource's properties and their desired values.</p> <note>
    /// <p>Cloud Control API currently supports JSON as a structured data format.</p>
    /// </note>
    /// <p>Specify the desired state as one of the following:</p>
    /// <ul>
    /// <li> <p>A JSON blob</p> </li>
    /// <li> <p>A local path containing the desired state in JSON data format</p> </li>
    /// </ul>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/userguide/resource-operations-create.html#resource-operations-create-desiredstate">Composing the desired state of the resource</a> in the <i>Amazon Web Services Cloud Control API User Guide</i>.</p>
    /// <p>For more information about the properties of a specific resource, refer to the related topic for the resource in the <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html">Resource and property types reference</a> in the <i>CloudFormation Users Guide</i>.</p>
    pub fn desired_state(&self) -> std::option::Option<&str> {
        self.desired_state.as_deref()
    }
}
impl std::fmt::Debug for CreateResourceInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("CreateResourceInput");
        formatter.field("type_name", &self.type_name);
        formatter.field("type_version_id", &self.type_version_id);
        formatter.field("role_arn", &self.role_arn);
        formatter.field("client_token", &self.client_token);
        formatter.field("desired_state", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl CreateResourceInput {
    /// Creates a new builder-style object to manufacture [`CreateResourceInput`](crate::operation::create_resource::CreateResourceInput).
    pub fn builder() -> crate::operation::create_resource::builders::CreateResourceInputBuilder {
        crate::operation::create_resource::builders::CreateResourceInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::create_resource::CreateResourceInput;
/// A builder for [`CreateResourceInput`](crate::operation::create_resource::CreateResourceInput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
pub struct CreateResourceInputBuilder {
    pub(crate) type_name: std::option::Option<std::string::String>,
    pub(crate) type_version_id: std::option::Option<std::string::String>,
    pub(crate) role_arn: std::option::Option<std::string::String>,
    pub(crate) client_token: std::option::Option<std::string::String>,
    pub(crate) desired_state: std::option::Option<std::string::String>,
}
impl CreateResourceInputBuilder {
    /// <p>The name of the resource type.</p>
    pub fn type_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.type_name = Some(input.into());
        self
    }
    /// <p>The name of the resource type.</p>
    pub fn set_type_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.type_name = input;
        self
    }
    /// <p>For private resource types, the type version to use in this resource operation. If you do not specify a resource version, CloudFormation uses the default version.</p>
    pub fn type_version_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.type_version_id = Some(input.into());
        self
    }
    /// <p>For private resource types, the type version to use in this resource operation. If you do not specify a resource version, CloudFormation uses the default version.</p>
    pub fn set_type_version_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.type_version_id = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Identity and Access Management (IAM) role for Cloud Control API to use when performing this resource operation. The role specified must have the permissions required for this operation. The necessary permissions for each event handler are defined in the <code> <a href="https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/resource-type-schema.html#schema-properties-handlers">handlers</a> </code> section of the <a href="https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/resource-type-schema.html">resource type definition schema</a>.</p>
    /// <p>If you do not specify a role, Cloud Control API uses a temporary session created using your Amazon Web Services user credentials.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/userguide/resource-operations.html#resource-operations-permissions">Specifying credentials</a> in the <i>Amazon Web Services Cloud Control API User Guide</i>.</p>
    pub fn role_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.role_arn = Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Identity and Access Management (IAM) role for Cloud Control API to use when performing this resource operation. The role specified must have the permissions required for this operation. The necessary permissions for each event handler are defined in the <code> <a href="https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/resource-type-schema.html#schema-properties-handlers">handlers</a> </code> section of the <a href="https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/resource-type-schema.html">resource type definition schema</a>.</p>
    /// <p>If you do not specify a role, Cloud Control API uses a temporary session created using your Amazon Web Services user credentials.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/userguide/resource-operations.html#resource-operations-permissions">Specifying credentials</a> in the <i>Amazon Web Services Cloud Control API User Guide</i>.</p>
    pub fn set_role_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.role_arn = input;
        self
    }
    /// <p>A unique identifier to ensure the idempotency of the resource request. As a best practice, specify this token to ensure idempotency, so that Amazon Web Services Cloud Control API can accurately distinguish between request retries and new resource requests. You might retry a resource request to ensure that it was successfully received.</p>
    /// <p>A client token is valid for 36 hours once used. After that, a resource request with the same client token is treated as a new request.</p>
    /// <p>If you do not specify a client token, one is generated for inclusion in the request.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/userguide/resource-operations.html#resource-operations-idempotency">Ensuring resource operation requests are unique</a> in the <i>Amazon Web Services Cloud Control API User Guide</i>.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.client_token = Some(input.into());
        self
    }
    /// <p>A unique identifier to ensure the idempotency of the resource request. As a best practice, specify this token to ensure idempotency, so that Amazon Web Services Cloud Control API can accurately distinguish between request retries and new resource requests. You might retry a resource request to ensure that it was successfully received.</p>
    /// <p>A client token is valid for 36 hours once used. After that, a resource request with the same client token is treated as a new request.</p>
    /// <p>If you do not specify a client token, one is generated for inclusion in the request.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/userguide/resource-operations.html#resource-operations-idempotency">Ensuring resource operation requests are unique</a> in the <i>Amazon Web Services Cloud Control API User Guide</i>.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// <p>Structured data format representing the desired state of the resource, consisting of that resource's properties and their desired values.</p> <note>
    /// <p>Cloud Control API currently supports JSON as a structured data format.</p>
    /// </note>
    /// <p>Specify the desired state as one of the following:</p>
    /// <ul>
    /// <li> <p>A JSON blob</p> </li>
    /// <li> <p>A local path containing the desired state in JSON data format</p> </li>
    /// </ul>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/userguide/resource-operations-create.html#resource-operations-create-desiredstate">Composing the desired state of the resource</a> in the <i>Amazon Web Services Cloud Control API User Guide</i>.</p>
    /// <p>For more information about the properties of a specific resource, refer to the related topic for the resource in the <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html">Resource and property types reference</a> in the <i>CloudFormation Users Guide</i>.</p>
    pub fn desired_state(mut self, input: impl Into<std::string::String>) -> Self {
        self.desired_state = Some(input.into());
        self
    }
    /// <p>Structured data format representing the desired state of the resource, consisting of that resource's properties and their desired values.</p> <note>
    /// <p>Cloud Control API currently supports JSON as a structured data format.</p>
    /// </note>
    /// <p>Specify the desired state as one of the following:</p>
    /// <ul>
    /// <li> <p>A JSON blob</p> </li>
    /// <li> <p>A local path containing the desired state in JSON data format</p> </li>
    /// </ul>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/userguide/resource-operations-create.html#resource-operations-create-desiredstate">Composing the desired state of the resource</a> in the <i>Amazon Web Services Cloud Control API User Guide</i>.</p>
    /// <p>For more information about the properties of a specific resource, refer to the related topic for the resource in the <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html">Resource and property types reference</a> in the <i>CloudFormation Users Guide</i>.</p>
    pub fn set_desired_state(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.desired_state = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateResourceInput`](crate::operation::create_resource::CreateResourceInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::create_resource::CreateResourceInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::create_resource::CreateResourceInput {
            type_name: self.type_name,
            type_version_id: self.type_version_id,
            role_arn: self.role_arn,
            client_token: self.client_token,
            desired_state: self.desired_state,
        })
    }
}
impl std::fmt::Debug for CreateResourceInputBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("CreateResourceInputBuilder");
        formatter.field("type_name", &self.type_name);
        formatter.field("type_version_id", &self.type_version_id);
        formatter.field("role_arn", &self.role_arn);
        formatter.field("client_token", &self.client_token);
        formatter.field("desired_state", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}