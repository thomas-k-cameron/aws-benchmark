// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_pending_aggregation_requests_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::describe_pending_aggregation_requests::DescribePendingAggregationRequestsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.limit != 0 {
        object.key("Limit").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.limit).into()),
        );
    }
    if let Some(var_1) = &input.next_token {
        object.key("NextToken").string(var_1.as_str());
    }
    Ok(())
}
