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

COPY --from=builder /app/target/release/pipeline-rtsp /app/pipeline-rtsp

RUN mkdir -p /app/videos

EXPOSE 8554

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD netstat -ln | grep :8554 || exit 1

CMD ["./pipeline-rtsp", "--file", "/app/videos/sample.mp4", "--port", "8554", "--mount", "/cam1"]