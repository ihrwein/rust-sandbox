#[derive(Debug)]
pub struct Tree {
    key: i32,
    value: i32,
    left: Branch,
    right: Branch,
}

impl Tree {
    pub fn new(k: i32, v: i32) -> Tree {
        Tree {
            key: k,
            value: v,
            left: Branch::new(),
            right: Branch::new(),
        }
    }

    pub fn get(&self, k: i32) -> Option<&i32> {
        if k == self.key {
            Some(&self.value)
        } else if k < self.key {
            self.left.get(k)
        } else {
            self.right.get(k)
        }
    }

    pub fn set(&mut self, k: i32, v: i32) {
        if k == self.key {
            self.value = v
        } else if k < self.key {
            self.left.set(k, v)
        } else {
            self.right.set(k, v)
        }
    }
}

#[derive(Debug)]
struct Branch(Option<Box<Tree>>);

impl Branch {
    fn new() -> Branch {
        Branch(None)
    }

    fn get(&self, k: i32) -> Option<&i32> {
        if let Some(ref t) = self.0 {
            t.get(k)
        } else {
            None
        }
    }

    fn set(&mut self, k: i32, v: i32) {
        if let Some(ref mut t) = self.0 {
            t.set(k, v);
        } else {
            let t = Tree {
                key: k,
                value: v,
                left: Branch::new(),
                right: Branch::new(),
            };
            *self = Branch(Some(Box::new(t)));
        }
    }
}

#[test]
fn set_get_missing() {
    let mut t = Tree::new(0, 0);
    t.set(1, 2);
    let v = t.get(3);
    assert_eq!(v, None);
}

#[test]
fn set_get_1() {
    let mut t = Tree::new(0, 0);
    t.set(1, 41);
    assert_eq!(t.get(1).unwrap(), &41);
}

#[test]
fn set_get_1_1() {
    let mut t = Tree::new(0, 0);
    t.set(1, 42);
    t.set(1, 41);
    assert_eq!(t.get(1).unwrap(), &41);
}

#[test]
fn set_get_1_2() {
    let mut t = Tree::new(0, 0);
    t.set(1, 41);
    t.set(2, 42);
    assert_eq!(t.get(1).unwrap(), &41);
    assert_eq!(t.get(2).unwrap(), &42);
}

#[test]
fn set_get_1_3_2() {
    let mut t = Tree::new(0, 0);
    t.set(1, 41);
    t.set(3, 43);
    t.set(2, 42);
    println!("{:?}", t);
    assert_eq!(t.get(1).unwrap(), &41);
    assert_eq!(t.get(2).unwrap(), &42);
    assert_eq!(t.get(3).unwrap(), &43);
}