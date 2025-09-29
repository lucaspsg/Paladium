# Paladium Video Pipeline

A production-ready video pipeline that demonstrates video ingestion, re-transport, and serving across heterogeneous networks and clients. This project simulates how Paladium handles video streaming from file sources to various client types.

## Overview

This pipeline consists of three main components:

1. **Pipeline 1** - File → RTSP: Reads MP4 files and serves them over RTSP
2. **Pipeline 2** - RTSP → SRT: Consumes RTSP and publishes to SRT endpoint
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
├── server/                  # Pipeline 3: SRT -> MediaMTX
│   ├── Dockerfile
│   ├── index.html
│   └── mediamtx.yml
├── monitoring/              # Monitoring stack configuration
│   ├── prometheus.yml
│   ├── datasource.yml
│   ├── dashboard-config.yml
│   └── mediamtx-dashboard.json
├── videos/                  # Sample video files
├── docker compose.yml
├── Makefile
└── README.md
```

## Prerequisites

- **Docker & Docker Compose**
- **Rust 1.75+** (for local development)
- **GStreamer** (for local development)
- **VLC Media Player** (for testing)
- **Sample MP4 file** (H.264)

## Quick Start

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
    - **Web UI (chrome-based browsers)**: http://localhost:8080
    - **VLC RTSP**: `rtsp://localhost:8554/cam1`
    - **VLC SRT**: `srt://localhost:8890?streamid=read:cam1`


### MediaMTX Server

**Exposed protocols:**
- **HLS**: http://localhost:8888/cam1/index.m3u8
- **WebRTC**: http://localhost:8889/cam1/whip
- **SRT**: srt://localhost:8890?streamid=read:cam1

**Actuators:**
- **API**: http://localhost:9997/v3/config/global/get (admin/admin)
- **Metrics**: http://localhost:9998/metrics (admin/admin)

## Monitoring

The pipeline includes a simple monitoring stack with Prometheus and Grafana for real-time observability.

### Access Monitoring

- **Grafana Dashboard**: http://localhost:3000 (admin/admin)
- **Prometheus**: http://localhost:9090

### Pre-configured Dashboard

The Grafana dashboard is automatically provisioned on startup and includes:

**Key Metrics:**
- Active Streams - Number of currently running streams
- SRT Connections - Real-time SRT connection count
- WebRTC Sessions - Active WebRTC sessions
- SRT Bandwidth (Mbps) - Incoming bandwidth per connection
- SRT Packet Loss - Network quality indicator

All panels refresh every 5 seconds.

**Default Credentials:**
- Grafana: `admin/admin`
- MediaMTX API: `admin/admin`
- MediaMTX Metrics: `admin/admin`

## Testing

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

1. **Web UI (chrome-based browser)**: http://localhost:8080
    - Tests both WebRTC and HLS connections
    - Real-time connection status

### Resilience Testing

1. **Stop Pipeline 1:**
   ```bash
   docker compose stop pipeline-rtsp
   ```

2. **Observe Pipeline 2 behavior:**
    - Should detect disconnection
    - Should attempt reconnection every 5 seconds
    - Should resume when Pipeline 1 is restarted

3. **Restart Pipeline 1:**
   ```bash
   docker compose start pipeline-rtsp
   ```

4. **Monitor the recovery in Grafana:**
    - Watch connection metrics drop and recover
    - Verify bandwidth returns to normal

## Container Logs

```bash
# View logs
make logs

# View specific service logs
docker compose logs -f pipeline-rtsp
docker compose logs -f mediamtx
docker compose logs -f grafana
```

### Metrics Endpoints

- **MediaMTX Metrics**: http://localhost:9998/metrics
- **Prometheus Targets**: http://localhost:9090/targets
- **Container Stats**: `docker stats`

## Configuration (local development)

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
