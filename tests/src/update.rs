#[cfg(test)]
mod tests {
    use qorm::{update_item::UpdateConfig, where_item::Or, Bind, Update};

    #[test]
    fn update_query() {
        let mut builder = Update::new("user", None);
        builder.set(vec![
            ("username", Bind::String("foo".to_string())),
            ("is_active", Bind::Bool(true)),
        ]);
        let (sql, binds) = builder.to_sql_with_bind();
        assert_eq!(sql, "UPDATE user SET username = ?,is_active = ?");
        let answer = [Bind::String("foo".to_string()), Bind::Bool(true)];
        assert_eq!(binds.len(), answer.len());
        for idx in 0..binds.len() {
            assert_eq!(binds[idx], answer[idx]);
        }
    }

    #[test]
    fn update_query_where() {
        let mut builder = Update::new(
            "user",
            Some(UpdateConfig {
                placeholder: "$%d".to_string(),
                start: Some(1),
            }),
        );
        builder.set(vec![
            ("username", Bind::String("foo".to_string())),
            ("is_active", Bind::Bool(true)),
        ]);
        builder.wheres("user.id", "=", Bind::Int(1));
        builder.wheres("user.username", "LIKE", Bind::String("%foo%".to_string()));
        builder.where_or(vec![
            Or {
                column: "user.is_active",
                operator: "IS",
                value: Bind::Bool(false),
            },
            Or {
                column: "user.deleted_at",
                operator: "IS NOT",
                value: Bind::Null,
            },
        ]);
        let (sql, binds) = builder.to_sql_with_bind();
        assert_eq!(
            sql,
            "UPDATE user SET username = $1,is_active = $2 WHERE user.id = $3 AND user.username LIKE $4 AND ( user.is_active IS $5 OR user.deleted_at IS NOT $6)".to_string()
        );
        let answer = [
            Bind::String("foo".to_string()),
            Bind::Bool(true),
            Bind::Int(1),
            Bind::String("%foo%".to_string()),
            Bind::Bool(false),
            Bind::Null,
        ];
        assert_eq!(binds.len(), answer.len());
        for idx in 0..answer.len() {
            assert_eq!(binds[idx], answer[idx]);
        }
    }
}
