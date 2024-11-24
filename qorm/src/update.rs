use crate::{update_item::UpdateConfig, where_item::Or, Bind};

fn is_index(pattern: String) -> bool {
    pattern.contains("%d")
}

#[derive(Clone, Debug)]
struct WhereInternal {
    pub column: String,
    pub operator: String,
    pub value: Bind,
}

pub struct Update {
    pub table_name: String,
    config: UpdateConfig,
    binds: Vec<Bind>,
    bind_index: Option<i32>,
    set_values: Option<Vec<(String, Bind)>>,
    where_and: Option<Vec<WhereInternal>>,
    where_or: Option<Vec<Vec<WhereInternal>>>,
}

impl Update {
    pub fn new(table_name: &str, config: Option<UpdateConfig>) -> Self {
        let config_select = match config {
            Some(data) => data,
            None => UpdateConfig {
                placeholder: "?".to_string(),
                start: Some(1),
            },
        };
        let bind_index = is_index(config_select.clone().placeholder);
        Self {
            table_name: table_name.to_string(),
            config: config_select.clone(),
            set_values: None,
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

    pub fn set(&mut self, value: Vec<(&str, Bind)>) -> &mut Self {
        if self.set_values.is_none() {
            self.set_values = Some(vec![]);
        }
        for (key, value) in value.into_iter() {
            self.set_values
                .as_mut()
                .unwrap()
                .push((key.to_string(), value));
        }
        self
    }

    fn parse_set(&mut self, sql: &mut String) {
        if self.set_values.is_none() {
            return;
        }
        sql.push_str(" SET ");
        let num_data = self.set_values.clone().unwrap().len();
        for (idx, (key, value)) in self.set_values.clone().unwrap().into_iter().enumerate() {
            if idx + 1 == num_data {
                sql.push_str(format!("{} = {}", key, self.gen_bind_key()).as_str());
            } else {
                sql.push_str(format!("{} = {},", key, self.gen_bind_key()).as_str());
            }
            self.binds.push(value.clone());
        }
    }

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

    pub fn to_sql(&mut self) -> String {
        self.binds = vec![];
        // Update
        let mut sql = format!("UPDATE {}", self.table_name).to_string();

        // Set
        self.parse_set(&mut sql);

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

    pub fn to_sql_with_bind(&mut self) -> (String, Vec<Bind>) {
        let sql = self.to_sql();
        (sql, self.binds.clone())
    }
}