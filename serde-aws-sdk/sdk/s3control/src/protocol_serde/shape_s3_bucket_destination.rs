// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_s3_bucket_destination(
    input: &crate::types::S3BucketDestination,
    writer: aws_smithy_xml::encode::ElWriter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.format {
        let mut inner_writer = scope.start_el("Format").finish();
        inner_writer.data(var_1.as_str());
    }
    if let Some(var_2) = &input.output_schema_version {
        let mut inner_writer = scope.start_el("OutputSchemaVersion").finish();
        inner_writer.data(var_2.as_str());
    }
    if let Some(var_3) = &input.account_id {
        let mut inner_writer = scope.start_el("AccountId").finish();
        inner_writer.data(var_3.as_str());
    }
    if let Some(var_4) = &input.arn {
        let mut inner_writer = scope.start_el("Arn").finish();
        inner_writer.data(var_4.as_str());
    }
    if let Some(var_5) = &input.prefix {
        let mut inner_writer = scope.start_el("Prefix").finish();
        inner_writer.data(var_5.as_str());
    }
    if let Some(var_6) = &input.encryption {
        let inner_writer = scope.start_el("Encryption");
        crate::protocol_serde::shape_storage_lens_data_export_encryption::ser_storage_lens_data_export_encryption(var_6, inner_writer)?
    }
    scope.finish();
    Ok(())
}

pub fn de_s3_bucket_destination(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::S3BucketDestination, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::S3BucketDestination::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Format") /* Format com.amazonaws.s3control#S3BucketDestination$Format */ =>  {
                let var_7 =
                    Some(
                        Result::<crate::types::Format, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::Format::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_format(var_7);
            }
            ,
            s if s.matches("OutputSchemaVersion") /* OutputSchemaVersion com.amazonaws.s3control#S3BucketDestination$OutputSchemaVersion */ =>  {
                let var_8 =
                    Some(
                        Result::<crate::types::OutputSchemaVersion, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::OutputSchemaVersion::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_output_schema_version(var_8);
            }
            ,
            s if s.matches("AccountId") /* AccountId com.amazonaws.s3control#S3BucketDestination$AccountId */ =>  {
                let var_9 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_account_id(var_9);
            }
            ,
            s if s.matches("Arn") /* Arn com.amazonaws.s3control#S3BucketDestination$Arn */ =>  {
                let var_10 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_arn(var_10);
            }
            ,
            s if s.matches("Prefix") /* Prefix com.amazonaws.s3control#S3BucketDestination$Prefix */ =>  {
                let var_11 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_prefix(var_11);
            }
            ,
            s if s.matches("Encryption") /* Encryption com.amazonaws.s3control#S3BucketDestination$Encryption */ =>  {
                let var_12 =
                    Some(
                        crate::protocol_serde::shape_storage_lens_data_export_encryption::de_storage_lens_data_export_encryption(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_encryption(var_12);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}