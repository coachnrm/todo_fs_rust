use dioxus::prelude::*;

use crate::ToDo;

#[cfg(feature = "server")]
use super::{db::get_db, model::ToDoSql};

#[server]
pub async fn get_todo_list () -> Result<Vec<ToDo>, ServerFnError> {
  let db = get_db().await;
  
  let rows : Vec<ToDoSql> = sqlx::query_as("SELECT * FROM todos").fetch_all(db).await.unwrap();

  let mut v = vec![];

  for row in rows {
    let todo = ToDo {
      id: row.id,
      content: row.content,
      completed: row.completed
    };
    v.push(todo);
  }

  Ok(v)
}

#[server]
pub async fn get_single_todo(id: i64) -> Result<ToDo, ServerFnError> {
  let db = get_db().await;

  let rows : Vec<ToDoSql> = sqlx::query_as("SELECT * FROM todos WHERE id = ?1").bind(&id).fetch_all(db).await.unwrap();

  if rows.len() == 0 {
    let msg = format!("Todo id : {} Not Found.", id);
    Err(ServerFnError::new(msg))
  } else {
    let todo = ToDo {
      id: rows[0].id,
      content: rows[0].content.clone(),
      completed: rows[0].completed
    };
    Ok(todo)
  }
}

#[server]
pub async fn add_new_todo(content: String, completed: bool) -> Result<i64, ServerFnError> {
  let db = get_db().await;

  let result = sqlx::query("INSERT INTO todos (content, completed) VALUES(?1, ?2)").bind(&content).bind(&completed).execute(db).await.unwrap();

  Ok(result.last_insert_rowid())
}

#[server]
pub async fn remove_todo(id: i64) -> Result<(), ServerFnError> {
  let db = get_db().await;

  let rows : Vec<ToDoSql> = sqlx::query_as("SELECT * FROM todos WHERE id = ?1").bind(&id).fetch_all(db).await.unwrap();

  if rows.len() == 0 {
    let msg = format!("Todo id : {} Not Found", id);
    Err(ServerFnError::new(msg))
  }else {
      sqlx::query("DELETE FROM todos WHERE id = ?1").bind(&id).execute(db).await.unwrap();
      Ok(())
  }
}
#[server]
pub async fn update_todo(id: i64, completed: bool) -> Result<(), ServerFnError> {
  let db = get_db().await;

  let rows : Vec<ToDoSql> = sqlx::query_as("SELECT * FROM todos WHERE id = ?1").bind(&id).fetch_all(db).await.unwrap();

  if rows.len() == 0 {
    let msg = format!("Todo Id : {} Not Found.", id);
    Err(ServerFnError::new(msg))
  } else {
      sqlx::query("UPDATE todos SET completed = ?1 WHERE id = ?2").bind(&!completed).bind(&id).execute(db).await.unwrap();
      Ok(())
  }
}