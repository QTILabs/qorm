#[cfg(test)]
mod tests {
    use qorm::{select_item::SelectConfig, where_item::Or, Bind, Select};

    #[test]
    fn init_select() {
        let mut builder = Select::new("user", None, None);
        assert_eq!(builder.to_sql(), "SELECT * FROM user user");
    }

    #[test]
    fn init_select_with_alias() {
        let mut builder = Select::new("user", Some("u"), None);
        assert_eq!(builder.to_sql(), "SELECT * FROM user u");
    }

    #[test]
    fn single_select_query() {
        let mut builder = Select::new("user", None, None);
        builder.select("user.id");
        assert_eq!(builder.to_sql(), "SELECT user.id FROM user user");
    }

    #[test]
    fn multiple_select_raw_query() {
        let mut builder = Select::new("user", None, None);
        builder.select("user.id").select("user.name");
        assert_eq!(builder.to_sql(), "SELECT user.id, user.name FROM user user");
    }

    #[test]
    fn single_join() {
        let mut builder = Select::new("user", None, None);
        builder.join(Some("LEFT"), "role", "user.role_id = role.id");
        assert_eq!(
            builder.to_sql(),
            "SELECT * FROM user user LEFT JOIN role ON user.role_id = role.id"
        );
    }

    #[test]
    fn multiple_join() {
        let mut builder = Select::new("user", None, None);
        builder.join(None, "role", "user.role_id = role.id");
        builder.join(Some("LEFT"), "location", "user.location_id = location.id");
        assert_eq!(builder.to_sql(), "SELECT * FROM user user JOIN role ON user.role_id = role.id LEFT JOIN location ON user.location_id = location.id");
    }

    #[test]
    fn single_where_query() {
        let mut builder = Select::new("user", None, None);
        builder.wheres("user.id", "=", Bind::Int(1));
        let (sql, binds) = builder.to_sql_with_bind();
        assert_eq!(sql, "SELECT * FROM user user WHERE user.id = ?");
        let answers = [Bind::Int(1)];
        assert_eq!(binds.len(), 1);
        for idx in 0..binds.len() {
            assert_eq!(binds[idx], answers[idx]);
        }
    }

    #[test]
    fn multiple_where_query() {
        let mut builder = Select::new(
            "user",
            None,
            Some(SelectConfig {
                placeholder: "$%d".to_string(),
                start: Some(1),
            }),
        );
        builder.wheres("user.id", "=", Bind::Int(1));
        builder.wheres("user.username", "=", Bind::String("Foo".to_string()));
        let (sql, binds) = builder.to_sql_with_bind();
        assert_eq!(
            sql,
            r#"SELECT * FROM user user WHERE user.id = $1 AND user.username = $2"#
        );
        let answers = [Bind::Int(1), Bind::String("Foo".to_string())];
        assert_eq!(binds.len(), 2);
        for idx in 0..binds.len() {
            assert_eq!(binds[idx], answers[idx]);
        }
    }

    #[test]
    fn single_where_or() {
        let mut builder = Select::new("user", None, None);
        builder.where_or(vec![
            Or {
                column: "user.id",
                operator: "=",
                value: Bind::Int(1),
            },
            Or {
                column: "user.is_active",
                operator: "=",
                value: Bind::Bool(true),
            },
        ]);
        let (sql, binds) = builder.to_sql_with_bind();
        assert_eq!(
            sql,
            r#"SELECT * FROM user user WHERE ( user.id = ? OR user.is_active = ?)"#
        );
        let answers = [Bind::Int(1), Bind::Bool(true)];
        assert_eq!(binds.len(), answers.len());
        for idx in 0..binds.len() {
            assert_eq!(binds[idx], answers[idx]);
        }
    }

