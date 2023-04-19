// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_verified_access_trust_provider_condensed(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<
    crate::types::VerifiedAccessTrustProviderCondensed,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    #[allow(unused_mut)]
    let mut builder = crate::types::VerifiedAccessTrustProviderCondensed::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("verifiedAccessTrustProviderId") /* VerifiedAccessTrustProviderId com.amazonaws.ec2#VerifiedAccessTrustProviderCondensed$VerifiedAccessTrustProviderId */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_verified_access_trust_provider_id(var_1);
            }
            ,
            s if s.matches("description") /* Description com.amazonaws.ec2#VerifiedAccessTrustProviderCondensed$Description */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_description(var_2);
            }
            ,
            s if s.matches("trustProviderType") /* TrustProviderType com.amazonaws.ec2#VerifiedAccessTrustProviderCondensed$TrustProviderType */ =>  {
                let var_3 =
                    Some(
                        Result::<crate::types::TrustProviderType, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::TrustProviderType::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_trust_provider_type(var_3);
            }
            ,
            s if s.matches("userTrustProviderType") /* UserTrustProviderType com.amazonaws.ec2#VerifiedAccessTrustProviderCondensed$UserTrustProviderType */ =>  {
                let var_4 =
                    Some(
                        Result::<crate::types::UserTrustProviderType, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::UserTrustProviderType::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_user_trust_provider_type(var_4);
            }
            ,
            s if s.matches("deviceTrustProviderType") /* DeviceTrustProviderType com.amazonaws.ec2#VerifiedAccessTrustProviderCondensed$DeviceTrustProviderType */ =>  {
                let var_5 =
                    Some(
                        Result::<crate::types::DeviceTrustProviderType, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::DeviceTrustProviderType::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_device_trust_provider_type(var_5);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}