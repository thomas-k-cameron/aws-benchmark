// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ModifyInstanceEventWindow`](crate::operation::modify_instance_event_window::builders::ModifyInstanceEventWindowFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::modify_instance_event_window::builders::ModifyInstanceEventWindowFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::modify_instance_event_window::builders::ModifyInstanceEventWindowFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`name(impl Into<String>)`](crate::operation::modify_instance_event_window::builders::ModifyInstanceEventWindowFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::modify_instance_event_window::builders::ModifyInstanceEventWindowFluentBuilder::set_name): <p>The name of the event window.</p>
    ///   - [`instance_event_window_id(impl Into<String>)`](crate::operation::modify_instance_event_window::builders::ModifyInstanceEventWindowFluentBuilder::instance_event_window_id) / [`set_instance_event_window_id(Option<String>)`](crate::operation::modify_instance_event_window::builders::ModifyInstanceEventWindowFluentBuilder::set_instance_event_window_id): <p>The ID of the event window.</p>
    ///   - [`time_ranges(Vec<InstanceEventWindowTimeRangeRequest>)`](crate::operation::modify_instance_event_window::builders::ModifyInstanceEventWindowFluentBuilder::time_ranges) / [`set_time_ranges(Option<Vec<InstanceEventWindowTimeRangeRequest>>)`](crate::operation::modify_instance_event_window::builders::ModifyInstanceEventWindowFluentBuilder::set_time_ranges): <p>The time ranges of the event window.</p>
    ///   - [`cron_expression(impl Into<String>)`](crate::operation::modify_instance_event_window::builders::ModifyInstanceEventWindowFluentBuilder::cron_expression) / [`set_cron_expression(Option<String>)`](crate::operation::modify_instance_event_window::builders::ModifyInstanceEventWindowFluentBuilder::set_cron_expression): <p>The cron expression of the event window, for example, <code>* 0-4,20-23 * * 1,5</code>.</p>  <p>Constraints:</p>  <ul>   <li> <p>Only hour and day of the week values are supported.</p> </li>   <li> <p>For day of the week values, you can specify either integers <code>0</code> through <code>6</code>, or alternative single values <code>SUN</code> through <code>SAT</code>.</p> </li>   <li> <p>The minute, month, and year must be specified by <code>*</code>.</p> </li>   <li> <p>The hour value must be one or a multiple range, for example, <code>0-4</code> or <code>0-4,20-23</code>.</p> </li>   <li> <p>Each hour range must be &gt;= 2 hours, for example, <code>0-2</code> or <code>20-23</code>.</p> </li>   <li> <p>The event window must be &gt;= 4 hours. The combined total time ranges in the event window must be &gt;= 4 hours.</p> </li>  </ul>  <p>For more information about cron expressions, see <a href="https://en.wikipedia.org/wiki/Cron">cron</a> on the <i>Wikipedia website</i>.</p>
    /// - On success, responds with [`ModifyInstanceEventWindowOutput`](crate::operation::modify_instance_event_window::ModifyInstanceEventWindowOutput) with field(s):
    ///   - [`instance_event_window(Option<InstanceEventWindow>)`](crate::operation::modify_instance_event_window::ModifyInstanceEventWindowOutput::instance_event_window): <p>Information about the event window.</p>
    /// - On failure, responds with [`SdkError<ModifyInstanceEventWindowError>`](crate::operation::modify_instance_event_window::ModifyInstanceEventWindowError)
    pub fn modify_instance_event_window(&self) -> crate::operation::modify_instance_event_window::builders::ModifyInstanceEventWindowFluentBuilder{
        crate::operation::modify_instance_event_window::builders::ModifyInstanceEventWindowFluentBuilder::new(self.handle.clone())
    }
}