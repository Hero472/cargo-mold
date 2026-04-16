use serde::{Serialize, Deserialize};
use strum::{VariantNames, EnumString, Display, AsRefStr};
use std::fmt::Debug;

pub trait Role: 
    Serialize + for<'de> Deserialize<'de> + 
    PartialEq + Clone + Send + Sync + Debug + 'static 
{
    fn as_str(&self) -> &str;
}

pub trait RequiredRole {
    const ROLE: &'static str;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display, AsRefStr, EnumString, VariantNames)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum DefaultRoles {
    Admin,
    User,
    Guest,
}

impl Role for DefaultRoles {
    fn as_str(&self) -> &str {
        self.as_ref() // Provided by AsRefStr
    }
}

pub struct Admin;
impl RequiredRole for Admin { 
    const ROLE: &'static str = <DefaultRoles as VariantNames>::VARIANTS[0]; 
}

pub struct User;
impl RequiredRole for User { 
    const ROLE: &'static str = <DefaultRoles as VariantNames>::VARIANTS[1]; 
}

pub struct Guest;
impl RequiredRole for Guest { 
    const ROLE: &'static str = <DefaultRoles as VariantNames>::VARIANTS[2]; 
}

#[macro_export]
macro_rules! ascii_lowercase {
    ($s:expr) => {{
        const INPUT: &str = $s;
        const OUTPUT: &str = {
            const fn lower_byte(b: u8) -> u8 {
                if b.is_ascii_uppercase() { b + 32 } else { b }
            }

            const fn convert<const N: usize>(input: &[u8]) -> [u8; N] {
                let mut out = [0; N];
                let mut i = 0;
                while i < N {
                    // SAFETY: The slice is guaranteed to have length N.
                    out[i] = lower_byte(unsafe { *input.as_ptr().add(i) });
                    i += 1;
                }
                out
            }

            // Store the array in an associated constant to give it 'static lifetime.
            struct Helper<const N: usize>;
            impl<const N: usize> Helper<N> {
                const ARRAY: [u8; N] = convert::<N>(INPUT.as_bytes());
            }

            // SAFETY: We only modified ASCII uppercase letters, so output is valid UTF-8.
            // The reference to Helper::ARRAY is truly 'static.
            unsafe { std::str::from_utf8_unchecked(&Helper::<{ INPUT.len() }>::ARRAY) }
        };
        OUTPUT
    }};
}

#[macro_export]
macro_rules! define_custom_roles {
    ($name:ident { $($variant:ident => $marker:ident),* $(,)? }) => {
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq,
            serde::Serialize, serde::Deserialize,
            strum::EnumString, strum::VariantNames, strum::Display, strum::AsRefStr
        )]
        #[serde(rename_all = "lowercase")]
        #[strum(serialize_all = "lowercase")]
        pub enum $name {
            $($variant),*
        }

        impl $crate::common::roles::Role for $name {
            fn as_str(&self) -> &str {
                self.as_ref()
            }
        }

        $(
            pub struct $marker;
            impl $crate::common::roles::RequiredRole for $marker {
                const ROLE: &'static str = $crate::ascii_lowercase!(
                    <$name as strum::VariantNames>::VARIANTS[$name::$variant as usize]
                );
            }
        )*
    };
}