// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `IpamState`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let ipamstate = unimplemented!();
/// match ipamstate {
///     IpamState::CreateComplete => { /* ... */ },
///     IpamState::CreateFailed => { /* ... */ },
///     IpamState::CreateInProgress => { /* ... */ },
///     IpamState::DeleteComplete => { /* ... */ },
///     IpamState::DeleteFailed => { /* ... */ },
///     IpamState::DeleteInProgress => { /* ... */ },
///     IpamState::IsolateComplete => { /* ... */ },
///     IpamState::IsolateInProgress => { /* ... */ },
///     IpamState::ModifyComplete => { /* ... */ },
///     IpamState::ModifyFailed => { /* ... */ },
///     IpamState::ModifyInProgress => { /* ... */ },
///     IpamState::RestoreInProgress => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `ipamstate` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `IpamState::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `IpamState::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `IpamState::NewFeature` is defined.
/// Specifically, when `ipamstate` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `IpamState::NewFeature` also yielding `"NewFeature"`.
///
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
#[allow(missing_docs)] // documentation missing in model
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum IpamState {
    #[allow(missing_docs)] // documentation missing in model
    CreateComplete,
    #[allow(missing_docs)] // documentation missing in model
    CreateFailed,
    #[allow(missing_docs)] // documentation missing in model
    CreateInProgress,
    #[allow(missing_docs)] // documentation missing in model
    DeleteComplete,
    #[allow(missing_docs)] // documentation missing in model
    DeleteFailed,
    #[allow(missing_docs)] // documentation missing in model
    DeleteInProgress,
    #[allow(missing_docs)] // documentation missing in model
    IsolateComplete,
    #[allow(missing_docs)] // documentation missing in model
    IsolateInProgress,
    #[allow(missing_docs)] // documentation missing in model
    ModifyComplete,
    #[allow(missing_docs)] // documentation missing in model
    ModifyFailed,
    #[allow(missing_docs)] // documentation missing in model
    ModifyInProgress,
    #[allow(missing_docs)] // documentation missing in model
    RestoreInProgress,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::primitives::UnknownVariantValue),
}
impl std::convert::From<&str> for IpamState {
    fn from(s: &str) -> Self {
        match s {
            "create-complete" => IpamState::CreateComplete,
            "create-failed" => IpamState::CreateFailed,
            "create-in-progress" => IpamState::CreateInProgress,
            "delete-complete" => IpamState::DeleteComplete,
            "delete-failed" => IpamState::DeleteFailed,
            "delete-in-progress" => IpamState::DeleteInProgress,
            "isolate-complete" => IpamState::IsolateComplete,
            "isolate-in-progress" => IpamState::IsolateInProgress,
            "modify-complete" => IpamState::ModifyComplete,
            "modify-failed" => IpamState::ModifyFailed,
            "modify-in-progress" => IpamState::ModifyInProgress,
            "restore-in-progress" => IpamState::RestoreInProgress,
            other => IpamState::Unknown(crate::primitives::UnknownVariantValue(other.to_owned())),
        }
    }
}
impl std::str::FromStr for IpamState {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(IpamState::from(s))
    }
}
impl IpamState {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            IpamState::CreateComplete => "create-complete",
            IpamState::CreateFailed => "create-failed",
            IpamState::CreateInProgress => "create-in-progress",
            IpamState::DeleteComplete => "delete-complete",
            IpamState::DeleteFailed => "delete-failed",
            IpamState::DeleteInProgress => "delete-in-progress",
            IpamState::IsolateComplete => "isolate-complete",
            IpamState::IsolateInProgress => "isolate-in-progress",
            IpamState::ModifyComplete => "modify-complete",
            IpamState::ModifyFailed => "modify-failed",
            IpamState::ModifyInProgress => "modify-in-progress",
            IpamState::RestoreInProgress => "restore-in-progress",
            IpamState::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &[
            "create-complete",
            "create-failed",
            "create-in-progress",
            "delete-complete",
            "delete-failed",
            "delete-in-progress",
            "isolate-complete",
            "isolate-in-progress",
            "modify-complete",
            "modify-failed",
            "modify-in-progress",
            "restore-in-progress",
        ]
    }
}
impl AsRef<str> for IpamState {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}