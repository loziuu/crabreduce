#[derive(Debug, PartialEq, Eq)]
pub enum WorkerState {
    Detached,
    Idle,
    Busy,
    BusyFull,
    Finished,
}
