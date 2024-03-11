use crate::common::resources::AssetId;
use crate::prelude::qp_storage::*;

#[derive(Debug, Component, Clone, PartialEq)]
pub struct CTexture {
    pub id: AssetId,
    pub atlas_location: Option<(u32, u32)>,
}
