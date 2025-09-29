FROM rust:1.85 AS builder

RUN apt update && apt install -y \
    libgstreamer1.0-dev \
    libgstreamer-plugins-base1.0-dev \
    libgstreamer-plugins-bad1.0-dev \
    libgstrtspserver-1.0-dev \
    pkg-config \
    gstreamer1.0-plugins-base \
    gstreamer1.0-plugins-good \
    gstreamer1.0-plugins-bad \
    gstreamer1.0-plugins-ugly \
    gstreamer1.0-libav \
    gstreamer1.0-tools \
    gstreamer1.0-x \
    gstreamer1.0-alsa \
    gstreamer1.0-gl \
    gstreamer1.0-gtk3 \
    gstreamer1.0-qt5 \
    gstreamer1.0-pulseaudio

WORKDIR /app

COPY Cargo.toml ./
COPY src ./src

RUN cargo build --release

# Runtime stage
FROM restreamio/gstreamer:latest-prod

# Suppress plugin warnings
ENV GST_DEBUG=1
ENV GST_REGISTRY_FORK=no

WORKDIR /app

COPY --from=builder /app/target/release/pipeline-rtsp-to-srt /app/pipeline-rtsp-to-srt

EXPOSE 8890

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD netstat -ln | grep :8890 || exit 1

CMD ["./pipeline-rtsp-to-srt", "--rtsp-url", "rtsp://pipeline-rtsp:8554/cam1", "--srt-url", "srt://mediamtx:8890?streamid=publish:cam1", "--reconnect-delay", "5"]