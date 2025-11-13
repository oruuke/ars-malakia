// model as application state (data)
#[derive(Debug, Default)]
pub struct Model {
    pub y_pos: usize,
    pub is_scrollable: bool,
    pub running_state: RunningState,
}
#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}
