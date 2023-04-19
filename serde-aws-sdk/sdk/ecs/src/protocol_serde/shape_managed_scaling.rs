// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_managed_scaling(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ManagedScaling,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.status {
        object.key("status").string(var_1.as_str());
    }
    if let Some(var_2) = &input.target_capacity {
        object.key("targetCapacity").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    if let Some(var_3) = &input.minimum_scaling_step_size {
        object.key("minimumScalingStepSize").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_3).into()),
        );
    }
    if let Some(var_4) = &input.maximum_scaling_step_size {
        object.key("maximumScalingStepSize").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    if let Some(var_5) = &input.instance_warmup_period {
        object.key("instanceWarmupPeriod").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_5).into()),
        );
    }
    Ok(())
}

pub(crate) fn de_managed_scaling<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<
    Option<crate::types::ManagedScaling>,
    aws_smithy_json::deserialize::error::DeserializeError,
>
where
    I: Iterator<
        Item = Result<
            aws_smithy_json::deserialize::Token<'a>,
            aws_smithy_json::deserialize::error::DeserializeError,
        >,
    >,
{
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::ManagedScalingBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "status" => {
                                builder = builder.set_status(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::types::ManagedScalingStatus::from(u.as_ref())
                                        })
                                    })
                                    .transpose()?,
                                );
                            }
                            "targetCapacity" => {
                                builder = builder.set_target_capacity(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
                                );
                            }
                            "minimumScalingStepSize" => {
                                builder = builder.set_minimum_scaling_step_size(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
                                );
                            }
                            "maximumScalingStepSize" => {
                                builder = builder.set_maximum_scaling_step_size(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
                                );
                            }
                            "instanceWarmupPeriod" => {
                                builder = builder.set_instance_warmup_period(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
                                );
                            }
                            _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                        }
                    }
                    other => {
                        return Err(
                            aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                                "expected object key or end object, found: {:?}",
                                other
                            )),
                        )
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(
            aws_smithy_json::deserialize::error::DeserializeError::custom(
                "expected start object or null",
            ),
        ),
    }
}