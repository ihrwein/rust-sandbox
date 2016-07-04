#[derive(Debug)]
struct TreeNode {
  key: u32,
  value: u32,
  left: Option<Box<TreeNode>>,
  right: Option<Box<TreeNode>>
}

fn get(t: &TreeNode, k: u32) -> Option<u32> {
  if (k == t.key) {
    Some(t.value)
  } else {
    let n = {
      if (k < t.key) {
        &t.left
      } else {
        &t.right
      }
    };
    match n {
      &None => None,
      &Some(ref sn) => get(sn.as_ref(), k)
    }
  }
}

fn set(t: &mut TreeNode, k: u32, v: u32) {
}

#[test]
fn it_works() {
}
