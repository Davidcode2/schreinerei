# COM-11

Status: Missing
Fit: Medium-high after COM-10
Priority: Later
Decision: Keep

Current state: There is no audio capture, transcription pipeline, or persisted transcript model.
Evidence: `frontend/package.json`, `src/modules/sites/*`

Implementation:
1. Ship raw voice-note capture first.
2. Add async transcription into structured transcript fields.
3. Keep transcript review/edit before using it as canonical documentation.
4. Store confidence and source attachment linkage.
