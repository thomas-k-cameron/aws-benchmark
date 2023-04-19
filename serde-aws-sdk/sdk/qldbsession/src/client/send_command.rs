// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`SendCommand`](crate::operation::send_command::builders::SendCommandFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`session_token(impl Into<String>)`](crate::operation::send_command::builders::SendCommandFluentBuilder::session_token) / [`set_session_token(Option<String>)`](crate::operation::send_command::builders::SendCommandFluentBuilder::set_session_token): <p>Specifies the session token for the current command. A session token is constant throughout the life of the session.</p>  <p>To obtain a session token, run the <code>StartSession</code> command. This <code>SessionToken</code> is required for every subsequent command that is issued during the current session.</p>
    ///   - [`start_session(StartSessionRequest)`](crate::operation::send_command::builders::SendCommandFluentBuilder::start_session) / [`set_start_session(Option<StartSessionRequest>)`](crate::operation::send_command::builders::SendCommandFluentBuilder::set_start_session): <p>Command to start a new session. A session token is obtained as part of the response.</p>
    ///   - [`start_transaction(StartTransactionRequest)`](crate::operation::send_command::builders::SendCommandFluentBuilder::start_transaction) / [`set_start_transaction(Option<StartTransactionRequest>)`](crate::operation::send_command::builders::SendCommandFluentBuilder::set_start_transaction): <p>Command to start a new transaction.</p>
    ///   - [`end_session(EndSessionRequest)`](crate::operation::send_command::builders::SendCommandFluentBuilder::end_session) / [`set_end_session(Option<EndSessionRequest>)`](crate::operation::send_command::builders::SendCommandFluentBuilder::set_end_session): <p>Command to end the current session.</p>
    ///   - [`commit_transaction(CommitTransactionRequest)`](crate::operation::send_command::builders::SendCommandFluentBuilder::commit_transaction) / [`set_commit_transaction(Option<CommitTransactionRequest>)`](crate::operation::send_command::builders::SendCommandFluentBuilder::set_commit_transaction): <p>Command to commit the specified transaction.</p>
    ///   - [`abort_transaction(AbortTransactionRequest)`](crate::operation::send_command::builders::SendCommandFluentBuilder::abort_transaction) / [`set_abort_transaction(Option<AbortTransactionRequest>)`](crate::operation::send_command::builders::SendCommandFluentBuilder::set_abort_transaction): <p>Command to abort the current transaction.</p>
    ///   - [`execute_statement(ExecuteStatementRequest)`](crate::operation::send_command::builders::SendCommandFluentBuilder::execute_statement) / [`set_execute_statement(Option<ExecuteStatementRequest>)`](crate::operation::send_command::builders::SendCommandFluentBuilder::set_execute_statement): <p>Command to execute a statement in the specified transaction.</p>
    ///   - [`fetch_page(FetchPageRequest)`](crate::operation::send_command::builders::SendCommandFluentBuilder::fetch_page) / [`set_fetch_page(Option<FetchPageRequest>)`](crate::operation::send_command::builders::SendCommandFluentBuilder::set_fetch_page): <p>Command to fetch a page.</p>
    /// - On success, responds with [`SendCommandOutput`](crate::operation::send_command::SendCommandOutput) with field(s):
    ///   - [`start_session(Option<StartSessionResult>)`](crate::operation::send_command::SendCommandOutput::start_session): <p>Contains the details of the started session that includes a session token. This <code>SessionToken</code> is required for every subsequent command that is issued during the current session.</p>
    ///   - [`start_transaction(Option<StartTransactionResult>)`](crate::operation::send_command::SendCommandOutput::start_transaction): <p>Contains the details of the started transaction.</p>
    ///   - [`end_session(Option<EndSessionResult>)`](crate::operation::send_command::SendCommandOutput::end_session): <p>Contains the details of the ended session.</p>
    ///   - [`commit_transaction(Option<CommitTransactionResult>)`](crate::operation::send_command::SendCommandOutput::commit_transaction): <p>Contains the details of the committed transaction.</p>
    ///   - [`abort_transaction(Option<AbortTransactionResult>)`](crate::operation::send_command::SendCommandOutput::abort_transaction): <p>Contains the details of the aborted transaction.</p>
    ///   - [`execute_statement(Option<ExecuteStatementResult>)`](crate::operation::send_command::SendCommandOutput::execute_statement): <p>Contains the details of the executed statement.</p>
    ///   - [`fetch_page(Option<FetchPageResult>)`](crate::operation::send_command::SendCommandOutput::fetch_page): <p>Contains the details of the fetched page.</p>
    /// - On failure, responds with [`SdkError<SendCommandError>`](crate::operation::send_command::SendCommandError)
    pub fn send_command(
        &self,
    ) -> crate::operation::send_command::builders::SendCommandFluentBuilder {
        crate::operation::send_command::builders::SendCommandFluentBuilder::new(self.handle.clone())
    }
}