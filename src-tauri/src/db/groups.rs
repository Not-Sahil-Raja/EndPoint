use sqlx::SqlitePool;

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow, Clone)]
pub struct Group {
    pub id: i64,
    pub name: String,
}

pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Group>, sqlx::Error> {
    sqlx::query_as::<_, Group>(
        "SELECT id, name 
           FROM groups
          ORDER BY name ASC",
    )
    .fetch_all(pool)
    .await
}

pub async fn create(pool: &SqlitePool, name: &str) -> Result<Group, sqlx::Error> {
    sqlx::query_as::<_, Group>("INSERT INTO groups (name) VALUES (?) RETURNING id, name")
        .bind(name)
        .fetch_one(pool)
        .await
}

pub async fn update(pool: &SqlitePool, id: i64, name: &str) -> Result<Group, sqlx::Error> {
    sqlx::query_as::<_, Group>(
        "UPDATE groups 
            SET name = ? 
          WHERE id = ? 
          RETURNING id, name",
    )
    .bind(name)
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn delete(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
    // First, update all servers in this group to have group_id = NULL (ungrouped)
    sqlx::query(
        "UPDATE servers 
            SET group_id = NULL 
          WHERE group_id = ?",
    )
    .bind(id)
    .execute(pool)
    .await?;

    // Then delete the group
    sqlx::query("DELETE FROM groups WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}
