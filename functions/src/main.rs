use std::cmp::Ordering;
use std::io;

fn main() {
    let name = input_name();

    index_selection(name);
}

fn input_name() -> String {
    println!("Please enter your name");

    let mut name = String::new();

    io::stdin().read_line(&mut name).expect("Nothing to read !");

    return name;
}

fn index_selection(name: String) {
    println!("Welcome {}", name);
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let len_a = a.len();

    println!("Length of a : {}", len_a);

    loop {
        let mut index = String::new();
        println!("Index you want to know");

        io::stdin()
            .read_line(&mut index)
            .expect("Nothing to read !");

        let index: usize = match index.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match index.cmp(&len_a) {
            Ordering::Less => println!("Here's your number {}", a[index]),
            Ordering::Equal => {
                println!("Too big !");
                break;
            },
            Ordering::Greater => {
                println!("Too big !");
                break;
            }
        }
    }
}
