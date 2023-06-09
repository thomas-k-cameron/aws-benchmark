// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutConfigurationRecorder`](crate::operation::put_configuration_recorder::builders::PutConfigurationRecorderFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`configuration_recorder(ConfigurationRecorder)`](crate::operation::put_configuration_recorder::builders::PutConfigurationRecorderFluentBuilder::configuration_recorder) / [`set_configuration_recorder(Option<ConfigurationRecorder>)`](crate::operation::put_configuration_recorder::builders::PutConfigurationRecorderFluentBuilder::set_configuration_recorder): <p>The configuration recorder object that records each configuration change made to the resources.</p>
    /// - On success, responds with [`PutConfigurationRecorderOutput`](crate::operation::put_configuration_recorder::PutConfigurationRecorderOutput)
    /// - On failure, responds with [`SdkError<PutConfigurationRecorderError>`](crate::operation::put_configuration_recorder::PutConfigurationRecorderError)
    pub fn put_configuration_recorder(
        &self,
    ) -> crate::operation::put_configuration_recorder::builders::PutConfigurationRecorderFluentBuilder
    {
        crate::operation::put_configuration_recorder::builders::PutConfigurationRecorderFluentBuilder::new(self.handle.clone())
    }
}
