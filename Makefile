# Paladium Video Pipeline Makefile

.PHONY: help build run stop clean demo test-vlc test-browser

# Default target
help:
	@echo "Paladium Video Pipeline - Available commands:"
	@echo ""
	@echo "  make build     - Build all Docker containers"
	@echo "  make run       - Start the entire pipeline"
	@echo "  make stop      - Stop all containers"
	@echo "  make clean     - Remove all containers and images"
	@echo "  make demo      - Run the complete demo"
	@echo "  make test-vlc  - Test VLC connections"
	@echo "  make test-browser - Test browser connections"
	@echo ""
	@echo "Prerequisites:"
	@echo "  - Docker and Docker Compose"
	@echo "  - A sample MP4 file in ./videos/sample.mp4"
	@echo "  - VLC media player for testing"

# Build all containers
build:
	@echo "Building Paladium video pipeline containers..."
	docker-compose build

# Run the complete pipeline
run:
	@echo "Starting Paladium video pipeline..."
	docker-compose up -d
	@echo ""
	@echo "Pipeline started! Access points:"
	@echo "  Web UI:        http://localhost:8080"
	@echo "  HLS:           http://localhost:8888/cam1/index.m3u8"
	@echo "  WebRTC:        http://localhost:8889/cam1/whip"
	@echo "  SRT:           srt://localhost:8890?streamid=read:cam1"
	@echo "  RTSP:          rtsp://localhost:8554/cam1"
	@echo "  MediaMTX API:  http://localhost:9997/v3/config/global/get"
	@echo "  Metrics:       http://localhost:9998/metrics"

# Stop all containers
stop:
	@echo "Stopping Paladium video pipeline..."
	docker-compose down

# Clean up containers and images
clean:
	@echo "Cleaning up Paladium video pipeline..."
	docker-compose down --rmi all --volumes --remove-orphans

# Run the complete demo
demo: build run
	@echo ""
	@echo "Demo is running! Here's how to test:"
	@echo ""
	@echo "1. Open VLC and test these URLs:"
	@echo "   - RTSP: rtsp://localhost:8554/cam1"
	@echo "   - SRT:  srt://localhost:8890?streamid=read:cam1"
	@echo ""
	@echo "2. Open your browser and go to:"
	@echo "   - Web UI: http://localhost:8080"
	@echo ""
	@echo "3. Check pipeline status:"
	@echo "   - API: http://localhost:9997/v3/config/global/get"
	@echo "   - Metrics: http://localhost:9998/metrics"
	@echo ""
	@echo "Press Ctrl+C to stop the demo"

# Test VLC connections
test-vlc:
	@echo "Testing VLC connections..."
	@echo "Open VLC and try these URLs:"
	@echo "  RTSP: rtsp://localhost:8554/cam1"
	@echo "  SRT:  srt://localhost:8890?streamid=read:cam1"
	@echo ""
	@echo "If VLC doesn't open automatically, copy the URLs above."

# Test browser connections
test-browser:
	@echo "Testing browser connections..."
	@echo "Open your browser and go to: http://localhost:8080"
	@echo "The web interface will test both WebRTC and HLS connections."

# Development commands
dev-build:
	@echo "Building Rust binaries for development..."
	cargo build --release

dev-run-pipeline1:
	@echo "Running Pipeline 1 (RTSP) in development mode..."
	cargo run --bin pipeline-rtsp -- --file ./videos/sample.mp4 --port 8554 --mount /cam1 --loop-video

dev-run-pipeline2:
	@echo "Running Pipeline 2 (RTSP to SRT) in development mode..."
	cargo run --bin pipeline-rtsp-to-srt -- --rtsp-url rtsp://localhost:8554/cam1 --srt-url srt://localhost:8890?streamid=publish:cam1

# Logs
logs:
	docker-compose logs -f

logs-rtsp:
	docker-compose logs -f pipeline-rtsp

logs-srt:
	docker-compose logs -f pipeline-rtsp-to-srt

logs-mediamtx:
	docker-compose logs -f mediamtx

# Status
status:
	@echo "Pipeline status:"
	docker-compose ps
	@echo ""
	@echo "Container health:"
	docker-compose ps --format "table {{.Name}}\t{{.Status}}\t{{.Ports}}"
