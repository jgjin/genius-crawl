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
};
use url::{
    percent_encoding::{
        utf8_percent_encode,
        DEFAULT_ENCODE_SET,
    },
};

use crate::{
    types::{
        APIResponse,
        SearchHit,
    },
    utils::{
        get_with_retry,
    },
};

pub fn search(
    query: &str,
    client: Arc<Client>,
) -> Result<APIResponse<SearchHit>, Box<dyn Error>> {
    let url = format!(
        "https://api.genius.com/search/?q={}",
        utf8_percent_encode(query, DEFAULT_ENCODE_SET),
    );
    info!("{}", url);
    get_with_retry(
        &url[..],
        client,
    )
}
