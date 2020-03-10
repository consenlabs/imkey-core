use crate::error::Error;
use regex::Regex;
use crate::Result;

pub fn check_path_validity(path: &str) -> Result<()> {
    //check depth
    let strings: Vec<&str> = path.split("/").collect();
    let depth = strings.len();
    if depth < 2 || depth > 10 || path.len() > 100{
//        return Err(Error::PathError);
        return Err(format_err!("PathError"));
    }

//    //check length
//    if path.chars().count() > 100 {
//        return Err(Error::PathError);
//    }

    //regx check
    let re = Regex::new(r"^m/[0-9'/]+$").unwrap();
    if !re.is_match(path) {
//        return Err(Error::PathError);
        return Err(format_err!("PathError"));
    }

    Ok(())
}
