use gltf::{Node, iter::Nodes};

pub fn s_create_model_from_gltf(
    file_path: &str
) -> Result<(), Box<dyn std::error::Error>> {
    println!("loading gltf");
    let (document, buffers, images) = gltf::import(file_path)?;
    println!("gltf loaded");
   
    let nodes = document.nodes();
    for scene in document.scenes() {
        for node in scene.nodes() {
            traverse_nodes(node, &nodes);
        //     if let Some(vertices) = node.mesh() {
        //         println!("{:?}", vertices);
        //     }
        //
        //
        //     // println!(
        //     //     "Node #{} has {} children",
        //     //     node.index(),
        //     //     node.children().count(),
        //     // );
        }
    }

    Ok(())
}

fn traverse_nodes(root: Node, nodes: &Nodes) {
    let mut stack = vec![root];
    
    while !stack.is_empty() {
        if let Some(node) = stack.pop() {
            println!("Node: {}", node.index());

            let mesh = node.mesh();
            println!("{:#?}", mesh);

            let mut children: Vec<Node> = node
                .children()
                .collect();
            
            children.reverse();

            stack.append(&mut children);
        }
    }
} 
