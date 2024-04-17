use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

#[derive(Debug, Clone, Eq, PartialEq)]
struct HuffmanNode {
    frequency: usize,
    character: Option<char>,
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
}

impl HuffmanNode {
    fn new_leaf(character: char, frequency: usize) -> Self {
        HuffmanNode {
            frequency,
            character: Some(character),
            left: None,
            right: None,
        }
    }

    fn new_node(frequency: usize, left: HuffmanNode, right: HuffmanNode) -> Self {
        HuffmanNode {
            frequency,
            character: None,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }
}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.frequency.cmp(&self.frequency)
    }
}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn build_tree(frequencies: &HashMap<char, usize>) -> HuffmanNode {
    let mut heap: BinaryHeap<Reverse<HuffmanNode>> = BinaryHeap::new();
    for (&character, &frequency) in frequencies.iter() {
        heap.push(Reverse(HuffmanNode::new_leaf(character, frequency)));
    }

    while heap.len() > 1 {
        let left = heap.pop().unwrap().0;
        let right = heap.pop().unwrap().0;
        let freq_sum = left.frequency + right.frequency;
        heap.push(Reverse(HuffmanNode::new_node(freq_sum, left, right)));
    }

    heap.pop().unwrap().0
}

fn generate_codes(node: &HuffmanNode, prefix: String, codes: &mut HashMap<char, String>) {
    if let Some(character) = node.character {
        codes.insert(character, prefix);
    } else {
        if let Some(ref left) = node.left {
            generate_codes(left, format!("{}0", prefix), codes);
        }
        if let Some(ref right) = node.right {
            generate_codes(right, format!("{}1", prefix), codes);
        }
    }
}

fn main() {
    let binding = std::fs::read_to_string("example.txt").unwrap();
    let text = binding.as_str();
    let mut frequencies = HashMap::new();

    for c in text.chars() {
        *frequencies.entry(c).or_insert(0) += 1;
    }

    let tree = build_tree(&frequencies);
    let mut codes = HashMap::new();
    generate_codes(&tree, String::new(), &mut codes);

    println!("Huffman Codes: {:?}", codes);
    let encoded: String = text.chars().map(|c| codes.get(&c).unwrap().clone()).collect();
    let _ = std::fs::write("huff.txt", encoded.clone());
    println!("Encoded Text: {}", encoded);
}