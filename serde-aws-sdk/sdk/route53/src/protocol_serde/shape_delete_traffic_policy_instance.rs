// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_traffic_policy_instance_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::delete_traffic_policy_instance::DeleteTrafficPolicyInstanceOutput,
    crate::operation::delete_traffic_policy_instance::DeleteTrafficPolicyInstanceError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::delete_traffic_policy_instance::DeleteTrafficPolicyInstanceError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::delete_traffic_policy_instance::DeleteTrafficPolicyInstanceError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidInput" => crate::operation::delete_traffic_policy_instance::DeleteTrafficPolicyInstanceError::InvalidInput({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidInputBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_input::de_invalid_input_xml_err(response.body().as_ref(), output).map_err(crate::operation::delete_traffic_policy_instance::DeleteTrafficPolicyInstanceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoSuchTrafficPolicyInstance" => crate::operation::delete_traffic_policy_instance::DeleteTrafficPolicyInstanceError::NoSuchTrafficPolicyInstance({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NoSuchTrafficPolicyInstanceBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_traffic_policy_instance::de_no_such_traffic_policy_instance_xml_err(response.body().as_ref(), output).map_err(crate::operation::delete_traffic_policy_instance::DeleteTrafficPolicyInstanceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "PriorRequestNotComplete" => crate::operation::delete_traffic_policy_instance::DeleteTrafficPolicyInstanceError::PriorRequestNotComplete({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::PriorRequestNotCompleteBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_prior_request_not_complete::de_prior_request_not_complete_xml_err(response.body().as_ref(), output).map_err(crate::operation::delete_traffic_policy_instance::DeleteTrafficPolicyInstanceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::delete_traffic_policy_instance::DeleteTrafficPolicyInstanceError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_traffic_policy_instance_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::delete_traffic_policy_instance::DeleteTrafficPolicyInstanceOutput,
    crate::operation::delete_traffic_policy_instance::DeleteTrafficPolicyInstanceError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_traffic_policy_instance::builders::DeleteTrafficPolicyInstanceOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
