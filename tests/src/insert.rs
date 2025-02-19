#[cfg(test)]
mod tests {
    use qorm::{insert_item::InsertConfig, Bind, Insert};

    #[test]
    fn insert_query() {
        let mut builder = Insert::new(
            "user",
            Some(InsertConfig {
                placeholder: "$%d".to_string(),
                start: Some(1),
            }),
        );
        builder.values(vec![
            ("username", Bind::String("foo".to_string())),
            ("is_active", Bind::Bool(true)),
        ]);
        builder.values(vec![("age", Bind::Int(12))]);
        let (sql, binds) = builder.to_sql_with_bind();
        assert_eq!(
            sql,
            "INSERT INTO user (username,is_active,age) VALUES ($1,$2,$3)"
        );
        let answer = [
            Bind::String("foo".to_string()),
            Bind::Bool(true),
            Bind::Int(12),
        ];
        assert_eq!(binds.len(), answer.len());
        for idx in 0..binds.len() {
            assert_eq!(binds[idx], answer[idx]);
        }
    }
}
