use crate::{delete_item::DeleteConfig, where_item::Or, Bind};

fn is_index(pattern: String) -> bool {
    pattern.contains("%d")
}

#[derive(Clone, Debug)]
struct WhereInternal {
    pub column: String,
    pub operator: String,
    pub value: Bind,
}

/// sql delete builder
pub struct Delete {
    pub table_name: String,
    config: DeleteConfig,
    binds: Vec<Bind>,
    bind_index: Option<i32>,
    where_and: Option<Vec<WhereInternal>>,
    where_or: Option<Vec<Vec<WhereInternal>>>,
}

impl Delete {
    /// Initialize Delete builder
    ///
    /// simple init
    /// ```rust
    /// use qorm::{Bind, Delete};
    ///
    /// let mut builder = Delete::new("todo", None);
    /// assert_eq!(builder.to_sql(), "DELETE FROM todo".to_string());
    /// ```
    ///
    /// override placeholder (default:?)
    /// ```
    /// use qorm::{delete_item::DeleteConfig, Bind, Delete};
    ///
    /// let mut builder = Delete::new(
    ///     "todo",
    ///     Some(DeleteConfig {
    ///         placeholder: "$%d".to_string(),
    ///         start: Some(1)
    ///     })
    /// );
    /// builder.wheres("id", "=", Bind::Int(1));
    /// let (sql, binds) = builder.to_sql_with_bind();
    /// assert_eq!(
    ///     sql, "DELETE FROM todo WHERE id = $1"
    /// );
    /// let x = vec![Bind::Int(1)];
    /// assert_eq!(binds.len(), x.len());
    /// for idx in 0..binds.len() {
    ///     assert_eq!(binds[idx], x[idx]);
    /// }
    /// ```
    pub fn new(table_name: &str, config: Option<DeleteConfig>) -> Self {
        let config_select = match config {
            Some(data) => data,
            None => DeleteConfig {
                placeholder: "?".to_string(),
                start: Some(1),
            },
        };
        let bind_index = is_index(config_select.clone().placeholder);
        Self {
            table_name: table_name.to_string(),
            config: config_select.clone(),
            where_and: None,
            where_or: None,
            binds: vec![],
            bind_index: match bind_index {
                true => Some(config_select.start.unwrap()),
                false => None,
            },
        }
    }

    fn gen_bind_key(&mut self) -> String {
        if self.bind_index.is_some() {
            let index = self.bind_index.unwrap();
            self.bind_index = Some(index + 1);
            self.config
                .placeholder
                .clone()
                .replace("%d", &index.to_string())
        } else {
            self.config.placeholder.clone()
        }
    }

    /// sql delete where and
    /// ```rust
    /// use qorm::{Bind, Delete};
    ///
    /// let mut builder = Delete::new("todo", None);
    /// builder.wheres("id", "=", Bind::Int(1));
    /// let (sql, binds) = builder.to_sql_with_bind();
    /// assert_eq!(sql, "DELETE FROM todo WHERE id = ?");
    /// let x = vec![Bind::Int(1)];
    /// assert_eq!(binds.len(), x.len());
    /// for idx in 0..binds.len() {
    ///     assert_eq!(binds[idx], x[idx]);
    /// }
    /// ```
    pub fn wheres(&mut self, column: &str, operator: &str, value: Bind) -> &mut Self {
        if self.where_and.is_none() {
            self.where_and = Some(vec![WhereInternal {
                column: column.to_string(),
                operator: operator.to_string(),
                value,
            }])
        } else {
            self.where_and.as_mut().unwrap().push(WhereInternal {
                column: column.to_string(),
                operator: operator.to_string(),
                value,
            });
        }
        self
    }

    fn parse_where(&mut self, sql: &mut String) {
        if self.where_and.is_none() {
            return;
        }
        for (idx, item) in self.where_and.clone().unwrap().iter().enumerate() {
            if idx + 1 == self.where_and.clone().unwrap().len() {
                sql.push_str(
                    format!(" {} {} {}", item.column, item.operator, self.gen_bind_key()).as_str(),
                );
            } else {
                sql.push_str(
                    format!(
                        " {} {} {} AND",
                        item.column,
                        item.operator,
                        self.gen_bind_key()
                    )
                    .as_str(),
                );
            }
            self.binds.push(item.value.clone());
        }
    }

