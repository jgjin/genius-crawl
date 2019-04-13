use std::{
    error::{
        Error,
    },
    sync::{
        Arc,
    },
};

use reqwest::{
    Client,
    StatusCode,
};
use serde::{
    de::{
        DeserializeOwned,
    },
};

pub fn get_with_retry<D: DeserializeOwned>(
    url: &str,
    client: Arc<Client>,
) -> Result<D, Box<dyn Error>> {
    let mut response = client.get(url)
        .bearer_auth("o-Bj65sLMdN-B_NmLfTGbNIHsTDsAuzWoe5PLwVnzCVCJF-NKpw2x3Iau0Y0n_K2")
        .send().map_err(|err| {
            format!("Error for {}: {}", url, err)
        })?;
    match response.status() {
        StatusCode::OK => Ok(response.json::<D>()?),
        status_code => panic!("Unexpected error code: {}", status_code),
    }
}
