use std::io;

pub fn select_file<T: ::std::fmt::Display>(options: &Vec<T>)
    -> Result<Option<usize>, io::Error> {
    if options.is_empty() {
        Ok(None)
    } else if options.len() == 1 {
        Ok(Some(0))
    } else {
        let mut max_entry = -1;
        for entry in options.iter() {
            max_entry += 1;
            println!("{}: {}", max_entry, entry);
        }
        let max_entry = max_entry; // remove mutability

        println!("Please select an option (0 - {}): ", max_entry);

        loop {
            let mut response = String::new();
            io::stdin().read_line(&mut response)?;
            response.pop(); // remove the newline
            if response.chars().next() == Some('q') {
                return Ok(None)
            } else {
                match response.parse::<usize>() {
                    Ok(selection) if selection < options.len() => {
                        return Ok(Some(selection))
                    },
                    _ => println!("Please enter only a number from 0 to {}", max_entry)
                };
            }
        }
    }
}

fn main() {
    use std::process::exit;

    let mut args = ::std::env::args();
    args.next();

    match select_file(&args.collect()) {
        Ok(index) => {
            if let Some(index) = index {
                exit(index as i32);
            }
        }
        Err(_) => exit(-1)
    }
}

