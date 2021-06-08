pub struct FmtHeader {
    pub subchunk_1_id: Vec<u8>,
    pub subchunk_1_size: u32,
    pub audio_format: u16,
    pub num_channels: u16,
    pub sample_rate: u32,
    pub byte_rate: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
}

impl FmtHeader {
    fn new(id: &'static str, 
           size: u32,
           audio_format: u16,
           num_channels: u16,
           sample_rate: u32,
           byte_rate: u32,
           block_align: u16,
           bits_per_sample: u16
        ) -> FmtHeader {
        FmtHeader {
            subchunk_1_id: id.as_bytes().to_vec(),
            subchunk_1_size: size,
            audio_format: audio_format,
            num_channels: num_channels,
            sample_rate: sample_rate,
            byte_rate: byte_rate,
            block_align: block_align,
            bits_per_sample: bits_per_sample
        }
    }
}

impl Default for FmtHeader {
    fn default () -> FmtHeader {
        FmtHeader{
            subchunk_1_id: br"fmt ".to_vec(),
            subchunk_1_size: 16,
            audio_format: 1,
            num_channels: 1,
            sample_rate: 44100,
            byte_rate: 88200,
            block_align: 2,
            bits_per_sample: 16
        }
    }
}