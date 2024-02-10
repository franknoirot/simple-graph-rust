
/// Generated SQL string from the [simple-graph library](https://github.com/dpapathanasiou/simple-graph/tree/main/sql)
pub const DELETE_EDGE: &str = r###"DELETE FROM edges WHERE source = ? AND target = ?"###;

/// Generated SQL string from the [simple-graph library](https://github.com/dpapathanasiou/simple-graph/tree/main/sql)
pub const DELETE_EDGES: &str = r###"DELETE FROM edges WHERE source = ? OR target = ?"###;

/// Generated SQL string from the [simple-graph library](https://github.com/dpapathanasiou/simple-graph/tree/main/sql)
pub const DELETE_INCOMING_EDGES: &str = r###"DELETE FROM edges WHERE target = ?"###;

/// Generated SQL string from the [simple-graph library](https://github.com/dpapathanasiou/simple-graph/tree/main/sql)
pub const DELETE_NODE: &str = r###"DELETE FROM nodes WHERE id = ?"###;

/// Generated SQL string from the [simple-graph library](https://github.com/dpapathanasiou/simple-graph/tree/main/sql)
pub const DELETE_OUTGOING_EDGES: &str = r###"DELETE FROM edges WHERE source = ?"###;

/// Generated SQL string from the [simple-graph library](https://github.com/dpapathanasiou/simple-graph/tree/main/sql)
pub const INSERT_EDGE: &str = r###"INSERT INTO edges VALUES(?, ?, json(?))"###;

/// Generated SQL string from the [simple-graph library](https://github.com/dpapathanasiou/simple-graph/tree/main/sql)
pub const INSERT_NODE: &str = r###"INSERT INTO nodes VALUES(json(?))
"###;

/// Generated SQL string from the [simple-graph library](https://github.com/dpapathanasiou/simple-graph/tree/main/sql)
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

/// Generated SQL string from the [simple-graph library](https://github.com/dpapathanasiou/simple-graph/tree/main/sql)
pub const SEARCH_EDGES_INBOUND: &str = r###"SELECT * FROM edges WHERE source = ?"###;

/// Generated SQL string from the [simple-graph library](https://github.com/dpapathanasiou/simple-graph/tree/main/sql)
pub const SEARCH_EDGES_OUTBOUND: &str = r###"SELECT * FROM edges WHERE target = ?"###;

/// Generated SQL string from the [simple-graph library](https://github.com/dpapathanasiou/simple-graph/tree/main/sql)
pub const SEARCH_EDGES: &str = r###"SELECT * FROM edges WHERE source = ? 
UNION
SELECT * FROM edges WHERE target = ?"###;

/// Generated SQL string from the [simple-graph library](https://github.com/dpapathanasiou/simple-graph/tree/main/sql)
pub const UPDATE_EDGE: &str = r###"UPDATE edges SET properties = json(?) WHERE source = ? AND target = ?"###;

/// Generated SQL string from the [simple-graph library](https://github.com/dpapathanasiou/simple-graph/tree/main/sql)
pub const UPDATE_NODE: &str = r###"UPDATE nodes SET body = json(?) WHERE id = ?"###;

/// Generated Jinja2 template strings that can create SQL function, from the [simple-graph library](https://github.com/dpapathanasiou/simple-graph/tree/main/sql)
pub const SEARCH_NODE_TEMPLATE: &str = r###"SELECT {{ result_column }} -- id|body
FROM nodes{% if tree %}, json_tree(body{% if key %}, '$.{{ key }}'{% endif %}){% endif %}{% if search_clauses %}
WHERE {% for search_clause in search_clauses %}
    {{ search_clause }}
{% endfor %}{% endif %}"###;

/// Generated Jinja2 template strings that can create SQL function, from the [simple-graph library](https://github.com/dpapathanasiou/simple-graph/tree/main/sql)
pub const SEARCH_WHERE_TEMPLATE: &str = r###"{% if and_or %}{{ and_or }}{% endif %}
{% if id_lookup %}id = ?{% endif %}
{% if key_value %}json_extract(body, '$.{{ key }}') {{ predicate }} ?{% endif %}
{% if tree %}{% if key %}(json_tree.key='{{ key }}' AND {% endif %}json_tree.value {{ predicate }} ?{% if key %}){% endif %}{% endif %}"###;

/// Generated Jinja2 template strings that can create SQL function, from the [simple-graph library](https://github.com/dpapathanasiou/simple-graph/tree/main/sql)
pub const TRAVERSE_TEMPLATE: &str = r###"WITH RECURSIVE traverse(x{% if with_bodies %}, y, obj{% endif %}) AS (
  SELECT id{% if with_bodies %}, '()', body {% endif %} FROM nodes WHERE id = ?
  UNION
  SELECT id{% if with_bodies %}, '()', body {% endif %} FROM nodes JOIN traverse ON id = x
  {% if inbound %}UNION
  SELECT source{% if with_bodies %}, '<-', properties {% endif %} FROM edges JOIN traverse ON target = x{% endif %}
  {% if outbound %}UNION
  SELECT target{% if with_bodies %}, '->', properties {% endif %} FROM edges JOIN traverse ON source = x{% endif %}
) SELECT x{% if with_bodies %}, y, obj {% endif %} FROM traverse;
"###;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imported_constants() {
        assert_eq!(
            DELETE_EDGE,
            "DELETE FROM edges WHERE source = ? AND target = ?"
        );
    }
}