# Paladium Video Pipeline

A production-ready video pipeline that demonstrates video ingestion, re-transport, and serving across heterogeneous networks and clients. This project simulates how Paladium handles video streaming from file sources to various client types.

## 🎯 Overview

This pipeline consists of three main components:

1. **Pipeline 1** - File → RTSP: Reads MP4 files and serves them over RTSP
2. **Pipeline 2** - RTSP → SRT: Consumes RTSP and publishes to SRT server
3. **Pipeline 3** - SRT/WebRTC/HLS Server: MediaMTX server exposing multiple protocols

## 🏗️ Architecture

```
[MP4 File] → [Pipeline 1: RTSP Server] → [Pipeline 2: RTSP→SRT] → [MediaMTX Server]
                                                                    ↓
                                                              [WebRTC/HLS/SRT]
                                                                    ↓
                                                              [VLC/Browser Clients]
```

## 📋 Prerequisites

- **Docker & Docker Compose** (recommended)
- **Rust 1.75+** (for development)
- **GStreamer** (for local development)
- **VLC Media Player** (for testing)
- **Sample MP4 file** (H.264 preferred)

## 🚀 Quick Start

### Option 1: Docker Compose (Recommended)

1. **Clone and setup:**
   ```bash
   git clone <repository>
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
   # or
   docker-compose up -d
   ```

4. **Test the pipeline:**
   - **Web UI**: http://localhost:8080
   - **VLC RTSP**: `rtsp://localhost:8554/cam1`
   - **VLC SRT**: `srt://localhost:8890?streamid=read:cam1`

### Option 2: Development Mode

1. **Build the Rust binaries:**
   ```bash
   make dev-build
   ```

2. **Start MediaMTX server:**
   ```bash
   docker run -d --name mediamtx \
     -p 8888:8888 -p 8889:8889 -p 8890:8890 \
     -p 9997:9997 -p 9998:9998 \
     -v $(pwd)/server/mediamtx.yml:/mediamtx.yml \
     bluenviron/mediamtx:latest /mediamtx.yml
   ```

3. **Run Pipeline 1 (RTSP):**
   ```bash
   make dev-run-pipeline1
   ```

4. **Run Pipeline 2 (RTSP→SRT):**
   ```bash
   make dev-run-pipeline2
   ```

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
  --loop-video
```

**Features:**
- Supports MP4, AVI, MOV, and other GStreamer-compatible formats
- H.264 encoding with baseline profile
- Configurable port and mount point
- Video looping option
- Graceful shutdown handling

### Pipeline 2 (RTSP→SRT)

```bash
# Command line options
./pipeline-rtsp-to-srt --help

# Example usage
./pipeline-rtsp-to-srt \
  --rtsp-url rtsp://localhost:8554/cam1 \
  --srt-url srt://localhost:8890?streamid=publish:cam1 \
  --reconnect-delay 5 \
  --max-retries 0
```

**Features:**
- Automatic reconnection with configurable backoff
- Error handling and logging
- Configurable retry limits
- Real-time stream monitoring

### MediaMTX Server

**Configuration file:** `server/mediamtx.yml`

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
   - Copy-to-clipboard URLs

2. **Direct HLS**: http://localhost:8888/cam1/index.m3u8
   - Open in browser or VLC
   - Low-latency HLS configuration

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
# View all containers
make status

# View logs
make logs

# View specific service logs
make logs-rtsp
make logs-srt
make logs-mediamtx
```

### Health Checks

All containers include health checks:
- **Pipeline 1**: Port 8554 availability
- **Pipeline 2**: Port 8890 availability  
- **MediaMTX**: API endpoint responsiveness

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

## 🛠️ Development

### Project Structure

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
├── server/                  # Pipeline 3: MediaMTX Server
│   ├── mediamtx.yml
│   ├── index.html
│   └── Dockerfile
├── videos/                  # Sample video files
├── docker-compose.yml
├── Makefile
└── README.md
```

### Building from Source

```bash
# Build all Rust binaries
cargo build --release

# Build specific pipeline
cargo build --release --bin pipeline-rtsp
cargo build --release --bin pipeline-rtsp-to-srt
```

### Adding New Features

1. **New video formats**: Modify GStreamer pipeline strings
2. **Additional protocols**: Extend MediaMTX configuration
3. **Monitoring**: Add Prometheus metrics
4. **Authentication**: Implement RTSP/SRT authentication

## 🐛 Troubleshooting

### Common Issues

1. **"No such file or directory" for video file**
   ```bash
   # Ensure video file exists
   ls -la ./videos/sample.mp4
   ```

2. **Port conflicts**
   ```bash
   # Check port usage
   netstat -tulpn | grep :8554
   ```

3. **GStreamer plugin errors**
   ```bash
   # Install missing plugins
   sudo apt-get install gstreamer1.0-plugins-*
   ```

4. **Container startup failures**
   ```bash
   # Check container logs
   docker-compose logs pipeline-rtsp
   ```

### Performance Tuning

1. **Reduce latency:**
   - Decrease HLS segment duration
   - Use WebRTC for real-time
   - Optimize GStreamer pipeline

2. **Improve reliability:**
   - Increase SRT buffer size
   - Implement retry logic
   - Add health monitoring

3. **Scale horizontally:**
   - Use load balancers
   - Implement CDN
   - Add multiple MediaMTX instances

## 📚 References

- [GStreamer Documentation](https://gstreamer.freedesktop.org/documentation/)
- [MediaMTX Documentation](https://github.com/bluenviron/mediamtx)
- [SRT Protocol](https://github.com/Haivision/srt)
- [WebRTC Standards](https://webrtc.org/)
- [HLS Specification](https://tools.ietf.org/html/rfc8216)

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## 📞 Support

For issues and questions:
- Create an issue on GitHub
- Check the troubleshooting section
- Review container logs for errors
