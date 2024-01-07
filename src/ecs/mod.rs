pub mod error;
pub mod component_registry;

pub use error::ECSError;
pub use component_registry::ComponentRegistry;


pub use component_derive::Component;
pub trait Component {
    fn my_type(&self) -> String;
}
