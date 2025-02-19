# Update

qorm sql update builder

## Update builder
Initialize Update builder
simple init
```rust
use qorm::{Bind, Update};

let mut builder = Update::new("user", None);
builder.set(vec![
    ("username", Bind::String("foo".to_string())),
    ("is_active", Bind::Bool(true)),
]);
builder.wheres("id", "=", Bind::Int(1));
let (sql, binds) = builder.to_sql_with_bind();
assert_eq!(sql, "UPDATE user SET username = ?,is_active = ? WHERE id = ?");
let x = vec![
    Bind::String("foo".to_string()),
    Bind::Bool(true),
    Bind::Int(1)
];
assert_eq!(x.len(), binds.len());
for idx in 0..binds.len() {
    assert_eq!(binds[idx], x[idx]);
}
```
override placeholder (default:?)
```rust
use qorm::{update_item::UpdateConfig, Bind, Update};

let mut builder = Update::new("user", Some(UpdateConfig{
    placeholder: "$%d".to_string(),
    start: Some(1)
}));
builder.set(vec![
    ("username", Bind::String("foo".to_string())),
    ("is_active", Bind::Bool(true)),
]);
builder.wheres("id", "=", Bind::Int(1));
let (sql, binds) = builder.to_sql_with_bind();
assert_eq!(sql, "UPDATE user SET username = $1,is_active = $2 WHERE id = $3");
let x = vec![
    Bind::String("foo".to_string()),
    Bind::Bool(true),
    Bind::Int(1)
];
assert_eq!(x.len(), binds.len());
for idx in 0..binds.len() {
    assert_eq!(binds[idx], x[idx]);
}
```

## Where And
update sql where and
```rust
use qorm::{Bind, Update};

let mut builder = Update::new("user", None);
builder.set(vec![
    ("is_active", Bind::Bool(true)),
]);
builder.wheres("id", "=", Bind::Int(1));
let (sql, binds) = builder.to_sql_with_bind();
assert_eq!(sql, "UPDATE user SET is_active = ? WHERE id = ?");
let x = vec![
    Bind::Bool(true),
    Bind::Int(1)
];
assert_eq!(x.len(), binds.len());
for idx in 0..binds.len() {
    assert_eq!(binds[idx], x[idx]);
}
```

## Where Or
update sql where or
```rust
use qorm::{where_item::Or, Bind, Update};

let mut builder = Update::new("user", None);
builder.set(vec![
    ("is_active", Bind::Bool(true)),
]);
builder.where_or(vec![
    Or {
        column: "is_active",
        operator: "IS",
        value: Bind::Bool(false)
    },
    Or {
        column: "username",
        operator: "=",
        value: Bind::String("John".to_string())
    }
]);
let (sql, binds) = builder.to_sql_with_bind();
assert_eq!(sql, "UPDATE user SET is_active = ? WHERE ( is_active IS ? OR username = ?)");
let x = vec![
    Bind::Bool(true),
    Bind::Bool(false),
    Bind::String("John".to_string()),
];
assert_eq!(x.len(), binds.len());
for idx in 0..binds.len() {
    assert_eq!(binds[idx], x[idx]);
}
```
