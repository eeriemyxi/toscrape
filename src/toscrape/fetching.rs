use reqwest::Result;
use reqwest::blocking::{Client, Response};
use std::sync::Arc;

pub(crate) fn get_client() -> Arc<Client> {
    Arc::new(Client::new())
}

pub(crate) fn fetch_page<T>(client: Arc<Client>, url: T) -> Result<Response>
where
    T: AsRef<str>,
{
    client.get(url.as_ref()).send()
}
