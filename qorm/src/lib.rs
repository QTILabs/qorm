#![allow(clippy::needless_doctest_main)]
//! simple sql query builder in rust.
//!
//! ## Installation
//! ```sh
//! cargo add --git https://github.com/QTILabs/qorm.git
//! ```
//! or on cargo.toml
//! ```toml
//! [dependencies]
//! qorm = { git = "https://github.com/QTILabs/qorm.git" }
//! ```
//!
//! ## Getting Started
//! ### MySql/Sqlite
//! ```rust,no_run
//! use qorm::{where_item::Or, Bind, Select};
//!
//! fn main() {
//!     let mut builder = Select::new("user", None, None);
//!     builder
//!         .select("user.id")
//!         .select("user.name")
//!         .select("user.is_done");
//!     builder.join(None, "role", "role.id = user.role_id");
//!     builder.join(Some("LEFT"), "location", "location.id = user.location_id");
//!     builder.wheres("user.username", "=", Bind::String("Foo".to_string()));
//!     builder.wheres("user.id", "=", Bind::Int(1));
//!     builder.wheres("user.is_done", "IS NOT", Bind::Null);
//!     builder.where_or(vec![
//!             Or {
//!                 column: "user.id",
//!                 operator: "=",
//!                 value: Bind::Int(1),
//!             },
//!             Or {
//!                 column: "user.is_active",
//!                 operator: "=",
//!                 value: Bind::Bool(true),
//!             },
//!     ]);
//!     builder.order_by(vec!["user.id ASC", "role.id DESC"]);
//!     builder.group_by(vec!["user.id"]);
//!     builder.limit(5);
//!     builder.offset(10);
//!     let (sql, binds) = builder.to_sql_with_bind();
//! }
//! ```
//! variable sql will output:
//! ```sql
//!     SELECT user.id, user.name, user.is_done
//!     FROM user user
//!     JOIN role ON role.id = user.role_id
//!     LEFT JOIN location ON location.id = user.location_id
//!     WHERE user.username = ?
//!     AND user.id = ?
//!     AND user.is_done IS NOT NULL
//!     AND ( user.id = ? OR user.is_active = ?)
//!     ORDER BY user.id ASC, role.id DESC
//!     GROUP BY user.id
//!     LIMIT 5
//!     OFFSET 10
//! ```
//! variable binds will output
//! ```sh
//!     vec![
//!        Bind::String("Foo".to_string()),
//!        Bind::Int(1),
//!        Bind::Int(1),
//!        Bind::Bool(true),
//!     ]
//! ```
//!
//! ### Postgresql
//! Since postgres use $1 rather than ?, you have to override the select config.
//! ```rust,no_run
//! use qorm::{select_item::SelectConfig, where_item::Or, Bind, Select};
//!
//! fn main() {
//!     let mut builder = Select::new(
//!         "user",
//!         None,
//!         Some(SelectConfig {
//!             placeholder: "$%d".to_string(),
//!             start: Some(1),
//!         }),
//!     );
//!     builder
//!         .select("user.id")
//!         .select("user.name")
//!         .select("user.is_done");
//!     builder.join(None, "role", "role.id = user.role_id");
//!     builder.join(Some("LEFT"), "location", "location.id = user.location_id");
//!     builder.wheres("user.username", "=", Bind::String("Foo".to_string()));
//!     builder.wheres("user.id", "=", Bind::Int(1));
//!     builder.wheres("user.is_done", "IS NOT", Bind::Null);
//!     builder.where_or(vec![
//!             Or {
//!                 column: "user.id",
//!                 operator: "=",
//!                 value: Bind::Int(1),
//!             },
//!             Or {
//!                 column: "user.is_active",
//!                 operator: "=",
//!                 value: Bind::Bool(true),
//!             },
//!     ]);
//!     builder.order_by(vec!["user.id ASC", "role.id DESC"]);
//!     builder.group_by(vec!["user.id"]);
//!     builder.limit(5);
//!     builder.offset(10);
//!     let (sql, binds) = builder.to_sql_with_bind();
//! }
//! ```
//! variable sql will output:
//! ```sql
//!     SELECT user.id, user.name, user.is_done
//!     FROM user user
//!     JOIN role ON role.id = user.role_id
//!     LEFT JOIN location ON location.id = user.location_id
//!     WHERE user.username = $1
//!     AND user.id = $2
//!     AND user.is_done IS NOT NULL
//!     AND ( user.id = $3 OR user.is_active = $4)
//!     ORDER BY user.id ASC, role.id DESC
//!     GROUP BY user.id
//!     LIMIT 5
//!     OFFSET 10
//! ```
//! variable binds will output
//! ```sh
//!     vec![
//!        Bind::String("Foo".to_string()),
//!        Bind::Int(1),
//!        Bind::Int(1),
//!        Bind::Bool(true),
//!     ]
//! ```
//!
//! ### Read More
//! - sql select builder [`Select`]
//! - sql insert builder [`Insert`]
//! - sql update builder [`Update`]
//! - sql delete builder [`Delete`]
//!
pub mod bind;
pub mod delete;
pub mod delete_item;
pub mod insert;
pub mod insert_item;
pub mod select;
pub mod select_item;
pub mod table;
pub mod update;
pub mod update_item;
pub mod where_item;

pub use bind::Bind;
pub use delete::Delete;
pub use insert::Insert;
pub use select::Select;
pub use update::Update;
