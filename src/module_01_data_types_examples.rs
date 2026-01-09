fn primitive_data_types() {
    // Integer types
    let token_supply: u128 = 1_000_000_000_000;
    let block_number: i64 = -1234567890;

    println!("Token Supply (u128): {}", token_supply);
    println!("Block Number (i64): {}", block_number);

    // Floating-point types
    let token_price: f32 = 3.14;
    let transaction_fee: f64 = 0.000123456789;
    println!("Token Price (f32): {}", token_price);
    println!("Transaction Fee (f64): {}", transaction_fee);

    // Boolean type
    let is_transaction_valid: bool = true;
    println!("Is the transaction valid? {}", is_transaction_valid);

    // Character and string types
    let token_symbol: char = 'T';
    println!("Token Symbol (char): {}", token_symbol);

    let wallet_address: &str = "0xABCDEF1234567890";
    println!("Wallet Address (&str): {}", wallet_address);

    let contract_name: String = String::from("MySmartContract");
    println!("Contract Name (String): {}", contract_name);
}

pub fn demo() {
    println!("\n");
    primitive_data_types();
}
