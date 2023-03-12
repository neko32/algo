
use crate::shared::MultiChildTreeNode;

pub fn exec(o:&Option<Box<MultiChildTreeNode<String>>>, buf:&mut Vec<String>) {
    match o {
        Some(n) => {
            buf.push(n.value.clone());
            if let Some(clds) = &n.children {
                clds.iter().for_each(|cld|{
                    exec(&Some(cld.to_owned()), buf);
                })
            }
        },
        None => (),
    }
}

pub fn run() {
    let node_i = MultiChildTreeNode::new("I".to_string());
    let node_j = MultiChildTreeNode::new("J".to_string());
    let node_k = MultiChildTreeNode::new("K".to_string());
    let node_e = MultiChildTreeNode::new("E".to_string());
    let mut node_g = MultiChildTreeNode::new("G".to_string());
    let mut node_f = MultiChildTreeNode::new("F".to_string());
    let node_h = MultiChildTreeNode::new("H".to_string());
    let mut node_b = MultiChildTreeNode::new("B".to_string());
    let node_c = MultiChildTreeNode::new("C".to_string());
    let mut node_d = MultiChildTreeNode::new("D".to_string());
    let mut node_a = MultiChildTreeNode::new("A".to_string());
    node_f.children = Some(vec![Box::new(node_i), Box::new(node_j)]);
    node_g.children = Some(vec![Box::new(node_k)]);
    node_b.children = Some(vec![Box::new(node_e), Box::new(node_f)]);
    node_d.children = Some(vec![Box::new(node_g), Box::new(node_h)]);
    node_a.children = Some(vec![Box::new(node_b), Box::new(node_c), Box::new(node_d)]);

    let mut buf:Vec<String> = Vec::new();
    exec(&Some(Box::new(node_a)), &mut buf);
    println!("result - {:?}", buf);
}