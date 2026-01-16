use crate::Identifier;
use std::collections::HashMap;
use std::sync::RwLock;

pub type BlockStateId = u32;

#[derive(Clone, Debug)]
pub struct BlockState {
    pub id: BlockStateId,
    pub identifier: Identifier,
    // Properties would be stored here in a full implementation
}

impl BlockState {
    pub fn new(id: BlockStateId, identifier: Identifier) -> Self {
        Self { id, identifier }
    }

    /// Air block state (ID 0).
    pub fn air() -> Self {
        Self::new(0, Identifier::hytale("air"))
    }
}

/// Registry for block types.
pub struct BlockRegistry {
    by_id: RwLock<Vec<BlockState>>,
    by_identifier: RwLock<HashMap<Identifier, BlockStateId>>,
}

impl BlockRegistry {
    pub fn new() -> Self {
        let registry = Self {
            by_id: RwLock::new(Vec::new()),
            by_identifier: RwLock::new(HashMap::new()),
        };
        // Register air as the first block (ID 0)
        registry.register(Identifier::hytale("air"));
        registry
    }

    /// Register a new block type and return its ID.
    pub fn register(&self, identifier: Identifier) -> BlockStateId {
        let mut by_id = self.by_id.write().unwrap();
        let mut by_identifier = self.by_identifier.write().unwrap();

        let id = by_id.len() as BlockStateId;
        let state = BlockState::new(id, identifier.clone());
        by_id.push(state);
        by_identifier.insert(identifier, id);
        id
    }

    /// Get a block state by ID.
    pub fn get(&self, id: BlockStateId) -> Option<BlockState> {
        let by_id = self.by_id.read().unwrap();
        by_id.get(id as usize).cloned()
    }

    /// Get a block state ID by identifier.
    pub fn get_id(&self, identifier: &Identifier) -> Option<BlockStateId> {
        let by_identifier = self.by_identifier.read().unwrap();
        by_identifier.get(identifier).copied()
    }

    /// Number of registered block states.
    pub fn len(&self) -> usize {
        self.by_id.read().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Default for BlockRegistry {
    fn default() -> Self {
        Self::new()
    }
}
