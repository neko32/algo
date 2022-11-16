
use derive_more::Display;
use std::default::Default;

#[derive(Display, Debug)]
#[display(fmt = "{{value:{}}}", value)]
pub struct TreeNode {
    pub value: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

#[derive(Display, Debug)]
#[display(fmt = "{{x:{},y:{}}}", x, y)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x_v: i32, y_v: i32) -> Self {
        Point { x: x_v, y: y_v }
    }
}

impl Default for Point {
    fn default() -> Self {
        Point::new(0, 0)
    }
}

pub fn add_treenode(n: &mut Box<TreeNode>, v: i32) -> () {
    if v < n.value {
        match n.left.as_mut() {
            None => {
                let newn = TreeNode {
                    value: v,
                    left: None,
                    right: None,
                };
                n.left = Some(Box::new(newn));
            }
            Some(mut left_node) => {
                add_treenode(&mut left_node, v);
            }
        }
    } else {
        match n.right.as_mut() {
            None => {
                let newn = TreeNode {
                    value: v,
                    left: None,
                    right: None,
                };
                n.right = Some(Box::new(newn));
            }
            Some(mut right_node) => {
                add_treenode(&mut right_node, v);
            }
        }
    }
}
