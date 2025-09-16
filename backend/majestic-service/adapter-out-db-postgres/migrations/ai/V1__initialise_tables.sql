CREATE EXTENSION IF NOT EXISTS citext;

CREATE TABLE ai_entities (
    id UUID PRIMARY KEY,
    name CITEXT NOT NULL UNIQUE,
    height NUMERIC(3, 1) NOT NULL,
    weight NUMERIC(5, 1) NOT NULL,
    gender_id UUID NOT NULL REFERENCES genders(id),
    birthday DATE NOT NULL,
    glb_file_url TEXT NOT NULL,
    created_on TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_modified_on TIMESTAMPTZ NOT NULL DEFAULT now()
);

COMMENT ON COLUMN ai_entities.height IS 'Height is in metres';
COMMENT ON COLUMN ai_entities.weight IS 'Weight is in kg';
COMMENT ON COLUMN ai_entities.glb_file_url IS 'Public URL to download the AI entity 3D model file';

CREATE TABLE personalities (
    id UUID PRIMARY KEY,
    name CITEXT NOT NULL UNIQUE,
    created_on TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_modified_on TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE ai_entities_personalities (
    ai_entity_id UUID NOT NULL REFERENCES ai_entities(id) ON DELETE CASCADE,
    personality_id UUID NOT NULL REFERENCES personalities(id) ON DELETE CASCADE,
    created_on TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_modified_on TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE genders (
    id UUID PRIMARY KEY,
    name CITEXT NOT NULL UNIQUE,
    created_on TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_modified_on TIMESTAMPTZ NOT NULL DEFAULT now()
);