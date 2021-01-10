#[cfg(test)]

mod tests_decoder {
    use std::time::Duration;
    use vibe_core::decoder::Decoder;

    #[test]

    fn test_mp3_decoding() {
        // Test on mp3
        let decoder = Decoder::open("tests/sounds/Test1.mp3").unwrap();

        let info = decoder.info();

        assert_eq!("MP3", format!("{}", info.format()));
        assert_eq!(info.sample_rate(), 44100); // Sample rate is just
        assert_eq!(info.channels(), 2); // Number of channels is just

        let mut num_samples: usize = 0;

        let sample_inter = decoder.into_samples().unwrap();
        for sample in sample_inter {
            match sample {
                Ok(_) => num_samples += 1,
                _ => continue,
            }
        }

        // Get approximate duration of the audio
        let duration =
            num_samples as u64 * 1_000 / (info.channels() as u64 * info.sample_rate() as u64);
        assert_eq!(Duration::from_millis(duration), Duration::from_millis(3056));
    }

    #[test]
    fn test_wav_decoding() {
        // Test on wav
        let decoder = Decoder::open("tests/sounds/Test1.wav").unwrap();

        let info = decoder.info();

        assert_eq!("WAV", format!("{}", info.format()));
        assert_eq!(info.sample_rate(), 48000); // Sample rate is just
        assert_eq!(info.channels(), 2); // Number of channels is just

        let mut num_samples: usize = 0;

        let sample_inter = decoder.into_samples().unwrap();
        for sample in sample_inter {
            match sample {
                Ok(_) => num_samples += 1,
                _ => continue,
            }
        }

        let duration =
            num_samples as u64 * 1_000 / (info.channels() as u64 * info.sample_rate() as u64);
        assert_eq!(Duration::from_millis(duration), Duration::from_millis(3000));
    }

    #[test]
    fn test_flac_decoding() {
        // Test on flac
        let decoder = Decoder::open("tests/sounds/Test1.flac").unwrap();

        let info = decoder.info();

        assert_eq!("FLAC", format!("{}", info.format()));
        assert_eq!(info.sample_rate(), 48000); // Sample rate is just
        assert_eq!(info.channels(), 2); // Number of channels is just

        let mut num_samples: usize = 0;

        let sample_inter = decoder.into_samples().unwrap();
        for sample in sample_inter {
            match sample {
                Ok(_) => num_samples += 1,
                _ => continue,
            }
        }

        let duration =
            num_samples as u64 * 1_000 / (info.channels() as u64 * info.sample_rate() as u64);
        assert_eq!(Duration::from_millis(duration), Duration::from_millis(3000));
    }

    #[test]
    fn test_ogg_decoding() {
        // Test on ogg
        let decoder = Decoder::open("tests/sounds/Test1.ogg").unwrap();

        let info = decoder.info();

        assert_eq!("OGG", format!("{}", info.format()));
        assert_eq!(info.sample_rate(), 44100); // Sample rate is just
        assert_eq!(info.channels(), 2); // Number of channels is just

        let mut num_samples: usize = 0;

        let sample_inter = decoder.into_samples().unwrap();
        for sample in sample_inter {
            match sample {
                Ok(_) => num_samples += 1,
                _ => continue,
            }
        }

        let duration =
            num_samples as u64 * 1_000 / (info.channels() as u64 * info.sample_rate() as u64);
        assert_eq!(Duration::from_millis(duration), Duration::from_millis(3000));
    }
}
