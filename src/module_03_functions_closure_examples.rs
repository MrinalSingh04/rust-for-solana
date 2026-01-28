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

pub fn higher_order_function_example() {
    fn apply_fee<F>(f: F, gas_limit: i32) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(gas_limit)
    }

    fn calculate_fee(gas_limit: i32) -> i32 {
        gas_limit * 2
    }

    let total_fee = apply_fee(calculate_fee, 100);
    println!(
        "Total fee calculated using higher-order function: {}",
        total_fee
    );
}

pub fn basic_closure_exapmle() {
    let transfer = |token: i32, fee: i32| -> i32 { token - fee };
    let final_balance: i32 = transfer(100, 5);
    println!("Final balance after transfer: {}", final_balance);
}

pub fn demo() {
    basic_function();
    multiple_return_example();
    higher_order_function_example();
    basic_closure_exapmle();
}
