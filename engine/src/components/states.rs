use crate::Component;

#[derive(Debug, Component, Default)]
pub struct CMouseBtnState {
    pub btn_left: bool,
    pub btn_right: bool,
    pub btn_middle: bool,
}
