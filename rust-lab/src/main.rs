include!("../fuctions/another.rs");

// static NAME: &str = "Rust";
// const AGE: i8 = 20;

// #[derive(PartialEq)]
// enum Status {
//     Active,
//     InActive,
// }

mod lifetimes;
mod owerenship_borrwing;


fn main() {
    // let something: String = String::from("Rust");
    // let name = String::from("Rust");
    // let mut number: i64 = 10;

    // number = 20;

    // let names2 = Vec::from("Rust");
    // let names = vec!["Rust", "Go", "Python"];

    // for name in names.iter() {
    //     println!("Name: {}", name);
    // }
    // let other = Other {
    //     something: String::from("Python"),
    // };

    // let some = Some::new(String::from("Rust"), 16, other, Status::Active);

    // println!("Name: {}", some.get_name());
    // println!("Status: {}", some.get_status());

    // let s1 = String::from("Rust");
    // let len = calculate(&s1);

    // println!("{} so'zining uzunligi: {}", s1, len)

    // owerenship_borrwing::owerenship_and_borrwing();
    // lifetimes::lifetimes()
    // nested_loop();
    for_loop();
}

// struct Other {
//     something: String,
// }

// struct Some {
//     name: String,
//     age: u8,
//     other: Other,
//     status: Status,
// }

// impl Some {
//     fn new(name: String, age: u8, other: Other, status: Status) -> Self {
//         Self {
//             name: name.to_string(),
//             age,
//             other,
//             status,
//         }
//     }

//     fn get_name(&self) -> &str {
//         &self.name
//     }

//     fn get_status(&self) -> &'static str {
//         if self.status == Status::Active {
//             "active"
//         } else {
//             "inactive"
//         }
//     }
// }


// fn calculate(s: &str) -> usize {
//     s.len()
// }