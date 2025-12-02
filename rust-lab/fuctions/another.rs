use std::vec;

pub fn another_function() {
    let mut number = 1;

    let result = loop {
        number += 1;
        if number == 10 {
            break number * 2;
        }
    };

    println!("{}", result);
}

pub fn nested_loop() {
    let mut count = 0;

    'counting_up: loop {
        println!("count == {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            };
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

pub fn while_loop() {
    let mut number = 10;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }
}

pub fn for_loop() {
    // let a = [123,45678,234567,23456,34567,323456,7,56723456]; // array

    // for element in a.iter() {
    //     println!("the value is: {element}");
    // }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
