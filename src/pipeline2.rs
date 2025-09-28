use anyhow::Result;
use gstreamer as gst;
use gstreamer::prelude::*;
use std::{env, thread, time::Duration};

fn main() -> Result<()> {
    gst::init()?;

    let args: Vec<String> = env::args().collect();

    let (rtsp_url, srt_url) = if args.len() > 2 {
        (&args[1], &args[2])
    } else {
        println!("Usage: {} <rtsp_url> <srt_url>", args[0]);
        return Ok(());
    };

    // Pipeline: RTSP -> video/audio demux -> mux into MPEG-TS -> SRT publish
    let pipeline_str = format!(
        "rtspsrc location={} name=src ! \
         rtph264depay ! h264parse ! queue ! mpegtsmux name=mux \
         src. ! rtpmp4adepay ! aacparse ! avdec_aac ! audioconvert ! audioresample ! avenc_aac ! queue ! mux. \
         mux. ! srtclientsink uri={}",
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
                    println!("Connection error: {}", err.error());
                    println!("Will keep trying to reconnect...");
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
            println!("Retrying connection in 5 seconds...");
            thread::sleep(Duration::from_secs(5));
        } else {
            break;
        }
    }

    Ok(())
}
