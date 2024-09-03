CREATE TABLE queue (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    priority BIGINT NOT NULL,
    parent UUID NOT NULL,
    metadata JSONB NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);