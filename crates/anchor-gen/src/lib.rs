//! Generates a crate for cross-program invocations to an Anchor program from a JSON IDL.
//!
//! # Usage
//!
//! In a new crate, write:
//!
//! ```skip
//! anchor_gen::generate_cpi_crate!("../../examples/govern-cpi/idl.json");
//!
//! declare_id!("GjphYQcbP1m3FuDyCTUJf2mUMxKPE3j6feWU1rxvC7Ps");
//! ```
//!
//! This will generate a fully functional Rust CPI client for your IDL.
//!
//! More examples can be found in the [examples/](https://github.com/cosmic-lab-inc/anchor-gen/tree/master/examples) directory.

extern crate self as anchor_gen;

pub use anchor_generate_cpi_crate::generate_cpi_crate;
pub use anchor_generate_cpi_interface::generate_cpi_interface;
pub use anchor_idl::derive_account_type;
pub use anchor_idl::derive_event_type;
pub use anchor_idl::derive_instruction_type;
pub use anchor_idl::Decode;
pub use anchor_idl::DiscrimToName;
pub use anchor_idl::NameToDiscrim;

pub mod prelude {
    pub use anchor_generate_cpi_crate::generate_cpi_crate;
    pub use anchor_generate_cpi_interface::generate_cpi_interface;
    // pub use anchor_idl::ident_name;
    pub use anchor_idl::derive_account_type;
    pub use anchor_idl::derive_event_type;
    pub use anchor_idl::derive_instruction_type;
    pub use anchor_idl::Decode;
    pub use anchor_idl::DiscrimToName;
    pub use anchor_idl::NameToDiscrim;
}
