use crate::Component;

#[derive(Debug, Component, Default, PartialEq)]
pub struct CMouseBtnState {
    pub btn_left: bool,
    pub btn_right: bool,
    pub btn_middle: bool,
}
