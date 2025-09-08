CREATE EXTENSION IF NOT EXISTS citext;
CREATE EXTENSION IF NOT EXISTS pgcryto;

CREATE TABLE ai_entities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name CITEXT NOT NULL UNIQUE,
    height NUMERIC(3, 1) NOT NULL,
    weight NUMERIC(5, 1) NOT NULL,
    gender_id UUID NOT NULL REFERENCES genders(id),
    glb_file_url TEXT NOT NULL
);

COMMENT ON COLUMN ai_entities.height IS 'Height is in metres';
COMMENT ON COLUMN ai_entities.weight IS 'Weight is in kg';

CREATE TABLE personalities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name CITEXT NOT NULL UNIQUE
);

CREATE TABLE ai_entities_personalities (
    ai_entity_id UUID NOT NULL REFERENCES ai_entities(id) ON DELETE CASCADE,
    personality_id UUID NOT NULL REFERENCES personalities(id) ON DELETE CASCADE
)

CREATE TABLE genders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name CITEXT NOT NULL UNIQUE
)