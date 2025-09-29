use anyhow::Result;
use clap::Parser;
use gstreamer as gst;
use gstreamer::prelude::*;
use std::{thread, time::Duration};
use tracing::{info, error};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 's', long, default_value = "rtsp://localhost:8554/cam1")]
    rtsp_url: String,

    #[arg(short = 'd', long, default_value = "srt://127.0.0.1:8890?streamid=publish:cam1")]
    srt_url: String,

    #[arg(short, long, default_value = "5")]
    reconnect_delay: u64,
}

fn create_pipeline(rtsp_url: &str, srt_url: &str) -> Result<gst::Pipeline> {
    let pipeline_str = format!(
        "rtspsrc location={} latency=100 ! rtph264depay ! mpegtsmux ! srtclientsink uri={}",
        rtsp_url, srt_url
    );

    let pipeline = gst::parse::launch(&pipeline_str)?;
    Ok(pipeline.downcast::<gst::Pipeline>().unwrap())
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let args = Args::parse();

    info!("Starting RTSP to SRT pipeline");
    info!("RTSP: {} -> SRT: {}", args.rtsp_url, args.srt_url);

    gst::init()?;

    loop {
        info!("Connecting...");

        let pipeline = match create_pipeline(&args.rtsp_url, &args.srt_url) {
            Ok(p) => p,
            Err(e) => {
                error!("Failed to create pipeline: {}", e);
                thread::sleep(Duration::from_secs(args.reconnect_delay));
                continue;
            }
        };

        if let Err(e) = pipeline.set_state(gst::State::Playing) {
            error!("Failed to start pipeline: {}", e);
            thread::sleep(Duration::from_secs(args.reconnect_delay));
            continue;
        }

        info!("Streaming...");

        let bus = pipeline.bus().unwrap();
        for msg in bus.iter_timed(gst::ClockTime::NONE) {
            use gst::MessageView;
            match msg.view() {
                MessageView::Error(err) => {
                    error!("Error: {}", err.error());
                    break;
                }
                MessageView::Eos(..) => {
                    error!("Stream ended");
                    break;
                }
                _ => (),
            }
        }

        pipeline.set_state(gst::State::Null).ok();

        info!("Reconnecting in {} seconds...", args.reconnect_delay);
        thread::sleep(Duration::from_secs(args.reconnect_delay));
    }
}