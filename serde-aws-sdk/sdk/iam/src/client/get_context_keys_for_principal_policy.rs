// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetContextKeysForPrincipalPolicy`](crate::operation::get_context_keys_for_principal_policy::builders::GetContextKeysForPrincipalPolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`policy_source_arn(impl Into<String>)`](crate::operation::get_context_keys_for_principal_policy::builders::GetContextKeysForPrincipalPolicyFluentBuilder::policy_source_arn) / [`set_policy_source_arn(Option<String>)`](crate::operation::get_context_keys_for_principal_policy::builders::GetContextKeysForPrincipalPolicyFluentBuilder::set_policy_source_arn): <p>The ARN of a user, group, or role whose policies contain the context keys that you want listed. If you specify a user, the list includes context keys that are found in all policies that are attached to the user. The list also includes all groups that the user is a member of. If you pick a group or a role, then it includes only those context keys that are found in policies attached to that entity. Note that all parameters are shown in unencoded form here for clarity, but must be URL encoded to be included as a part of a real HTML request.</p>  <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>.</p>
    ///   - [`policy_input_list(Vec<String>)`](crate::operation::get_context_keys_for_principal_policy::builders::GetContextKeysForPrincipalPolicyFluentBuilder::policy_input_list) / [`set_policy_input_list(Option<Vec<String>>)`](crate::operation::get_context_keys_for_principal_policy::builders::GetContextKeysForPrincipalPolicyFluentBuilder::set_policy_input_list): <p>An optional list of additional policies for which you want the list of context keys that are referenced.</p>  <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> used to validate this parameter is a string of characters consisting of the following:</p>  <ul>   <li> <p>Any printable ASCII character ranging from the space character (<code>\u0020</code>) through the end of the ASCII character range</p> </li>   <li> <p>The printable characters in the Basic Latin and Latin-1 Supplement character set (through <code>\u00FF</code>)</p> </li>   <li> <p>The special characters tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and carriage return (<code>\u000D</code>)</p> </li>  </ul>
    /// - On success, responds with [`GetContextKeysForPrincipalPolicyOutput`](crate::operation::get_context_keys_for_principal_policy::GetContextKeysForPrincipalPolicyOutput) with field(s):
    ///   - [`context_key_names(Option<Vec<String>>)`](crate::operation::get_context_keys_for_principal_policy::GetContextKeysForPrincipalPolicyOutput::context_key_names): <p>The list of context keys that are referenced in the input policies.</p>
    /// - On failure, responds with [`SdkError<GetContextKeysForPrincipalPolicyError>`](crate::operation::get_context_keys_for_principal_policy::GetContextKeysForPrincipalPolicyError)
    pub fn get_context_keys_for_principal_policy(&self) -> crate::operation::get_context_keys_for_principal_policy::builders::GetContextKeysForPrincipalPolicyFluentBuilder{
        crate::operation::get_context_keys_for_principal_policy::builders::GetContextKeysForPrincipalPolicyFluentBuilder::new(self.handle.clone())
    }
}