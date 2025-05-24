pub mod timer_manager;
pub mod focus_timer;
pub mod long_break_timer;
pub mod micro_break_timer;
pub mod timer_state;

// 重新导出主要的公共接口
pub use timer_manager::{TimerManager, CycleState};
pub use timer_state::{TimerState, SessionPhase, TimerEvent};
pub use focus_timer::FocusTimer;
pub use long_break_timer::LongBreakTimer;
pub use micro_break_timer::{
    MicroBreakTimer, 
    EnhancedMicroBreakScheduler, 
    MicroBreakStats,
    // 保持向后兼容
    EnhancedMicroBreakScheduler as MicroBreakScheduler
}; 