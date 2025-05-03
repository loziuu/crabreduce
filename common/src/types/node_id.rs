use std::fmt::Display;

type NodeIdResult = Result<NodeId, NodeIdError>;

#[derive(Debug, Clone)]
pub struct NodeId {
    value: String,
}

#[derive(Debug)]
pub enum NodeIdError {
    InvalidValue(&'static str),
}

impl NodeId {
    pub fn try_new(value: String) -> NodeIdResult {
        let sanitize = value.trim();

        if sanitize.is_empty() {
            return Err(NodeIdError::InvalidValue("NodeId cannot be empty"));
        }

        if sanitize.len() < 5 || sanitize.len() > 100 {
            return Err(NodeIdError::InvalidValue(
                "Node id length must be between 5 and 100 characters",
            ));
        }

        // TODO: Special characters?
        Ok(Self {
            value: sanitize.to_string(),
        })
    }

    pub fn raw(value: String) -> Self {
        Self { value }
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

impl Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::NodeId;

    #[test]
    fn empty_node_id() {
        let res = NodeId::try_new(" ".to_string());

        assert!(res.is_err());
    }

    #[test]
    fn too_short_node_id() {
        let res = NodeId::try_new("test".to_string());

        assert!(res.is_err());
    }

    #[test]
    fn too_long_node_id() {
        let res = NodeId::try_new("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".to_string());

        assert!(res.is_err());
    }

    #[test]
    fn valid_node_id() {
        let node_id = NodeId::try_new("Node_id_1".to_string());

        assert!(node_id.is_ok());
    }

    #[test]
    fn to_string() {
        let node_id = NodeId::try_new("Node_id_1".to_string()).unwrap();

        assert_eq!("Node_id_1".to_string(), node_id.to_string());
    }
}
