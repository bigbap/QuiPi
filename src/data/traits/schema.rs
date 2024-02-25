use crate::{
    prelude::{
        qp_ecs::VersionedIndex,
        GlobalRegistry
    },
    QPResult
};

pub trait ISchema: Clone {
    fn build_entity(
        &self,
        _registry: &mut GlobalRegistry
    ) -> QPResult<VersionedIndex> {
        unimplemented!()
    }

    fn from_entity(
        _entity: VersionedIndex,
        _registry: &GlobalRegistry
    ) -> Option<Self> {
        unimplemented!()
    }

    fn load_resource(
        &self,
        _registry: &mut GlobalRegistry
    ) -> QPResult<u64> {
        unimplemented!()
    }

    fn from_resource(
        _id: u64,
        _registry: &GlobalRegistry
    ) -> Option<Self> {
        unimplemented!()
    }
}