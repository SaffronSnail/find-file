#[macro_use]
extern crate clap;

use std::io;

/// Prompts the user to select from a number of options
///
/// # Examples
/// ```
/// use std::io;
/// let input = io::stdin();
/// let mut input = input.lock();
/// let output = io::stdout();
/// let mut output = output.lock();
/// select_file(vec!["foo" "bar" "baz"], input, output);
/// select_file(find_file(".", "plan.sh"), input, output);
/// ```
pub fn select_file<T: ::std::fmt::Display, In: io::BufRead, Out: io::Write>(
    options: &Vec<T>,
    input: &mut In,
    output: &mut Out,
) -> Result<Option<usize>, io::Error> {
    // if we are given no options, stop here
    if options.is_empty() {
        Ok(None)
    // if we only have one option, there is no selection to make
    } else if options.len() == 1 {
        Ok(Some(0))
    // if we have more than one option, this function does actual work
    } else {
        // track the highest option available to the user in order to make the
        // prompt more helpful and aid in input validation; enumerate doesn't
        // really win us anything here, as we'd still be setting a variable on
        // every loop
        let mut max_entry = -1;
        for entry in options.iter() {
            max_entry += 1;
            write!(output, "{}: {}\n", max_entry, entry)?;
        }
        // we have now figured out what the highest valid input is, so strip
        // mutability from the variable
        let max_entry = max_entry;

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

/// The binary reads all of the arguments as options and prompts the user on std
/// in/out; the selected option is printed to std err, to make it easier for
/// scripts to distinguish between the user prompt and the result
fn main() {
    // clap for command line parsing and help/version flag generation
    let matches = clap_app!(select_file =>
        (about: "Prompts the user to select from a set of options")
        (version: crate_version!())
        (author: "Bryan Ferris <primummoven@gmail.com>")
        (@arg OPTIONS: +required ... "The set of options that the user can choose to select from")
    ).get_matches();
    let options = matches.values_of("OPTIONS").unwrap().collect();

    // read/write to/from std in/out
    let input = io::stdin();
    let mut input = input.lock();
    let output = io::stdout();
    let mut output = output.lock();

    match select_file(&options, &mut input, &mut output) {
        Ok(index) => {
            if let Some(index) = index {
                use std::io::Write;
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

    use io::{BufReader, Read};
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
                    None => return Ok(i),
                }
            }
            return Ok(buf_len);
        }
    }

    #[test]
    fn returns_selected_index() {
        let mut options = vec!["foo", "bar", "baz"];
        let input = String::from("1\n");

        let mut output = io::sink();
        match select_file(
            &mut options,
            &mut BufReader::new(StringReader::from(&input)),
            &mut output,
        ) {
            Ok(Some(i)) => assert_eq!(i, 1),
            Ok(None) => panic!("Returned None, expected 1!"),
            Err(e) => panic!(format!("Error! {}", e)),
        }
    }
}