    /// sql delete where or
    /// ```rust
    /// use qorm::{where_item::Or, Bind, Delete};
    ///
    /// let mut builder = Delete::new("todo", None);
    /// builder.where_or(vec![
    ///     Or {
    ///         column: "id",
    ///         operator: "=",
    ///         value: Bind::Int(1)
    ///     },
    ///     Or {
    ///         column: "id",
    ///         operator: "=",
    ///         value: Bind::Int(2)
    ///     }
    /// ]);
    /// let (sql, binds) = builder.to_sql_with_bind();
    /// assert_eq!(sql, "DELETE FROM todo WHERE ( id = ? OR id = ?)");
    /// let x = vec![Bind::Int(1), Bind::Int(2)];
    /// assert_eq!(binds.len(), x.len());
    /// for idx in 0..binds.len() {
    ///     assert_eq!(binds[idx], x[idx]);
    /// }
    /// ```
    pub fn where_or(&mut self, wheres: Vec<Or>) -> &mut Self {
        if self.where_or.is_none() {
            self.where_or = Some(vec![wheres
                .iter()
                .map(|f| WhereInternal {
                    column: f.column.to_string(),
                    operator: f.operator.to_string(),
                    value: f.value.clone(),
                })
                .collect()]);
        } else {
            self.where_or.as_mut().unwrap().push(
                wheres
                    .iter()
                    .map(|f| WhereInternal {
                        column: f.column.to_string(),
                        operator: f.operator.to_string(),
                        value: f.value.clone(),
                    })
                    .collect(),
            );
        }
        self
    }

    fn parse_where_or(&mut self, sql: &mut String) {
        if self.where_or.is_none() {
            return;
        }
        for (idx, or_vec) in self.where_or.clone().unwrap().iter().enumerate() {
            if idx == 0 {
                sql.push_str(" (");
            } else {
                sql.push_str(" AND (");
            }

            for (idx, item) in or_vec.iter().enumerate() {
                if idx + 1 == or_vec.clone().len() {
                    sql.push_str(
                        format!(" {} {} {}", item.column, item.operator, self.gen_bind_key())
                            .as_str(),
                    );
                } else {
                    sql.push_str(
                        format!(
                            " {} {} {} OR",
                            item.column,
                            item.operator,
                            self.gen_bind_key()
                        )
                        .as_str(),
                    );
                }
                self.binds.push(item.value.clone());
            }
            sql.push(')');
        }
    }

    /// get generated sql query
    /// ```rust
    /// use qorm::{Bind, Delete};
    ///
    /// let mut builder = Delete::new("todo", None);
    /// assert_eq!(builder.to_sql(), "DELETE FROM todo");
    /// ```
    pub fn to_sql(&mut self) -> String {
        self.binds = vec![];
        // DELETE
        let mut sql = format!("DELETE FROM {}", self.table_name).to_string();

        // Where
        if self.where_and.is_some() || self.where_or.is_some() {
            sql.push_str(" WHERE");
        }
        // And
        self.parse_where(&mut sql);

        // Or
        if self.where_and.is_some() && self.where_or.is_some() {
            sql.push_str(" AND");
        }
        self.parse_where_or(&mut sql);

        sql
    }

    /// get generated sql query and it's bind
    /// ```rust
    /// use qorm::{Bind, Delete};
    ///
    /// let mut builder = Delete::new("todo", None);
    /// builder.wheres("id", "=", Bind::Int(1));
    /// let (sql, binds) = builder.to_sql_with_bind();
    /// assert_eq!(sql, "DELETE FROM todo WHERE id = ?");
    /// let x = vec![Bind::Int(1)];
    /// assert_eq!(binds.len(), x.len());
    /// for idx in 0..binds.len() {
    ///     assert_eq!(binds[idx], x[idx]);
    /// }
    /// ```
    pub fn to_sql_with_bind(&mut self) -> (String, Vec<Bind>) {
        let sql = self.to_sql();
        (sql, self.binds.clone())
    }
}
