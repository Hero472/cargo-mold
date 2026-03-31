use serde::{Serialize, Deserialize};
use strum::{VariantNames, EnumString, VariantArray, Display, AsRefStr};
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
macro_rules! define_custom_roles {
    ($name:ident { $($variant:ident => $marker:ident),* $(,)? }) => {
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, 
            serde::Serialize, serde::Deserialize, 
            strum::EnumString, strum::VariantNames, strum::Display
        )]
        #[serde(rename_all = "lowercase")]
        #[strum(serialize_all = "lowercase")]
        pub enum $name {
            $($variant),*
        }

        impl $crate::Role for $name {
            fn as_str(&self) -> &str {
                use strum::VariantNames;
                <$name as VariantNames>::VARIANTS[*self as usize]
            }
        }

        // We use a counter to map the Marker to the correct index in VARIANTS
        const _: () = {
            let mut i = 0;
            $(
                pub struct $marker;
                impl $crate::common::roles::RequiredRole for $marker {
                    // This pulls the lowercase string directly from strum's metadata
                    const ROLE: &'static str = <$name as strum::VariantNames>::VARIANTS[i];
                }
                i += 1;
            )*
        };
    };
}