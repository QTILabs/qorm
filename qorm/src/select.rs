use crate::{select_item::SelectConfig, table::Table, where_item::Or, Bind};

fn is_index(pattern: String) -> bool {
    pattern.contains("%d")
}

#[derive(Clone, Debug)]
struct WhereInternal {
    pub column: String,
    pub operator: String,
    pub value: Bind,
}

#[derive(Clone, Debug)]
struct JoinInternal {
    pub join_type: Option<String>,
    pub table_name: String,
    pub on: String,
}

pub struct Select {
    pub table_name: Table,
    config: SelectConfig,
    select: Option<Vec<String>>,
    join: Option<Vec<JoinInternal>>,
    where_and: Option<Vec<WhereInternal>>,
    where_or: Option<Vec<Vec<WhereInternal>>>,
    order_by_query: Option<Vec<String>>,
    group_by_query: Option<Vec<String>>,
    limit: Option<i64>,
    offset: Option<i64>,
    bind_index: Option<i32>,
    binds: Vec<Bind>,
}

impl Select {
    pub fn new(table_name: &str, alias: Option<&str>, config: Option<SelectConfig>) -> Self {
        let config_select = match config {
            Some(data) => data,
            None => SelectConfig {
                placeholder: "?".to_string(),
                start: Some(1),
            },
        };
        let bind_index = is_index(config_select.clone().placeholder);
        Self {
            table_name: Table {
                name: table_name.to_string(),
                alias: alias.map(|x| x.to_string()),
            },
            config: config_select.clone(),
            select: None,
            join: None,
            where_and: None,
            where_or: None,
            order_by_query: None,
            group_by_query: None,
            limit: None,
            offset: None,
            bind_index: match bind_index {
                true => Some(config_select.start.unwrap()),
                false => None,
            },
            binds: vec![],
        }
    }

    fn get_alias(&self) -> String {
        self.table_name
            .alias
            .clone()
            .unwrap_or(self.table_name.name.clone())
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

    fn parse_from(&self, sql: &mut String) {
        sql.push_str(format!(" FROM {} {}", self.table_name.name, self.get_alias()).as_str());
    }

    pub fn select(&mut self, raw: &str) -> &mut Self {
        if self.select.is_none() {
            self.select = Some(vec![raw.to_string()]);
        } else {
            self.select.as_mut().unwrap().push(raw.to_string());
        }
        self
    }

    fn parse_select(&self, sql: &mut String) {
        if self.select.is_none() {
            return;
        }

        for (idx, item) in self.select.clone().unwrap().iter().enumerate() {
            if idx + 1 == self.select.clone().unwrap().len() {
                sql.push_str(format!(" {}", item).as_str());
            } else {
                sql.push_str(format!(" {},", item).as_str());
            }
        }
    }

    pub fn join(&mut self, join_type: Option<&str>, table_name: &str, on: &str) -> &mut Self {
        if self.join.is_none() {
            self.join = Some(vec![JoinInternal {
                join_type: join_type.map(|f| f.to_string()),
                table_name: table_name.to_string(),
                on: on.to_string(),
            }]);
        } else {
            self.join.as_mut().unwrap().push(JoinInternal {
                join_type: join_type.map(|f| f.to_string()),
                table_name: table_name.to_string(),
                on: on.to_string(),
            });
        }
        self
    }

    fn parse_join(&self, sql: &mut String) {
        if self.join.is_none() {
            return;
        }

        for item in self.join.clone().unwrap() {
            if item.join_type.is_some() {
                sql.push_str(
                    format!(
                        " {} JOIN {} ON {}",
                        item.join_type.unwrap(),
                        item.table_name,
                        item.on
                    )
                    .as_str(),
                );
            } else {
                sql.push_str(format!(" JOIN {} ON {}", item.table_name, item.on).as_str());
            }
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
            self.bind_push(item.value.clone());
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
                self.bind_push(item.value.clone());
            }
            sql.push(')');
        }
    }

    pub fn order_by(&mut self, raw: Vec<&str>) -> &mut Self {
        if self.order_by_query.is_none() {
            self.order_by_query = Some(raw.iter().map(|f| f.to_string()).collect())
        } else {
            for item in raw {
                self.order_by_query.as_mut().unwrap().push(item.to_string());
            }
        }

        self
    }

    fn parse_order_by(&self, sql: &mut String) {
        if self.order_by_query.clone().is_none() {
            return;
        }
        sql.push_str(" ORDER BY ");
        for (idx, item) in self.order_by_query.clone().unwrap().iter().enumerate() {
            if idx + 1 == self.order_by_query.clone().unwrap().len() {
                sql.push_str(item.as_str());
            } else {
                sql.push_str(format!("{}, ", item).as_str());
            }
        }
    }

    pub fn group_by(&mut self, raw: Vec<&str>) -> &mut Self {
        if self.group_by_query.is_none() {
            self.group_by_query = Some(raw.iter().map(|f| f.to_string()).collect())
        } else {
            for item in raw {
                self.group_by_query.as_mut().unwrap().push(item.to_string());
            }
        }

        self
    }

    fn parse_group_by(&self, sql: &mut String) {
        if self.group_by_query.clone().is_none() {
            return;
        }
        sql.push_str(" GROUP BY ");
        for (idx, item) in self.group_by_query.clone().unwrap().iter().enumerate() {
            if idx + 1 == self.group_by_query.clone().unwrap().len() {
                sql.push_str(item.as_str());
            } else {
                sql.push_str(format!("{}, ", item).as_str());
            }
        }
    }

    pub fn limit(&mut self, limit: i64) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    fn parse_limit(&self, sql: &mut String) {
        if self.limit.is_none() {
            return;
        }
        sql.push_str(format!(" LIMIT {}", self.limit.unwrap()).as_str());
    }

    pub fn offset(&mut self, offset: i64) -> &mut Self {
        self.offset = Some(offset);
        self
    }

    fn parse_offset(&self, sql: &mut String) {
        if self.offset.is_none() {
            return;
        }
        sql.push_str(format!(" OFFSET {}", self.offset.unwrap()).as_str());
    }

    fn bind_push(&mut self, raw: Bind) -> &mut Self {
        self.binds.push(raw);
        self
    }

    pub fn to_sql(&mut self) -> String {
        self.binds = vec![];
        // Select
        let mut sql = "SELECT".to_string();
        if self.select.is_none() {
            sql.push_str(" *");
        }
        self.parse_select(&mut sql);

        self.parse_from(&mut sql);

        // Join
        self.parse_join(&mut sql);

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
        // Order By
        self.parse_order_by(&mut sql);
        // Group By
        self.parse_group_by(&mut sql);
        // limit
        self.parse_limit(&mut sql);
        // offset
        self.parse_offset(&mut sql);
        sql
    }

    pub fn to_sql_with_bind(&mut self) -> (String, Vec<Bind>) {
        let sql = self.to_sql();
        (sql, self.binds.clone())
    }
}
