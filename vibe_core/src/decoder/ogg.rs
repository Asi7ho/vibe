use std::{fs::File, io::Read, io::Seek, path::Path};

use lewton::inside_ogg::OggStreamReader;

use crate::{AudioFormat, AudioInfo, Sample};

pub struct VorbisDecoder {
    reader: OggStreamReader<File>,
    channels: usize,
    sample_rate: u32,
}

impl VorbisDecoder {
    #[inline]
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, ()> {
        let f = File::open(path).map_err(|_| ())?;
        let reader = match OggStreamReader::new(f) {
            Ok(reader) => reader,
            Err(_) => return Err(()),
        };

        let channels = reader.ident_hdr.audio_channels as usize;
        let sample_rate = reader.ident_hdr.audio_sample_rate;

        Ok(Self {
            reader: reader,
            channels: channels,
            sample_rate: sample_rate,
        })
    }

    #[inline]
    pub fn info(&self) -> AudioInfo {
        AudioInfo {
            format: AudioFormat::Ogg,
            sample_rate: self.sample_rate,
            channels: self.channels,
        }
    }

    #[inline]
    pub fn into_samples(mut self) -> Result<Box<dyn Iterator<Item = Result<Sample, ()>>>, ()> {
        let current_packet = self.reader.read_dec_packet_itl().map_err(|_| ())?;

        Ok(Box::new(OggSampleIterator {
            reader: self.reader,
            current_packet: current_packet,
            packet_cursor: 0,
        }))
    }
}

struct OggSampleIterator<T: Read + Seek> {
    reader: OggStreamReader<T>,
    current_packet: Option<Vec<i16>>,
    packet_cursor: usize,
}

impl<T: Read + Seek> OggSampleIterator<T> {
    #[inline(always)]
    fn next_packet(&mut self) -> Result<(), ()> {
        self.packet_cursor = 0;
        self.current_packet = match self.reader.read_dec_packet_itl() {
            Ok(packet) => packet,
            Err(_) => return Err(()),
        };
        Ok(())
    }
}

impl<T: Read + Seek> Iterator for OggSampleIterator<T> {
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
                        if let Err(err) = self.next_packet() {
                            return Some(Err(err));
                        }
                    }

                    // Convert the sample and return it
                    let sample_float = sample as f32 / i16::MAX as f32;
                    return Some(Ok(sample_float));
                }
                None => {
                    if let Err(err) = self.next_packet() {
                        return Some(Err(err));
                    }
                    continue;
                }
            }
        }
        None
    }
}
