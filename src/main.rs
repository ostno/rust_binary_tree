struct Node {
    value: u16,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {

    fn insert(&mut self, value: u16) {

        if value > self.value {
            match self.right.as_mut() {
                None => {
                    self.right = Some(Box::new(Node {
                        value,
                        left: None,
                        right: None,
                    }))
                }
                Some(right) => right.insert(value),
            }
        } else {
            match self.left.as_mut() {
                None => {
                    self.left = Some(Box::new(Node {
                        value,
                        left: None,
                        right: None,
                    }))
                }
                Some(left) => left.insert(value),
            }
        }

    }

    fn flatten(&self) -> Vec<u16> {

        let nodes = self.list_nodes();
        let result = nodes.into_iter().map(|node| node.value).collect();

        result

    }

    fn list_nodes(&self) -> Vec<&Node> {

        let mut result = Vec::new();

        result.push(self);

        match self.left.as_ref() {
            Some(left) => result.extend(&left.list_nodes()),
            None => (),
        }

        match self.right.as_ref() {
            Some(right) => result.extend(&right.list_nodes()),
            None => (),
        }

        result

    }

}

fn main() {
    println!("Run 'cargo test'");
}

#[test]
fn should_return_1_from_a_new_node() {
    let node = Node {
        value: 1,
        left: None,
        right: None,
    };
    assert_eq!(node.value, 1);
}

#[test]
fn should_insert_greater_value_on_the_right() {
    let mut node = Node {
        value: 2,
        left: None,
        right: None,
    };
    node.insert(3);
    assert_eq!(node.right.unwrap().value, 3);
}

#[test]
fn should_insert_lower_value_on_the_left() {
    let mut node = Node {
        value: 2,
        left: None,
        right: None,
    };
    node.insert(1);
    assert_eq!(node.left.unwrap().value, 1);
}

#[test]
fn should_insert_large_number_on_the_right_on_a_bigger_tree() {
    let mut node = Node {
        value: 2,
        left: None,
        right: None,
    };
    node.insert(1);
    node.insert(3);
    node.insert(4);
    assert_eq!(node.right.unwrap().right.unwrap().value, 4);
}

#[test]
fn should_return_an_array_of_values_on_flatten_with_a_complex_tree() {
    let mut node = Node {
        value: 5,
        left: None,
        right: None,
    };
    node.insert(1);
    node.insert(6);
    node.insert(3);
    node.insert(4);
    node.insert(9);
    assert_eq!(node.flatten(), [5, 1, 3, 4, 6, 9]);
}
