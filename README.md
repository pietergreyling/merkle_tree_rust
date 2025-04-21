# merkle_tree_rust
An experimental implementation of a Merkle / Hash Tree using the Rust programming language.

## What is a Merkle Tree?

![hash_.png](./images/hash_tree.png)

A Merkle Tree, often referred to as a hash tree, represents a fundamental data structure in computer science and cryptography. 

Its structure is hierarchical, resembling an inverted tree. At the base of this tree are the "leaf" nodes, each containing a cryptographic hash derived from a distinct block of data. Moving upwards, every node that is not a leaf—termed an internal node, branch, or inode—stores a cryptographic hash computed from the combined hashes of its direct child nodes.

The primary function of a Merkle Tree is to enable the efficient and secure verification of large datasets. 

By summarizing potentially vast amounts of data into a single root hash, it allows parties to confirm data integrity without transmitting the entire dataset. 

This structure serves as a more sophisticated evolution of simpler concepts like hash lists or hash chains. 

Its utility is demonstrated in numerous real-world systems requiring robust data verification, including prominent blockchains like Bitcoin and Ethereum, distributed databases such as Apache Cassandra, and version control systems like Git, primarily for ensuring data integrity and facilitating efficient synchronization between distributed copies of data.

This implementation illustrates the core principles of a Merkle Tree:

1. **Leaf Nodes**: Each data block is hashed to create leaf nodes
2. **Internal Nodes**: Parent nodes are created by hashing the concatenation of their children's hashes
3. **Tree Structure**: The tree is built bottom-up by combining pairs of nodes
4. **Root Hash**: The root of the tree represents a cryptographic summary of all data
5. **Verification**: Data can be verified using a proof path (though the full verification logic would be more complex in a production system)

Key components of this implementation:

- Uses SHA-256 for hashing
- Includes both leaf and internal nodes
- Builds the tree from the bottom up
- Supports verification of data against the root hash using Merkle proofs
- Handles odd numbers of nodes at each level

This implementation is deliberately simple to focus on the fundamental concepts rather than optimization or advanced features. 



