pub const FILE_METADATA: bool = false;
pub const MULTITHREADED: bool = true;

pub const MAX_LENGTH: usize = 32767;
pub const MAX_SEQUENCE: usize = 65535;
pub const MAX_PACKET_LENGTH: usize = 1 + 2 + 2 + 2 + 32767;

pub const TYPE_DATA: usize = 1;
pub const TYPE_ACK: usize = 2;
pub const TYPE_FIN: usize = 3;
pub const TYPE_FINACK: usize = 4;
pub const TYPE_MDATA: usize = 5;
pub const TYPE_MACK: usize = 6;

pub const AVAILABLE_TYPES: [usize; 6] = if FILE_METADATA {
    [
        TYPE_DATA,
        TYPE_ACK,
        TYPE_FIN,
        TYPE_FINACK,
        TYPE_MDATA,
        TYPE_MACK,
    ]
} else {
    [TYPE_DATA, TYPE_ACK, TYPE_FIN, TYPE_FINACK, 0, 0]
};

pub const MAX_SENDER_THREADS: usize = if MULTITHREADED { 5 } else { 1 };
pub const RESEND_TIMER: usize = 2;
pub const RESEND_TIMER_TICK: usize = 2;

pub const CHUNK_SIZE: usize = 32767;
pub const STOP_MESSAGE: &str = "BerhentiDong";
