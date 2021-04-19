use std::env;
use std::path::Path;
use std::{thread, time};

fn main() -> std::io::Result<()> {
    let mut useTcp = false;
    let mut useFile = false;
    println!("arg.len={}", env::args().len());

    let mut skip = false;
    for i in 1..env::args().len() {
        if skip {
            skip = false;
            continue;
        }
        let arg = match env::args().nth(i) {
            Option::Some(val) => val,
            Option::None => "".to_owned(),
        };

        if arg == "-f" {
            println!("f");

            let file = match env::args().nth(i + 1) {
                Option::Some(val) => val,
                Option::None => panic!("f is required"),
            };
            useFile = true;
            skip = true;
        }
        if arg == "-p" {
            println!("P");

            let host = match env::args().nth(i) {
                Option::Some(val) => val,
                Option::None => panic!("p is required"),
            };
            useTcp = true;
            skip = true;
        }
        
        println!("arg[{}] = {}", i, arg);
    }
    let filename = env::args().nth(1).expect("file name required");

    let exists = Path::new(&filename).exists();

    let mut found = false;
    for _i in  0..5{
        if exists {
            found = true;
            break;
        } else {
            let wait = time::Duration::from_millis(1000);
            println!("waiting");
            thread::sleep(wait);
        }
    }
    if found {
        Ok(())
    } else {
        panic!("file didn't appear");
    }
}
