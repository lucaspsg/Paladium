/*
    comment teste1
*/
use anyhow::Result;
use clap::Parser;
use gstreamer as gst;
use gstreamer_rtsp_server as gst_rtsp_server;
use gstreamer_rtsp_server::prelude::*;
use std::path::Path;
use tracing::{info, error};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

    #[arg(short, long)]
    file: String,

    #[arg(short, long, default_value = "8554")]
    port: u16,

    #[arg(short, long, default_value = "/cam1")]
    mount: String,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    if !Path::new(&args.file).exists() {
        error!("Video file not found: {}", args.file);
        std::process::exit(1);
    }

    info!("Starting RTSP server for file: {}", args.file);

    gst::init()?;

    let server = gst_rtsp_server::RTSPServer::new();
    server.set_service(&args.port.to_string());

    let mounts = server.mount_points().unwrap();
    let factory = gst_rtsp_server::RTSPMediaFactory::new();

    let pipeline_str = format!(
        "filesrc location=\"{}\" ! qtdemux ! h264parse ! avdec_h264 ! x264enc ! video/x-h264,profile=baseline ! rtph264pay name=pay0 pt=96",
        args.file
    );

    factory.set_launch(&pipeline_str);
    factory.set_shared(true);

    mounts.add_factory(&args.mount, factory);

    server.attach(None)?;

    info!("RTSP server running at rtsp://localhost:{}{}", args.port, args.mount);
    info!("Press Ctrl+C to stop");

    let main_loop = glib::MainLoop::new(None, false);

    let main_loop_clone = main_loop.clone();
    ctrlc::set_handler(move || {
        info!("Received interrupt signal, shutting down...");
        main_loop_clone.quit();
    })?;

    main_loop.run();

    info!("RTSP server stopped");
    Ok(())
}
