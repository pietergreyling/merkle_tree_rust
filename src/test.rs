#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merkle_root_consistency() {
        let data1 = vec!["a", "b", "c", "d"];
        let tree1 = MerkleTree::new(data1);

        let data2 = vec!["a", "b", "c", "d"];
        let tree2 = MerkleTree::new(data2);

        assert_eq!(tree1.root(), tree2.root());
    }

    #[test]
    fn test_merkle_root_changes_on_data_change() {
        let data1 = vec!["a", "b", "c", "d"];
        let tree1 = MerkleTree::new(data1);

        let data2 = vec!["a", "b", "x", "d"];
        let tree2 = MerkleTree::new(data2);

        assert_ne!(tree1.root(), tree2.root());
    }

    #[test]
    fn test_merkle_root_with_odd_number_of_leaves() {
        let data = vec!["a", "b", "c"];
        let tree = MerkleTree::new(data);
        assert!(!tree.root().is_empty());
    }

    #[test]
    fn test_single_element_merkle_tree() {
        let data = vec!["only"];
        let tree = MerkleTree::new(data);
        assert_eq!(tree.root(), &hash("only"));
    }
}