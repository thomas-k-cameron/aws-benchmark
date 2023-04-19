// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ExecuteTransaction`](crate::operation::execute_transaction::builders::ExecuteTransactionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`transact_statements(Vec<ParameterizedStatement>)`](crate::operation::execute_transaction::builders::ExecuteTransactionFluentBuilder::transact_statements) / [`set_transact_statements(Option<Vec<ParameterizedStatement>>)`](crate::operation::execute_transaction::builders::ExecuteTransactionFluentBuilder::set_transact_statements): <p>The list of PartiQL statements representing the transaction to run.</p>
    ///   - [`client_request_token(impl Into<String>)`](crate::operation::execute_transaction::builders::ExecuteTransactionFluentBuilder::client_request_token) / [`set_client_request_token(Option<String>)`](crate::operation::execute_transaction::builders::ExecuteTransactionFluentBuilder::set_client_request_token): <p>Set this value to get remaining results, if <code>NextToken</code> was returned in the statement response.</p>
    ///   - [`return_consumed_capacity(ReturnConsumedCapacity)`](crate::operation::execute_transaction::builders::ExecuteTransactionFluentBuilder::return_consumed_capacity) / [`set_return_consumed_capacity(Option<ReturnConsumedCapacity>)`](crate::operation::execute_transaction::builders::ExecuteTransactionFluentBuilder::set_return_consumed_capacity): <p>Determines the level of detail about either provisioned or on-demand throughput consumption that is returned in the response. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_TransactGetItems.html">TransactGetItems</a> and <a href="https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_TransactWriteItems.html">TransactWriteItems</a>.</p>
    /// - On success, responds with [`ExecuteTransactionOutput`](crate::operation::execute_transaction::ExecuteTransactionOutput) with field(s):
    ///   - [`responses(Option<Vec<ItemResponse>>)`](crate::operation::execute_transaction::ExecuteTransactionOutput::responses): <p>The response to a PartiQL transaction.</p>
    ///   - [`consumed_capacity(Option<Vec<ConsumedCapacity>>)`](crate::operation::execute_transaction::ExecuteTransactionOutput::consumed_capacity): <p>The capacity units consumed by the entire operation. The values of the list are ordered according to the ordering of the statements.</p>
    /// - On failure, responds with [`SdkError<ExecuteTransactionError>`](crate::operation::execute_transaction::ExecuteTransactionError)
    pub fn execute_transaction(
        &self,
    ) -> crate::operation::execute_transaction::builders::ExecuteTransactionFluentBuilder {
        crate::operation::execute_transaction::builders::ExecuteTransactionFluentBuilder::new(
            self.handle.clone(),
        )
    }
}