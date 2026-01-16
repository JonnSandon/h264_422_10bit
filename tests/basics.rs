use h264_422_10bit::H264Decoder;

#[test]
fn crate_compiles_and_decoder_constructs() {
    let mut decoder = H264Decoder::new();
    let data: &[u8] = &[];
    let frames = decoder.push_annex_b(data).unwrap();
    assert!(frames.is_empty());
}
