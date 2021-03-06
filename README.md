# vibe

A basic music player for .flac, .wav, .mp3, .ogg files

The app has been tested on MacOs only.

## Build/Run

To build the app:
`cargo build` in root


To run the app:
`cargo run` in root


To have an executable file, you need `cargo bundle`

Navigate to vibe_gui and type command: 
`cargo bundle` (for debug) or 
`cargo bundle --release` (for release)

## Things to do
- [x] Implement mp3 decoder
- [x] Implement wav decoder
- [x] Implement flac decoder
- [x] Implement ogg decoder
- [x] Implement stream to default output device
- [x] Implement player
- [x] Construct basic ui for player
- [] Find a better way to compute audio duration for mp3 and ogg files
- [] Implement a way to know the current time in the stream
- [] Find a way to update progress bar for time progression in audio
- [x] Implement file selection
- [x] Do Error engineering
- [] Test on other platform