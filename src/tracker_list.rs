use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub(crate) static ref TRACKERS: Mutex<Vec<String>> = {
        let mut data: Vec<String> = Vec::new();
        data.push("udp%3A%2F%2Ftracker.openbittorrent.com%3A80".to_string());
        data.push("udp%3A%2F%2Fopentor.org%3A2710".to_string());
        data.push("udp%3A%2F%2Ftracker.ccc.de%3A80".to_string());
        data.push("udp%3A%2F%2Ftracker.blackunicorn.xyz%3A6969".to_string());
        data.push("udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969".to_string());
        data.push("udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969".to_string());

        Mutex::new(data)
    };
}