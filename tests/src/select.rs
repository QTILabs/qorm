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
        builder.select("user", "id");
        assert_eq!(builder.to_sql(), "SELECT user.id FROM user user");
    }

    #[test]
    fn multiple_select_query() {
        let mut builder = Select::new("user", None, None);
        builder.select("user", "id");
        builder.select("user", "name");
        assert_eq!(builder.to_sql(), "SELECT user.id, user.name FROM user user");
    }

    #[test]
    fn single_select_raw_query() {
        let mut builder = Select::new("user", None, None);
        builder.select_raw("user.id");
        assert_eq!(builder.to_sql(), "SELECT user.id FROM user user");
    }

    #[test]
    fn multiple_select_raw_query() {
        let mut builder = Select::new("user", None, None);
        builder.select_raw("user.id").select_raw("user.name");
        assert_eq!(builder.to_sql(), "SELECT user.id, user.name FROM user user");
    }

    #[test]
    fn combine_select() {
        let mut builder = Select::new("user", None, None);
        builder.select("user", "id").select("user", "name");
        builder.select_raw("user.is_active");
        assert_eq!(
            builder.to_sql(),
            "SELECT user.id, user.name, user.is_active FROM user user"
        );
    }

    #[test]
    fn single_where_query() {
        let mut builder = Select::new("user", None, None);
        builder.wheres("user.id", "=", Bind::Int(1));
        let (sql, binds) = builder.to_sql_with_bind();
        assert_eq!(sql, "SELECT * FROM user user WHERE user.id = ?");
        let answers = vec![Bind::Int(1)];
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
        let answers = vec![Bind::Int(1), Bind::String("Foo".to_string())];
        assert_eq!(binds.len(), 2);
        for idx in 0..binds.len() {
            assert_eq!(binds[idx], answers[idx]);
        }
    }

    #[test]
    fn single_where_raw_query() {
        let mut builder = Select::new("user", None, None);
        builder.where_raw("user.id = 1");
        assert_eq!(
            builder.to_sql(),
            "SELECT * FROM user user WHERE user.id = 1"
        );
    }

    #[test]
    fn multiple_where_raw_query() {
        let mut builder = Select::new("user", None, None);
        builder.where_raw("user.id = 1");
        builder.where_raw(r#"user.username = "Foo""#);
        assert_eq!(
            builder.to_sql(),
            r#"SELECT * FROM user user WHERE user.id = 1 AND user.username = "Foo""#
        );
    }

    #[test]
    fn single_where_bind_query() {
        let mut builder = Select::new("user", None, None);
        builder.where_raw("user.id = ?");
        builder.bind_raw(Bind::Int(1));
        let (sql, binds) = builder.to_sql_with_bind();
        assert_eq!(sql, "SELECT * FROM user user WHERE user.id = ?");
        assert_eq!(binds, vec![Bind::Int(1)]);
    }

    #[test]
    fn multiple_where_bind_query() {
        let mut builder = Select::new("user", None, None);
        builder.where_raw("user.id = ?");
        builder.bind_raw(Bind::Int(1));
        builder.where_raw(r#"user.username = ?"#);
        builder.bind_raw(Bind::String("Foo".to_string()));
        let (sql, binds) = builder.to_sql_with_bind();
        assert_eq!(
            sql,
            r#"SELECT * FROM user user WHERE user.id = ? AND user.username = ?"#
        );
        assert_eq!(binds, vec![Bind::Int(1), Bind::String("Foo".to_string())]);
    }

    #[test]
    fn combine_where() {
        let mut builder = Select::new(
            "user",
            None,
            Some(SelectConfig {
                placeholder: "$%d".to_string(),
                start: Some(1),
            }),
        );
        builder.wheres("user.id", "=", Bind::Int(1)).wheres(
            "user.username",
            "=",
            Bind::String("Foo".to_string()),
        );
        builder.where_raw("user.is_active = ?");
        builder.bind_raw(Bind::Bool(true));
        let (sql, binds) = builder.to_sql_with_bind();
        assert_eq!(
            sql,
            r#"SELECT * FROM user user WHERE user.id = $1 AND user.username = $2 AND user.is_active = ?"#
        );
        assert_eq!(
            binds,
            vec![
                Bind::Int(1),
                Bind::String("Foo".to_string()),
                Bind::Bool(true)
            ]
        );
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
        let answers = vec![Bind::Int(1), Bind::Bool(true)];
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
        let answers = vec![
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
    fn single_where_or_raw() {
        let mut builder = Select::new("user", None, None);
        builder.where_or_raw(vec!["user.id = 1", "user.is_active = true"]);
        assert_eq!(
            builder.to_sql(),
            r#"SELECT * FROM user user WHERE ( user.id = 1 OR user.is_active = true)"#
        );
    }

    #[test]
    fn multiple_where_or_raw() {
        let mut builder = Select::new("user", None, None);
        builder.where_raw(r#"user.username = "Foo""#);
        builder.where_or_raw(vec!["user.id = 1", "user.is_active = true"]);
        assert_eq!(
            builder.to_sql(),
            r#"SELECT * FROM user user WHERE user.username = "Foo" AND ( user.id = 1 OR user.is_active = true)"#
        );
    }

    #[test]
    fn single_order_by_query() {
        let mut builder = Select::new("user", None, None);
        builder.order_by_raw(vec!["user.id ASC"]);
        assert_eq!(
            builder.to_sql(),
            r#"SELECT * FROM user user ORDER BY user.id ASC"#
        );
    }

    #[test]
    fn multiple_order_by_query() {
        let mut builder = Select::new("user", None, None);
        builder.order_by_raw(vec!["user.id ASC"]);
        builder.order_by_raw(vec!["user.username DESC", "user.profile ASC"]);
        assert_eq!(
            builder.to_sql(),
            r#"SELECT * FROM user user ORDER BY user.id ASC, user.username DESC, user.profile ASC"#
        );
    }

    #[test]
    fn full_query_raw() {
        let mut builder = Select::new("user", None, None);
        builder
            .select_raw("user.id")
            .select_raw("user.name")
            .select_raw("user.is_done");
        builder.where_raw(r#"user.username = "Foo""#);
        builder.where_raw("user.id = 1");
        builder.where_or_raw(vec!["user.id = 1", "user.is_active = true"]);
        builder.where_raw("user.is_active = true");
        builder.order_by_raw(vec!["user.id ASC"]);
        builder.order_by_raw(vec!["user.username DESC", "user.profile ASC"]);
        assert_eq!(
            builder.to_sql(),
            r#"SELECT user.id, user.name, user.is_done FROM user user WHERE user.username = "Foo" AND user.id = 1 AND user.is_active = true AND ( user.id = 1 OR user.is_active = true) ORDER BY user.id ASC, user.username DESC, user.profile ASC"#
        );
    }

    #[test]
    fn full_query() {
        let mut builder = Select::new("user", None, None);
        builder
            .select("user", "id")
            .select("user", "name")
            .select("user", "is_done");
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
        // builder.order_by_raw(vec!["user.id ASC"]);
        // builder.order_by_raw(vec!["user.username DESC", "user.profile ASC"]);
        let (sql, binds) = builder.to_sql_with_bind();
        assert_eq!(
            sql,
            r#"SELECT user.id, user.name, user.is_done FROM user user WHERE user.username = ? AND user.id = ? AND ( user.id = ? OR user.is_active = ?) AND ( user.is_active = ?)"#
        );
        let answer = vec![
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
