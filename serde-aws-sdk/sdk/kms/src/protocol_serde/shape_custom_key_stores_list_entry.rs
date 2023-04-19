// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_custom_key_stores_list_entry<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<
    Option<crate::types::CustomKeyStoresListEntry>,
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
            let mut builder = crate::types::builders::CustomKeyStoresListEntryBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "CustomKeyStoreId" => {
                                builder = builder.set_custom_key_store_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "CustomKeyStoreName" => {
                                builder = builder.set_custom_key_store_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "CloudHsmClusterId" => {
                                builder = builder.set_cloud_hsm_cluster_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "TrustAnchorCertificate" => {
                                builder = builder.set_trust_anchor_certificate(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "ConnectionState" => {
                                builder = builder.set_connection_state(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::types::ConnectionStateType::from(u.as_ref())
                                        })
                                    })
                                    .transpose()?,
                                );
                            }
                            "ConnectionErrorCode" => {
                                builder = builder.set_connection_error_code(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::types::ConnectionErrorCodeType::from(u.as_ref())
                                        })
                                    })
                                    .transpose()?,
                                );
                            }
                            "CreationDate" => {
                                builder = builder.set_creation_date(
                                    aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                        tokens.next(),
                                        aws_smithy_types::date_time::Format::EpochSeconds,
                                    )?,
                                );
                            }
                            "CustomKeyStoreType" => {
                                builder = builder.set_custom_key_store_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::types::CustomKeyStoreType::from(u.as_ref())
                                        })
                                    })
                                    .transpose()?,
                                );
                            }
                            "XksProxyConfiguration" => {
                                builder = builder.set_xks_proxy_configuration(
                                    crate::protocol_serde::shape_xks_proxy_configuration_type::de_xks_proxy_configuration_type(tokens)?
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