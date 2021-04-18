use std::env;
use std::path::Path;
use std::{thread, time};

fn main() -> std::io::Result<()> {
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
