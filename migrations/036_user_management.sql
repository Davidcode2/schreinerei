ALTER TABLE users
    ADD COLUMN is_original_admin BOOLEAN NOT NULL DEFAULT FALSE,
    ADD COLUMN deleted_at TIMESTAMPTZ;

WITH ranked_admins AS (
    SELECT
        users.id,
        ROW_NUMBER() OVER (
            PARTITION BY users.tenant_id
            ORDER BY
                CASE WHEN EXISTS (
                    SELECT 1
                    FROM onboarding_sessions
                    WHERE onboarding_sessions.tenant_id = users.tenant_id
                      AND lower(onboarding_sessions.admin_email) = lower(users.email)
                ) THEN 0 ELSE 1 END,
                users.created_at,
                users.id
        ) AS tenant_rank
    FROM users
    WHERE users.role = 'admin'
)
UPDATE users
SET is_original_admin = TRUE
FROM ranked_admins
WHERE users.id = ranked_admins.id
  AND ranked_admins.tenant_rank = 1;

CREATE UNIQUE INDEX users_one_original_admin_per_tenant
    ON users (tenant_id)
    WHERE is_original_admin;

CREATE FUNCTION protect_original_admin()
RETURNS TRIGGER AS $$
BEGIN
    IF OLD.is_original_admin IS DISTINCT FROM NEW.is_original_admin THEN
        RAISE EXCEPTION 'is_original_admin is immutable';
    END IF;
    IF OLD.is_original_admin AND (
        OLD.role IS DISTINCT FROM NEW.role
        OR OLD.deleted_at IS DISTINCT FROM NEW.deleted_at
    ) THEN
        RAISE EXCEPTION 'original admin cannot be modified';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER users_original_admin_protected
    BEFORE UPDATE OF is_original_admin, role, deleted_at ON users
    FOR EACH ROW
    EXECUTE FUNCTION protect_original_admin();
