use hyprcurl::{Browser, Curl};
use std::error::Error;

pub(crate) fn fetch_page(url: &str) -> Result<(Curl, String), Box<dyn Error>> {
    let mut curl = Curl::new()?;
    let mut buffer = Vec::new();

    curl.set_url(url)?;
    curl.set_browser_impersonation(Browser::ChromeLatest)?;

    curl.perform(&mut buffer)?;

    Ok((curl, String::from_utf8_lossy(&buffer).to_string()))
}
