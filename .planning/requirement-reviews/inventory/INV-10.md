# INV-10

Status: Partial
Fit: Strong
Priority: Soon
Decision: Keep with bounded-context split

Current state: Inventory is material-centric. Vehicles/tools already live in fleet. Large machines and loan PCs are not modeled as stockable inventory items.
Evidence: `src/modules/inventory/domain/material.rs`, `src/modules/fleet/domain/*.rs`

Implementation:
1. Generalize inventory toward stockable `InventoryItem`/SKU concepts.
2. Keep individually tracked machines/tools/loan PCs in Assets, not Inventory.
3. Use category capabilities instead of one giant polymorphic table.
4. Preserve simple worker flows for common consumables.
