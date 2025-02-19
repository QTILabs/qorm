# Delete

qorm sql delete builder

## Delete Builder
simple init
```rust
use qorm::{Bind, Delete};
let mut builder = Delete::new("todo", None);
assert_eq!(builder.to_sql(), "DELETE FROM todo".to_string());
```
override placeholder (default:?)
```rust
use qorm::{delete_item::DeleteConfig, Bind, Delete};
let mut builder = Delete::new(
    "todo",
    Some(DeleteConfig {
        placeholder: "$%d".to_string(),
        start: Some(1)
    })
);
builder.wheres("id", "=", Bind::Int(1));
let (sql, binds) = builder.to_sql_with_bind();
assert_eq!(
    sql, "DELETE FROM todo WHERE id = $1"
);
let x = vec![Bind::Int(1)];
assert_eq!(binds.len(), x.len());
for idx in 0..binds.len() {
    assert_eq!(binds[idx], x[idx]);
}
```

## Where and
delete sql where and
```rust
use qorm::{Bind, Delete};

let mut builder = Delete::new("todo", None);
builder.wheres("id", "=", Bind::Int(1));
let (sql, binds) = builder.to_sql_with_bind();
assert_eq!(sql, "DELETE FROM todo WHERE id = ?");
let x = vec![Bind::Int(1)];
assert_eq!(binds.len(), x.len());
for idx in 0..binds.len() {
    assert_eq!(binds[idx], x[idx]);
}
```

## Where or
delete sql where or
```rust
use qorm::{where_item::Or, Bind, Delete};

let mut builder = Delete::new("todo", None);
builder.where_or(vec![
    Or {
        column: "id",
        operator: "=",
        value: Bind::Int(1)
    },
    Or {
        column: "id",
        operator: "=",
        value: Bind::Int(2)
    }
]);
let (sql, binds) = builder.to_sql_with_bind();
assert_eq!(sql, "DELETE FROM todo WHERE ( id = ? OR id = ?)");
let x = vec![Bind::Int(1), Bind::Int(2)];
assert_eq!(binds.len(), x.len());
for idx in 0..binds.len() {
    assert_eq!(binds[idx], x[idx]);
}
```
