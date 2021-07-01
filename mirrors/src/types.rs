#[derive(Debug, Clone, Copy)]
pub enum TrackerState {
    NotInitialized,
    Initializing,
    Tracking,
    Lost,
}