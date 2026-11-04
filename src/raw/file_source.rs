//! src/raw/file_source.rs - FileSource (pcap-файл, P3.37 часть 2).

use pcap::Capture;
use super::source::RawSource;

pub struct FileSource {
    name: String,
    cap: Capture<pcap::Offline>,
}

impl FileSource {
    pub fn open(path: &str) -> Result<Self, String> {
        let cap = Capture::from_file(path).map_err(|e| e.to_string())?;
        Ok(Self { name: path.to_string(), cap })
    }
}

impl RawSource for FileSource {
    fn name(&self) -> &str { &self.name }
    fn next(&mut self) -> Result<Option<Vec<u8>>, String> {
        match self.cap.next_packet() {
            Ok(pkt) => Ok(Some(pkt.data.to_vec())),
            Err(pcap::Error::NoMorePackets) => Ok(None),
            Err(e) => Err(e.to_string()),
        }
    }
}
