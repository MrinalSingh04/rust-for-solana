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

pub fn match_example(day: u8) {
    let block_day = match day {
        1 => "Block production on Monday",
        2 => "Validation on Tuesday",
        3 => "Consensus meeting on Wednesday",
        4 => "Network upgrade on Thursday",
        5 => "Security audit on Friday",
        6 => "Community event on Saturday",
        7 => "Rest day on Sunday",
        _ => "Invalid day",
    };
    println!("The day is: {}", block_day);
}

pub fn while_loop_example() {
    let mut pending_transactions: i32 = 0;

    while pending_transactions < 5 {
        println!(
            "Processing transaction number: {}",
            pending_transactions + 1
        );
        pending_transactions += 1;
    }
}

pub fn for_loop_example() {
    let staking_rewards = [10, 20, 30, 40, 50];

    for reward in staking_rewards.iter() {
        println!("Staking reward received: {}", reward);
    }

    for block in 1..=5 {
        println!("Validating block number: {}", block);
    }
}

pub fn demo() {
    println!("\n");
    basic_if_else();

    print!("\n");
    match_example(4);

    print!("\n");
    while_loop_example();

    print!("\n");
    for_loop_example();
}
