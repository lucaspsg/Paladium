use anyhow::Result;
use clap::Parser;
use gstreamer as gst;
use gstreamer::prelude::*;
use std::{sync::{Arc, atomic::{AtomicBool, Ordering}}, thread, time::Duration};
use tracing::{info, error, warn, debug};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// RTSP source URL
    #[arg(short = 's', long, default_value = "rtsp://localhost:8554/cam1")]
    rtsp_url: String,

    /// SRT destination URL
    #[arg(short = 'd', long, default_value = "srt://127.0.0.1:8890?streamid=publish:cam1")]
    srt_url: String,

    /// Reconnection delay in seconds
    #[arg(short, long, default_value = "5")]
    reconnect_delay: u64,

    /// Maximum reconnection attempts (0 = infinite)
    #[arg(short = 'm', long, default_value = "0")]
    max_retries: u32,
}

struct PipelineManager {
    rtsp_url: String,
    srt_url: String,
    reconnect_delay: u64,
    max_retries: u32,
    current_retries: u32,
    shutdown: Arc<AtomicBool>,
}

impl PipelineManager {
    fn new(args: Args, shutdown: Arc<AtomicBool>) -> Self {
        Self {
            rtsp_url: args.rtsp_url,
            srt_url: args.srt_url,
            reconnect_delay: args.reconnect_delay,
            max_retries: args.max_retries,
            current_retries: 0,
            shutdown,
        }
    }

    fn should_retry(&self) -> bool {
        !self.shutdown.load(Ordering::Relaxed) &&
            (self.max_retries == 0 || self.current_retries < self.max_retries)
    }

    fn increment_retries(&mut self) {
        self.current_retries += 1;
    }

    fn reset_retries(&mut self) {
        self.current_retries = 0;
    }

    fn create_pipeline(&self) -> Result<gst::Pipeline> {
        let pipeline_str = format!(
            "rtspsrc location={} latency=100 ! \
             rtph264depay ! \
             mpegtsmux ! \
             srtclientsink uri={}",
            self.rtsp_url, self.srt_url
        );

        debug!("Creating pipeline: {}", pipeline_str);

        let pipeline = gst::parse::launch(&pipeline_str)?;
        Ok(pipeline.downcast::<gst::Pipeline>().unwrap())
    }

    fn run_pipeline(&mut self) -> Result<()> {
        loop {
            if !self.should_retry() {
                if self.shutdown.load(Ordering::Relaxed) {
                    info!("Shutdown requested, stopping pipeline...");
                } else {
                    error!("Maximum retry attempts ({}) reached. Exiting.", self.max_retries);
                }
                break;
            }

            info!("Connecting to RTSP source: {}", self.rtsp_url);
            info!("Publishing to SRT: {}", self.srt_url);

            let pipeline = match self.create_pipeline() {
                Ok(p) => p,
                Err(e) => {
                    error!("Failed to create pipeline: {}", e);
                    self.increment_retries();
                    if self.should_retry() {
                        let retry_display = if self.max_retries == 0 {
                            "∞".to_string()
                        } else {
                            self.max_retries.to_string()
                        };
                        info!("Retrying in {} seconds... (attempt {}/{})",
                              self.reconnect_delay,
                              self.current_retries,
                              retry_display);
                        self.sleep_with_interrupt(self.reconnect_delay);
                    }
                    continue;
                }
            };

            match pipeline.set_state(gst::State::Playing) {
                Ok(_) => {
                    info!("Pipeline started successfully, streaming RTSP to SRT...");
                    self.reset_retries();
                },
                Err(e) => {
                    error!("Failed to start pipeline: {}", e);
                    pipeline.set_state(gst::State::Null).ok();
                    self.increment_retries();
                    if self.should_retry() {
                        let retry_display = if self.max_retries == 0 {
                            "∞".to_string()
                        } else {
                            self.max_retries.to_string()
                        };
                        info!("Retrying in {} seconds... (attempt {}/{})",
                              self.reconnect_delay,
                              self.current_retries,
                              retry_display);
                        self.sleep_with_interrupt(self.reconnect_delay);
                    }
                    continue;
                }
            }

            let bus = pipeline.bus().unwrap();
            let mut should_restart = false;

            for msg in bus.iter_timed(gst::ClockTime::NONE) {
                if self.shutdown.load(Ordering::Relaxed) {
                    info!("Shutdown requested, stopping pipeline...");
                    break;
                }

                use gst::MessageView;
                match msg.view() {
                    MessageView::Error(err) => {
                        error!("Pipeline error: {} - {:?}", err.error(), err.debug());
                        should_restart = true;
                        break;
                    }
                    MessageView::Eos(..) => {
                        warn!("RTSP stream ended (EOS), reconnecting...");
                        should_restart = true;
                        break;
                    }
                    MessageView::StateChanged(state) => {
                        if let Some(src) = state.src() {
                            if src.name() == "pipeline0" {
                                debug!("Pipeline state changed: {:?} -> {:?}", state.old(), state.current());
                            }
                        }
                    }
                    MessageView::Warning(_warning) => {
                        warn!("Pipeline warning received");
                    }
                    _ => (),
                }
            }

            pipeline.set_state(gst::State::Null).ok();

            if should_restart && self.should_retry() {
                self.increment_retries();
                let retry_display = if self.max_retries == 0 {
                    "∞".to_string()
                } else {
                    self.max_retries.to_string()
                };
                info!("Retrying in {} seconds... (attempt {}/{})",
                      self.reconnect_delay,
                      self.current_retries,
                      retry_display);
                self.sleep_with_interrupt(self.reconnect_delay);
            } else {
                break;
            }
        }

        Ok(())
    }

    fn sleep_with_interrupt(&self, seconds: u64) {
        let sleep_intervals = seconds * 10; // Check every 100ms
        for _ in 0..sleep_intervals {
            if self.shutdown.load(Ordering::Relaxed) {
                break;
            }
            thread::sleep(Duration::from_millis(100));
        }
    }
}

fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    info!("Starting RTSP to SRT pipeline");
    info!("RTSP source: {}", args.rtsp_url);
    info!("SRT destination: {}", args.srt_url);

    // Initialize GStreamer
    gst::init()?;

    // Set up signal handling
    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_clone = shutdown.clone();

    ctrlc::set_handler(move || {
        info!("Received interrupt signal, shutting down...");
        shutdown_clone.store(true, Ordering::Relaxed);
    })?;

    let mut manager = PipelineManager::new(args, shutdown);

    if let Err(e) = manager.run_pipeline() {
        error!("Pipeline manager error: {}", e);
    }

    info!("RTSP to SRT pipeline stopped");
    Ok(())
}