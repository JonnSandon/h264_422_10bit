# h264_422_10bit

Decode-only, analysis-focused H.264/AVC 4:2:2 10-bit decoder in Rust.

Goals:

- Pure Rust, no FFmpeg bindings
- Non-realtime decoding for analysis and tooling
- Support for High 4:2:2 profiles and 10-bit depth
- Frame/field extraction and thumbnail generation

Status: early skeleton. API and internals are subject to change.
