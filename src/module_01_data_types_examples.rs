pub fn primitive_data_types() {
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

pub fn arithmetic_operations() {
    let account_balance: i32 = 1000;
    let transaction_amount: i32 = 250;

    println!(
        "Account Balance:{}, Transaction Amount:{}",
        account_balance, transaction_amount
    );
    println!(
        "New Balance after transaction:{}",
        account_balance - transaction_amount
    ); // Subtraction
    println!(
        "Double transaction amount (for staking):{}",
        transaction_amount * 2
    ); // Multiplication for staking rewards
    println!(
        "Division of shared distribution: 1000 /4 = {}",
        account_balance / 4
    ); // example of distributing funds between 4 parties
    println!(
        "Reaminder when dividing transaction fee: 1000 % 3={}",
        account_balance % 3
    ); // Modulus for fee calculation

    let gas_price: f64 = 0.00000002;
    let gas_used: f64 = 21000.0;
    println!("Gas Price = {}, Gas Used = {}", gas_price, gas_used);
    println!("Total Gas Fee: {:.8}", gas_price * gas_used); // :.8 for precision 
}

pub fn logical_operations() {
    let is_staking: bool = true;
    let has_sufficient_balance: bool = false;

    println!(
        "Is staking = {}, Has Sufficient Balance = {}",
        is_staking, has_sufficient_balance
    );
    println!(
        "Can perform staking = {}",
        is_staking && has_sufficient_balance
    );
    println!(
        "Can either perform staking or withdraw = {}",
        is_staking || has_sufficient_balance
    );
    println!("Negative staking status: !is_staking = {}", !is_staking);
}

pub fn demo() {
    println!("\n");
    primitive_data_types();

    println!("\n");
    arithmetic_operations();

    println!("\n");
    logical_operations();
}
