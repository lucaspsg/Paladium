.PHONY: help build run stop clean demo

help:
	@echo "Paladium Video Pipeline - Available commands:"
	@echo ""
	@echo "  make demo      - Run the complete demo"
	@echo "  make stop      - Stop all containers"
	@echo "  make logs      - Output container logs"
	@echo "  make clean     - Remove all containers and images"
	@echo "  make build     - Build all Docker containers"
	@echo "  make run       - Start the entire pipeline"
	@echo ""
	@echo "Prerequisites:"
	@echo "  - Docker and Docker Compose"
	@echo "  - A sample MP4 file in ./videos/sample.mp4"
	@echo "  - VLC media player for testing"

build:
	@echo "Building Paladium video pipeline containers..."
	docker compose build

run:
	@echo "Starting Paladium video pipeline..."
	docker compose up -d
	@echo ""
	@echo "Pipeline started! Access points:"
	@echo "  Web UI:        http://localhost:8080"
	@echo "  HLS:           http://localhost:8888/cam1/index.m3u8"
	@echo "  WebRTC:        http://localhost:8889/cam1/whip"
	@echo "  SRT:           srt://localhost:8890?streamid=read:cam1"
	@echo "  RTSP:          rtsp://localhost:8554/cam1"
	@echo "  MediaMTX API:  http://localhost:9997/v3/config/global/get"
	@echo "  Metrics:       http://localhost:9998/metrics"

stop:
	@echo "Stopping Paladium video pipeline..."
	docker compose down

clean:
	@echo "Cleaning up Paladium video pipeline..."
	docker compose down --rmi all --volumes --remove-orphans

demo: build run
	@echo ""
	@echo "Demo is running! Here's how to test:"
	@echo ""
	@echo "1. Open VLC and test these URLs:"
	@echo "   - RTSP: rtsp://localhost:8554/cam1"
	@echo "   - SRT:  srt://localhost:8890?streamid=read:cam1"
	@echo ""
	@echo "2. Open your browser (chrome-based) and go to:"
	@echo "   - Web UI: http://localhost:8080"
	@echo ""
	@echo "3. Check pipeline status:"
	@echo "   - API: http://localhost:9997/v3/config/global/get"
	@echo "   - Metrics: http://localhost:9998/metrics"
	@echo ""
	@echo "Press Ctrl+C to stop the demo"

logs:
	docker compose logs -f