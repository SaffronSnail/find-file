use std::io;

pub fn select_file<T: ::std::fmt::Display, In: io::BufRead, Out: io::Write>(
    options: &Vec<T>,
    input: &mut In,
    output: &mut Out,
) -> Result<Option<usize>, io::Error> {
    if options.is_empty() {
        Ok(None)
    } else if options.len() == 1 {
        Ok(Some(0))
    } else {
        let mut max_entry = -1;
        for entry in options.iter() {
            max_entry += 1;
            write!(output, "{}: {}\n", max_entry, entry)?;
        }
        let max_entry = max_entry; // remove mutability

        write!(output, "Please select an option (0 - {}): ", max_entry)?;
        output.flush()?;

        loop {
            let mut response = String::new();
            input.read_line(&mut response)?;
            response.pop(); // remove the newline
            if response.chars().next() == Some('q') {
                return Ok(None);
            } else {
                match response.parse::<usize>() {
                    Ok(selection) if selection < options.len() => return Ok(Some(selection)),
                    _ => write!(output, "Please enter only a number from 0 to {}", max_entry)?,
                };
            }
        }
    }
}

fn main() {
    let mut args = ::std::env::args();
    args.next();

    let input = io::stdin();
    let mut input = input.lock();

    let output = io::stdout();
    let mut output = output.lock();
    let options = args.collect();
    match select_file(&options, &mut input, &mut output) {
        Ok(index) => {
            if let Some(index) = index {
                use ::std::io::Write;
                write!(io::stderr(), "{}", options[index]).unwrap();
            }
        }
        Err(e) => {
            println!("Error when selecting file!");
            println!("{}", e);
            ::std::process::exit(-1)
        }
    }
}

#[cfg(test)]
mod test {
    use std::io;

    use io::{BufReader, Read, stdout};
    use super::select_file;

    struct StringReader<'a> {
        data: ::std::str::Bytes<'a>,
    }
    impl<'a> StringReader<'a> {
        fn from(s: &'a String) -> StringReader {
            StringReader { data: s.bytes() }
        }
    }
    impl<'a> Read for StringReader<'a> {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
            let buf_len = buf.len();
            for i in 0..buf_len {
                match self.data.next() {
                    Some(b) => buf[i] = b,
                    None    => return Ok(i),
                }
            }
            return Ok(buf_len);
        }
    }

    #[test]
    fn returns_selected_index() {
        let mut options = vec!["foo", "bar", "baz"];
        let input = String::from("1\n");

        let output = stdout();
        let mut output = output.lock();
        match select_file(
            &mut options,
            &mut BufReader::new(StringReader::from(&input)),
            &mut output) {
            Ok(Some(i)) => assert_eq!(i, 1),
            Ok(None)    => panic!("Returned None, expected 1!"),
            Err(e)      => panic!(format!("Error! {}", e)),
        }
    }
}
