pub(crate) struct Clique {
    pub preds: Vec<u32>,
    pub nodes: Vec<u32>
}

impl Clique {
    pub fn new(preds: &Vec<u32>, nodes: &Vec<u32>) -> Self {
        Clique { preds: preds.clone(), nodes:nodes.clone() }
    }

    pub fn merge(&mut self, c: &Clique) {
        self.preds.append(&mut c.preds.clone());
        self.nodes.append(&mut c.nodes.clone());
    }

    pub fn node_intersection(&self, c: &Clique) -> Vec<u32> {
        let mut intersection: Vec<u32> = Vec::new();

        for node in &self.nodes {
            if c.nodes.contains(&node) {
                intersection.push(node.clone());
            }
        }

        return intersection;
    }
}