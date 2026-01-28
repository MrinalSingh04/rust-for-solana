// Functions

// pub fn add(x: u8, y: u8) {
//     let sum: u8 = x + y;
//     println!("The sum of {} and {} is {}", x, y, sum);
// }

// pub fn add() {
//     let num1: u8 = 10;
//     let num2: u8 = 20;
//     let sum: u8 = num1 + num2;
//     println!("The sum of {} and {} is {}", num1, num2, sum);
// }

// pub fn add(num1: u8, num2: u8) -> u8 {
//     return num1 + num2;
// }

//Ownership --> for memory management

pub fn ownership_example() {
    // let a = 5;
    // let b = a; //copy
    // println!("a: {}, b: {}", a, b);

    // let str1 = String::from("Hello");
    // let str2 = str1; //move
    // println!("str1: {}, str2: {}", str1, str2); //str1 is no longer valid
}

pub fn demo() {
    // add(5,7);
    // add();

    // let result = add(15, 25);
    // println!("The sum is {}", result);

    ownership_example();
}
