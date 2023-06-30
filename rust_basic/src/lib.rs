#![deny(missing_docs)]

//! Basic data structures and algorithms in Rust.
//!
//! The project is used for practice with Rust programming language in the
//! following aspects: grammar, safe code, unsafe code, memory management, and
//! supported tools.
//!
//! Data structures and algorithms are grouped and built into containers. There
//! are basic types: Stack, Queue, Vector, String, HashMap, HashSet, Tree,
//! PriorityQueue, RedBlackTree, DirectedGraph and more. All implementations are
//! made from scratch without dependencies, except for: 1 - `std::ptr` and
//! `std::alloc` to manage memory; 2 - `std::iter` to implement standard
//! iterators; 3 - `std::cmp` to implement comparison operators.
//!
//! There are Entry APIs and Non Entry APIs. Entry APIs are labeled `entry`. Non
//! Entry APIs have no tag. Entry APIs expose the main functions of the module,
//! so they are worth reading for the first time. Non Entry APIs on the other
//! hand, may be distracting and should be skipped until they show up next to
//! Entry APIs.

pub mod binary_search_tree;
pub mod directed_graph;
pub mod grid;
pub mod hash;
pub mod hash_map;
pub mod hash_set;
pub mod linked_list;
pub mod priority_queue;
pub mod queue;
pub mod red_black_tree;
pub mod stack;
pub mod string;
pub mod sudoku;
pub mod tree;
pub mod undirected_graph;
pub mod vector;

#[doc(inline)]
pub use binary_search_tree::BinarySearchTree;

#[doc(inline)]
pub use directed_graph::DirectedGraph;

#[doc(inline)]
pub use grid::Grid;

#[doc(inline)]
pub use hash_map::HashMap;

#[doc(inline)]
pub use linked_list::LinkedList;

#[doc(inline)]
pub use priority_queue::PriorityQueue;

#[doc(inline)]
pub use queue::Queue;

#[doc(inline)]
pub use red_black_tree::RedBlackTree;

#[doc(inline)]
pub use hash_set::HashSet;

#[doc(inline)]
pub use stack::Stack;

#[doc(inline)]
pub use string::String;

#[doc(inline)]
pub use sudoku::Sudoku;

#[doc(inline)]
pub use tree::Tree;

#[doc(inline)]
pub use undirected_graph::UndirectedGraph;

#[doc(inline)]
pub use vector::Vector;

#[doc(inline)]
pub use hash::{murmur_v2_64, murmur_v3_32, Hashable};
