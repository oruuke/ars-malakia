// model as application state (data)
#[derive(Debug, Default)]
pub struct Model {
    pub y_pos: u16,
    pub max_scroll: u16,
    pub page: u16,
    pub running_state: RunningState,
}
#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}
