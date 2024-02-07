use crate::{
    Component,
    schemas::ISchema
};

#[derive(Component)]
pub struct CPrefab {
    pub schema: Box<dyn ISchema>
}

