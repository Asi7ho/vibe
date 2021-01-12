#[cfg(test)]

mod tests_decoder {
    use std::{io::BufReader, time::Duration};
    use vibe_core::decoder::Decoder;

    #[test]

    fn test_mp3_decoding() {
        // Test on mp3
        let file = std::fs::File::open("tests/sounds/Test1.mp3").unwrap();
        let decoder = Decoder::new(BufReader::new(file)).unwrap();

        let info = decoder.info();

        assert_eq!("MP3", format!("{}", info.format()));
        assert_eq!(info.sample_rate(), 44100); // Sample rate is just
        assert_eq!(info.channels(), 2); // Number of channels is just
        assert_eq!(info.duration(), Some(Duration::from_millis(3042))); //Approximate duration of audio file
    }

    #[test]
    fn test_wav_decoding() {
        // Test on wav
        let file = std::fs::File::open("tests/sounds/Test1.wav").unwrap();
        let decoder = Decoder::new(BufReader::new(file)).unwrap();

        let info = decoder.info();

        assert_eq!("WAV", format!("{}", info.format()));
        assert_eq!(info.sample_rate(), 48000); // Sample rate is just
        assert_eq!(info.channels(), 2); // Number of channels is just
        assert_eq!(info.duration(), Some(Duration::from_millis(3000))); // Duration of audio file is just
    }

    #[test]
    fn test_flac_decoding() {
        // Test on flac
        let file = std::fs::File::open("tests/sounds/Test1.flac").unwrap();
        let decoder = Decoder::new(BufReader::new(file)).unwrap();

        let info = decoder.info();

        assert_eq!("FLAC", format!("{}", info.format()));
        assert_eq!(info.sample_rate(), 48000); // Sample rate is just
        assert_eq!(info.channels(), 2); // Number of channels is just
        assert_eq!(info.duration(), Some(Duration::from_millis(3000))); // Duration of audio file is just
    }

    #[test]
    fn test_ogg_decoding() {
        // Test on ogg
        let file = std::fs::File::open("tests/sounds/Test1.ogg").unwrap();
        let decoder = Decoder::new(BufReader::new(file)).unwrap();

        let info = decoder.info();

        assert_eq!("OGG", format!("{}", info.format()));
        assert_eq!(info.sample_rate(), 44100); // Sample rate is just
        assert_eq!(info.channels(), 2); // Number of channels is just
        assert_eq!(info.duration(), Some(Duration::from_millis(2963))); //Approximate duration of audio file
    }
}
