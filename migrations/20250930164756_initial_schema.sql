-- main table: posts
CREATE TABLE post (
    id BIGSERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    slug TEXT UNIQUE NOT NULL,
    content_md TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- table of tags
CREATE TABLE tag (
    id BIGSERIAL PRIMARY KEY,
    name TEXT UNIQUE NOT NULL
);

-- posts and tags join
CREATE TABLE post_tag (
    post_id BIGINT NOT NULL REFERENCES post(id) ON DELETE CASCADE,
    tag_id  BIGINT NOT NULL REFERENCES tag(id) ON DELETE CASCADE,
    PRIMARY KEY (post_id, tag_id)
);

-- text indexes
CREATE INDEX posts_content_idx
    ON post USING GIN (to_tsvector('simple', content_md));

CREATE INDEX posts_title_idx
    ON post USING GIN (to_tsvector('simple', title));
