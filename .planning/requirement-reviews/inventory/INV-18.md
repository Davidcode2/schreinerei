# INV-18

Status: Missing
Fit: Strong
Priority: Soon
Decision: Keep, but move to Assets/Maintenance

Current state: Sharpening/external-service states are not modeled in Inventory. Generic maintenance ideas belong closer to tracked assets than stock quantities.
Evidence: `src/modules/fleet/domain/tool.rs`, `.planning/FEATURES.md`

Implementation:
1. Classify serviceable cutters/blades as asset instances.
2. Model external-service workflow in Assets/Maintenance.
3. Keep consumable stock counts separate from service state.
4. Link asset instances back to inventory categories where useful.
