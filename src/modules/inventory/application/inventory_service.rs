use crate::common::error::AppError;
use crate::modules::inventory::infrastructure::MaterialRepository;

/// Service for inventory business logic
pub struct InventoryService {
    #[allow(dead_code)]
    material_repo: MaterialRepository,
}

impl InventoryService {
    pub fn new(material_repo: MaterialRepository) -> Self {
        Self { material_repo }
    }
}
