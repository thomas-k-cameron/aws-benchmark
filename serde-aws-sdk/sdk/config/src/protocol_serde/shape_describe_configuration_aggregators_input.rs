// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_configuration_aggregators_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::describe_configuration_aggregators::DescribeConfigurationAggregatorsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.configuration_aggregator_names {
        let mut array_2 = object.key("ConfigurationAggregatorNames").start_array();
        for item_3 in var_1 {
            {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.next_token {
        object.key("NextToken").string(var_4.as_str());
    }
    if input.limit != 0 {
        object.key("Limit").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.limit).into()),
        );
    }
    Ok(())
}