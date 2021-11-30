use std::borrow::{Borrow, BorrowMut};

struct Node<T: PartialEq + Copy + Ord + std::fmt::Debug> {
    element :T,
    next_node : Option<Box<Node<T>>>
}

impl <T: PartialEq + Copy + Ord + std::fmt::Debug> Node <T> {
    fn new (val_t:T) ->Self {
        Node{
            element: val_t,
            next_node: None
        }
    }

    fn append(&mut self, element:Box<Node<T>>){
        self.next_node = Some(element);
    }
}

pub struct LinkedList<T: PartialEq + Copy + Ord + std::fmt::Debug> {
    head: Option<Box<Node<T>>>,
    tail: Option<Box<Node<T>>>,
    length : i32
}


impl <T:PartialEq + Copy + Ord + std::fmt::Debug> LinkedList <T> {
    pub fn new() ->Self {
        let head = None;
        let tail = None;
        LinkedList {
            head,
            tail,
            length:0,
        }
    }

    pub fn push(&mut self, value:T){
        /*
        this will try to insert into linked list
        */
        let node = Box::new(Node::new(value));
        match self.tail {
            None => self.head = Some(node),
            Some(ref mut tail) => tail.append(node),
        }
        self.length +=1;
    }

    pub fn print(&mut self){
        let mut temp = self.head.as_ref();
        while temp.is_some() {
            println!("{:?}", &temp.unwrap().element);
            temp = temp.unwrap().next_node.as_ref();
        }
    }

    pub fn pop(&mut self) {
        if self.length == 0 {
            println!("no element to pop")
        }else {
            let  temp = self.head.take();
            self.head = match temp {
                Some(it) => {
                    Some(it)
                },
                None => None,
            };
        }

    }
}