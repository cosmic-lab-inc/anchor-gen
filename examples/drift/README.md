```rust
anchor_gen::generate_cpi_crate("path/to/idl.json");
// I'm using this crate under the alias "drift_cpi" since there are many programs using this macro
// which all export the same InstructionType/AccountType enum

// data from a transaction instruction
let data: Vec<u8> = [1,2,3,4,4,5,1,3,4,1,3,4,2,4];
let ix: drift_cpi::InstructionType = drift_cpi::InstructionType::decode(&data[..])?;
match ix {
drift_cpi::InstructionType::PlacePerpOrder(ix) => {},
drift_cpi::InstructionType::CancelOrders(ix) => {},
drift_cpi::InstructionType::PlaceAndTakePerpOrder(ix) => {},
_ => { // 50 other ix types... }
};

let acct: solana_sdk::account::Account = rpc.get_account(&key).await?;
let data: Vec<u8> = acct.data;
let ix: drift_cpi::AccountType = drift_cpi::AccountType::decode(&data[..])?;
match ix {
drift_cpi::AccountType::User(user) => {},
drift_cpi::AccountType::PerpMarket(market) => {},
drift_cpi::AccountType::SpotMarket(market) => {},
_ => { // 10 other account types... }
};
```