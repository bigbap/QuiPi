use crate::{
    Component,
    schema::ISchema
};

#[derive(Component)]
pub struct CPrefab {
    pub schema: Box<dyn ISchema>
}

