use crate::table::Table;

pub struct Builder {
    pub table_name: Table,
    select_query: Option<Vec<String>>,
    where_and_query: Option<Vec<String>>,
    where_or_query: Option<Vec<Vec<String>>>,
    order_by_query: Option<Vec<String>>,
    group_by_query: Option<Vec<String>>,
}

impl Builder {
    pub fn new(table_name: &str, alias: Option<&str>) -> Self {
        Self {
            table_name: Table {
                name: table_name.to_string(),
                alias: alias.map(|x| x.to_string()),
            },
            select_query: None,
            where_and_query: None,
            where_or_query: None,
            order_by_query: None,
            group_by_query: None,
        }
    }

    fn get_alias(&self) -> String {
        self.table_name
            .alias
            .clone()
            .unwrap_or(self.table_name.name.clone())
    }

    fn parse_from(&self, sql: &mut String) {
        if self.where_and_query.is_some() || self.where_or_query.is_some() {
            sql.push_str(format!(" FROM {} {} ", self.table_name.name, self.get_alias()).as_str());
        } else {
            sql.push_str(format!(" FROM {} {}", self.table_name.name, self.get_alias()).as_str());
        }
    }

    pub fn select_raw(&mut self, raw: &str) -> &mut Self {
        if self.select_query.is_none() {
            self.select_query = Some(vec![raw.to_string()]);
        } else {
            self.select_query.as_mut().unwrap().push(raw.to_string());
        }
        self
    }

    fn parse_select_raw(&self, sql: &mut String) {
        sql.push_str("SELECT");
        if self.select_query.is_some() {
            for (idx, item) in self.select_query.clone().unwrap().iter().enumerate() {
                if idx + 1 == self.select_query.clone().unwrap().len() {
                    sql.push_str(format!(" {}", item).as_str());
                } else {
                    sql.push_str(format!(" {},", item).as_str());
                }
            }
        } else {
            sql.push_str(" *");
        }
    }

    pub fn where_raw(&mut self, raw: &str) -> &mut Self {
        if self.where_and_query.is_none() {
            self.where_and_query = Some(vec![raw.to_string()]);
        } else {
            self.where_and_query.as_mut().unwrap().push(raw.to_string());
        }
        self
    }

    fn parse_where_raw(&self, sql: &mut String) {
        if self.where_and_query.is_none() {
            return;
        }
        for (idx, item) in self.where_and_query.clone().unwrap().iter().enumerate() {
            if idx + 1 == self.where_and_query.clone().unwrap().len() {
                sql.push_str(format!(" {}", item).as_str());
            } else {
                sql.push_str(format!(" {} AND", item).as_str());
            }
        }
    }

    pub fn where_or_raw(&mut self, raw: Vec<&str>) -> &mut Self {
        if self.where_or_query.is_none() {
            self.where_or_query = Some(vec![raw.iter().map(|f| f.to_string()).collect()]);
        } else {
            self.where_or_query
                .as_mut()
                .unwrap()
                .push(raw.iter().map(|f| f.to_string()).collect());
        }
        self
    }

    fn parse_where_or_raw(&self, sql: &mut String) {
        if self.where_or_query.is_none() {
            return;
        }
        for or_vec in self.where_or_query.clone().unwrap().iter() {
            if self.where_and_query.is_some() {
                sql.push_str(" AND (");
            } else {
                sql.push_str(" (");
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

    pub fn to_sql(&self) -> String {
        let mut sql = "".to_string();
        self.parse_select_raw(&mut sql);
        self.parse_from(&mut sql);
        // Where
        if self.where_and_query.is_some() || self.where_or_query.is_some() {
            sql.push_str("WHERE");
        }
        // And
        self.parse_where_raw(&mut sql);
        // Or
        self.parse_where_or_raw(&mut sql);
        // Order By
        self.parse_order_by_raw(&mut sql);
        // Group By
        self.parse_group_by_raw(&mut sql);

        sql
    }
}
