#[cfg(feature = "server")]
#[derive(sqlx::FromRow)]
pub struct ToDoSql {
    pub id : i64,
    pub content: String,
    pub completed: bool
}