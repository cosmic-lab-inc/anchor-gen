use anchor_lang::solana_program::hash::hash;
use heck::{ToPascalCase, ToSnakeCase};

pub fn ident_name<'a, T: ?Sized + 'a>() -> String {
  let full_ident_name = std::any::type_name::<T>();
  match full_ident_name.rsplit_once("::") {
    Some((_path, ident_name)) => ident_name.to_string(),
    None => full_ident_name.to_string(), // Handle cases without a path
  }
}

pub trait Decode: Sized {
  /// Deserialize a program account into its defined (struct) type using Borsh.
  /// utf8 discriminator is the human-readable discriminator, such as "User", and usually the name
  /// of the struct marked with the #[account] Anchor macro that derives the Discriminator trait.
  fn decode(data: &[u8]) -> std::result::Result<Self, Box<dyn std::error::Error>>;
}

pub trait NameToDiscrim: Sized {
  /// Deserialize a program account into its defined (struct) type using Borsh.
  /// utf8 discriminator is the human-readable discriminator, such as "User", and usually the name
  /// of the struct marked with the #[account] Anchor macro that derives the Discriminator trait.
  fn name_to_discrim(name: &str) -> std::result::Result<[u8; 8], Box<dyn std::error::Error>>;
}

pub trait DiscrimToName: Sized {
  /// Deserialize a program account into its defined (struct) type using Borsh.
  /// utf8 discriminator is the human-readable discriminator, such as "User", and usually the name
  /// of the struct marked with the #[account] Anchor macro that derives the Discriminator trait.
  fn discrim_to_name(discrim: [u8; 8]) -> std::result::Result<String, Box<dyn std::error::Error>>;
}

/// Derives the account discriminator from the account name as Anchor does.
/// Accounts are PascalCase.
pub fn account_discriminator(name: &str) -> [u8; 8] {
  let name = name.to_pascal_case();
  let mut discriminator = [0u8; 8];
  let hashed = hash(format!("account:{}", name).as_bytes()).to_bytes();
  discriminator.copy_from_slice(&hashed[..8]);
  discriminator
}

/// Derives the instruction discriminator from the instruction name as Anchor does.
/// Instructions are snake_case.
pub fn instruction_discriminator(name: &str) -> [u8; 8] {
  let name = name.to_snake_case();
  let mut discriminator = [0u8; 8];
  let hashed = hash(format!("global:{}", name).as_bytes()).to_bytes();
  discriminator.copy_from_slice(&hashed[..8]);
  discriminator
}

#[macro_export]
macro_rules! derive_account_type {
    ($vis:vis enum $ident:ident {
        $($variant:ident ($account_type:ty)),*$(,)?
    }) => {
        #[repr(C)]
        #[derive(Clone)]
        #[derive(anchor_lang::prelude::AnchorDeserialize, anchor_lang::prelude::AnchorSerialize)]
        $vis enum $ident {
            $($variant($account_type),)*
        }

        impl $crate::Decode for $ident {
          fn decode(data: &[u8]) -> std::result::Result<Self, Box<dyn std::error::Error>> {
            let discrim: &[u8; 8] = data[..8].try_into().map_err(|e| {
              Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Instruction data is not 8 bytes or more".to_string()))
            })?;
            match discrim {
              $(
                $variant if discrim == &$crate::account_discriminator(&$crate::ident_name::<$account_type>()) => {
                    let acct = <$account_type>::try_from_slice(&data[8..])?;
                    Ok(Self::$variant(acct.clone()))
                },
              )*
              _ => Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Invalid account discriminator".to_string())))
            }
          }
        }

        impl $crate::NameToDiscrim for $ident {
            fn name_to_discrim(name: &str) -> std::result::Result<[u8; 8], Box<dyn std::error::Error>> {
                match name {
                    $(
                      $variant if name == $crate::ident_name::<$account_type>() => {
                          let discrim = $crate::account_discriminator(&$crate::ident_name::<$account_type>());
                          Ok(discrim)
                      },
                    )*
                    _ => Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Invalid account name".to_string())))
                }
            }
        }

        impl $crate::DiscrimToName for $ident {
            fn discrim_to_name(discrim: [u8; 8]) -> std::result::Result<String, Box<dyn std::error::Error>> {
                match discrim {
                    $(
                      $variant if discrim == $crate::account_discriminator(&$crate::ident_name::<$account_type>()) => {
                          let name = $crate::ident_name::<$account_type>();
                          Ok(name)
                      },
                    )*
                    _ => Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Invalid account discriminator".to_string())))
                }
            }
        }
    };
}

#[macro_export]
macro_rules! derive_instruction_type {
    ($vis:vis enum $ident:ident {
        $($variant:ident($ix_type:path)),*$(,)?
    }) => {
        // #[derive(Clone)]
        #[derive(anchor_lang::prelude::AnchorSerialize, anchor_lang::prelude::AnchorDeserialize)]
        $vis enum $ident {
            $($variant($ix_type),)*
        }

        impl $crate::Decode for $ident {
          fn decode(data: &[u8]) -> std::result::Result<Self, Box<dyn std::error::Error>> {
            let discrim: &[u8; 8] = data[..8].try_into().map_err(|e| {
              Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Instruction data is not 8 bytes or more".to_string()))
            })?;
            match discrim {
                $(
                  $variant if discrim == &$crate::instruction_discriminator(&$crate::ident_name::<$ix_type>()) => {
                      let ix = <$ix_type>::deserialize(&mut &data[8..])?;
                       Ok(Self::$variant(ix))
                  },
                )*
                _ => Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Invalid instruction discriminator".to_string())))
            }
          }
        }

        impl $crate::NameToDiscrim for $ident {
            fn name_to_discrim(name: &str) -> std::result::Result<[u8; 8], Box<dyn std::error::Error>> {
                match name {
                    $(
                      $variant if name == $crate::ident_name::<$ix_type>() => {
                          let discrim = $crate::instruction_discriminator(&$crate::ident_name::<$ix_type>());
                          Ok(discrim)
                      },
                    )*
                    _ => Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Invalid instruction name".to_string())))
                }
            }
        }

        impl $crate::DiscrimToName for $ident {
            fn discrim_to_name(discrim: [u8; 8]) -> std::result::Result<String, Box<dyn std::error::Error>> {
                match discrim {
                    $(
                      $variant if discrim == $crate::instruction_discriminator(&$crate::ident_name::<$ix_type>()) => {
                          let name = $crate::ident_name::<$ix_type>();
                          Ok(name)
                      },
                    )*
                    _ => Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Invalid instruction discriminator".to_string())))
                }
            }
        }
    };
}