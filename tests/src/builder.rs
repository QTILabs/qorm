#[cfg(test)]
mod tests {
    use qorm::builder::Builder;

    #[test]
    fn init_builder() {
        let builder = Builder::new("user", None);
        assert_eq!(builder.to_sql(), "SELECT * FROM user user");
    }

    #[test]
    fn init_builder_with_alias() {
        let builder = Builder::new("user", Some("u"));
        assert_eq!(builder.to_sql(), "SELECT * FROM user u");
    }

    #[test]
    fn single_select_query() {
        let mut builder = Builder::new("user", None);
        builder.select_raw("user.id");
        assert_eq!(builder.to_sql(), "SELECT user.id FROM user user");
    }

    #[test]
    fn multiple_select_query() {
        let mut builder = Builder::new("user", None);
        builder.select_raw("user.id").select_raw("user.name");
        assert_eq!(builder.to_sql(), "SELECT user.id, user.name FROM user user");
    }

    #[test]
    fn single_where_query() {
        let mut builder = Builder::new("user", None);
        builder.where_raw("user.id = 1");
        assert_eq!(
            builder.to_sql(),
            "SELECT * FROM user user WHERE user.id = 1"
        );
    }

    #[test]
    fn multiple_where_query() {
        let mut builder = Builder::new("user", None);
        builder.where_raw("user.id = 1");
        builder.where_raw(r#"user.username = "Foo""#);
        assert_eq!(
            builder.to_sql(),
            r#"SELECT * FROM user user WHERE user.id = 1 AND user.username = "Foo""#
        );
    }

    #[test]
    fn single_where_or_query() {
        let mut builder = Builder::new("user", None);
        builder.where_or_raw(vec!["user.id = 1", "user.is_active = true"]);
        assert_eq!(
            builder.to_sql(),
            r#"SELECT * FROM user user WHERE ( user.id = 1 OR user.is_active = true)"#
        );
    }

    #[test]
    fn multiple_where_or_query() {
        let mut builder = Builder::new("user", None);
        builder.where_raw(r#"user.username = "Foo""#);
        builder.where_or_raw(vec!["user.id = 1", "user.is_active = true"]);
        assert_eq!(
            builder.to_sql(),
            r#"SELECT * FROM user user WHERE user.username = "Foo" AND ( user.id = 1 OR user.is_active = true)"#
        );
    }

    #[test]
    fn single_order_by_query() {
        let mut builder = Builder::new("user", None);
        builder.order_by_raw(vec!["user.id ASC"]);
        assert_eq!(
            builder.to_sql(),
            r#"SELECT * FROM user user ORDER BY user.id ASC"#
        );
    }

    #[test]
    fn multiple_order_by_query() {
        let mut builder = Builder::new("user", None);
        builder.order_by_raw(vec!["user.id ASC"]);
        builder.order_by_raw(vec!["user.username DESC", "user.profile ASC"]);
        assert_eq!(
            builder.to_sql(),
            r#"SELECT * FROM user user ORDER BY user.id ASC, user.username DESC, user.profile ASC"#
        );
    }

    #[test]
    fn full_query() {
        let mut builder = Builder::new("user", None);
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
}
