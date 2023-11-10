use reqwest::{self};
use std::sync::Arc;
use std::thread;
use std::time::Instant;

pub enum RuntimeType {
    SYNC,
    ASYNC,
    MULTITHREADED,
}

pub struct UrlPinger {
    pub urls: Vec<String>,
    pub runtime: RuntimeType,
}

#[derive(Debug)]
pub struct PingResult {
    pub url: String,
    pub status_code: u16,
    pub duration_in_nano_seconds: u128,
}

impl UrlPinger {
    pub fn new(urls: Vec<String>, runtime: &str) -> UrlPinger {
        let runtime = match runtime {
            "sync" => RuntimeType::SYNC,
            "async" => RuntimeType::ASYNC,
            "multi" => RuntimeType::MULTITHREADED,
            _ => panic!("Mode {runtime} not in ('sync', 'async', 'multi'"),
        };
        UrlPinger { urls, runtime }
    }
    pub fn from_comma_seperated_string(urls: &str, runtime: &str) -> UrlPinger {
        let mut urls_as_vec: Vec<String> = Vec::new();
        for url in urls.split(",") {
            urls_as_vec.push(url.to_string());
        }
        UrlPinger::new(urls_as_vec, runtime)
    }

    pub fn ping_urls(&self) -> Vec<PingResult> {
        match self.runtime {
            RuntimeType::SYNC => self.ping_urls_sync(),
            RuntimeType::ASYNC => self.ping_urls_async(),
            RuntimeType::MULTITHREADED => self.ping_urls_multithread(),
        }
    }

    fn ping_urls_sync(&self) -> Vec<PingResult> {
        let mut results: Vec<PingResult> = Vec::new();
        let client = reqwest::blocking::Client::new();
        for url in self.urls.iter() {
            let start = Instant::now();
            let status_code: u16 = Self::get_url_status_code(&client, &url);
            let end = start.elapsed();

            results.push(PingResult {
                url: url.clone(),
                status_code,
                duration_in_nano_seconds: end.as_nanos(),
            });
        }
        results
    }

    fn get_url_status_code(client: &reqwest::blocking::Client, url: &str) -> u16 {
        let response = client.get(url).send();
        match response {
            Ok(response) => response.status().as_u16(),
            Err(_) => 404,
        }
    }

    fn ping_urls_async(&self) -> Vec<PingResult> {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let client = reqwest::Client::new(); // Create a single shared instance of the client.

                let futures = self.urls.iter().map(|url| {
                    let url_clone = url.clone();
                    let client = &client;
                    async move {
                        let start = Instant::now();
                        let status_code = Self::get_url_status_code_async(client, &url_clone).await;
                        let end = start.elapsed();

                        PingResult {
                            url: url_clone,
                            status_code,
                            duration_in_nano_seconds: end.as_nanos(),
                        }
                    }
                });

                futures::future::join_all(futures).await
            })
    }
    async fn get_url_status_code_async(client: &reqwest::Client, url: &str) -> u16 {
        let response = client.get(url).send().await;
        match response {
            Ok(response) => response.status().as_u16(),
            Err(_) => 404,
        }
    }

    fn ping_urls_multithread(&self) -> Vec<PingResult> {
        let client = Arc::new(reqwest::blocking::Client::new());
        let mut threads = Vec::new();

        for url in self.urls.iter() {
            let client = Arc::clone(&client);
            let url_clone = url.clone();
            threads.push(thread::spawn(move || {
                let start = Instant::now();
                let status_code: u16 = Self::get_url_status_code(&client, &url_clone);
                let end = start.elapsed();
                PingResult {
                    url: url_clone,
                    status_code,
                    duration_in_nano_seconds: end.as_nanos(),
                }
            }))
        }

        let mut results: Vec<PingResult> = Vec::new();
        for thread in threads {
            results.push(thread.join().unwrap())
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use std::iter::zip;

    use super::*;

    fn sync_pinger() -> UrlPinger {
        let urls = "https://example.com,htx:example.com,https://google.com/hype".to_string();
        UrlPinger::from_comma_seperated_string(&urls, "sync")
    }

    fn async_pinger() -> UrlPinger {
        let urls = "https://example.com,htx:example.com,https://google.com/hype".to_string();
        UrlPinger::from_comma_seperated_string(&urls, "async")
    }

    fn thread_pinger() -> UrlPinger {
        let urls = "https://example.com,htx:example.com,https://google.com/hype".to_string();
        UrlPinger::from_comma_seperated_string(&urls, "multi")
    }

    #[test]
    fn from_comma_seperated_string_returns_url_pinger() {
        let urls = "a,b".to_string();
        let pinger = UrlPinger::from_comma_seperated_string(&urls, "sync");
        assert_eq!(vec!["a", "b"], *pinger.urls);
    }

    #[test]
    fn ping_urls_handles_good_and_bad_requests() {
        let pingers = [sync_pinger(), async_pinger(), thread_pinger()];

        for pinger in pingers {
            let results = pinger.ping_urls();
            let expected_status_codes = [200, 404, 404];
            for (actual_result, expected_code) in zip(results, expected_status_codes) {
                assert_eq!(actual_result.status_code, expected_code)
            }
        }
    }

    #[test]
    fn ping_urls_returns_valid_request_duration() {
        let pingers = [sync_pinger(), async_pinger(), thread_pinger()];

        for pinger in pingers {
            let results = pinger.ping_urls();
            for result in results.iter() {
                assert!(result.duration_in_nano_seconds > 0);
            }
        }
    }

    #[test]
    fn sync_pinger_is_slower_than_async_and_threaded() {
        let urls = "http://example1.com,http://example2.com,http://example3.com,http://example4.com,http://example5.com,http://example6.com,http://example7.com,http://example8.com,http://example9.com,http://example10.com,http://example11.com,http://example12.com,http://example13.com,http://example14.com,http://example15.com,http://example16.com,http://example17.com,http://example18.com,http://example19.com,http://example20.com,http://example21.com,http://example22.com,http://example23.com,http://example24.com,http://example25.com,http://example26.com,http://example27.com,http://example28.com,http://example29.com,http://example30.com,http://example31.com,http://example32.com,http://example33.com,http://example34.com,http://example35.com,http://example36.com,http://example37.com,http://example38.com,http://example39.com,http://example40.com";

        let sync_pinger = UrlPinger::from_comma_seperated_string(urls, "sync");
        let async_pinger = UrlPinger::from_comma_seperated_string(urls, "async");
        let threaded_pinger = UrlPinger::from_comma_seperated_string(urls, "multi");

        let sync_start = Instant::now();
        sync_pinger.ping_urls();
        let sync_duration = sync_start.elapsed();
        println!("Synchronous duration: {:?}", sync_duration);

        let async_start = Instant::now();
        async_pinger.ping_urls();
        let async_duration = async_start.elapsed();
        println!("Asynchronous duration: {:?}", async_duration);

        let threaded_start = Instant::now();
        threaded_pinger.ping_urls();
        let threaded_duration = threaded_start.elapsed();
        println!("Threaded duration: {:?}", threaded_duration);

        assert!(async_duration < sync_duration);
        assert!(threaded_duration < sync_duration);
    }
}
