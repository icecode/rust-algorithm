use core::fmt;
use std::fmt::{Debug, Formatter};
use std::io::{BufWriter, stdout};
use std::rc::Rc;

use ferris_says;
use std::cmp::{Ordering};

mod bin_tree;
mod linked_list;
mod array_list;

const PI: f64 = 3.14159265359;

fn main() {
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    ferris_says::say(wf_title().as_bytes(), 2048, &mut writer);
    var_ownership();
    var_ownership_copy();
    print_raw_args(1, 2, PI);
    println!("π values is {}", PI);
    let mut lst: array_list::ArrayList<NodeCopy> = array_list::ArrayList::new();
    let n1 = NodeCopy { data: 1 };
    let n2 = n1.clone();
    let n3 = n2;
    let n4 = n3;
    lst.push(n1);
    lst.push(n2);
    lst.push(n3);
    lst.push(n4);
    lst.add_to_index(0, NodeCopy { data: 0 });


    let mut l2: linked_list::LinkedList<NodeCopy> = linked_list::LinkedList::new();
    l2.push_front(NodeCopy { data: 1 });
    l2.push_front(NodeCopy { data: 2 });
    l2.push_front(NodeCopy { data: 3 });
    l2.push_front(NodeCopy { data: 4 });
    l2.push_back(NodeCopy { data: 0 });
    l2.push_back(NodeCopy { data: -1 });
    print!("LinkedList:[");
    l2.foreach(|d| {
        print!("CodeCopy:{},", d.data);
    });
    println!("]");
    l2.pop_back();
    print!("LinkedList:[");
    l2.foreach(|d| {
        print!("CodeCopy:{},", d.data);
    });
    println!("]");

    print!("ArrayList[");
    lst.foreach(|t| {
        print!("CodeCopy:{},", t.data)
    });
    println!("]");
    println!("-----------------");

    print!("BinTree[");
    let mut bin_tree:bin_tree::BinTree<NodeCopy> = bin_tree::BinTree::new();
    bin_tree.insert(NodeCopy { data: 3 });
    bin_tree.insert(NodeCopy { data: 1 });
    bin_tree.insert(NodeCopy { data: 2 });
    bin_tree.insert(NodeCopy { data: 4 });
    bin_tree.insert(NodeCopy { data: 5 });
    bin_tree.foreach( |t| {
        print!("CodeCopy:{},", t.data)
    });
    println!("]");
    println!("finish...")

}


struct LinkedNode<T> {
    data: T,
    next: Option<Box<LinkedNode<T>>>,
}

fn fn_option_args_move<T: Debug>(v: Option<T>) {
    match v {
        Some(val) => println!("{:?}", val),
        None => println!("nothing")
    }
}


struct Node {
    data: i32,
    left: Rc<Option<Node>>,
    right: Rc<Option<Node>>,
}

#[derive(Copy, Clone, Eq)]
struct NodeCopy {
    data: i32
}

impl PartialEq for NodeCopy {

    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}


impl Ord for NodeCopy {

    fn cmp(&self, other: &Self) -> Ordering {
        let ret = self.data - other.data;
        if ret == 0 {
            Ordering::Equal
        } else if ret < 0 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl PartialOrd for NodeCopy {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ret = self.data - other.data;
        Some(if ret == 0 {
            Ordering::Equal
        } else if ret < 0 {
            Ordering::Less
        } else {
            Ordering::Greater
        })
    }


}

fn var_ownership() {
    let a = Node { data: 1, left: Rc::new(Option::None), right: Rc::new(Option::None) };
    let call_fn = |node: Node| -> i32{ node.data };
    println!("call before: {}", a.data);
    call_fn(a);
    // call failure ...
    //println!("call after: {}", a.data);
}

fn var_ownership_copy() {
    let a = NodeCopy { data: 1 };
    let call_fn = |node: NodeCopy| -> i32{ node.data };
    println!("call before: {}", a.data);
    call_fn(a);
    println!("call after: {}", a.data);
}

fn wf_title() -> String {
    String::from("悟饭游戏厅")
}

fn print_raw_args(a: i32, b: i64, c: f64) -> String {
    String::from(format!("a={}, b={}, c={}", a, b, c))
}

#[test]
fn test_wf_title() {
    assert_eq!(wf_title(), "悟饭游戏厅");
}


#[test]
fn test_print_raw_args() {
    assert_eq!(print_raw_args(1, 2, 3.1), "a=1, b=2, c=3.1")
}


#[derive(Copy, Clone)]
struct Color(u8, u8, u8);

impl Debug for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Color").field(&self.0).field(&self.1).field(&self.2).finish()
    }
}

#[test]
fn test_fn_option_args_move() {
    let iv_opt = Some(Color(0, 0, 0));
    fn_option_args_move(iv_opt);
    println!("ivOpt: {:?}", iv_opt.unwrap())
}

struct EmptyVal;

#[test]
fn test_zero_size_type() {
    println!("empty struct:{}", std::mem::size_of::<EmptyVal>());
    println!("():{}", std::mem::size_of::<()>());
    println!("():{}", std::mem::size_of::<[(); 0]>());
}