    #[test]
    fn multiple_where_or() {
        let mut builder = Select::new("user", None, None);
        builder.where_or(vec![Or {
            column: "user.username",
            operator: "=",
            value: Bind::String("Foo".to_string()),
        }]);
        builder.where_or(vec![
            Or {
                column: "user.id",
                operator: "=",
                value: Bind::Int(1),
            },
            Or {
                column: "user.is_active",
                operator: "=",
                value: Bind::Bool(true),
            },
        ]);
        let (sql, binds) = builder.to_sql_with_bind();
        assert_eq!(
            sql,
            r#"SELECT * FROM user user WHERE ( user.username = ?) AND ( user.id = ? OR user.is_active = ?)"#
        );
        let answers = [
            Bind::String("Foo".to_string()),
            Bind::Int(1),
            Bind::Bool(true),
        ];
        assert_eq!(binds.len(), answers.len());
        for idx in 0..binds.len() {
            assert_eq!(binds[idx], answers[idx]);
        }
    }

    #[test]
    fn single_order_by_query() {
        let mut builder = Select::new("user", None, None);
        builder.order_by(vec!["user.id ASC"]);
        assert_eq!(
            builder.to_sql(),
            r#"SELECT * FROM user user ORDER BY user.id ASC"#
        );
    }

    #[test]
    fn multiple_order_by_query() {
        let mut builder = Select::new("user", None, None);
        builder.order_by(vec!["user.id ASC"]);
        builder.order_by(vec!["user.username DESC", "user.profile ASC"]);
        assert_eq!(
            builder.to_sql(),
            r#"SELECT * FROM user user ORDER BY user.id ASC, user.username DESC, user.profile ASC"#
        );
    }

    #[test]
    fn single_group_by_query() {
        let mut builder = Select::new("user", None, None);
        builder.group_by(vec!["user.id"]);
        assert_eq!(
            builder.to_sql(),
            r#"SELECT * FROM user user GROUP BY user.id"#
        );
    }

    #[test]
    fn multiple_group_by_query() {
        let mut builder = Select::new("user", None, None);
        builder.group_by(vec!["user.id"]);
        builder.group_by(vec!["user.username", "user.profile"]);
        assert_eq!(
            builder.to_sql(),
            r#"SELECT * FROM user user GROUP BY user.id, user.username, user.profile"#
        );
    }

    #[test]
    fn full_query() {
        let mut builder = Select::new("user", None, None);
        builder
            .select("user.id")
            .select("user.name")
            .select("user.is_done");
        builder.join(None, "role", "role.id = user.role_id");
        builder.join(Some("LEFT"), "location", "location.id = user.location_id");
        builder.wheres("user.username", "=", Bind::String("Foo".to_string()));
        builder.wheres("user.id", "=", Bind::Int(1));
        builder.where_or(vec![
            Or {
                column: "user.id",
                operator: "=",
                value: Bind::Int(1),
            },
            Or {
                column: "user.is_active",
                operator: "=",
                value: Bind::Bool(true),
            },
        ]);
        builder.where_or(vec![Or {
            column: "user.is_active",
            operator: "=",
            value: Bind::Bool(true),
        }]);
        builder.order_by(vec!["user.id ASC"]);
        builder.group_by(vec!["user.id"]);
        let (sql, binds) = builder.to_sql_with_bind();
        assert_eq!(
            sql,
            r#"SELECT user.id, user.name, user.is_done FROM user user JOIN role ON role.id = user.role_id LEFT JOIN location ON location.id = user.location_id WHERE user.username = ? AND user.id = ? AND ( user.id = ? OR user.is_active = ?) AND ( user.is_active = ?) ORDER BY user.id ASC GROUP BY user.id"#
        );
        let answer = [
            Bind::String("Foo".to_string()),
            Bind::Int(1),
            Bind::Int(1),
            Bind::Bool(true),
            Bind::Bool(true),
        ];
        assert_eq!(binds.len(), answer.len());
        for idx in 0..binds.len() {
            assert_eq!(answer[idx], binds[idx]);
        }
    }
}
