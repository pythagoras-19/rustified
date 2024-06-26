pub fn entry() {
    let node = Node::new(65);
    let mut ll = LinkedList::new(node);
    ll.append(66);
    ll.print();
}

#[derive(Debug)]
pub(crate) struct Node {
    data: i64,
    next: Option<Box<Node>>
}

impl Node {
     pub fn new(data: i64) -> Self {
        Self{ data, next: None }
    }

     pub fn get_data(&self) -> i64 {
        self.data
    }
}
#[derive(Debug)]
pub (crate) struct LinkedList {
    head: Node
}

impl LinkedList {
     pub fn new(node: Node) -> Self {
        println!("===LINKED LIST ===");
        Self{ head: node }
    }

     pub fn append(&mut self, data: i64) {
        let mut current = &mut self.head;
        while current.next.is_some() {
            current = current.next.as_mut().unwrap();
        }
        current.next = Some(Box::new(Node::new(data)));
    }

     pub fn print(&self) {
        let mut current = Some(&self.head);
        while let Some(node) = current {
            print!("{}-->", node.get_data());
            current = node.next.as_deref(); // to convert from Option<&Box<T>> to Option<&T>
        }
        println!("NULL");
    }
}