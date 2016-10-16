
fn main() {
    for i in 1..read_max() + 1 {
        let fizzy = i % 3 == 0;
        let buzzy = i % 5 == 0;
        if fizzy && buzzy {
            println!("fizz-buzz");
        } else if fizzy {
            println!("fizz");
        } else if buzzy {
            println!("buzz");
        } else {
            println!("{}", i);
        }
    }
}

fn read_max() -> usize {
    use std::env;
    match env::args().nth(1).and_then(|val| val.parse().ok()) {
        Some(max) => max,
        _ => {
            println!("First parameter was not a positive number, counting to 25 instead!");
            25
        },
    }
}
