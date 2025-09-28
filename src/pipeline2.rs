use anyhow::Result;
use gstreamer as gst;
use gstreamer::prelude::*;
use std::{thread, time::Duration};

fn main() -> Result<()> {
    gst::init()?;

    // Fixed URLs
    let rtsp_url = "rtsp://localhost:8554/cam1";
    let srt_url = "srt://127.0.0.1:8890?streamid=publish:cam1";

    let pipeline_str = format!(
        "rtspsrc location={} latency=100 ! \
         rtph264depay ! \
         mpegtsmux ! \
         srtclientsink uri={}",
        rtsp_url, srt_url
    );

    loop {
        println!("Connecting to RTSP source...");

        let pipeline = match gst::parse::launch(&pipeline_str) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Failed to create pipeline: {}", e);
                println!("Retrying in 5 seconds...");
                thread::sleep(Duration::from_secs(5));
                continue;
            }
        };

        match pipeline.set_state(gst::State::Playing) {
            Ok(_) => println!("Pipeline started, streaming RTSP to SRT..."),
            Err(e) => {
                eprintln!("Failed to start pipeline: {}", e);
                pipeline.set_state(gst::State::Null).ok();
                println!("Retrying in 5 seconds...");
                thread::sleep(Duration::from_secs(5));
                continue;
            }
        }

        let bus = pipeline.bus().unwrap();
        let mut should_restart = false;

        for msg in bus.iter_timed(gst::ClockTime::NONE) {
            use gst::MessageView;
            match msg.view() {
                MessageView::Error(err) => {
                    eprintln!("Connection error: {}", err.error());
                    should_restart = true;
                    break;
                }
                MessageView::Eos(..) => {
                    println!("RTSP stream ended, reconnecting...");
                    should_restart = true;
                    break;
                }
                _ => (),
            }
        }

        pipeline.set_state(gst::State::Null).ok();

        if should_restart {
            println!("Retrying in 5 seconds...");
            thread::sleep(Duration::from_secs(5));
        } else {
            break;
        }
    }

    Ok(())
}