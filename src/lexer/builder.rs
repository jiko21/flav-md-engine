pub mod builder {
    use crate::lexer::lexer::lexer::ElementNode;

    #[derive(Debug, PartialEq)]
    pub struct MdNode {
        element_nodes: Vec<ElementNode>,
    }

    impl MdNode {
        pub fn new(element_nodes: Vec<ElementNode>) -> Self {
            MdNode { element_nodes }
        }
    }
}
