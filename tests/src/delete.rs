#[cfg(test)]
mod tests {
    use qorm::{delete_item::DeleteConfig, where_item::Or, Bind, Delete};

    #[test]
    fn delete_query() {
        let mut builder = Delete::new("user", None);
        assert_eq!(builder.to_sql(), "DELETE FROM user".to_string());
    }

    #[test]
    fn delete_query_where() {
        let mut builder = Delete::new(
            "user",
            Some(DeleteConfig {
                placeholder: "$%d".to_string(),
                start: Some(1),
            }),
        );
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
            "DELETE FROM user WHERE user.id = $1 AND user.username LIKE $2 AND ( user.is_active IS $3 OR user.deleted_at IS NOT $4)".to_string()
        );
        let answer = [
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
