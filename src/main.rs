use std::io;

fn main() {
    // dream_number();
    // fizz_buzz();
    // fibonacci()
}

fn fibonacci() {
    let input = get_usr_number();

    let mut last = 1;
    let mut next = 1;

    while last <= input {
        println!("{last}");
        let temp = last;
        last = next;
        next = temp + next;
    }
}

fn get_usr_number() -> u32 {
    let mut input = String::new();

    loop {
        println!("Enter dream number");


        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(_) => {
                println!("Error reading line");
            },
        };

        match input.trim().parse() {
            Ok(num) => {
                return num;
            },
            Err(_) => {
                println!("Error parsing");
                continue;
            },
        };
    };
}

fn fizz_buzz() {
    println!("Enter fizzy number");

    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Cool. there's been an error reading the line");

    let input: u32 = input.trim().parse().unwrap();

    for i in 1..input {
        if i % 3 == 0 && i % 5 == 0 {
            println!("fizzbuzz")
        } else if i % 3 == 0 {
            println!("fizz")
        } else if i % 5 == 0 {
            println!("buzz")
        } else {
            println!("{i}")
        }
    };
}

fn dream_number() {
    println!("Enter dream number");

    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => (),
        Err(_) => println!("Error reading line"),
    };

    let input: i32 = input.trim().parse().unwrap();

    for i in 1..input {
        println!("{i}..");
    }

    println!("** DREAM NUMBER **");
    println!("{input}");
}
