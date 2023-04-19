// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CancelExportTask`](crate::operation::cancel_export_task::builders::CancelExportTaskFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`export_task_id(impl Into<String>)`](crate::operation::cancel_export_task::builders::CancelExportTaskFluentBuilder::export_task_id) / [`set_export_task_id(Option<String>)`](crate::operation::cancel_export_task::builders::CancelExportTaskFluentBuilder::set_export_task_id): <p>The ID of the export task. This is the ID returned by <code>CreateInstanceExportTask</code>.</p>
    /// - On success, responds with [`CancelExportTaskOutput`](crate::operation::cancel_export_task::CancelExportTaskOutput)
    /// - On failure, responds with [`SdkError<CancelExportTaskError>`](crate::operation::cancel_export_task::CancelExportTaskError)
    pub fn cancel_export_task(
        &self,
    ) -> crate::operation::cancel_export_task::builders::CancelExportTaskFluentBuilder {
        crate::operation::cancel_export_task::builders::CancelExportTaskFluentBuilder::new(
            self.handle.clone(),
        )
    }
}