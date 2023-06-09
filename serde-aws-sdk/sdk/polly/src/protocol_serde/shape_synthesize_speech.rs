// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_synthesize_speech_input(
    input: &crate::operation::synthesize_speech::SynthesizeSpeechInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_synthesize_speech_input::ser_synthesize_speech_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_synthesize_speech_http_response(
    op_response: &mut aws_smithy_http::operation::Response,
) -> std::result::Result<
    crate::operation::synthesize_speech::SynthesizeSpeechOutput,
    crate::operation::synthesize_speech::SynthesizeSpeechError,
> {
    #[allow(unused_variables)]
    let (response, properties) = op_response.parts_mut();
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::synthesize_speech::builders::SynthesizeSpeechOutputBuilder::default();
        let _ = response;
        output = output.set_audio_stream(Some(
            crate::protocol_serde::shape_synthesize_speech_output::de_audio_stream_payload(
                response.body_mut(),
            )?,
        ));
        output = output.set_content_type(
            crate::protocol_serde::shape_synthesize_speech_output::de_content_type_header(
                response.headers(),
            )
            .map_err(|_| {
                crate::operation::synthesize_speech::SynthesizeSpeechError::unhandled(
                    "Failed to parse ContentType from header `Content-Type",
                )
            })?,
        );
        output = output.set_request_characters(
            crate::protocol_serde::shape_synthesize_speech_output::de_request_characters_header(
                response.headers(),
            )
            .map_err(|_| {
                crate::operation::synthesize_speech::SynthesizeSpeechError::unhandled(
                    "Failed to parse RequestCharacters from header `x-amzn-RequestCharacters",
                )
            })?,
        );
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_synthesize_speech_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::synthesize_speech::SynthesizeSpeechOutput,
    crate::operation::synthesize_speech::SynthesizeSpeechError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::synthesize_speech::SynthesizeSpeechError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::operation::synthesize_speech::SynthesizeSpeechError::unhandled(generic),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "EngineNotSupportedException" => crate::operation::synthesize_speech::SynthesizeSpeechError::EngineNotSupportedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::EngineNotSupportedExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_engine_not_supported_exception::de_engine_not_supported_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::synthesize_speech::SynthesizeSpeechError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidSampleRateException" => crate::operation::synthesize_speech::SynthesizeSpeechError::InvalidSampleRateException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidSampleRateExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_sample_rate_exception::de_invalid_sample_rate_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::synthesize_speech::SynthesizeSpeechError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidSsmlException" => crate::operation::synthesize_speech::SynthesizeSpeechError::InvalidSsmlException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidSsmlExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_ssml_exception::de_invalid_ssml_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::synthesize_speech::SynthesizeSpeechError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "LanguageNotSupportedException" => crate::operation::synthesize_speech::SynthesizeSpeechError::LanguageNotSupportedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::LanguageNotSupportedExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_language_not_supported_exception::de_language_not_supported_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::synthesize_speech::SynthesizeSpeechError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "LexiconNotFoundException" => crate::operation::synthesize_speech::SynthesizeSpeechError::LexiconNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::LexiconNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_lexicon_not_found_exception::de_lexicon_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::synthesize_speech::SynthesizeSpeechError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "MarksNotSupportedForFormatException" => crate::operation::synthesize_speech::SynthesizeSpeechError::MarksNotSupportedForFormatException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::MarksNotSupportedForFormatExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_marks_not_supported_for_format_exception::de_marks_not_supported_for_format_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::synthesize_speech::SynthesizeSpeechError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceFailureException" => crate::operation::synthesize_speech::SynthesizeSpeechError::ServiceFailureException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ServiceFailureExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_failure_exception::de_service_failure_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::synthesize_speech::SynthesizeSpeechError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "SsmlMarksNotSupportedForTextTypeException" => crate::operation::synthesize_speech::SynthesizeSpeechError::SsmlMarksNotSupportedForTextTypeException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::SsmlMarksNotSupportedForTextTypeExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_ssml_marks_not_supported_for_text_type_exception::de_ssml_marks_not_supported_for_text_type_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::synthesize_speech::SynthesizeSpeechError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TextLengthExceededException" => crate::operation::synthesize_speech::SynthesizeSpeechError::TextLengthExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::TextLengthExceededExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_text_length_exceeded_exception::de_text_length_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::synthesize_speech::SynthesizeSpeechError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::synthesize_speech::SynthesizeSpeechError::generic(generic)
    })
}
