#[cfg(test)]

mod tests_player {

    use vibe_engine::player::Player;

    #[test]

    fn test_player() {
        let mut player = Player::new().unwrap();
        player.create_stream("tests/sounds/Test1.mp3");
        player.play_stream();

        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(1000));
            player.pause_stream();
        });

        std::thread::sleep(std::time::Duration::from_millis(3000));
    }
}
