use crate::{
    prelude::{
        qp_ecs::VersionedIndex,
        Registry
    },
    QPResult
};

pub trait ISchema: Clone {
    fn build_entity(
        &self,
        _registry: &mut Registry
    ) -> QPResult<VersionedIndex> {
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
    ) -> QPResult<u64> {
        unimplemented!()
    }

    fn from_resource(
        _id: u64,
        _registry: &Registry
    ) -> Option<Self> {
        unimplemented!()
    }
}