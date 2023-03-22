#![allow(unused)]

use clap::Parser;
use std;
use std::process::Stdio;

use ffmpeg_cli::{FfmpegBuilder, File, Parameter};
use futures::{future::ready, StreamExt};

#[derive(Parser)]
struct Cli {
    source: String,
    destination: String,
}


#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let builder = FfmpegBuilder::new()
        .stderr(Stdio::piped())
        .input(File::new(&args.source))
        .output(
            File::new(&args.destination)
                .option(Parameter::KeyValue("c:v", "libx264"))
                .option(Parameter::KeyValue("c:a", "aac"))
                .option(Parameter::KeyValue("strict", "-2"))
                .option(Parameter::KeyValue("crf", "25")),
        );

    let ffmpeg = builder.run().await.unwrap();

    ffmpeg
        .progress
        .for_each(|x| {
            dbg!(x.unwrap());
            ready(())
        })
        .await;

    let output = ffmpeg.process.wait_with_output().unwrap();

    println!(
        "{}\nstderr:\n{}",
        output.status,
        std::str::from_utf8(&output.stderr).unwrap()
    );
}
