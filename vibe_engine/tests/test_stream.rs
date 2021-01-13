#[cfg(test)]

mod tests_stream {
    use std::fs::File;
    use vibe_core::decoder::Decoder;
    use vibe_engine::stream::OutputStream;

    #[test]

    fn test_stream_mp3() {
        let file = File::open("tests/sounds/Test1.mp3").unwrap();
        let decoder = Decoder::new(file).unwrap();
        let output_stream = OutputStream::new::<f32>(decoder).unwrap();
        output_stream.play();
    }

    #[test]

    fn test_stream_wav() {
        let file = File::open("tests/sounds/Test1.wav").unwrap();
        let decoder = Decoder::new(file).unwrap();
        let output_stream = OutputStream::new::<f32>(decoder).unwrap();
        output_stream.play();
    }

    #[test]

    fn test_stream_ogg() {
        let file = File::open("tests/sounds/Test1.ogg").unwrap();
        let decoder = Decoder::new(file).unwrap();
        let output_stream = OutputStream::new::<f32>(decoder).unwrap();
        output_stream.play();
    }

    #[test]

    fn test_stream_flac() {
        let file = File::open("tests/sounds/Test1.flac").unwrap();
        let decoder = Decoder::new(file).unwrap();
        let output_stream = OutputStream::new::<f32>(decoder).unwrap();
        output_stream.play();
    }
}
