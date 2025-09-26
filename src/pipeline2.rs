use anyhow::Result;
use gstreamer as gst;
use gstreamer::prelude::*;
use std::{thread, time::Duration};

fn main() -> Result<()> {
    gst::init()?;

    let pipeline_str = "\
        rtspsrc location=rtsp://localhost:8554/cam1 name=src \
        src. ! rtph264depay ! h264parse ! queue ! mpegtsmux name=mux \
        src. ! rtpmp4gdepay ! aacparse ! queue ! mux. \
        mux. ! srtclientsink uri=srt://:9999";

    loop {
        println!("Starting pipeline...");
        let pipeline = gst::parse::launch(&pipeline_str)?;

        pipeline.set_state(gst::State::Playing)?;
        println!("RTSP to SRT pipeline started");

        let bus = pipeline.bus().unwrap();
        let mut need_restart = false;

        for msg in bus.iter_timed(gst::ClockTime::NONE) {
            use gst::MessageView;
            match msg.view() {
                MessageView::Error(err) => {
                    eprintln!("Error: {} - {}", err.error(), err.debug().unwrap_or_default());
                    need_restart = true;
                    break;
                }
                MessageView::Eos(..) => {
                    println!("End of stream");
                    need_restart = true;
                    break;
                }
                _ => (),
            }
        }

        pipeline.set_state(gst::State::Null)?;

        if need_restart {
            println!("Retrying in 5 seconds...");
            thread::sleep(Duration::from_secs(5));
        } else {
            break;
        }
    }

    Ok(())
}
