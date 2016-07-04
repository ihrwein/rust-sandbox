#[derive(Debug)]
struct TreeNode {
  key: u32,
  value: u32,
  left: Option<Box<TreeNode>>,
  right: Option<Box<TreeNode>>
}

impl TreeNode {
  fn new() -> TreeNode {
    TreeNode {
      key: 0,
      value: 0,
      left: None,
      right: None
    }
  }

  fn get(self: &TreeNode, k: u32) -> Option<u32> {
    if (k == self.key) {
      Some(self.value)
    } else {
      let n = {
        if (k < self.key) {
          &self.left
        } else {
          &self.right
        }
      };
      match n {
        &None => None,
        &Some(ref sn) => sn.as_ref().get(k)
      }
    }
  }

  fn set(self: &mut TreeNode, k: u32, v: u32) {
  }
}

#[test]
fn it_works() {
  let t = TreeNode::new();
  println!("{:?}", t)
}
