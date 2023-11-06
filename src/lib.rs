use reqwest::{self};
use std::time::Instant;

pub struct UrlPinger {
    pub urls: Vec<String>,
}

#[derive(Debug)]
pub struct PingResult {
    pub url: String,
    pub status_code: u16,
    pub duration_in_nano_seconds: u128,
}

impl UrlPinger {
    pub fn new(urls: String) -> UrlPinger {
        let mut url_vec: Vec<String> = Vec::new();
        for url in urls.split(",") {
            url_vec.push(url.to_string());
        }

        UrlPinger { urls: url_vec }
    }

    pub fn ping_urls(&self) -> Vec<PingResult> {
        let mut results: Vec<PingResult> = Vec::new();
        for url in self.urls.iter() {
            let start = Instant::now();
            let status_code: u16 = self.get_url_status_code(&url);
            let end = start.elapsed();

            results.push(PingResult {
                url: url.clone(),
                status_code,
                duration_in_nano_seconds: end.as_nanos(),
            });
        }
        results
    }

    fn get_url_status_code(&self, url: &str) -> u16 {
        let response = reqwest::blocking::get(url);
        match response {
            Ok(response) => response.status().as_u16(),
            Err(_) => 404,
        }
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

        for (actual_result, expected_code) in zip(results, expected_status_codes) {
            assert_eq!(actual_result.status_code, expected_code)
        }
    }

    #[test]
    fn ping_urls_returns_valid_request_duration() {
        let urls = "https://example.com,https://google.com/hype,htx:example.com".to_string();
        let pinger = UrlPinger::new(urls);

        let results = pinger.ping_urls();
        for result in results.iter() {
            assert!(result.duration_in_nano_seconds > 0);
        }
    }
}
