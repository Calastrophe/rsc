#[derive(Debug, Clone, Copy)]
pub enum Message {
    Run,
    Tick,
    Pause,
    Resume,
    Restart,
    StepOver,
    StepForward,
    StepBackward,
    SetBreakpoint(u32),
    RemoveBreakpoint(u32),
}
