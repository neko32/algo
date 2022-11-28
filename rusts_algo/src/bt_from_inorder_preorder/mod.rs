
use crate::shared::{TreeNode, traverse};

pub fn exec(preorder:&[i32], inorder:&[i32]) -> Option<Box<TreeNode>> {
    build(preorder, inorder)
}

fn build(preorder:&[i32], inorder:&[i32]) -> Option<Box<TreeNode>> {
    let plen = preorder.len();

    println!("@{:?}:{:?}", preorder, inorder);

    if plen == 0 {
        return None;
    }

    let mut idx = 0;
    loop {
        if inorder[idx] == preorder[0] {
            break;
        }
        idx += 1;
    }

    // let (in_l, in_r) = inorder.split_at(idx);
    let in_l = &inorder[0..idx];
    let in_r = &inorder[idx + 1..];
    let in_l_len = in_l.len();
    let inorder_mid_idx = 1 + in_l_len;
    let pre_l = &preorder[1..inorder_mid_idx];
    let pre_r = &preorder[inorder_mid_idx..];

    let newn = Box::new(
        TreeNode {
            value: preorder[0],
            left: build(pre_l, in_l),
            right: build(pre_r, in_r),
        }
    );
    Some(newn)
}

pub fn run() {
    let inorder = &[9, 3, 15, 20, 7];
    let preorder = &[3, 9, 20, 15, 7];
    let mut buf:Vec<i32> = Vec::new();
    println!("preorder - {:?}, inorder - {:?}", preorder, inorder);
    if let Some(boxn) = exec(preorder, inorder) {
        traverse(boxn, &mut buf);
    }
    println!("result trace - {:?}", buf);
}