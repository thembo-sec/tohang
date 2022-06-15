use sqlx::sqlite::SqlitePool;

async fn add_task(pool: &SqlitePool, task: String) -> anyhow::Result<i64>{
    
    let mut conn = pool.acquire().await?;

    let id = sqlx::query!(
        r#"
        INSERT INTO tasks (task)
        VALUES ( ?1 )
        "#,
        task
    )
    .execute(&mut conn)
    .await?
    .last_insert_rowid();

    Ok(id)

}

async fn list_tasks(pool: &SqlitePool) -> anyhow::Result<sqlx::Result<Vec<sqlx::sqlite::SqliteRow>>>{

    let tasks = sqlx::query!(
        r#"
        SELECT id, task, done
        FROM tasks
        ORDER BY id
        "#
    )
    .fetch_all(pool)
    .await?;
    
    Ok(tasks)

}