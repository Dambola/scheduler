CREATE TABLE queue (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    priority BIGINT NOT NULL,
    task TEXT NOT NULL,
    metadata JSONB NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);