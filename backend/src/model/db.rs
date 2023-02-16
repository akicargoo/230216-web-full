use std::{env, time::Duration, fs, path::PathBuf};

use sqlx::{Postgres, Pool, postgres::PgPoolOptions};

pub type Db = Pool<Postgres>;


// struct PgConfig {
//     host: String,
//     db: String,
//     user: String,
//     pwd: String,
//     max_con: u32,
// }

// impl PgConfig {
//     fn new() -> Self {
//         PgConfig  {
//             host: env::var("PG_HOST").unwrap(),
//             db: env::var("PG_DB").unwrap(),
//             user: env::var("PG_USER").unwrap(),
//             pwd: env::var("PG_PWD").unwrap(),
//             max_con: env::var("PG_MAX_CON").unwrap().parse::<u32>().unwrap(),
//         }
//     }

//     fn config_format(&self) -> String {
//         format!("postgres://{}:{}@{}/{}", self.user, self.pwd, self.host, self.db)
//     }

//     fn max_con(&self) -> u32 {
//         self.max_con
//     }
// }


// pub async fn init_db() -> Result<Db, sqlx::Error> {
//     new_db_pool(PgConfig::new()).await
// }

// async fn new_db_pool(pg: PgConfig) -> Result<Db, sqlx::Error> {
//     let con_string = pg.config_format();
//     PgPoolOptions::new()
//         .max_connections(pg.max_con())
//         .connect(&con_string)
//         .await
// }

const PG_HOST: &str = "192.168.111.145";
const PG_ROOT_DB: &str = "postgres";
const PG_ROOT_USER: &str = "postgres";
const PG_ROOT_PWD: &str = "postgres";

//app_db 
const PG_APP_DB: &str = "app_db";
const PG_APP_USER: &str = "app_user";
const PG_APP_PWD: &str = "111111";
const PG_APP_MAX_CON: u32 = 5;

//sql files
const SQL_DIR: &str = "sql/";
const SQL_RECREATE: &str = "sql/00-recreate-db.sql";


pub async fn init_db() -> Result<Db, sqlx::Error> {

    // -- Create the db with PG_ROOT (dev only)
    // -- db connection init 과 같이 시작시 한번만 작업을 돌릴것이기 때문에  
    //    스코프로 1번만 실행되게 라이프타임을 건다.
    {
        let root_db = new_db_pool(PG_HOST, PG_ROOT_DB, PG_ROOT_USER, PG_ROOT_PWD, 1).await?;
        pexec(&root_db, SQL_RECREATE).await?;
    }

    // --RUN the app sql files 
    let app_db = new_db_pool(PG_HOST, PG_APP_DB, PG_APP_USER, PG_APP_PWD, 1).await?;
    let mut paths: Vec<PathBuf> = fs::read_dir(SQL_DIR)?
        .into_iter()
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();
    paths.sort();

    //excute each sql file
    for path in paths {
        if let Some(path) = path.to_str() {
            // only .sql and not the recreate file
            if path.ends_with(".sql") && path != SQL_RECREATE {
                pexec(&app_db, path).await?;
            }
        }
    }

    // returning the app db
    new_db_pool(PG_HOST, PG_APP_DB, PG_APP_USER, PG_APP_PWD, PG_APP_MAX_CON).await
}

async fn pexec(db: &Db, file: &str) -> Result<(), sqlx::Error> {
    // Read the file
    let content = fs::read_to_string(file).map_err(|ex| {
        println!("ERROR reading {} (cause: {:?})", file, ex);
        ex
    })?;

    let sqls: Vec<&str> = content.split(";").collect();

    for sql in sqls {
        match sqlx::query(sql).execute(db).await {
            Ok(_) => (),
            Err(ex) => {
                println!("WARNING - pexec - Sql file '{}' FAILED cause: {}", file, ex);
            }
        }
    }

    Ok(())
}

async fn new_db_pool(host: &str, db: &str, user: &str, pwd: &str, max_con: u32) -> Result<Db, sqlx::Error> {
    let con_string = format!("postgres://{}:{}@{}/{}", user, pwd, host, db);
    PgPoolOptions::new()
        .max_connections(max_con)
        .connect(&con_string)
        .await
}


#[cfg(test)]
#[path = "../_tests/model_db.rs"]
mod tests;



