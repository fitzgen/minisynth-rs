use id_arena::{Arena, Id};
use std::collections::HashMap;

#[derive(Default)]
pub struct Context {
    idents: Arena<String>,
    interned: HashMap<String, StringId>,

    nodes: Arena<Node>,
}

pub type StringId = Id<String>;
pub type NodeId = Id<Node>;

pub enum Node {
    Identifier(StringId),
    Addition(NodeId, NodeId),
    Subtraction(NodeId, NodeId),
    Multiplication(NodeId, NodeId),
    Division(NodeId, NodeId),
    RightShift(NodeId, NodeId),
    LeftShift(NodeId, NodeId),
    Const(i64),
    Negation(NodeId),
    Conditional(NodeId, NodeId, NodeId),
}

impl Context {
    pub fn intern<S: AsRef<str> + Into<String>>(&mut self, s: S) -> StringId {
        if let Some(id) = self.interned.get(s.as_ref()) {
            return *id;
        }

        let s = s.into();
        let id = self.idents.alloc(s.clone());
        self.interned.insert(s, id);
        id
    }

    pub fn new_identifer<S: AsRef<str> + Into<String>>(&mut self, s: S) -> NodeId {
        let s = self.intern(s);
        self.nodes.alloc(Node::Identifier(s))
    }

    pub fn new_addition(&mut self, l: NodeId, r: NodeId) -> NodeId {
        self.nodes.alloc(Node::Addition(l, r))
    }

    pub fn new_subtraction(&mut self, l: NodeId, r: NodeId) -> NodeId {
        self.nodes.alloc(Node::Subtraction(l, r))
    }

    pub fn new_multiplication(&mut self, l: NodeId, r: NodeId) -> NodeId {
        self.nodes.alloc(Node::Multiplication(l, r))
    }

    pub fn new_division(&mut self, l: NodeId, r: NodeId) -> NodeId {
        self.nodes.alloc(Node::Division(l, r))
    }

    pub fn new_right_shift(&mut self, l: NodeId, r: NodeId) -> NodeId {
        self.nodes.alloc(Node::RightShift(l, r))
    }

    pub fn new_left_shift(&mut self, l: NodeId, r: NodeId) -> NodeId {
        self.nodes.alloc(Node::LeftShift(l, r))
    }

    pub fn new_const(&mut self, i: i64) -> NodeId {
        self.nodes.alloc(Node::Const(i))
    }

    pub fn new_negation(&mut self, i: NodeId) -> NodeId {
        self.nodes.alloc(Node::Negation(i))
    }

    pub fn new_conditional(
        &mut self,
        condition: NodeId,
        consequent: NodeId,
        alternative: NodeId,
    ) -> NodeId {
        self.nodes
            .alloc(Node::Conditional(condition, consequent, alternative))
    }

    pub fn node_ref(&self, id: NodeId) -> &Node {
        &self.nodes[id]
    }

    pub fn interned(&self, id: StringId) -> &str {
        &self.idents[id]
    }
}
