pub mod shader;

pub use shader::Shader;

pub fn register_resources(registry: &engine::Registry) {
    let mut reg_res = registry.resources.borrow_mut();
    
    reg_res.register_component::<Shader>(); 
}
