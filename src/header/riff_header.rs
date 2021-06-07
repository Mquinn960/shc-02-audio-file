pub struct RiffHeader {
    pub chunk_id: Vec<u8>,
    pub chunk_size: u32,
    pub format: Vec<u8>,
}

impl RiffHeader {

    fn new(id: &'static str, size: u32, format: &'static str) -> RiffHeader {
        RiffHeader {
            chunk_id: id.as_bytes().to_vec(),
            chunk_size: size,
            format: format.as_bytes().to_vec()
        }
    }

}

impl Default for RiffHeader {
    fn default () -> RiffHeader {
        RiffHeader{
            chunk_id: br"NONE".to_vec(),
            chunk_size: 0,
            format: br"NONE".to_vec()
        }
    }
}