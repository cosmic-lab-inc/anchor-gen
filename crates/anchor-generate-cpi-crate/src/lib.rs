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
//! More examples can be found in the [examples/](https://github.com/cosmic-lab-inc/anchor-gen/tree/master/examples)
//! directory.

use anchor_idl::GeneratorOptions;
use quote::quote;
use syn::{parse_macro_input, LitStr};

/// Generates an Anchor CPI crate from a JSON file.
///
/// # Arguments
///
/// * `input` - Path to a JSON IDL relative to the crate's the Cargo.toml.
///
/// # Examples
///
/// ```
/// anchor_generate_cpi_crate::generate_cpi_crate!("../../examples/govern-cpi/idl.json");
/// declare_id!("GjphYQcbP1m3FuDyCTUJf2mUMxKPE3j6feWU1rxvC7Ps");
/// # fn main() -> Result<()> {
/// let _my_governor = GovernanceParameters {
///     quorum_votes: 0,
///     timelock_delay_seconds: 0,
///     voting_period: 0,
///     voting_delay: 0,
/// };
/// #   Ok(())
/// # }
/// ```
#[proc_macro]
pub fn generate_cpi_crate(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let id_literal = parse_macro_input!(input as LitStr);
    let opts = GeneratorOptions {
        idl_path: id_literal.value(),
        ..Default::default()
    };

    let gen = opts.to_generator();
    let mut ts: proc_macro2::TokenStream = gen.generate_cpi_interface();

    // let event_variants = gen.event_types().into_iter().map(|ident| {
    //     let variant_name = ident.clone();
    //     quote! { #variant_name(#ident) }
    // });

    let event_variants = gen.instruction_types().into_iter().map(|ident| {
        let variant_name = ident.clone();

        // Construct the path prefix
        let path_prefix: syn::Path = syn::parse_str("events").unwrap();

        // Create a new PathSegment with the input Ident
        let mut segments = path_prefix.segments.clone();
        segments.push(syn::PathSegment::from(ident));

        // Combine the path prefix and the Ident
        let full_path = syn::Path {
            leading_colon: path_prefix.leading_colon,
            segments,
        };

        quote! { #variant_name(#full_path) }
    });
    
    if event_variants.len() > 0 {
        let event_ts = quote! {
            anchor_gen::derive_event_type!(
                pub enum EventType {
                    #(#event_variants,)*
                }
            );
        };
        ts.extend(event_ts);
    }

    let acct_variants = gen.account_types().into_iter().map(|ident| {
        let variant_name = ident.clone();
        quote! { #variant_name(#ident) }
    });
    if acct_variants.len() > 0 {
        let account_ts = quote! {
            anchor_gen::derive_account_type!(
                pub enum AccountType {
                    #(#acct_variants,)*
                }
            );
        };
        ts.extend(account_ts);
    }

    let ix_variants = gen.instruction_types().into_iter().map(|ident| {
        let variant_name = ident.clone();

        // Construct the path prefix
        let path_prefix: syn::Path = syn::parse_str("instruction").unwrap();

        // Create a new PathSegment with the input Ident
        let mut segments = path_prefix.segments.clone();
        segments.push(syn::PathSegment::from(ident));

        // Combine the path prefix and the Ident
        let full_path = syn::Path {
            leading_colon: path_prefix.leading_colon,
            segments,
        };

        quote! { #variant_name(#full_path) }
    });
    if ix_variants.len() > 0 {
        let ix_ts = quote! {
            anchor_gen::derive_instruction_type!(
                pub enum InstructionType {
                    #(#ix_variants,)*
                }
            );
        };
        ts.extend(ix_ts);
    }

    ts.into()
}
