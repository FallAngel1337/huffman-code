use std::collections::{BTreeMap, HashMap};

#[derive(Debug)]
pub struct Node {
    pub symbol: char,
    pub prob: usize,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>
}

impl Node {
    pub fn new(symbol: char, prob: usize, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Self {
        Node {
            symbol,
            prob,
            left,
            right
        }
    }

    pub fn generate_nodes(message: &str) -> Vec<Node> {
        let mut nodes = Vec::new();
        let map  = Node::get_map(message);
    
        for (a, b) in map {
            nodes.push(Node { symbol: a, prob: b, left: None, right: None })
        }

        nodes
    }

    pub fn create_branch(nodes: &mut Vec<Node>) {
        let n1 = nodes.pop().unwrap();
        let n2 = nodes.pop().unwrap();

        let new_node = Node::new('*', n1.prob + n2.prob, 
                       Some(Box::new(n1)), Some(Box::new(n2)));
        nodes.push(new_node);
    }

    pub fn build_dict(result: &mut Vec<(char, Vec<u8>)>) -> HashMap<char, Vec<u8>> {
        let mut dict: HashMap<char, Vec<u8>> = HashMap::new();

        for (a, b) in result.iter() {
            if *a != '*' {
                dict.insert(*a, b.clone());
            }
        }

        dict
    }

    pub fn generate_codes(&self) -> Vec<(char, Vec<u8>)> {

        // Traverse the tree using a simple queue algorithm. 
        // Store results in pair (Node, turned) where "turned" determines 
        //  if we descended left or right.

        let mut stack = Vec::<(&Node, Vec<u8>)>::new();
        let mut result = Vec::<(char, Vec<u8>)>::new();

        stack.push((self, vec![]));

        // Loop until you reach all nodes that don't have left or right children.
        while !stack.is_empty() {

            let (node, codes) = stack.pop().unwrap();
            let copied_node = node.clone();
            let copied_codes = codes.clone();

            // Push left or right child if node has reference to it.
            // Add copied vector of codes with 1 or 0 at the end for right/left.

            result.push((copied_node.symbol, copied_codes));

            if let Some(nod) = &node.right {
                let mut new_codes = codes.clone();
                new_codes.push(1);
                stack.push((&(*nod), new_codes));
            }

            if let Some(nod) = &node.left {
                let mut new_codes = codes.clone();
                new_codes.push(0);
                stack.push((&(*nod), new_codes));
            }

        }

        result
    }
    
    // aabcadbca
    fn get_map(message: &str) -> Vec<(char, usize)> {
        let mut map = BTreeMap::new();
        
        for c in message.chars() {
            map.entry(c).or_insert(Node::count_chars(message, c));
        }
        
        map.into_iter().map(|(a, b)| (a, b)).collect()
    }
    
    fn count_chars(msg: &str, c: char) -> usize {
        let mut count: usize = 0;
        
        for a in msg.chars() {
            if a == c {
                count += 1;
            }
        }
        
        count
    }
    
}

// Considering only utf-8 chars
pub fn encode(message: &str) {
    let mut nodes = Node::generate_nodes(message);
    let mut length = nodes.len();
    
    while length < nodes.len() * 2 - 1 {
        Node::create_branch(&mut nodes);
        length -= 1;
    }
    
    let mut result: Vec<(char, Vec<u8>)> = nodes[0].generate_codes();

    if nodes.len() == 1 {        
        let mut codes = Vec::new();
        codes.push(1);

        result.push((message.chars().next().unwrap(), codes));
    }

    let dict = Node::build_dict(&mut result);

    // println!("DICT: {:?}", dict);

    let mut total = 0;
    for (_, v) in dict.iter() {
        total += v.len()
    }

    println!("TOTAL SIZE: {} >> {}", message.len() * 8, total);
}