#[cfg(test)]

mod tests_player {

    use std::fs::File;
    use vibe_core::decoder::Decoder;
    use vibe_engine::player::Player;

    #[test]

    fn test_player() {
        let mut player = Player::new();
        let file = File::open("tests/sounds/Test1.mp3").expect("File not found");
        let decoder = Decoder::new(file).expect("Decoding error");
        player.create_stream(decoder);
        player.play_stream();

        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(1000));
            player.pause_stream();
        });

        std::thread::sleep(std::time::Duration::from_millis(3000));
    }
}
