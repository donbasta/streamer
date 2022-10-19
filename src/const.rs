const FILE_METADATA: bool = false;
const MULTITHREADED: bool = true;

const MAX_LENGTH: usize = 32767;
const MAX_SEQUENCE: usize = 65535;
const MAX_PACKET_LENGTH: usize = 1 + 2 + 2 + 2 + 32767;

const TYPE_DATA: usize = 0;
const TYPE_ACK: usize = 1;
const TYPE_FIN: usize = 2;
const TYPE_FINACK: usize = 3;
const TYPE_MDATA: usize = 4;
const TYPE_MACK: usize = 5;

const AVAILABLE_TYPES: Vec<usize> = if FILE_METADATA {
    [
        TYPE_DATA,
        TYPE_ACK,
        TYPE_FIN,
        TYPE_FINACK,
        TYPE_MDATA,
        TYPE_MACK,
    ]
} else {
    [TYPE_DATA, TYPE_ACK, TYPE_FIN, TYPE_FINACK]
};

const MAX_SENDER_THREADS: usize = if MULTITHREADED { 5 } else { 1 };
const RESEND_TIMER: usize = 2;
const RESEND_TIMER_TICK: usize = 2;

const CHUNK_SIZE: usize = 32767;
const STOP_MESSAGE: &str = "BerhentiDong";
