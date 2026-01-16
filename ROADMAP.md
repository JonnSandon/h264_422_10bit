# Roadmap

## Phase 1 — Bitstream and parameter sets

- [x] Annex B NAL splitting
- [x] NAL header parsing
- [x] Emulation prevention removal
- [x] SPS/PPS parsing skeleton
- [ ] SPS/PPS validation and logging

## Phase 2 — IDR-only decoding

- [ ] Slice header parsing
- [ ] CAVLC implementation
- [ ] Intra prediction
- [ ] Inverse transform
- [ ] Basic frame reconstruction
- [ ] YUV 4:2:2 10-bit buffer representation
- [ ] YUV -> RGB8 conversion for thumbnails

## Phase 3 — CABAC and inter prediction

- [ ] CABAC implementation
- [ ] Motion vector parsing
- [ ] Inter prediction
- [ ] Reference frame management

## Phase 4 — Fields and advanced features

- [ ] Field pictures (PAFF)
- [ ] MBAFF (if needed)
- [ ] Deblocking filter (optional for thumbnails)
