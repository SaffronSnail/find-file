use std::io;
use std::path::{Path,PathBuf};

fn find_file<P: AsRef<Path>, N: AsRef<Path>>(path: P, name: N)
    -> io::Result<Vec<PathBuf>> {
    use std::fs;

    let name = name.as_ref();
    let mut ret = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            ret.append(&mut find_file(entry.path(), name)?);
        } else if entry.path().ends_with(name) {
            ret.push(entry.path());
        }
    }

    Ok(ret)
}

fn main() {
    use std::env;

    let mut args = env::args();
    args.next(); // skip the progam name
    let working_directory = args.next();
    for path in find_file(working_directory.expect("The first argument is required to be the working directory!"),
                          args.next().expect("The second argument is required to be the filename to search for!")
                          .as_str()).expect("") {
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
        use std::path::{PathBuf,Path};
        use std::fs::{create_dir,File,remove_dir_all};

        const FILENAME: &'static str = "foobar.sh";

        fn check_result<T, E: ::std::fmt::Display>(result: Result<T, E>) {
            match result {
                Ok(_) => {},
                Err(e) => panic!("Error! {}", e)
            }
        }

        fn test<P: AsRef<Path>>(path: &P, expected: &HashSet<PathBuf>) -> Result<(), ::std::io::Error> {
            let result = find_file(path, FILENAME)?.into_iter().collect::<HashSet<_>>();
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

