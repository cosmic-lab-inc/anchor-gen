pub fn get_type_name<'a, T: ?Sized + 'a>() -> String {
  let full_type_name = std::any::type_name::<T>();
  match full_type_name.rsplit_once("::") {
    Some((_path, type_name)) => type_name.to_string(),
    None => full_type_name.to_string(), // Handle cases without a path
  }
}

pub trait DecodeAccount: Sized {
  /// Deserialize a program account into its defined (struct) type using Borsh.
  /// utf8 discriminant is the human-readable discriminant, such as "User", and usually the name
  /// of the struct marked with the #[account] Anchor macro that derives the Discriminator trait.
  fn decode(utf8_discrim: &str, data: &[u8]) -> std::result::Result<Self, Box<dyn std::error::Error>>;
}

pub trait DecodeInstruction: Sized {
  /// Deserialize a program instruction into its defined (struct) type using Borsh.
  /// utf8 discriminant is the human-readable discriminant, such as "PlacePerpOrder", and usually the name
  /// of the struct marked with an Anchor macro that derives the Discriminator trait.
  fn decode(utf8_discrim: &str, data: &[u8]) -> std::result::Result<Self, Box<dyn std::error::Error>>;
}

#[macro_export]
macro_rules! decode_account {
    ($vis:vis enum $ident:ident {
        $($variant:ident ($account_type:ty)),*$(,)?
    }) => {
        #[repr(C)]
        #[derive(anchor_lang::prelude::AnchorDeserialize, anchor_lang::prelude::AnchorSerialize)]
        #[derive(Clone)]
        $vis enum $ident {
            $($variant($account_type),)*
        }

        impl $crate::DecodeAccount for $ident {
            fn decode(utf8_discrim: &str, data: &[u8]) -> std::result::Result<Self, Box<dyn std::error::Error>> {
                match utf8_discrim {
                    $(
                      $variant if utf8_discrim == $crate::get_type_name::<$account_type>() => {
                          let acct = <$account_type>::try_from_slice(&data[8..])?;
                          Ok(Self::$variant(acct.clone()))
                      },
                    )*
                    _ => Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Invalid account discriminant".to_string())))
                }
            }
        }
    };
}

#[macro_export]
macro_rules! decode_instruction {
    ($vis:vis enum $ident:ident {
        $($variant:ident($ix_type:path)),*$(,)?
    }) => {
        // #[derive(Clone)]
        #[derive(anchor_lang::prelude::AnchorSerialize, anchor_lang::prelude::AnchorDeserialize)]
        $vis enum $ident {
            $($variant($ix_type),)*
        }

        impl $crate::DecodeInstruction for $ident {
            fn decode(utf8_discrim: &str, data: &[u8]) -> std::result::Result<Self, Box<dyn std::error::Error>> {
                match utf8_discrim {
                    $(
                      $variant if utf8_discrim == $crate::get_type_name::<$ix_type>() => {
                          let ix = <$ix_type>::deserialize(&mut &data[8..])?;
                           Ok(Self::$variant(ix))
                      },
                    )*
                    _ => Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Invalid instruction discriminant".to_string())))
                }
            }
        }
    };
}