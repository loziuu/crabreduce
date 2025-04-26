type NodeIdResult = Result<NodeId, NodeIdError>;

pub struct NodeId {
    value: String,
}

pub enum NodeIdError {
    InvalidValue,
}

impl NodeId {
    pub fn try_new(value: String) -> NodeIdResult {
        if value.is_empty() {
            return Err(NodeIdError::InvalidValue);
        }

        if value.len() < 5 || value.len() > 100 {
            return Err(NodeIdError::InvalidValue);
        }

        // TODO: Special characters?
        Ok(Self { value })
    }
}

impl TryFrom<String> for NodeId {
    type Error = NodeIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        NodeId::try_new(value)
    }
}
