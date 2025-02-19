# Select

qorm sql select builder

## Select builder

simple init
```rust
use qorm::Select;

let mut builder = Select::new("todo", None, None);
let sql = builder.to_sql();
assert_eq!(sql, "SELECT * FROM todo todo");
```

adding table alias
```rust
use qorm::Select;

let mut builder = Select::new("todo", Some("t"), None);
let sql = builder.to_sql();
assert_eq!(sql, "SELECT * FROM todo t");
```

override placeholder (default: ?)
```rust
use qorm::{select_item::SelectConfig, Bind, Select};

let mut builder = Select::new("todo", None, Some(SelectConfig {
    placeholder: "#%d".to_string(),
    start: Some(0)
}));
builder.wheres("todo.id", "=", Bind::Int(1));
builder.wheres("todo.name", "=", Bind::String("hello".to_string()));
let sql = builder.to_sql();
assert_eq!(sql, "SELECT * FROM todo todo WHERE todo.id = #0 AND todo.name = #1");
```

## Select
```rust
use qorm::Select;

let mut builder = Select::new("todo", Some("t"), None);
builder.select("id");
builder.select("t.name");
let sql = builder.to_sql();
assert_eq!(sql, "SELECT id, t.name FROM todo t");
```

aggregation
```rust
use qorm::Select;

let mut builder = Select::new("todo", None, None);
builder.select("count(todo)");
let sql = builder.to_sql();
assert_eq!(sql, "SELECT count(todo) FROM todo todo");
```

## Join
sql join
```rust
use qorm::Select;

let mut builder = Select::new("todo", Some("t"), None);
builder.join(None, "user u", "t.created_by = u.id");
builder.join(Some("LEFT"), "user_profile up", "u.id = up.user_id");
let sql = builder.to_sql();
assert_eq!(sql, "SELECT * FROM todo t JOIN user u ON t.created_by = u.id LEFT JOIN user_profile up ON u.id = up.user_id");
```

## Where And
sql where and
```rust
use qorm::{Bind, Select};

let mut builder = Select::new("todo", Some("t"), None);
builder.wheres("t.is_done", "=", Bind::Bool(true));
builder.wheres("t.name", "LIKE", Bind::String("%todo%".to_string()));
builder.wheres("t.created_by", "IS NOT", Bind::Null); // null will not added to binds vector
let (sql, binds) = builder.to_sql_with_bind();
assert_eq!(sql, "SELECT * FROM todo t WHERE t.is_done = ? AND t.name LIKE ? AND t.created_by IS NOT NULL");
let x = [
    Bind::Bool(true),
    Bind::String("%todo%".to_string())
];
assert_eq!(binds.len(), x.len());
for idx in 0..binds.len() {
    assert_eq!(x[idx], binds[idx]);
}
```

## Where Or
sql where or
```rust
use qorm::{where_item::Or, Bind, Select};

let mut builder = Select::new("user", Some("u"), None);
builder.where_or(vec![
    Or {
        column: "u.id",
        operator: "<",
        value: Bind::Int(10)
    },
    Or {
        column: "u.is_admin",
        operator: "IS",
        value: Bind::Bool(true)
    }
]);
let (sql, binds) = builder.to_sql_with_bind();
assert_eq!(sql, "SELECT * FROM user u WHERE ( u.id < ? OR u.is_admin IS ?)");
let x = [
    Bind::Int(10),
    Bind::Bool(true),
];
assert_eq!(binds.len(), x.len());
for idx in 0..binds.len() {
    assert_eq!(x[idx], binds[idx]);
}
```

## Group By
sql group by
```rust
use qorm::{Bind, Select};

let mut builder = Select::new("todo", Some("t"), None);
builder.group_by(vec!["t.created_by"]);
let sql = builder.to_sql();
assert_eq!(sql, "SELECT * FROM todo t GROUP BY t.created_by");
```

## Limit
sql limit
```rust
use qorm::{Bind, Select};
let mut builder = Select::new("todo", Some("t"), None);
builder.limit(10);
let sql = builder.to_sql();
assert_eq!(sql, "SELECT * FROM todo t LIMIT 10");
```

## Offset
sql offset
```rust
use qorm::{Bind, Select};

let mut builder = Select::new("todo", Some("t"), None);
builder.offset(10);
let sql = builder.to_sql();
assert_eq!(sql, "SELECT * FROM todo t OFFSET 10");
```

## Generate SQL query
get generated sql query
```rust
use qorm::{Bind, Select};
let mut builder = Select::new("todo", Some("t"), None);
let sql = builder.to_sql();
assert_eq!(sql, "SELECT * FROM todo t");
```

## Generate SQL query and Bind
get generated sql query and it's bind
```rust
use qorm::{Bind, Select};
let mut builder = Select::new("todo", Some("t"), None);
builder.wheres("t.is_done", "=", Bind::Bool(true));
let (sql, binds) = builder.to_sql_with_bind();
assert_eq!(sql, "SELECT * FROM todo t WHERE t.is_done = ?");
let x = [
    Bind::Bool(true),
];
assert_eq!(binds.len(), x.len());
for idx in 0..binds.len() {
    assert_eq!(x[idx], binds[idx]);
}
```
