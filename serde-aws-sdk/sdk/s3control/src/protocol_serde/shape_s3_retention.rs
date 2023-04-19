// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_s3_retention(
    input: &crate::types::S3Retention,
    writer: aws_smithy_xml::encode::ElWriter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.retain_until_date {
        let mut inner_writer = scope.start_el("RetainUntilDate").finish();
        inner_writer.data(
            var_1
                .fmt(aws_smithy_types::date_time::Format::DateTimeWithOffset)?
                .as_ref(),
        );
    }
    if let Some(var_2) = &input.mode {
        let mut inner_writer = scope.start_el("Mode").finish();
        inner_writer.data(var_2.as_str());
    }
    scope.finish();
    Ok(())
}

pub fn de_s3_retention(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::S3Retention, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::S3Retention::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("RetainUntilDate") /* RetainUntilDate com.amazonaws.s3control#S3Retention$RetainUntilDate */ =>  {
                let var_3 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.s3control#TimeStamp`)"))
                        ?
                    )
                ;
                builder = builder.set_retain_until_date(var_3);
            }
            ,
            s if s.matches("Mode") /* Mode com.amazonaws.s3control#S3Retention$Mode */ =>  {
                let var_4 =
                    Some(
                        Result::<crate::types::S3ObjectLockRetentionMode, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::S3ObjectLockRetentionMode::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_mode(var_4);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}