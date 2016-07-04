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
fn new_empty() {
  let mut t = TreeNode::new();
  assert_eq!(t.get(0), None);
}

#[test]
fn set_get_missing() {
  let mut t = TreeNode::new();
  t.set(1,2);
  let v = t.get(3);
  assert_eq!(v, None);
}

#[test]
fn set_get_1() {
  let mut t = TreeNode::new();
  t.set(1,41);
  let v = t.get(1);
  println!("{:?}", v);
  assert_eq!(v.unwrap(), 41);
}

#[test]
fn set_get_1_1() {
  let mut t = TreeNode::new();
  t.set(1,42);
  t.set(1,41);
  assert_eq!(t.get(1).unwrap(), 41);
}

#[test]
fn set_get_1_2() {
  let mut t = TreeNode::new();
  t.set(1,41);
  t.set(2,42);
  assert_eq!(t.get(1).unwrap(), 41);
  assert_eq!(t.get(2).unwrap(), 42);
}

#[test]
fn set_get_1_3_2() {
  let mut t = TreeNode::new();
  t.set(1,41);
  t.set(3,43);
  t.set(2,42);
  assert_eq!(t.get(1).unwrap(), 41);
  assert_eq!(t.get(2).unwrap(), 42);
  assert_eq!(t.get(3).unwrap(), 43);
}
