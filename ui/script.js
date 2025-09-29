let hls = null;
let webrtcPC = null;

// HLS Functions
function startHLS() {
    const video = document.getElementById('hlsVideo');
    const status = document.getElementById('hlsStatus');
    const hlsUrl = 'http://localhost:8888/cam1/index.m3u8';

    status.textContent = 'Connecting...';
    status.className = 'status connecting pulse';

    if (Hls.isSupported()) {
        if (hls) {
            hls.destroy();
        }

        hls = new Hls({
            debug: false,
            enableWorker: true,
            lowLatencyMode: true,
            backBufferLength: 90,
            maxLoadingDelay: 4,
            maxBufferLength: 30,
            maxBufferSize: 60 * 1000 * 1000,
            levelLoadingMaxRetry: 2,
            fragLoadingMaxRetry: 2
        });

        hls.loadSource(hlsUrl);
        hls.attachMedia(video);

        hls.on(Hls.Events.MANIFEST_PARSED, () => {
            console.log('HLS: Manifest parsed, starting playback');
            status.textContent = 'Connected';
            status.className = 'status connected';
            video.play().catch(e => console.log('HLS autoplay prevented:', e));
        });

        hls.on(Hls.Events.ERROR, (event, data) => {
            if (data.fatal) {
                console.error('HLS Fatal Error:', data);
                status.textContent = 'Connection Error';
                status.className = 'status disconnected';

                switch(data.type) {
                    case Hls.ErrorTypes.NETWORK_ERROR:
                        console.log('HLS: Fatal network error, cannot recover');
                        hls.destroy();
                        hls = null;
                        break;
                    case Hls.ErrorTypes.MEDIA_ERROR:
                        console.log('HLS: Media error, trying to recover...');
                        hls.recoverMediaError();
                        break;
                    default:
                        console.log('HLS: Fatal error, cannot recover');
                        hls.destroy();
                        hls = null;
                        break;
                }
            }
        });
    } else if (video.canPlayType('application/vnd.apple.mpegurl')) {
        // Safari native HLS
        video.src = hlsUrl;
        video.addEventListener('loadedmetadata', () => {
            status.textContent = 'Connected';
            status.className = 'status connected';
            video.play().catch(e => console.log('HLS autoplay prevented:', e));
        });

        video.addEventListener('error', () => {
            status.textContent = 'Connection Error';
            status.className = 'status disconnected';
        });
    } else {
        status.textContent = 'HLS Not Supported';
        status.className = 'status disconnected';
    }
}

function stopHLS() {
    const video = document.getElementById('hlsVideo');
    const status = document.getElementById('hlsStatus');

    if (hls) {
        hls.destroy();
        hls = null;
    }

    video.src = '';
    video.load();
    status.textContent = 'Disconnected';
    status.className = 'status disconnected';
}

// WebRTC Functions
async function startWebRTC() {
    const video = document.getElementById('webrtcVideo');
    const status = document.getElementById('webrtcStatus');
    const webrtcUrl = 'http://localhost:8889/cam1/whep';

    status.textContent = 'Connecting...';
    status.className = 'status connecting pulse';

    try {
        // Create RTCPeerConnection
        webrtcPC = new RTCPeerConnection({
            iceServers: [{ urls: 'stun:stun.l.google.com:19302' }]
        });

        // Handle incoming stream
        webrtcPC.ontrack = (event) => {
            console.log('WebRTC: Received stream');
            video.srcObject = event.streams[0];
            status.textContent = 'Connected';
            status.className = 'status connected';
        };

        webrtcPC.onconnectionstatechange = () => {
            console.log('WebRTC connection state:', webrtcPC.connectionState);
            if (webrtcPC.connectionState === 'failed' || webrtcPC.connectionState === 'disconnected') {
                status.textContent = 'Connection Error';
                status.className = 'status disconnected';
            }
        };

        // Add transceiver for receiving video
        webrtcPC.addTransceiver('video', { direction: 'recvonly' });

        // Create offer
        const offer = await webrtcPC.createOffer();
        await webrtcPC.setLocalDescription(offer);

        // Send offer to WebRTC endpoint
        const response = await fetch(webrtcUrl, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/sdp',
            },
            body: offer.sdp
        });

        if (!response.ok) {
            throw new Error(`WebRTC request failed: ${response.status}`);
        }

        const answerSDP = await response.text();
        await webrtcPC.setRemoteDescription({
            type: 'answer',
            sdp: answerSDP
        });

        console.log('WebRTC: Connection established');

    } catch (error) {
        console.error('WebRTC Error:', error);
        status.textContent = 'Connection Error';
        status.className = 'status disconnected';

        if (webrtcPC) {
            webrtcPC.close();
            webrtcPC = null;
        }
    }
}

function stopWebRTC() {
    const video = document.getElementById('webrtcVideo');
    const status = document.getElementById('webrtcStatus');

    if (webrtcPC) {
        webrtcPC.close();
        webrtcPC = null;
    }

    video.srcObject = null;
    status.textContent = 'Disconnected';
    status.className = 'status disconnected';
}

// Manual connection only - no auto-connect
window.addEventListener('load', () => {
    console.log('Page loaded. Click Connect buttons to start streams.');
});

// Cleanup on page unload
window.addEventListener('beforeunload', () => {
    stopHLS();
    stopWebRTC();
});