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

pub fn infinite_loop_example() {
    let mut attempts = 0;

    loop {
        println!("Checking blockchain state... attempt: {}", attempts + 1);
        attempts += 1;

        if attempts == 3 {
            println!("Maximum attempts reached. Exiting loop.");
            break;
        }
    }
}

pub fn match_pattern_example(number: i32) {
    match number {
        1 => println!("Executing token transfer"),
        2 | 3 | 4 | 7 => println!("Executing a prime validator operation"),
        10..=19 => println!("Performing governance action between blocks 10 and 19"),
        _ => println!("No matching operation found"),
    }
}

pub fn let_if_example(reputation_score: i32) {
    if reputation_score >= 90 {
        println!("Reputation is Excellent");
    } else if reputation_score >= 75 {
        println!("Reputation is Good");
    } else if reputation_score >= 50 {
        println!("Reputation is Average");
    } else {
        println!("Reputation is Poor");
    }
}

pub fn match_return_example(status_code: i32) -> &'static str {
    match status_code {
        200 => "Transaction Successful",
        404 => "Transaction Not Found",
        500 => "Internal Server Error",
        _ => "Unknown Status Code",
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

    print!("\n");
    infinite_loop_example();

    print!("\n");
    match_pattern_example(13);

    print!("\n");
    let_if_example(82);

    print!("\n");
    let status_message: &str = match_return_example(200);
    println!("Status Message: {}", status_message);
}
