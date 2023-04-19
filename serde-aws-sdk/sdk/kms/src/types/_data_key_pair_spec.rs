// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `DataKeyPairSpec`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let datakeypairspec = unimplemented!();
/// match datakeypairspec {
///     DataKeyPairSpec::EccNistP256 => { /* ... */ },
///     DataKeyPairSpec::EccNistP384 => { /* ... */ },
///     DataKeyPairSpec::EccNistP521 => { /* ... */ },
///     DataKeyPairSpec::EccSecgP256K1 => { /* ... */ },
///     DataKeyPairSpec::Rsa2048 => { /* ... */ },
///     DataKeyPairSpec::Rsa3072 => { /* ... */ },
///     DataKeyPairSpec::Rsa4096 => { /* ... */ },
///     DataKeyPairSpec::Sm2 => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `datakeypairspec` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `DataKeyPairSpec::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `DataKeyPairSpec::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `DataKeyPairSpec::NewFeature` is defined.
/// Specifically, when `datakeypairspec` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `DataKeyPairSpec::NewFeature` also yielding `"NewFeature"`.
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
pub enum DataKeyPairSpec {
    #[allow(missing_docs)] // documentation missing in model
    EccNistP256,
    #[allow(missing_docs)] // documentation missing in model
    EccNistP384,
    #[allow(missing_docs)] // documentation missing in model
    EccNistP521,
    #[allow(missing_docs)] // documentation missing in model
    EccSecgP256K1,
    #[allow(missing_docs)] // documentation missing in model
    Rsa2048,
    #[allow(missing_docs)] // documentation missing in model
    Rsa3072,
    #[allow(missing_docs)] // documentation missing in model
    Rsa4096,
    #[allow(missing_docs)] // documentation missing in model
    Sm2,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::primitives::UnknownVariantValue),
}
impl std::convert::From<&str> for DataKeyPairSpec {
    fn from(s: &str) -> Self {
        match s {
            "ECC_NIST_P256" => DataKeyPairSpec::EccNistP256,
            "ECC_NIST_P384" => DataKeyPairSpec::EccNistP384,
            "ECC_NIST_P521" => DataKeyPairSpec::EccNistP521,
            "ECC_SECG_P256K1" => DataKeyPairSpec::EccSecgP256K1,
            "RSA_2048" => DataKeyPairSpec::Rsa2048,
            "RSA_3072" => DataKeyPairSpec::Rsa3072,
            "RSA_4096" => DataKeyPairSpec::Rsa4096,
            "SM2" => DataKeyPairSpec::Sm2,
            other => {
                DataKeyPairSpec::Unknown(crate::primitives::UnknownVariantValue(other.to_owned()))
            }
        }
    }
}
impl std::str::FromStr for DataKeyPairSpec {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(DataKeyPairSpec::from(s))
    }
}
impl DataKeyPairSpec {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            DataKeyPairSpec::EccNistP256 => "ECC_NIST_P256",
            DataKeyPairSpec::EccNistP384 => "ECC_NIST_P384",
            DataKeyPairSpec::EccNistP521 => "ECC_NIST_P521",
            DataKeyPairSpec::EccSecgP256K1 => "ECC_SECG_P256K1",
            DataKeyPairSpec::Rsa2048 => "RSA_2048",
            DataKeyPairSpec::Rsa3072 => "RSA_3072",
            DataKeyPairSpec::Rsa4096 => "RSA_4096",
            DataKeyPairSpec::Sm2 => "SM2",
            DataKeyPairSpec::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &[
            "ECC_NIST_P256",
            "ECC_NIST_P384",
            "ECC_NIST_P521",
            "ECC_SECG_P256K1",
            "RSA_2048",
            "RSA_3072",
            "RSA_4096",
            "SM2",
        ]
    }
}
impl AsRef<str> for DataKeyPairSpec {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}