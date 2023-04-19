// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_eks_properties(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::EksProperties,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.pod_properties {
        #[allow(unused_mut)]
        let mut object_2 = object.key("podProperties").start_object();
        crate::protocol_serde::shape_eks_pod_properties::ser_eks_pod_properties(
            &mut object_2,
            var_1,
        )?;
        object_2.finish();
    }
    Ok(())
}

pub(crate) fn de_eks_properties<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<
    Option<crate::types::EksProperties>,
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
            let mut builder = crate::types::builders::EksPropertiesBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "podProperties" => {
                                builder = builder.set_pod_properties(
                                    crate::protocol_serde::shape_eks_pod_properties::de_eks_pod_properties(tokens)?
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