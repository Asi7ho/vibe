#[cfg(test)]

mod tests_stream {
    use std::fs::File;
    use vibe_core::decoder::Decoder;
    use vibe_engine::stream::AudioStream;

    #[test]

    fn test_stream_mp3() {
        let file = File::open("tests/sounds/Test1.mp3").expect("File not found");
        let decoder = Decoder::new(file).expect("Decoding error");
        let info = decoder.info();

        let duration = info.duration().unwrap();

        let audio_stream = AudioStream::new::<f32, File>(decoder).expect("Stream error");

        audio_stream.play();

        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(500));
            audio_stream.pause();
            std::thread::sleep(std::time::Duration::from_millis(500));
            audio_stream.play();
            std::thread::sleep(std::time::Duration::from_millis(500));
            audio_stream.stop();
        });

        std::thread::sleep(duration);
    }

    #[test]

    fn test_stream_wav() {
        let file = File::open("tests/sounds/Test1.wav").expect("File not found");
        let decoder = Decoder::new(file).expect("Decoding error");
        let info = decoder.info();

        let duration = info.duration().unwrap();

        let audio_stream = AudioStream::new::<f32, File>(decoder).expect("Stream error");

        audio_stream.play();

        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(500));
            audio_stream.pause();
            std::thread::sleep(std::time::Duration::from_millis(500));
            audio_stream.play();
            std::thread::sleep(std::time::Duration::from_millis(500));
            audio_stream.stop();
        });

        std::thread::sleep(duration);
    }

    #[test]

    fn test_stream_ogg() {
        let file = File::open("tests/sounds/Test1.ogg").expect("File not found");
        let decoder = Decoder::new(file).expect("Decoding error");
        let info = decoder.info();

        let duration = info.duration().unwrap();

        let audio_stream = AudioStream::new::<f32, File>(decoder).expect("Stream error");

        audio_stream.play();

        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(500));
            audio_stream.pause();
            std::thread::sleep(std::time::Duration::from_millis(500));
            audio_stream.play();
            std::thread::sleep(std::time::Duration::from_millis(500));
            audio_stream.stop();
        });

        std::thread::sleep(duration);
    }

    #[test]

    fn test_stream_flac() {
        let file = File::open("tests/sounds/Test1.flac").expect("File not found");
        let decoder = Decoder::new(file).expect("Decoding error");
        let info = decoder.info();

        let duration = info.duration().unwrap();

        let audio_stream = AudioStream::new::<f32, File>(decoder).expect("Stream error");

        audio_stream.play();

        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(500));
            audio_stream.pause();
            std::thread::sleep(std::time::Duration::from_millis(500));
            audio_stream.play();
            std::thread::sleep(std::time::Duration::from_millis(500));
            audio_stream.stop();
        });

        std::thread::sleep(duration);
    }
}
