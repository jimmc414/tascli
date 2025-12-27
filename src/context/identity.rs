use rusqlite::Connection;

/// Runtime context containing the current user and namespace.
/// This is resolved at startup and passed through to all command handlers.
#[derive(Debug, Clone)]
pub struct Context {
    pub current_user_id: i64,
    pub current_user_name: String,
    pub current_namespace_id: i64,
    pub current_namespace_name: String,
}

impl Context {
    /// Resolve the current identity context.
    /// Priority: --as flag > CTM_USER env > config > system $USER
    ///
    /// This will query the database to get the user and namespace IDs.
    /// If the user doesn't exist, it returns an error.
    pub fn resolve(
        conn: &Connection,
        as_user: Option<&str>,
        namespace: Option<&str>,
    ) -> Result<Self, String> {
        // Resolve username with priority: --as flag > CTM_USER env > system $USER
        let username = as_user
            .map(|s| s.to_string())
            .or_else(|| std::env::var("CTM_USER").ok())
            .or_else(|| std::env::var("USER").ok())
            .or_else(|| std::env::var("USERNAME").ok()) // Windows fallback
            .unwrap_or_else(|| "default".to_string());

        // Look up user in database
        let user_result: Result<(i64, String), rusqlite::Error> = conn.query_row(
            "SELECT id, name FROM users WHERE name = ?1",
            [&username],
            |row| Ok((row.get(0)?, row.get(1)?)),
        );

        let (user_id, user_name) = user_result.map_err(|_| {
            format!(
                "User '{}' not found. Run a command first to auto-create, or use 'ctm user create {}'",
                username, username
            )
        })?;

        // Resolve namespace with priority: --ns flag > CTM_NAMESPACE env > "default"
        let ns_name = namespace
            .map(|s| s.to_string())
            .or_else(|| std::env::var("CTM_NAMESPACE").ok())
            .unwrap_or_else(|| "default".to_string());

        // Look up namespace and verify user has access
        let ns_result: Result<i64, rusqlite::Error> = conn.query_row(
            "SELECT n.id FROM namespaces n
             INNER JOIN user_namespaces un ON n.id = un.namespace_id
             WHERE n.name = ?1 AND un.user_id = ?2",
            rusqlite::params![&ns_name, user_id],
            |row| row.get(0),
        );

        let namespace_id = ns_result.map_err(|_| {
            format!(
                "Namespace '{}' not found or user '{}' does not have access",
                ns_name, user_name
            )
        })?;

        Ok(Context {
            current_user_id: user_id,
            current_user_name: user_name,
            current_namespace_id: namespace_id,
            current_namespace_name: ns_name,
        })
    }

    /// Get the default context (for backwards compatibility or tests).
    /// This assumes the auto-setup has created the default user and namespace.
    pub fn default_from_db(conn: &Connection) -> Result<Self, String> {
        Self::resolve(conn, None, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::get_test_conn;

    #[test]
    fn test_context_resolve_default() {
        let (conn, _temp_file) = get_test_conn();

        // After init_table runs, there should be a default user and namespace
        let context = Context::default_from_db(&conn);
        assert!(context.is_ok(), "Should resolve default context: {:?}", context.err());

        let ctx = context.unwrap();
        assert_eq!(ctx.current_namespace_name, "default");
        assert!(ctx.current_user_id > 0);
        assert!(ctx.current_namespace_id > 0);
    }

    #[test]
    fn test_context_resolve_nonexistent_user() {
        let (conn, _temp_file) = get_test_conn();

        let result = Context::resolve(&conn, Some("nonexistent_user"), None);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[test]
    fn test_context_resolve_nonexistent_namespace() {
        let (conn, _temp_file) = get_test_conn();

        // Get the default user name first
        let ctx = Context::default_from_db(&conn).unwrap();

        // Try to access a non-existent namespace
        let result = Context::resolve(&conn, Some(&ctx.current_user_name), Some("nonexistent_ns"));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }
}
