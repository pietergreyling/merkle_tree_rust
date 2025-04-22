# merkle_tree_rust
An experimental implementation of a Merkle / Hash Tree using the Rust programming language.

THIS IS A WORK IN PROGRESS - A self teaching exercise.

## What is a Merkle Tree?

![hash_tree.png](./images/hash_tree.png)

![merkle-tree-structure.png](./images/merkle-tree-structure.png)

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

## The Rust Crypto Ecosystem

Rust benefits from a vibrant and active ecosystem for cryptographic operations, largely curated under the RustCrypto project umbrella. 

A key component of this ecosystem is the digest crate. This crate defines a set of common traits, most notably the Digest trait, which specifies a standard interface for cryptographic hash functions. Adhering to this trait allows developers to write code that is generic over different hash algorithms, promoting modularity and code reuse.

Numerous crates implement specific hash functions and conform to the digest traits. 

Popular and secure options include:

* sha2: Provides implementations of the SHA-2 family (SHA-256, SHA-512, etc.).
* sha3: Implements the SHA-3 family (Keccak).
* blake2: Implements the BLAKE2 family of hash functions.

While implementations for older or cryptographically weaker algorithms like MD5 and SHA-1 also exist , their use is generally discouraged in new applications due to known vulnerabilities.

The use of traits like Digest encourages abstraction. 

The core logic of building a Merkle Tree—hashing leaves and then iteratively hashing pairs—is independent of the specific hash function employed. 

By utilizing the Digest trait, a Merkle Tree implementation could potentially switch between, for example, SHA-256 and SHA-512 with minimal modification. 

While the implementation presented here will use SHA-256 for simplicity, structuring the internal hashing operations to align with the Digest trait's methods reflects good Rust practice and highlights the ecosystem's modular design.

### Recommendation: sha2 Crate

For this simple Merkle Tree implementation, the sha2 crate, specifically using the Sha256 algorithm, is recommended. This choice is justified because:
* SHA-256 is a robust, widely trusted, and standardized cryptographic hash function frequently employed in Merkle Tree contexts, including Bitcoin.
* The sha2 crate provides a high-quality, pure-Rust implementation that integrates seamlessly with the digest crate's traits.
* It is actively maintained as part of the RustCrypto project.
To use this crate, add the following line to your project's Cargo.toml file under the [dependencies] section:
[dependencies]
sha2 = "0.10"

The basic usage pattern for hashing data with Sha256 via the Digest trait involves creating a hasher instance, feeding data into it using the update method, and finally calling finalize to retrieve the hash result. Here is a minimal example:

```rust

use sha2::{Sha256, Digest};

// Create a new SHA-256 hasher
let mut hasher = Sha256::new();

// Update the hasher with data (can be called multiple times)
hasher.update(b"some data");
hasher.update(b"more data");

// Finalize the hash computation and get the result
// finalize() consumes the hasher instance
let hash_result = hasher.finalize();

// hash_result is a GenericArray<u8, U32> representing the 32-byte SHA-256 hash
println!("SHA-256 Hash: {:x}", hash_result);

```

## Also See:

[Merkle Tree Structure](https://www.researchgate.net/figure/Merkle-tree-structure_fig1_368493549)


