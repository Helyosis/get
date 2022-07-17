use regex::Regex;
use reqwest::Url;
use exitfailure::ExitFailure;

pub fn clean(url: &String) -> Result<String, ExitFailure> {
    let url = Url::parse(url.as_str())?;
    let filename = url.path_segments().and_then(
        |s| s.last()
    ).unwrap_or("output");

    let mut filename_string = String::new();
    filename_string.push_str(filename);

    let re = Regex::new(r"_[a-f0-9]+")?;
    filename_string = re.replace_all(&filename_string, "").to_string();

    return Ok(filename_string);
}