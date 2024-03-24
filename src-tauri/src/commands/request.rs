
const URL_CHROMEDRIVER_GOOD_VERSIONS_WITH_DOWNLOAD: &str = "https://googlechromelabs.github.io/chrome-for-testing/known-good-versions-with-downloads.json";
const URL_CHROMEDRIVER_GOOD_LAST_KNOWN_VERSIONS_WITH_DOWNLOAD: &str = "https://googlechromelabs.github.io/chrome-for-testing/last-known-good-versions-with-downloads.json";

pub fn download_chromedriver_versions() -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let res = client.get(URL_CHROMEDRIVER_GOOD_VERSIONS_WITH_DOWNLOAD).send()?;
    let v = res.text()?;
    Ok(v)
}

pub fn download_chromedriver_known_versions() -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let res = client.get(URL_CHROMEDRIVER_GOOD_LAST_KNOWN_VERSIONS_WITH_DOWNLOAD).send()?;
    let v = res.text()?;
    Ok(v)
}

// TODOS
// [] asyncで処理
// [Rust のreqwest を使った非同期HTTP Client のお試し \#Rust \- Qiita](https://qiita.com/mypsychology0/items/60d638ffa8231f4379c0)