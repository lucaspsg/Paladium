# Paladium Video Pipeline

A production-ready video pipeline that demonstrates video ingestion, re-transport, and serving across heterogeneous networks and clients. This project simulates how Paladium handles video streaming from file sources to various client types.

## 🎯 Overview

This pipeline consists of three main components:

1. **Pipeline 1** - File → RTSP: Reads MP4 files and serves them over RTSP
2. **Pipeline 2** - RTSP → SRT: Consumes RTSP and publishes to SRT server
3. **Pipeline 3** - SRT/WebRTC/HLS Server: MediaMTX server exposing multiple protocols

## Project Structure

```
paladium/
├── pipeline-rtsp/           # Pipeline 1: File → RTSP
│   ├── src/main.rs
│   ├── Cargo.toml
│   └── Dockerfile
├── pipeline-rtsp-to-srt/    # Pipeline 2: RTSP → SRT
│   ├── src/main.rs
│   ├── Cargo.toml
│   └── Dockerfile
├── ui/                      # User Interface: Interacts with Pipeline3 (MediaMTX server)
│   ├── index.html
├── videos/                  # Sample video files
├── docker-compose.yml       # MediaMTX server and container coordenation
├── Makefile
└── README.md
```

## 📋 Prerequisites

- **Docker & Docker Compose**
- **Rust 1.75+** (for development)
- **GStreamer** (for local development)
- **VLC Media Player** (for testing)
- **Sample MP4 file** (H.264)

## 🚀 Quick Start

1. **Clone and setup:**
   ```bash
   git clone git@github.com:lucaspsg/Paladium.git
   cd paladium
   ```

2. **Add a sample video:**
   ```bash
   # Place your MP4 file in the videos directory
   cp /path/to/your/video.mp4 ./videos/sample.mp4
   ```

3. **Run the complete pipeline:**
   ```bash
   make demo
   ```

4. **Test the pipeline:**
   - **Web UI**: http://localhost:8080
   - **VLC RTSP**: `rtsp://localhost:8554/cam1`
   - **VLC SRT**: `srt://localhost:8890?streamid=read:cam1`

## 🔧 Configuration

### Pipeline 1 (RTSP Server)

```bash
# Command line options
./pipeline-rtsp --help

# Example usage
./pipeline-rtsp \
  --file /path/to/video.mp4 \
  --port 8554 \
  --mount /cam1 \
```

**Features:**
- Supports MP4, AVI, MOV, and other GStreamer-compatible formats
- H.264 encoding with baseline profile
- Configurable port and mount point

### Pipeline 2 (RTSP→SRT)

```bash
# Command line options
./pipeline-rtsp-to-srt --help

# Example usage
./pipeline-rtsp-to-srt \
  --rtsp-url rtsp://localhost:8554/cam1 \
  --srt-url srt://localhost:8890?streamid=publish:cam1 \
  --reconnect-delay 5 \
```

**Features:**
- Automatic reconnection with configurable backoff
- Error handling and logging
- Real-time stream monitoring

### MediaMTX Server

**Exposed protocols:**
- **HLS**: http://localhost:8888/cam1/index.m3u8
- **WebRTC**: http://localhost:8889/cam1/whip
- **SRT**: srt://localhost:8890?streamid=read:cam1

**Monitoring:**
- **API**: http://localhost:9997/v3/config/global/get
- **Metrics**: http://localhost:9998/metrics

## 🧪 Testing

### VLC Testing

1. **RTSP Stream:**
   ```
   Open VLC → Media → Open Network Stream
   URL: rtsp://localhost:8554/cam1
   ```

2. **SRT Stream:**
   ```
   Open VLC → Media → Open Network Stream
   URL: srt://localhost:8890?streamid=read:cam1
   ```

### Browser Testing

1. **Web UI**: http://localhost:8080
   - Tests both WebRTC and HLS connections
   - Real-time connection status

### Resilience Testing

1. **Stop Pipeline 1:**
   ```bash
   docker-compose stop pipeline-rtsp
   ```

2. **Observe Pipeline 2 behavior:**
   - Should detect disconnection
   - Should attempt reconnection every 5 seconds
   - Should resume when Pipeline 1 is restarted

3. **Restart Pipeline 1:**
   ```bash
   docker-compose start pipeline-rtsp
   ```

## 📊 Monitoring

### Container Status

```bash
# View logs
make logs
```

### Metrics

- **MediaMTX Metrics**: http://localhost:9998/metrics
- **Container Stats**: `docker stats`

## 🔍 Protocol Comparison

### RTSP vs SRT

| Feature | RTSP | SRT |
|---------|------|-----|
| **Latency** | Low (100-500ms) | Very Low (50-200ms) |
| **Reliability** | Good | Excellent (ARQ) |
| **Firewall** | Complex (multiple ports) | Simple (single port) |
| **Encryption** | Optional | Built-in |
| **Use Case** | Local networks | Internet/WAN |

**When to choose RTSP:**
- Local network streaming
- Legacy system compatibility
- Simple setup requirements

**When to choose SRT:**
- Internet/WAN streaming
- High reliability requirements
- Firewall-friendly deployment
- Low latency needs

### WebRTC vs HLS

| Feature | WebRTC | HLS |
|---------|--------|-----|
| **Latency** | Very Low (50-200ms) | Medium (2-10s) |
| **Browser Support** | Modern browsers | Universal |
| **Scalability** | Limited | Excellent |
| **CDN Support** | Limited | Excellent |
| **Use Case** | Real-time communication | Content delivery |

**When to choose WebRTC:**
- Real-time applications
- Interactive streaming
- Low latency requirements
- Modern browser environments

**When to choose HLS:**
- Content delivery networks
- Maximum compatibility
- Scalable distribution
- Mobile applications
