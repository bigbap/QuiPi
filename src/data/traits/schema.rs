use crate::prelude::{
    ecs::VersionedIndex,
    Registry
};

pub trait ISchema: Clone {
    fn build_entity(
        &self,
        _registry: &mut Registry
    ) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
        unimplemented!()
    }

    fn from_entity(
        _entity: VersionedIndex,
        _registry: &Registry
    ) -> Option<Self> {
        unimplemented!()
    }

    fn load_resource(
        &self,
        _registry: &mut Registry
    ) -> Result<u64, Box<dyn std::error::Error>> {
        unimplemented!()
    }

    fn from_resource(
        _id: u64,
        _registry: &Registry
    ) -> Option<Self> {
        unimplemented!()
    }
}