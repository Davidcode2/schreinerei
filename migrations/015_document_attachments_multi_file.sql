DO $$
DECLARE
    attachment_activity_unique text;
BEGIN
    SELECT constraint_name
    INTO attachment_activity_unique
    FROM information_schema.table_constraints
    WHERE table_name = 'site_activity_attachments'
      AND constraint_type = 'UNIQUE'
      AND constraint_name LIKE '%activity%'
    LIMIT 1;

    IF attachment_activity_unique IS NOT NULL THEN
        EXECUTE format(
            'ALTER TABLE site_activity_attachments DROP CONSTRAINT %I',
            attachment_activity_unique
        );
    END IF;
END $$;

ALTER TABLE site_activity_attachments
    ADD COLUMN IF NOT EXISTS original_filename TEXT NOT NULL DEFAULT 'attachment';
