#![feature(test)]
extern crate test;
extern crate bintree;

use test::Bencher;

#[bench]
fn bench_iterative_inorder(b: &mut Bencher) {
    let tree = bintree::gen_bin_tree(9);
    b.iter(move || tree.inorder_iter().fold(0, |acc, x| acc + x.1));
}

#[bench]
fn bench_recursive_inorder(b: &mut Bencher) {
    let tree = bintree::gen_bin_tree(9);
    b.iter(move || {
        let mut acc = 0;
        tree.inorder(&mut |_, v| acc += *v);
    });
}
