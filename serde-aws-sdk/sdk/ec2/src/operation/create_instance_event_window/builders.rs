// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_instance_event_window::_create_instance_event_window_output::CreateInstanceEventWindowOutputBuilder;

pub use crate::operation::create_instance_event_window::_create_instance_event_window_input::CreateInstanceEventWindowInputBuilder;

/// Fluent builder constructing a request to `CreateInstanceEventWindow`.
///
/// <p>Creates an event window in which scheduled events for the associated Amazon EC2 instances can run.</p>
/// <p>You can define either a set of time ranges or a cron expression when creating the event window, but not both. All event window times are in UTC.</p>
/// <p>You can create up to 200 event windows per Amazon Web Services Region.</p>
/// <p>When you create the event window, targets (instance IDs, Dedicated Host IDs, or tags) are not yet associated with it. To ensure that the event window can be used, you must associate one or more targets with it by using the <code>AssociateInstanceEventWindow</code> API.</p> <important>
/// <p>Event windows are applicable only for scheduled events that stop, reboot, or terminate instances.</p>
/// <p>Event windows are <i>not</i> applicable for:</p>
/// <ul>
/// <li> <p>Expedited scheduled events and network maintenance events. </p> </li>
/// <li> <p>Unscheduled maintenance such as AutoRecovery and unplanned reboots.</p> </li>
/// </ul>
/// </important>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/event-windows.html">Define event windows for scheduled events</a> in the <i>Amazon EC2 User Guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateInstanceEventWindowFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::create_instance_event_window::builders::CreateInstanceEventWindowInputBuilder
            }
impl CreateInstanceEventWindowFluentBuilder {
    /// Creates a new `CreateInstanceEventWindow`.
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
            crate::operation::create_instance_event_window::CreateInstanceEventWindow,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::create_instance_event_window::CreateInstanceEventWindowError,
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
        crate::operation::create_instance_event_window::CreateInstanceEventWindowOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::create_instance_event_window::CreateInstanceEventWindowError,
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
    ///     let deserialized_parameters: crate::operation::create_instance_event_window::builders::CreateInstanceEventWindowInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.create_instance_event_window().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::create_instance_event_window::builders::CreateInstanceEventWindowInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>The name of the event window.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the event window.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// Appends an item to `TimeRanges`.
    ///
    /// To override the contents of this collection use [`set_time_ranges`](Self::set_time_ranges).
    ///
    /// <p>The time range for the event window. If you specify a time range, you can't specify a cron expression.</p>
    pub fn time_ranges(mut self, input: crate::types::InstanceEventWindowTimeRangeRequest) -> Self {
        self.inner = self.inner.time_ranges(input);
        self
    }
    /// <p>The time range for the event window. If you specify a time range, you can't specify a cron expression.</p>
    pub fn set_time_ranges(
        mut self,
        input: std::option::Option<
            std::vec::Vec<crate::types::InstanceEventWindowTimeRangeRequest>,
        >,
    ) -> Self {
        self.inner = self.inner.set_time_ranges(input);
        self
    }
    /// <p>The cron expression for the event window, for example, <code>* 0-4,20-23 * * 1,5</code>. If you specify a cron expression, you can't specify a time range.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Only hour and day of the week values are supported.</p> </li>
    /// <li> <p>For day of the week values, you can specify either integers <code>0</code> through <code>6</code>, or alternative single values <code>SUN</code> through <code>SAT</code>.</p> </li>
    /// <li> <p>The minute, month, and year must be specified by <code>*</code>.</p> </li>
    /// <li> <p>The hour value must be one or a multiple range, for example, <code>0-4</code> or <code>0-4,20-23</code>.</p> </li>
    /// <li> <p>Each hour range must be &gt;= 2 hours, for example, <code>0-2</code> or <code>20-23</code>.</p> </li>
    /// <li> <p>The event window must be &gt;= 4 hours. The combined total time ranges in the event window must be &gt;= 4 hours.</p> </li>
    /// </ul>
    /// <p>For more information about cron expressions, see <a href="https://en.wikipedia.org/wiki/Cron">cron</a> on the <i>Wikipedia website</i>.</p>
    pub fn cron_expression(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.cron_expression(input.into());
        self
    }
    /// <p>The cron expression for the event window, for example, <code>* 0-4,20-23 * * 1,5</code>. If you specify a cron expression, you can't specify a time range.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Only hour and day of the week values are supported.</p> </li>
    /// <li> <p>For day of the week values, you can specify either integers <code>0</code> through <code>6</code>, or alternative single values <code>SUN</code> through <code>SAT</code>.</p> </li>
    /// <li> <p>The minute, month, and year must be specified by <code>*</code>.</p> </li>
    /// <li> <p>The hour value must be one or a multiple range, for example, <code>0-4</code> or <code>0-4,20-23</code>.</p> </li>
    /// <li> <p>Each hour range must be &gt;= 2 hours, for example, <code>0-2</code> or <code>20-23</code>.</p> </li>
    /// <li> <p>The event window must be &gt;= 4 hours. The combined total time ranges in the event window must be &gt;= 4 hours.</p> </li>
    /// </ul>
    /// <p>For more information about cron expressions, see <a href="https://en.wikipedia.org/wiki/Cron">cron</a> on the <i>Wikipedia website</i>.</p>
    pub fn set_cron_expression(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_cron_expression(input);
        self
    }
    /// Appends an item to `TagSpecifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to apply to the event window.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        self.inner = self.inner.tag_specifications(input);
        self
    }
    /// <p>The tags to apply to the event window.</p>
    pub fn set_tag_specifications(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::TagSpecification>>,
    ) -> Self {
        self.inner = self.inner.set_tag_specifications(input);
        self
    }
}