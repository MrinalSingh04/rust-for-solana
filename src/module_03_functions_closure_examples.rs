pub fn basic_function() {
    fn add_gas_fee(base_fee: i32, gas_used: i32) -> i32 {
        base_fee + gas_used
    }
    let total_fee: i32 = add_gas_fee(100, 50);
    println!("Total fee including gas: {}", total_fee);
}

pub fn multiple_return_example() {
    fn calculate_transaction(x: i32, y: i32) -> (i32, i32, i32) {
        let total_tokens: i32 = x + y;
        let gas_fee: i32 = x + y;
        let balance_after_tx: i32 = x - y;
        (total_tokens, gas_fee, balance_after_tx)
    }
    let (tokens, gas_fee, balance_after_tx) = calculate_transaction(8, 3);
    println!(
        "Tokens: {}, Gas Fee: {}, Balance After Tx: {}",
        tokens, gas_fee, balance_after_tx
    );
}



pub fn demo() {
    basic_function();
    multiple_return_example();
}
