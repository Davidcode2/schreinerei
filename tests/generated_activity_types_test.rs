use std::fs;

#[test]
fn generated_activity_response_includes_creator_name() {
    let generated = fs::read_to_string("frontend/src/types/generated.ts")
        .expect("read generated activity types");

    assert!(
        generated.contains("creator_name: string"),
        "generated ActivityResponse should include creator_name"
    );
}

#[test]
fn generated_attachment_response_keeps_existing_viewer_fields() {
    let generated = fs::read_to_string("frontend/src/types/generated.ts")
        .expect("read generated activity types");

    for field in [
        "attachment_id: string",
        "filename: string",
        "mime_type: string",
        "url: string",
        "thumbnail_url: string | null",
    ] {
        assert!(
            generated.contains(field),
            "missing generated field: {field}"
        );
    }
}
