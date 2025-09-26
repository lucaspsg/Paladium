use anyhow::Result;
use gstreamer as gst;
use gstreamer_rtsp_server as gst_rtsp_server;
use gstreamer_rtsp_server::prelude::*;
use std::env;

fn main() -> Result<()> {
    gst::init()?;

    let args: Vec<String> = env::args().collect();
    let video_file = if args.len() > 1 {
        &args[1]
    } else {
        println!("Usage: {} <path_to_mp4_file>", args[0]);
        return Ok(());
    };

    let server = gst_rtsp_server::RTSPServer::new();
    server.set_service("8554");

    let mounts = server.mount_points().unwrap();
    let factory = gst_rtsp_server::RTSPMediaFactory::new();

    let pipeline_str = format!(
        "filesrc location=\"{}\" ! qtdemux name=demux \
     demux.video_0 ! queue ! h264parse ! rtph264pay name=pay0 pt=96 \
     demux.audio_0 ! queue ! aacparse ! rtpmp4gpay name=pay1 pt=97",
        video_file
    );

    factory.set_launch(&pipeline_str);
    factory.set_shared(true);

    mounts.add_factory("/cam1", factory);

    server.attach(None)?;
    println!("RTSP server running at rtsp://localhost:8554/cam1");
    println!("Press Ctrl+C to stop");

    let main_loop = glib::MainLoop::new(None, false);
    main_loop.run();

    Ok(())
}
