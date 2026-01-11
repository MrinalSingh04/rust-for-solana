pub fn basic_if_else() {
    let transaction_amount: i32 = 100;

    if transaction_amount > 0 {
        println!("Transaction is valid");
    } else if transaction_amount < 0 {
        println!("Invalid transaction: Negative amount.");
    } else {
        println!("Transaction amount is zero,no trasfer.");
    }
}

pub fn demo() {
    println!("\n");
    basic_if_else();
}
