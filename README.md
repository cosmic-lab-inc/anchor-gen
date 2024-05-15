# anchor-gen

Generates a crate for cross-program invocations to an Anchor program from a JSON IDL.

## Usage

In a new crate, write:

```rust
anchor_gen::generate_cpi_crate!("../../examples/govern-cpi/idl.json");
declare_id!("GjphYQcbP1m3FuDyCTUJf2mUMxKPE3j6feWU1rxvC7Ps");
```

This will generate a fully functional Rust CPI client for your IDL.

More examples can be found in the [examples/](https://github.com/saber-hq/anchor-gen/tree/master/examples) directory.

License: Apache-2.0

## Example
```rust
use solana_sdk::bs58;
use solana_transaction_status::{UiInstruction, UiMessage, UiParsedInstruction};
use anchor_gen::prelude::*;

generate_cpi_crate!("idl.json");
declare_id!("dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH");


#[test]
fn accounts() -> anyhow::Result<()>  {
  let rpc = solana_client::rpc_client::RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
  let key = solana_sdk::pubkey!("H5jfagEnMVNH3PMc2TU2F7tNuXE6b4zCwoL5ip1b4ZHi");
  let acct = rpc.get_account(&key)?;
  let data: Vec<u8> = acct.data;
  let ix: AccountType = AccountType::decode(&data[..]).map_err(
    |e| anyhow::anyhow!("Failed to decode account: {:?}", e)
  )?;
  match ix {
    AccountType::User(user) => println!("{}", user.settled_perp_pnl),
    AccountType::PerpMarket(market) => println!("{}", market.amm.oracle),
    AccountType::SpotMarket(market) => println!("{}", market.flash_loan_amount),
    _ => {}
  }
  Ok(())
}

#[test]
fn instructions() -> anyhow::Result<()>  {
  use std::str::FromStr;
  use solana_transaction_status::UiTransactionEncoding;
  use solana_transaction_status::EncodedTransaction;

  let rpc = solana_client::rpc_client::RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
  let key = solana_sdk::pubkey!("H5jfagEnMVNH3PMc2TU2F7tNuXE6b4zCwoL5ip1b4ZHi");
  let results = rpc.get_signatures_for_address(&key)?;
  if !results.is_empty() {
    let result = results[0].clone();
    let signature = solana_sdk::signature::Signature::from_str(&result.signature)?;
    let tx = rpc.get_transaction(&signature, UiTransactionEncoding::JsonParsed)?;
    if let EncodedTransaction::Json(tx) = tx.transaction.transaction {
      if let UiMessage::Parsed(msg) = tx.message {
        for ui_ix in msg.instructions {
          if let UiInstruction::Parsed(ui_parsed_ix) = ui_ix {
            match ui_parsed_ix {
              UiParsedInstruction::Parsed(parsed_ix) => {
                println!("parsed ix for program \"{}\": {:#?}", parsed_ix.program, parsed_ix.parsed)
              }
              UiParsedInstruction::PartiallyDecoded(ui_decoded_ix) => {
                let data: Vec<u8> = bs58::decode(ui_decoded_ix.data.clone()).into_vec()?;
                // only match instruction if it belongs to the IDL that generated this crate (the Drift program)
                if data.len() >= 8 && ui_decoded_ix.program_id == id().to_string() {
                  if let Ok(discrim) = data[..8].try_into() {
                    let ix = InstructionType::decode(&data[..]).map_err(
                      |e| anyhow::anyhow!("Failed to decode instruction: {:?}", e)
                    )?;
                    let name = InstructionType::discrim_to_name(discrim).unwrap();
                    match ix {
                      InstructionType::PlacePerpOrder(ix) => {
                        println!("{}, {:#?}", name, ix._params);
                      }
                      InstructionType::PlaceAndTakePerpOrder(ix) => {
                        println!("{}, {:#?}", name, ix._params);
                      }
                      InstructionType::PlaceOrders(ix) => {
                        for params in ix._params {
                          println!("{}, {:#?}", name, params);
                        }
                      }
                      _ => {}
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
  Ok(())
}
```