use crate::{
    VersionedIndex,
    components::CModelNode,
    Registry,
};

pub fn s_model_traversal<'a>(
    registry: &'a Registry,
    root_id: VersionedIndex,
    root: &'a CModelNode,
) -> Vec<(VersionedIndex, &'a CModelNode)> {
    let mut stack = vec![(root, root_id)];
    let mut result = vec![];

    while let Some((node, id)) = stack.pop() {
        if !node.children.is_empty() {
            let mut children = node.children.clone();
            children.reverse();

            let mut children = children
                .iter()
                .filter_map(|child| registry.get_component::<CModelNode>(child)
                    .map(|c_node| (c_node, *child)))
                .collect();

            stack.append(&mut children);
        } else {
            result.push((id, node));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::ec_store::CTag;
    use super::*;

    fn build_registry() -> Registry {
        let mut registry = Registry::init().unwrap();

        registry.register_component::<CModelNode>();

        registry
    }

    fn build_node(
        tag: &str,
        registry: &mut Registry
    ) -> VersionedIndex {
        registry.create_entity(tag).unwrap()
            .with(CModelNode {
                children: vec![],
                ..CModelNode::default()
            }).unwrap()
            .done().unwrap()
    }

    #[test]
    fn traversal() {
        let mut registry = build_registry();
        let level1 = build_node("level1", &mut registry);
        let level1_1 = build_node("level1_1", &mut registry);
        let level1_2 = build_node("level1_2", &mut registry);
        let level1_3 = build_node("level1_3", &mut registry);
        let level1_1_1 = build_node("level1_1_1", &mut registry);
        let level1_2_1 = build_node("level1_2_1", &mut registry);
        let level1_2_2 = build_node("level1_2_2", &mut registry);
        let level1_3_1 = build_node("level1_3_1", &mut registry);
        let level1_3_2 = build_node("level1_3_2", &mut registry);
        let level1_3_3 = build_node("level1_3_3", &mut registry);

        let node = registry.get_component_mut::<CModelNode>(&level1_1).unwrap();
        node.children = vec![level1_1_1];

        let node = registry.get_component_mut::<CModelNode>(&level1_2).unwrap();
        node.children = vec![level1_2_1, level1_2_2];

        let node = registry.get_component_mut::<CModelNode>(&level1_3).unwrap();
        node.children = vec![level1_3_1, level1_3_2, level1_3_3];

        let node = registry.get_component_mut::<CModelNode>(&level1).unwrap();
        node.children = vec![level1_1, level1_2, level1_3];
        
        let mut check_against = vec![
            "level1".to_string(),
            "level1_3".to_string(),
            "level1_2".to_string(),
            "level1_1".to_string(),
            "level1_3_3".to_string(),
            "level1_3_2".to_string(),
            "level1_3_1".to_string(),
            "level1_2_2".to_string(),
            "level1_2_1".to_string(),
            "level1_1_1".to_string(),
        ];

        let root = registry.get_component::<CModelNode>(&level1).unwrap();
        let result = s_model_traversal(&registry, level1, root);

        for (id, _node) in result {
            let tag = registry.get_component::<CTag>(&id).unwrap();
            let next_tag = check_against.pop().unwrap();

            assert_eq!(next_tag, tag.tag.to_string());
        }
    }
}
