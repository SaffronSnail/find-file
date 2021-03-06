#[macro_use]
extern crate clap;

use std::io;
use std::path::{Path, PathBuf};

/// Finds all files with the given name in the tree rooted at path
///
/// # Examples
/// ```
/// find_file("/home/user", ".vimrc");
/// find_file(".", "plan.sh");
/// ```
pub fn find_file<P: AsRef<Path>, N: AsRef<Path>>(path: P, name: N) -> io::Result<Vec<PathBuf>> {
    use std::fs;

    let name = name.as_ref();

    // list of matches
    let mut ret = Vec::new();

    // search all files and subdirectories for the specified name
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            ret.append(&mut find_file(entry.path(), name)?);
        } else if entry.path().ends_with(name) {
            ret.push(entry.path());
        }
    }

    // if we get to this point, we encountered no errors and found all matches
    Ok(ret)
}

/// The binary reads the arguments from the command line and prints the results
fn main() {
    let matches = clap_app!(find_file =>
              (about: "Searches a directory tree for files with a specified file name and prints the results")
              (version: crate_version!())
              (author: "Bryan Ferris <primummoven@gmail.com>")
              (@arg SEARCH_DIRECTORY: +required "The directory to search for files in")
              (@arg SEARCH_TERM:      +required "The filename to search for")
    ).get_matches();

    // get values of required arguments
    let search_directory = PathBuf::from(matches.value_of("SEARCH_DIRECTORY").unwrap());
    let search_term = matches.value_of("SEARCH_TERM").unwrap();

    for path in find_file(search_directory, search_term).unwrap() {
        println!("{}", path.display());
    }
}

#[cfg(test)]
mod tests {
    use super::find_file;

    #[test]
    fn finds_entries() {
        use std::collections::HashSet;
        use std::env::set_current_dir;
        use std::path::{PathBuf, Path};
        use std::fs::{create_dir, File, remove_dir_all};

        const FILENAME: &'static str = "foobar.sh";

        fn check_result<T, E: ::std::fmt::Display>(result: Result<T, E>) {
            match result {
                Ok(_) => {}
                Err(e) => panic!("Error! {}", e),
            }
        }

        fn test<P: AsRef<Path>>(
            path: &P,
            expected: &HashSet<PathBuf>,
        ) -> Result<(), ::std::io::Error> {
            let result = find_file(path, FILENAME)?
                .into_iter()
                .collect::<HashSet<_>>();
            println!("result: {:?}", result);
            println!("expected: {:?}", expected);
            println!("=============================================");
            assert_eq!(result.symmetric_difference(&expected).next(), None);
            Ok(())
        }

        let mut expected_results = HashSet::new();
        let root_dir = "find_file_find_entries_test_environment";

        if Path::exists(PathBuf::from(root_dir).as_path()) {
            println!("Removing {}", root_dir);
            check_result(remove_dir_all(root_dir));
        }
        println!("Creating {}", root_dir);
        check_result(create_dir(root_dir));
        println!("Moving into {}", root_dir);
        check_result(set_current_dir(root_dir));

        let mut dir_structure = String::new();
        for dirname in [".", "upper", "middle", "lower"].into_iter() {
            dir_structure.push_str(dirname);
            dir_structure.push_str("/");
            if !dir_structure.ends_with("./") {
                println!("Creating {}", dir_structure);
                check_result(create_dir(&dir_structure));
            }

            let mut current_file = dir_structure.clone();
            current_file.push_str(FILENAME);
            println!("Creating {}", current_file);
            check_result(File::create(&current_file));
            expected_results.insert(PathBuf::from(&current_file));

            println!("Testing find_file");
            check_result(test::<&'static str>(&".", &expected_results));
        }

        println!("Moving into parent directory");
        check_result(set_current_dir(".."));
        println!("Removing {}", root_dir);
        check_result(remove_dir_all(root_dir));
    }
}
