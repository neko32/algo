pub mod preorder_traversal {

    use crate::shared::{add_treenode, TreeNode};

    pub fn exec(v: Vec<i32>) -> Vec<i32> {
        let root: Box<TreeNode> = build_tree(&v);
        let mut tracker_on_heap = Box::new(vec![*v.first().unwrap()]);
        preorder_trav(root, &mut tracker_on_heap);
        *tracker_on_heap
    }

    pub fn run() {
        let v: Vec<i32> = vec![5, 9, 2, 10, 1, 4];
        let r = exec(v);
        println!("{:?}", r);
    }

    fn preorder_trav(node: Box<TreeNode>, tracker: &mut Box<Vec<i32>>) -> () {
        if let Some(left_node) = node.left {
            tracker.push(left_node.value);
            preorder_trav(left_node, tracker);
        }
        if let Some(right_node) = node.right {
            tracker.push(right_node.value);
            preorder_trav(right_node, tracker);
        }
    }

    fn build_tree(v: &Vec<i32>) -> Box<TreeNode> {
        let first = *v.first().unwrap();
        let mut root: Box<TreeNode> = Box::new(TreeNode {
            value: first,
            left: None,
            right: None,
        });

        for val in v.iter().skip(1) {
            add_treenode(&mut root, *val);
        }

        root
    }
}
