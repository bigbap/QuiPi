// use gltf::Node;

// // TODO
// pub fn s_create_model_from_gltf(
//     file_path: &str,
//     _tag: &str,
// ) -> Result<Vec<ObjectConfig>, Box<dyn std::error::Error>> {
//     let mut result: Vec<ObjectConfig> = vec![];
//     let (document, _buffers, _images) = gltf::import(file_path)?;
   
//     let _nodes = document.nodes();

//     // we only support single scene loading for now
//     if let Some(scene) = document.scenes().next() {
//         for root in scene.nodes() {
//             let mut stack = vec![root];

//             while !stack.is_empty() {
//                 if let Some(node) = stack.pop() {
//                     let mut children: Vec<Node> = vec![];

//                     for child in node.children() {
//                         children.push(child);
//                     };

//                     if !children.is_empty() {
//                         children.reverse();

//                         stack.append(&mut children);
//                     } else {
//                         let mesh_list = node.mesh();
                        
//                         for _mesh in mesh_list.iter() {
//                             // println!("{:#?}", mesh);

//                             result.push(ObjectConfig {
//                                 ..ObjectConfig::default()
//                             });
//                         };
//                     }
//                 }
//             }
//         }
//     }

//     println!("gltf loaded");

//     Ok(result)
// }
