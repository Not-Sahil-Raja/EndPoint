use sqlx::SqlitePool;

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow, Clone)]
pub struct Server {
    pub id: i64,
    pub name: String,
    pub url: String,
    pub status: Option<String>,
    pub last_checked: Option<String>,
    pub response_time: Option<i64>,
    pub group_id: Option<i64>,
    pub sync_enabled: i64,
}

pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Server>, sqlx::Error> {
    sqlx::query_as::<_, Server>(
        "SELECT id, name, url, status, last_checked, response_time, group_id, sync_enabled
           FROM servers
          ORDER BY name ASC",
    )
    .fetch_all(pool)
    .await
}

pub async fn get_one(pool: &SqlitePool, id: i64) -> Result<Server, sqlx::Error> {
    sqlx::query_as::<_, Server>(
        "SELECT id, name, url, status, last_checked, response_time, group_id, sync_enabled
           FROM servers
          WHERE id = ?",
    )
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn insert(
    pool: &SqlitePool,
    name: &str,
    url: &str,
    group_id: Option<i64>,
) -> Result<Server, sqlx::Error> {
    let id = sqlx::query("INSERT INTO servers (name, url, group_id) VALUES (?, ?, ?)")
        .bind(name)
        .bind(url)
        .bind(group_id)
        .execute(pool)
        .await?
        .last_insert_rowid();

    get_one(pool, id).await
}

pub async fn update(
    pool: &SqlitePool,
    id: i64,
    name: &str,
    url: &str,
    group_id: Option<i64>,
) -> Result<Server, sqlx::Error> {
    sqlx::query("UPDATE servers SET name = ?, url = ?, group_id = ? WHERE id = ?")
        .bind(name)
        .bind(url)
        .bind(group_id)
        .bind(id)
        .execute(pool)
        .await?;

    get_one(pool, id).await
}

pub async fn delete(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM servers WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn get_servers_by_groups_map(
    pool: &SqlitePool,
) -> Result<std::collections::HashMap<Option<i64>, Vec<Server>>, sqlx::Error> {
    let servers = sqlx::query_as::<_, Server>(
        "SELECT id, name, url, status, last_checked, response_time, group_id, sync_enabled
           FROM servers
          ORDER BY group_id, name ASC",
    )
    .fetch_all(pool)
    .await?;

    let mut groups_map: std::collections::HashMap<Option<i64>, Vec<Server>> =
        std::collections::HashMap::new();

    for server in servers {
        groups_map
            .entry(server.group_id)
            .or_insert_with(Vec::new)
            .push(server);
    }

    Ok(groups_map)
}

pub async fn update_status(
    pool: &SqlitePool,
    id: i64,
    status: &str,
    response_time: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE servers
            SET status        = ?,
                response_time = ?,
                last_checked  = datetime('now')
          WHERE id = ?",
    )
    .bind(status)
    .bind(response_time)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn toggle_sync_enabled(pool: &SqlitePool, id: i64) -> Result<Server, sqlx::Error> {
    // First get current server to check current sync_enabled value
    let server = get_one(pool, id).await?;
    let new_sync_enabled = if server.sync_enabled == 1 { 0 } else { 1 };

    sqlx::query("UPDATE servers SET sync_enabled = ? WHERE id = ?")
        .bind(new_sync_enabled)
        .bind(id)
        .execute(pool)
        .await?;

    get_one(pool, id).await
}

pub async fn get_sync_enabled_servers(pool: &SqlitePool) -> Result<Vec<Server>, sqlx::Error> {
    sqlx::query_as::<_, Server>(
        "SELECT id, name, url, status, last_checked, response_time, group_id, sync_enabled
           FROM servers
          WHERE sync_enabled = 1
          ORDER BY name ASC",
    )
    .fetch_all(pool)
    .await
}

pub async fn toggle_group_sync_enabled(
    pool: &SqlitePool,
    group_id: Option<i64>,
    enabled: bool,
) -> Result<Vec<Server>, sqlx::Error> {
    let sync_value = if enabled { 1 } else { 0 };

    let query = match group_id {
        Some(id) => {
            sqlx::query_as::<_, Server>(
                "UPDATE servers 
                 SET sync_enabled = ? 
                 WHERE group_id = ? 
                 RETURNING id, name, url, status, last_checked, response_time, group_id, sync_enabled"
            )
            .bind(sync_value)
            .bind(id)
        }
        None => {
            sqlx::query_as::<_, Server>(
                "UPDATE servers 
                 SET sync_enabled = ? 
                 WHERE group_id IS NULL 
                 RETURNING id, name, url, status, last_checked, response_time, group_id, sync_enabled"
            )
            .bind(sync_value)
        }
    };

    query.fetch_all(pool).await
}
