use std::{fs::{File},
    io::{self, copy, Read},
};

use reqwest::Url;
use exitfailure::ExitFailure;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{header, Client};

struct DownloadProgress<R> {
    inner: R,
    progress_bar: ProgressBar,
}

impl<R: Read> Read for DownloadProgress<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf).map(|n| {
            self.progress_bar.inc(n as u64);
            n
        })
    }
}

pub fn download(url: &str, mut dest: &File) -> Result<(), ExitFailure> {
    let url = Url::parse(url)?;
    let client = Client::new();

    let total_size = {
        let resp = client.head(url.as_str()).send()?;
        if resp.status().is_success() {
            resp.headers()
                .get(header::CONTENT_LENGTH)
                .and_then(|ct_len| ct_len.to_str().ok())
                .and_then(|ct_len| ct_len.parse().ok())
                .unwrap_or(0)
        } else {
            return Err(failure::err_msg(format!(
                "Couldn't download URL: {}. Error: {:?}",
                url,
                resp.status(),
            )).into());
        }
    };

    let request = client.get(url.as_str());
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
                 .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                 .progress_chars("#>-"));

    let mut source = DownloadProgress {
        progress_bar: pb,
        inner: request.send()?,
    };

    let _ = copy(&mut source, &mut dest)?;

    println!(
        "Download of '{}' has been completed.", url.to_string()
    );

    Ok(())
}