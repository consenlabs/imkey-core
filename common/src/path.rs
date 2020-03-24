use crate::error::CommonError;
use regex::Regex;
use crate::Result;

pub fn check_path_validity(path: &str) -> Result<()> {
    //check depth and length
    let strings: Vec<&str> = path.split("/").collect();
    let depth = strings.len();
    if depth < 2 || depth > 10 || path.len() > 100{
        return Err(CommonError::ImkeyPathIllegal.into());
    }
    //regx check
    let re = Regex::new(r"^m/[0-9'/]+$").unwrap();
    if !re.is_match(path) {
        return Err(CommonError::ImkeyPathIllegal.into());
    }
    Ok(())
}
