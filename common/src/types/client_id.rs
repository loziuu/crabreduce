type NodeIdResult = Result<NodeId, NodeIdError>;

pub struct NodeId {
    value: String,
}

pub enum NodeIdError {
    InvalidValue(&'static str),
}

impl NodeId {
    pub fn try_new(value: String) -> NodeIdResult {
        if value.is_empty() {
            return Err(NodeIdError::InvalidValue("NodeId cannot be empty"));
        }

        if value.len() < 5 || value.len() > 100 {
            return Err(NodeIdError::InvalidValue(
                "Node id length must be between 5 and 100 characters",
            ));
        }

        // TODO: Special characters?
        Ok(Self { value })
    }

    pub fn id(&self) -> &str {
        &self.value
    }
}

impl TryFrom<String> for NodeId {
    type Error = NodeIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        NodeId::try_new(value)
    }
}
