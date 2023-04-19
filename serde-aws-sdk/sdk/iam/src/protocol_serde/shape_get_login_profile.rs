// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_login_profile_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::get_login_profile::GetLoginProfileOutput,
    crate::operation::get_login_profile::GetLoginProfileError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::get_login_profile::GetLoginProfileError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::operation::get_login_profile::GetLoginProfileError::unhandled(generic),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "NoSuchEntity" => {
            crate::operation::get_login_profile::GetLoginProfileError::NoSuchEntityException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::NoSuchEntityExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_entity_exception::de_no_such_entity_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::get_login_profile::GetLoginProfileError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ServiceFailure" => {
            crate::operation::get_login_profile::GetLoginProfileError::ServiceFailureException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ServiceFailureExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_failure_exception::de_service_failure_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::get_login_profile::GetLoginProfileError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::get_login_profile::GetLoginProfileError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_login_profile_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::get_login_profile::GetLoginProfileOutput,
    crate::operation::get_login_profile::GetLoginProfileError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::get_login_profile::builders::GetLoginProfileOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_get_login_profile::de_get_login_profile(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::operation::get_login_profile::GetLoginProfileError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_get_login_profile(
    inp: &[u8],
    mut builder: crate::operation::get_login_profile::builders::GetLoginProfileOutputBuilder,
) -> Result<
    crate::operation::get_login_profile::builders::GetLoginProfileOutputBuilder,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("GetLoginProfileResponse")) {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected GetLoginProfileResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("GetLoginProfileResult")) {
            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected GetLoginProfileResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("LoginProfile") /* LoginProfile com.amazonaws.iam.synthetic#GetLoginProfileOutput$LoginProfile */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_login_profile::de_login_profile(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_login_profile(var_1);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(
            "expected GetLoginProfileResult tag",
        ));
    };
    Ok(builder)
}