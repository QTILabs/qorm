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
    pub table_name: String,
    pub on: String,
}

pub struct Select {
    pub table_name: Table,
    config: SelectConfig,
    select: Option<Vec<String>>,
    join: Option<Vec<JoinInternal>>,
    join_raw: Option<Vec<String>>,
    where_and: Option<Vec<WhereInternal>>,
    where_and_raw: Option<Vec<String>>,
    where_or: Option<Vec<Vec<WhereInternal>>>,
    where_or_raw: Option<Vec<Vec<String>>>,
    order_by_query: Option<Vec<String>>,
    group_by_query: Option<Vec<String>>,
    binds: Vec<Bind>,
    bind_raws: Vec<Bind>,
    bind_index: Option<i32>,
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
            join_raw: None,
            where_and: None,
            where_and_raw: None,
            where_or: None,
            where_or_raw: None,
            order_by_query: None,
            group_by_query: None,
            binds: vec![],
            bind_raws: vec![],
            bind_index: match bind_index {
                true => Some(config_select.start.unwrap()),
                false => None,
            },
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
        if self.where_and_raw.is_some()
            || self.where_and.is_some()
            || self.where_or_raw.is_some()
            || self.where_or.is_some()
            || self.join_raw.is_some()
            || self.join.is_some()
        {
            sql.push_str(format!(" FROM {} {} ", self.table_name.name, self.get_alias()).as_str());
        } else {
            sql.push_str(format!(" FROM {} {}", self.table_name.name, self.get_alias()).as_str());
        }
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

    pub fn join(&mut self, table_name: &str, on: &str) -> &mut Self {
        if self.join.is_none() {
            self.join = Some(vec![JoinInternal {
                table_name: table_name.to_string(),
                on: on.to_string(),
            }]);
        } else {
            self.join.as_mut().unwrap().push(JoinInternal {
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

        for (idx, item) in self.join.clone().unwrap().iter().enumerate() {
            if idx == 0 {
                sql.push_str(format!("JOIN {} ON {}", item.table_name, item.on).as_str());
            } else {
                sql.push_str(format!(" JOIN {} ON {}", item.table_name, item.on).as_str());
            }
        }
    }

    pub fn join_raw(&mut self, raw: &str) -> &mut Self {
        if self.join_raw.is_none() {
            self.join_raw = Some(vec![raw.to_string()]);
        } else {
            self.join_raw.as_mut().unwrap().push(raw.to_string());
        }
        self
    }

    fn parse_join_raw(&self, sql: &mut String) {
        if self.join_raw.is_none() {
            return;
        }
        for (idx, item) in self.join_raw.clone().unwrap().iter().enumerate() {
            if idx == 0 {
                sql.push_str(format!("JOIN {}", item).as_str());
            } else {
                sql.push_str(format!(" JOIN {}", item).as_str());
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

    pub fn where_raw(&mut self, raw: &str) -> &mut Self {
        if self.where_and_raw.is_none() {
            self.where_and_raw = Some(vec![raw.to_string()]);
        } else {
            self.where_and_raw.as_mut().unwrap().push(raw.to_string());
        }
        self
    }

    fn parse_where_raw(&self, sql: &mut String) {
        if self.where_and_raw.is_none() {
            return;
        }
        for (idx, item) in self.where_and_raw.clone().unwrap().iter().enumerate() {
            if idx + 1 == self.where_and_raw.clone().unwrap().len() {
                sql.push_str(format!(" {}", item).as_str());
            } else {
                sql.push_str(format!(" {} AND", item).as_str());
            }
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

    pub fn where_or_raw(&mut self, raw: Vec<&str>) -> &mut Self {
        if self.where_or_raw.is_none() {
            self.where_or_raw = Some(vec![raw.iter().map(|f| f.to_string()).collect()]);
        } else {
            self.where_or_raw
                .as_mut()
                .unwrap()
                .push(raw.iter().map(|f| f.to_string()).collect());
        }
        self
    }

    fn parse_where_or_raw(&self, sql: &mut String) {
        if self.where_or_raw.is_none() {
            return;
        }
        for (idx, or_vec) in self.where_or_raw.clone().unwrap().iter().enumerate() {
            if idx == 0 {
                sql.push_str(" (");
            } else {
                sql.push_str(" AND (");
            }

            for (idx, item) in or_vec.iter().enumerate() {
                if idx + 1 == or_vec.clone().len() {
                    sql.push_str(format!(" {}", item).as_str());
                } else {
                    sql.push_str(format!(" {} OR", item).as_str());
                }
            }
            sql.push(')');
        }
    }

    pub fn order_by_raw(&mut self, raw: Vec<&str>) -> &mut Self {
        if self.order_by_query.is_none() {
            self.order_by_query = Some(raw.iter().map(|f| f.to_string()).collect())
        } else {
            for item in raw {
                self.order_by_query.as_mut().unwrap().push(item.to_string());
            }
        }

        self
    }

    fn parse_order_by_raw(&self, sql: &mut String) {
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

    pub fn group_by_raw(&mut self, raw: Vec<&str>) -> &mut Self {
        if self.group_by_query.is_none() {
            self.group_by_query = Some(raw.iter().map(|f| f.to_string()).collect())
        } else {
            for item in raw {
                self.group_by_query.as_mut().unwrap().push(item.to_string());
            }
        }

        self
    }

    fn parse_group_by_raw(&self, sql: &mut String) {
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

    pub fn bind_raw(&mut self, raw: Bind) -> &mut Self {
        self.bind_raws.push(raw);
        self
    }

    fn bind_push(&mut self, raw: Bind) -> &mut Self {
        self.binds.push(raw);
        self
    }

    pub fn to_sql(&mut self) -> String {
        // Select
        let mut sql = "SELECT".to_string();
        if self.select.is_none() {
            sql.push_str(" *");
        }
        self.parse_select(&mut sql);

        self.parse_from(&mut sql);

        // Join
        self.parse_join(&mut sql);
        self.parse_join_raw(&mut sql);
        if (self.where_and_raw.is_some()
            || self.where_and.is_some()
            || self.where_or_raw.is_some()
            || self.where_or.is_some())
            && (self.join_raw.is_some() || self.join.is_some())
        {
            sql.push(' ');
        }

        // Where
        if self.where_and_raw.is_some()
            || self.where_and.is_some()
            || self.where_or_raw.is_some()
            || self.where_or.is_some()
        {
            sql.push_str("WHERE");
        }
        // And
        self.parse_where(&mut sql);
        if self.where_and.is_some() && self.where_and_raw.is_some() {
            sql.push_str(" AND");
        }
        self.parse_where_raw(&mut sql);
        // Or
        if (self.where_and_raw.is_some() || self.where_and.is_some())
            && (self.where_or_raw.is_some() || self.where_or.is_some())
        {
            sql.push_str(" AND");
        }
        self.parse_where_or(&mut sql);
        self.parse_where_or_raw(&mut sql);
        // Order By
        self.parse_order_by_raw(&mut sql);
        // Group By
        self.parse_group_by_raw(&mut sql);

        sql
    }

    pub fn to_sql_with_bind(&mut self) -> (String, Vec<Bind>) {
        let sql = self.to_sql();
        let mut all_bind: Vec<Bind> = vec![];
        let mut binds = self.binds.clone();
        let mut bind_raw = self.bind_raws.clone();
        all_bind.append(&mut binds);
        all_bind.append(&mut bind_raw);
        self.binds = vec![];
        (sql, all_bind)
    }
}
