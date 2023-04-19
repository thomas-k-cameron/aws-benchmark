// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetServerCertificate`](crate::operation::get_server_certificate::builders::GetServerCertificateFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`server_certificate_name(impl Into<String>)`](crate::operation::get_server_certificate::builders::GetServerCertificateFluentBuilder::server_certificate_name) / [`set_server_certificate_name(Option<String>)`](crate::operation::get_server_certificate::builders::GetServerCertificateFluentBuilder::set_server_certificate_name): <p>The name of the server certificate you want to retrieve information about.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    /// - On success, responds with [`GetServerCertificateOutput`](crate::operation::get_server_certificate::GetServerCertificateOutput) with field(s):
    ///   - [`server_certificate(Option<ServerCertificate>)`](crate::operation::get_server_certificate::GetServerCertificateOutput::server_certificate): <p>A structure containing details about the server certificate.</p>
    /// - On failure, responds with [`SdkError<GetServerCertificateError>`](crate::operation::get_server_certificate::GetServerCertificateError)
    pub fn get_server_certificate(
        &self,
    ) -> crate::operation::get_server_certificate::builders::GetServerCertificateFluentBuilder {
        crate::operation::get_server_certificate::builders::GetServerCertificateFluentBuilder::new(
            self.handle.clone(),
        )
    }
}