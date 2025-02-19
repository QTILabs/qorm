# Insert

qorm sql insert builder

## Insert builder

simple init
```rust
use qorm::{Bind, Insert};

let mut builder = Insert::new("user", None);
builder.values(vec![
    ("username", Bind::String("foo".to_string())),
    ("is_active", Bind::Bool(true))
]);
let (sql, binds) = builder.to_sql_with_bind();
assert_eq!(
    sql,
    "INSERT INTO user (username,is_active) VALUES (?,?)"
);
let x = [
    Bind::String("foo".to_string()),
    Bind::Bool(true)
];
assert_eq!(binds.len(), x.len());
for idx in 0..binds.len() {
    assert_eq!(binds[idx], x[idx]);
}
```
override placeholder
```rust
use qorm::{insert_item::InsertConfig, Bind, Insert};

let mut builder = Insert::new("user", Some(InsertConfig {
    placeholder: "$%d".to_string(),
    start: Some(1),
}));
builder.values(vec![
    ("username", Bind::String("foo".to_string())),
    ("is_active", Bind::Bool(true))
]);
let (sql, binds) = builder.to_sql_with_bind();
assert_eq!(
    sql,
    "INSERT INTO user (username,is_active) VALUES ($1,$2)"
);
let x = [
    Bind::String("foo".to_string()),
    Bind::Bool(true)
];
assert_eq!(binds.len(), x.len());
for idx in 0..binds.len() {
    assert_eq!(binds[idx], x[idx]);
}
```
