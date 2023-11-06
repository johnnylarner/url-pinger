use reqwest::{self};

pub struct UrlPinger {
    pub urls: Vec<String>
}

#[derive(Debug)]
pub struct PingResult {
    pub url: String,
    pub status_code: u16
}

impl UrlPinger {
    pub fn new(urls: String) ->  UrlPinger{
        let mut url_vec: Vec<String> = vec![];
        for url in urls.split(",") {
            url_vec.push(url.to_string());
        }

        UrlPinger{urls: url_vec}
    }

    pub fn ping_urls(self) -> Vec<PingResult> {
        let mut results:Vec<PingResult> = vec![];
        for url in self.urls.iter() {
            let response = reqwest::blocking::get(url);
            let status_code = match response {
                Ok(response) =>  response.status().as_u16(),
                Err(_) => 404
            };
            results.push(PingResult{url: url.to_string(),  status_code});
        }
        results

    }
}


#[cfg(test)]
mod tests {
    use std::iter::zip;

    use super::*;

    #[test]
    fn new_returns_url_pinger() {
        let urls = "a,b".to_string();
        let pinger = UrlPinger::new(urls);
        assert_eq!(vec!["a", "b"], *pinger.urls);
    }

    #[test]
    fn ping_urls_handles_good_and_bad_requests() {
        let urls = "https://example.com,htx:example.com,https://google.com/hype".to_string();
        let pinger = UrlPinger::new(urls);

        let results = pinger.ping_urls();
        let expected_status_codes = [200, 404, 404];

        for  (actual_result, expected_code) in zip(results, expected_status_codes) {
            assert_eq!(actual_result.status_code, expected_code)
        }
    }


}