# Warning

As of version v1.1.1 the method of connecting to Unitree Go2 via WebRTC changed. If you have the latest firmware version please use legion's version from this repo: https://github.com/legion1581/go2_webrtc_connect


# Usage


## build
```

git clone https://github.com/tfoldi/go2webrtc-rs
cd go2webrtc-rs
cargo build
```

## Execute


Map video stream to an udp port:

```
# if you have security token for your dog:
export GO2_TOKEN="..."


./target/debug/go2webrtc-rs  --robot 192.168.1.1
```

## View

View the stream from ffmpeg
```
ffplay -i connection.sdp -protocol_whitelist file,udp,rtp  -flags low_delay  -probesize 32  -vf setpts=0
```

View camera from opencv

```
export OPENCV_FFMPEG_CAPTURE_OPTIONS="protocol_whitelist;file,rtp,udp,flags;low_delay,probesize;32,vf;setpts=0"
python go2cv.py

```

# License

BSDL-2
