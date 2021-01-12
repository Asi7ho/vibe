use std::{
    io::Read,
    io::{Seek, SeekFrom},
};

use std::time::Duration;

use lewton::inside_ogg::OggStreamReader;

use crate::{AudioFormat, AudioInfo, Sample};

pub struct VorbisDecoder<R>
where
    R: Read + Seek,
{
    reader: OggStreamReader<R>,
    channels: usize,
    sample_rate: u32,
    duration: Option<Duration>,
    current_packet: Option<Vec<i16>>,
    packet_cursor: usize,
}

impl<R> VorbisDecoder<R>
where
    R: Read + Seek,
{
    #[inline]
    pub fn new(mut data: R) -> Result<Self, R> {
        if !is_ogg(data.by_ref()) {
            return Err(data);
        }

        // TODO: Is there a better way to compute duration
        let duration = compute_duration(data.by_ref());

        let mut reader = OggStreamReader::new(data).unwrap();

        let channels = reader.ident_hdr.audio_channels as usize;
        let sample_rate = reader.ident_hdr.audio_sample_rate;
        let current_packet = reader.read_dec_packet_itl().unwrap();
        let packet_cursor = 0;

        Ok(Self {
            reader,
            channels,
            sample_rate,
            duration,
            current_packet,
            packet_cursor,
        })
    }

    /// Get duration audio file
    #[inline]
    fn duration(&self) -> Option<Duration> {
        self.duration
    }

    #[inline]
    pub fn info(&self) -> AudioInfo {
        AudioInfo {
            format: AudioFormat::Ogg,
            sample_rate: self.sample_rate,
            channels: self.channels,
            duration: self.duration(),
        }
    }
}

impl<R> Iterator for VorbisDecoder<R>
where
    R: Read + Seek,
{
    type Item = Result<Sample, ()>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(packet) = self.current_packet.as_ref() {
            match packet.get(self.packet_cursor) {
                Some(sample) => {
                    // Increment the cursor
                    self.packet_cursor += 1;

                    let sample = *sample;

                    // Get the next packet if done reading this one
                    if self.packet_cursor >= packet.len() {
                        self.packet_cursor = 0;
                        self.current_packet = match self.reader.read_dec_packet_itl() {
                            Ok(packet) => packet,
                            Err(_) => return Some(Err(())),
                        };
                    }

                    // Convert the sample and return it
                    let sample_float = sample as f32 / i16::MAX as f32;
                    return Some(Ok(sample_float));
                }
                None => {
                    self.packet_cursor = 0;
                    self.current_packet = match self.reader.read_dec_packet_itl() {
                        Ok(packet) => packet,
                        Err(_) => return Some(Err(())),
                    };
                    continue;
                }
            }
        }
        None
    }
}

fn is_ogg<R>(mut data: R) -> bool
where
    R: Read + Seek,
{
    let stream_pos = data.seek(SeekFrom::Current(0)).unwrap();
    let is_ogg = OggStreamReader::new(data.by_ref()).is_ok();
    data.seek(SeekFrom::Start(stream_pos)).unwrap();

    return is_ogg;
}

/// Compute duration
fn compute_duration<R>(mut data: R) -> Option<Duration>
where
    R: Read + Seek,
{
    let stream_pos = data.seek(SeekFrom::Current(0)).unwrap();

    let mut reader = OggStreamReader::new(data.by_ref()).unwrap();
    let channels = reader.ident_hdr.audio_channels as u64;
    let sample_rate = reader.ident_hdr.audio_sample_rate as u64;
    let sample_channel = channels * sample_rate;

    let mut duration: u64 = 0;
    while let Some(pck_samples) = reader.read_dec_packet_itl().unwrap() {
        duration += pck_samples.len() as u64 * 1_000 / sample_channel;
    }

    data.seek(SeekFrom::Start(stream_pos)).unwrap();
    return Some(Duration::from_millis(duration));
}
