// 
// Initial implementation of a Merkle Tree in Rust
// This code provides a basic structure for creating a Merkle Tree
//
// A Merkle Tree is a data structure used for storing and verifying data integrity 
// using the tree's root hash.
// It is commonly used in blockchain technology and distributed systems.
//
// This implementation uses SHA-256 for hashing and provides methods to create
// a Merkle Tree from a list of data blocks, compute the root hash, and verify
// the inclusion of a data block in the tree using a proof.
// It includes a simple example of how to use the Merkle Tree
// and demonstrates the hashing process using SHA-256.
//
// This code is for educational purposes and may not be suitable for production use.
// It is recommended to use established libraries for cryptographic operations
// and data structures in real applications.
//

use sha2::{Sha256, Digest};
// use hex;

/// A node in the Merkle Tree
#[derive(Debug)]
struct Node {
    hash: Vec<u8>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    /// Create a new leaf node with the given data
    fn new_leaf(data: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let hash = hasher.finalize().to_vec();
        
        Node {
            hash,
            left: None,
            right: None,
        }
    }
    
    /// Create a new internal node with left and right children
    fn new_parent(left: Node, right: Node) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(&left.hash);
        hasher.update(&right.hash);
        let hash = hasher.finalize().to_vec();
        
        Node {
            hash,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hash_hex = self.hash.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();
        
        write!(f, "{}", hash_hex)
    }
}

/// A Merkle Tree data structure
#[derive(Debug)]
struct MerkleTree {
    root: Option<Node>,
}

impl MerkleTree {
    /// Create a new Merkle Tree from a list of data blocks
    fn new(data_blocks: Vec<Vec<u8>>) -> Self {
        if data_blocks.is_empty() {
            return MerkleTree { root: None };
        }
        
        // Convert data blocks to leaf nodes
        let mut nodes: Vec<Node> = data_blocks.iter()
            .map(|data| Node::new_leaf(data))
            .collect();
        
        // Build the tree bottom-up
        while nodes.len() > 1 {
            let mut next_level = Vec::new();
            
            // Process pairs of nodes
            for chunk in nodes.chunks(2) {
                if chunk.len() == 2 {
                    // Create a parent node with two children
                    let parent = Node::new_parent(chunk[0].clone(), chunk[1].clone());
                    next_level.push(parent);
                } else {
                    // If there's an odd node left, promote it to the next level
                    next_level.push(chunk[0].clone());
                }
            }
            
            nodes = next_level;
        }
        
        MerkleTree { root: Some(nodes.remove(0)) }
    }
    
    /// Get the root hash of the Merkle Tree
    fn root_hash(&self) -> Option<Vec<u8>> {
        self.root.as_ref().map(|node| node.hash.clone())
    }
    
    /// Verify that a data block is part of the tree by providing a proof
    fn verify(&self, data: &[u8], proof: &[Vec<u8>], proof_path: &[bool]) -> bool {
        if proof.len() != proof_path.len() || self.root.is_none() {
            return false;
        }
        
        // Calculate the leaf node hash
        let mut hasher = Sha256::new();
        hasher.update(data);
        let mut current_hash = hasher.finalize().to_vec();
        
        // Traverse the proof path and compute the root hash
        for (i, sibling_hash) in proof.iter().enumerate() {
            let mut hasher = Sha256::new();
            
            // If proof_path[i] is true, then current_hash is the right child
            // If proof_path[i] is false, then current_hash is the left child
            if proof_path[i] {
                hasher.update(sibling_hash);
                hasher.update(&current_hash);
            } else {
                hasher.update(&current_hash);
                hasher.update(sibling_hash);
            }
            
            current_hash = hasher.finalize().to_vec();
        }
        
        // Compare the computed root hash with the tree's root hash
        current_hash == self.root.as_ref().unwrap().hash
    }
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Node {
            hash: self.hash.clone(),
            left: self.left.clone(),
            right: self.right.clone(),
        }
    }
}

// Example usage
// fn main() {
//     let data = vec!["a", "b", "c", "d"];
//     let merkle_tree = MerkleTree::new(data);
//     println!("Merkle Root: {}", merkle_tree.root());
// }

// Example usage
// fn main() {
//     // Create some data blocks
//     let data_blocks = vec![
//         b"Block 1".to_vec(),
//         b"Block 2".to_vec(),
//         b"Block 3".to_vec(),
//         b"Block 4".to_vec(),
//     ];
    
//     // Build a Merkle Tree
//     let tree = MerkleTree::new(data_blocks.clone());
    
//     // Print the root hash
//     if let Some(root_hash) = tree.root_hash() {
//         println!("Root Hash: {}", root_hash.iter()
//             .map(|b| format!("{:02x}", b))
//             .collect::<String>());
//     }
    
//     // Simple verification demonstration
//     // In a real implementation, we would generate and verify proofs
//     // This is just placeholder code to show the concept
//     println!("\nVerification Example (placeholder):");
//     println!("To properly verify data, we would need to generate a Merkle proof");
//     println!("which includes sibling hashes along the path from the leaf to the root.");
// }

fn main() {
    // Example usage with a few data blocks
    let data = vec!;

    println!("Input Data Blocks: {}", data.len());

    match build_merkle_root(data) {
        Some(root) => {
            // Print the Merkle Root as a hexadecimal string
            println!("Merkle Root: {}", hex::encode(root));
        }
        None => {
            println!("Cannot compute Merkle Root for empty data.");
        }
    }

    // Example with empty data
    let empty_data: Vec<Vec<u8>> = Vec::new();
    println!("\nInput Data Blocks: {}", empty_data.len());
     match build_merkle_root(empty_data) {
        Some(root) => {
             // This part should not be reached for empty data
            println!("Merkle Root: {}", hex::encode(root));
        }
        None => {
            println!("Cannot compute Merkle Root for empty data.");
        }
    }

    // Example with one data block
    let single_data = vec!;
    println!("\nInput Data Blocks: {}", single_data.len());
     match build_merkle_root(single_data) {
        Some(root) => {
            println!("Merkle Root: {}", hex::encode(root));
        }
        None => {
            println!("Cannot compute Merkle Root for empty data.");
        }
    }
}

