pub struct DataHeader {
    pub subchunk_2_id: Vec<u8>,
    pub subchunk_2_size: u32
}

impl DataHeader {
    fn new(id: &'static str, size: u32) -> DataHeader {
        DataHeader {
            subchunk_2_id: id.as_bytes().to_vec(),
            subchunk_2_size: size
        }
    }
}

impl Default for DataHeader {
    fn default () -> DataHeader {
        DataHeader{
            subchunk_2_id: br"data".to_vec(),
            subchunk_2_size: 0
        }
    }
}
