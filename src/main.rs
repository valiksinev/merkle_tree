use sha256::digest;

#[derive(Clone)]
struct Node {
    block: String,
    hash: String,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn main() {
    let blocks = vec!["one", "two", "three", "four", "five", "six"];

    let leaves :Vec<Node> = blocks.into_iter()
        .map(|a|
            Node{
                block: a.to_string(),
                hash: digest(a),
                left: None,
                right: None,
            }
        ).collect();

    let tree = calc_tree(&leaves);
    print(&tree);
}


fn print(tree: &Node) {

    println!("block: {}", tree.block);
    println!("hash: {}", tree.hash);
    println!("left: {}", tree.left.clone().map_or(" - ".to_string(), |a| a.block) );
    println!("right: {}", tree.right.clone().map_or(" - ".to_string(), |a| a.block));
    println!("");

    if let Some(left) = &tree.left{
        print(left)
    }
    if let Some(right) = &tree.right{
        print(right)
    }
}


fn calc_tree(nodes: &[Node]) -> Node {

    if nodes.len() == 2 {
        return Node{
            block: format!("{}, {}", nodes[0].block, nodes[1].block),
            hash: digest(format!("{}{}", nodes[0].hash, nodes[1].hash)),
            left: Some(Box::new(nodes[0].clone())),
            right: Some(Box::new(nodes[1].clone())),
        }
    }

    let calc = |nodes: &[Node]| -> Node {
        let mid = nodes.len()/2;
        let left = calc_tree(&nodes[..mid]);
        let right = calc_tree(&nodes[mid..]);
        calc_tree(&[left, right])
    };

    if nodes.len() % 2 == 1 {
        let mut aligned = nodes.to_vec();
        aligned.push(nodes.last().unwrap().clone());
        if aligned.len() == 2 {
            calc_tree(&aligned)
        } else {
            calc(&aligned)
        }
    } else {
        calc(nodes)
    }
}