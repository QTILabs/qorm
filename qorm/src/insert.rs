use crate::{insert_item::InsertConfig, table::Table, Bind};

fn is_index(pattern: String) -> bool {
    pattern.contains("%d")
}

/// Insert
///
/// qorm sql insert builder
pub struct Insert {
    pub table_name: Table,
    config: InsertConfig,
    bind_index: Option<i32>,
    binds: Vec<Bind>,
    insert_values: Option<Vec<(String, Bind)>>,
}

impl Insert {
    /// Initialize Insert builder
    ///
    /// simple init
    /// ```rust
    /// use qorm::{Bind, Insert};
    ///
    /// let mut builder = Insert::new("user", None);
    /// builder.values(vec![
    ///     ("username", Bind::String("foo".to_string())),
    ///     ("is_active", Bind::Bool(true))
    /// ]);
    /// let (sql, binds) = builder.to_sql_with_bind();
    /// assert_eq!(
    ///     sql,
    ///     "INSERT INTO user (username,is_active) VALUES (?,?)"
    /// );
    /// let x = [
    ///     Bind::String("foo".to_string()),
    ///     Bind::Bool(true)
    /// ];
    /// assert_eq!(binds.len(), x.len());
    /// for idx in 0..binds.len() {
    ///     assert_eq!(binds[idx], x[idx]);
    /// }
    /// ```
    ///
    /// override placeholder
    /// ```rust
    /// use qorm::{insert_item::InsertConfig, Bind, Insert};
    ///
    /// let mut builder = Insert::new("user", Some(InsertConfig {
    ///     placeholder: "$%d".to_string(),
    ///     start: Some(1),
    /// }));
    /// builder.values(vec![
    ///     ("username", Bind::String("foo".to_string())),
    ///     ("is_active", Bind::Bool(true))
    /// ]);
    /// let (sql, binds) = builder.to_sql_with_bind();
    /// assert_eq!(
    ///     sql,
    ///     "INSERT INTO user (username,is_active) VALUES ($1,$2)"
    /// );
    /// let x = [
    ///     Bind::String("foo".to_string()),
    ///     Bind::Bool(true)
    /// ];
    /// assert_eq!(binds.len(), x.len());
    /// for idx in 0..binds.len() {
    ///     assert_eq!(binds[idx], x[idx]);
    /// }
    /// ```
    pub fn new(table_name: &str, config: Option<InsertConfig>) -> Self {
        let config_select = match config {
            Some(data) => data,
            None => InsertConfig {
                placeholder: "?".to_string(),
                start: Some(1),
            },
        };
        let bind_index = is_index(config_select.clone().placeholder);
        Self {
            table_name: Table {
                name: table_name.to_string(),
                alias: None,
            },
            config: config_select.clone(),
            bind_index: match bind_index {
                true => Some(config_select.start.unwrap()),
                false => None,
            },
            binds: vec![],
            insert_values: None,
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

    /// sql insert values
    ///
    /// how to use see [`Insert::new`]
    pub fn values(&mut self, value: Vec<(&str, Bind)>) -> &mut Self {
        if self.insert_values.is_none() {
            self.insert_values = Some(vec![]);
        }
        for (key, value) in value.into_iter() {
            self.insert_values
                .as_mut()
                .unwrap()
                .push((key.to_string(), value));
        }
        self
    }

    fn parse_values(&mut self, sql: &mut String) {
        if self.insert_values.is_none() {
            return;
        }
        let mut columns = " (".to_string();
        let mut values = "(".to_string();
        let num_data = self.insert_values.clone().unwrap().len();
        for (idx, (key, value)) in self.insert_values.clone().unwrap().into_iter().enumerate() {
            if idx + 1 == num_data {
                columns.push_str(key.as_str());
                values.push_str(self.gen_bind_key().as_str());
            } else {
                columns.push_str(format!("{},", key).as_str());
                values.push_str(format!("{},", self.gen_bind_key()).as_str());
            }
            self.binds.push(value.clone());
        }
        columns.push(')');
        values.push(')');
        sql.push_str(&columns);
        sql.push_str(" VALUES ");
        sql.push_str(&values);
    }

    /// get generated sql query
    ///
    /// how to use see [`Insert::new`]
    pub fn to_sql(&mut self) -> String {
        self.binds = vec![];
        // Insert
        let mut sql = format!("INSERT INTO {}", self.table_name.name);
        self.parse_values(&mut sql);
        sql
    }

    /// get generated sql query and it's bind
    ///
    /// how to use see [`Insert::new`]
    pub fn to_sql_with_bind(&mut self) -> (String, Vec<Bind>) {
        let sql = self.to_sql();
        (sql, self.binds.clone())
    }
}
