pub const DELETE_EDGE: &str = r###"DELETE FROM edges WHERE source = ? OR target = ?"###;
pub const DELETE_NODE: &str = r###"DELETE FROM nodes WHERE id = ?"###;
pub const INSERT_EDGE: &str = r###"INSERT INTO edges VALUES(?, ?, json(?))"###;
pub const INSERT_NODE: &str = r###"INSERT INTO nodes VALUES(json(?))
"###;
pub const SCHEMA: &str = r###"CREATE TABLE IF NOT EXISTS nodes (
    body TEXT,
    id   TEXT GENERATED ALWAYS AS (json_extract(body, '$.id')) VIRTUAL NOT NULL UNIQUE
);

CREATE INDEX IF NOT EXISTS id_idx ON nodes(id);

CREATE TABLE IF NOT EXISTS edges (
    source     TEXT,
    target     TEXT,
    properties TEXT,
    UNIQUE(source, target, properties) ON CONFLICT REPLACE,
    FOREIGN KEY(source) REFERENCES nodes(id),
    FOREIGN KEY(target) REFERENCES nodes(id)
);

CREATE INDEX IF NOT EXISTS source_idx ON edges(source);
CREATE INDEX IF NOT EXISTS target_idx ON edges(target);
"###;
pub const SEARCH_EDGES_INBOUND: &str = r###"SELECT * FROM edges WHERE source = ?"###;
pub const SEARCH_EDGES_OUTBOUND: &str = r###"SELECT * FROM edges WHERE target = ?"###;
pub const SEARCH_EDGES: &str = r###"SELECT * FROM edges WHERE source = ? 
UNION
SELECT * FROM edges WHERE target = ?"###;
pub const UPDATE_NODE: &str = r###"UPDATE nodes SET body = json(?) WHERE id = ?"###;
