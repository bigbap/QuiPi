use engine::Component;

#[derive(Debug, Default, Component, PartialEq)]
pub struct DrawComponent {
    pub shader_id: Option<usize>
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cmp = DrawComponent {
            shader_id: Some(3)
        };

        assert_eq!(DrawComponent::type_name(), "DrawComponent");
        assert_eq!(cmp.my_type(), "DrawComponent");
    }
}
