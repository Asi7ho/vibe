#[cfg(test)]

mod tests_stream {
    use std::fs::File;

    use vibe_engine::player::Player;

    #[test]

    fn test_play() {
        let file = File::open("tests/sounds/Test1.mp3").unwrap();
        let mut player = Player::new().unwrap();

        player.add_to_queue(file);
        player.send_next_to_stream();
    }
}
