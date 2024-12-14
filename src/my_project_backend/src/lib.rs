use candid::Nat;

// #[ic_cdk::query]
// fn greet(name: String) -> String {
//     format!("Hello, {}!", name)
// }

#[ic_cdk::query]
fn calculate_currency_price(currency_a: Nat, currency_b: Nat) -> Nat {
    currency_a * currency_b
}