// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_organizations_access_report::_get_organizations_access_report_output::GetOrganizationsAccessReportOutputBuilder;

pub use crate::operation::get_organizations_access_report::_get_organizations_access_report_input::GetOrganizationsAccessReportInputBuilder;

/// Fluent builder constructing a request to `GetOrganizationsAccessReport`.
///
/// <p>Retrieves the service last accessed data report for Organizations that was previously generated using the <code> <code>GenerateOrganizationsAccessReport</code> </code> operation. This operation retrieves the status of your report job and the report contents.</p>
/// <p>Depending on the parameters that you passed when you generated the report, the data returned could include different information. For details, see <code>GenerateOrganizationsAccessReport</code>.</p>
/// <p>To call this operation, you must be signed in to the management account in your organization. SCPs must be enabled for your organization root. You must have permissions to perform this operation. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_access-advisor.html">Refining permissions using service last accessed data</a> in the <i>IAM User Guide</i>.</p>
/// <p>For each service that principals in an account (root user, IAM users, or IAM roles) could access using SCPs, the operation returns details about the most recent access attempt. If there was no attempt, the service is listed without details about the most recent attempt to access the service. If the operation fails, it returns the reason that it failed.</p>
/// <p>By default, the list is sorted by service namespace.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetOrganizationsAccessReportFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::get_organizations_access_report::builders::GetOrganizationsAccessReportInputBuilder
            }
impl GetOrganizationsAccessReportFluentBuilder {
    /// Creates a new `GetOrganizationsAccessReport`.
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
            crate::operation::get_organizations_access_report::GetOrganizationsAccessReport,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::get_organizations_access_report::GetOrganizationsAccessReportError,
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
        crate::operation::get_organizations_access_report::GetOrganizationsAccessReportOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::get_organizations_access_report::GetOrganizationsAccessReportError,
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
    ///     let deserialized_parameters: crate::operation::get_organizations_access_report::builders::GetOrganizationsAccessReportInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.get_organizations_access_report().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::get_organizations_access_report::builders::GetOrganizationsAccessReportInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The identifier of the request generated by the <code>GenerateOrganizationsAccessReport</code> operation.</p>
    pub fn job_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.job_id(input.into());
        self
    }
    /// <p>The identifier of the request generated by the <code>GenerateOrganizationsAccessReport</code> operation.</p>
    pub fn set_job_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_job_id(input);
        self
    }
    /// <p>Use this only when paginating results to indicate the maximum number of items you want in the response. If additional items exist beyond the maximum you specify, the <code>IsTruncated</code> response element is <code>true</code>.</p>
    /// <p>If you do not include this parameter, the number of items defaults to 100. Note that IAM might return fewer results, even when there are more results available. In that case, the <code>IsTruncated</code> response element returns <code>true</code>, and <code>Marker</code> contains a value to include in the subsequent call that tells the service where to continue from.</p>
    pub fn max_items(mut self, input: i32) -> Self {
        self.inner = self.inner.max_items(input);
        self
    }
    /// <p>Use this only when paginating results to indicate the maximum number of items you want in the response. If additional items exist beyond the maximum you specify, the <code>IsTruncated</code> response element is <code>true</code>.</p>
    /// <p>If you do not include this parameter, the number of items defaults to 100. Note that IAM might return fewer results, even when there are more results available. In that case, the <code>IsTruncated</code> response element returns <code>true</code>, and <code>Marker</code> contains a value to include in the subsequent call that tells the service where to continue from.</p>
    pub fn set_max_items(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_items(input);
        self
    }
    /// <p>Use this parameter only when paginating results and only after you receive a response indicating that the results are truncated. Set it to the value of the <code>Marker</code> element in the response that you received to indicate where the next call should start.</p>
    pub fn marker(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.marker(input.into());
        self
    }
    /// <p>Use this parameter only when paginating results and only after you receive a response indicating that the results are truncated. Set it to the value of the <code>Marker</code> element in the response that you received to indicate where the next call should start.</p>
    pub fn set_marker(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_marker(input);
        self
    }
    /// <p>The key that is used to sort the results. If you choose the namespace key, the results are returned in alphabetical order. If you choose the time key, the results are sorted numerically by the date and time.</p>
    pub fn sort_key(mut self, input: crate::types::SortKeyType) -> Self {
        self.inner = self.inner.sort_key(input);
        self
    }
    /// <p>The key that is used to sort the results. If you choose the namespace key, the results are returned in alphabetical order. If you choose the time key, the results are sorted numerically by the date and time.</p>
    pub fn set_sort_key(mut self, input: std::option::Option<crate::types::SortKeyType>) -> Self {
        self.inner = self.inner.set_sort_key(input);
        self
    }
}