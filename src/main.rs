use colored::Colorize;
use sqlx::PgPool;
use std::{
    io::BufRead,
    path::{Path, PathBuf},
    process::exit,
};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
use clap::{Parser, Subcommand};
use tabled::Tabled;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get a list of codes by title
    Get {
        /// Title to load keys from
        #[arg(short, long)]
        title: String,
        /// How many keys to load
        #[arg(short, long)]
        count: i64,
    },
    /// Load codes from a text file
    Load {
        /// File path to load
        #[arg(short, long)]
        file: std::path::PathBuf,
        /// Title to save keys in this file with to the database
        #[arg(short, long)]
        title: String,
    },
    /// List codes in the database
    List,
    /// Perform database migrations. You usually need to only do this once.
    Migrate,
}

async fn create_key_export(keys: Vec<String>, collection_name: &str) -> Result<PathBuf> {
    // If the exports folder doesn't exist, create it
    let export_folder = Path::new("./exports");
    if !Path::new("./exports").exists() {
        std::fs::create_dir("./exports")?;
    }
    let file_name = format!(
        "export_{}_{}x.txt",
        collection_name.replace(' ', "_"),
        keys.len()
    );
    let file_path = export_folder.join(file_name);
    std::fs::write(&file_path, keys.join("\n"))?;
    warn(&format!("Exported saved to {:?}", file_path));
    Ok(file_path)
}

async fn get_codes(database: &PgPool, collection_name: &str, count: i64) -> Result<()> {
    let mut tx = database.begin().await?;
    // Check collection exists
    let collection = sqlx::query!(
        "select * from collections where title = $1 limit 1",
        collection_name
    )
    .fetch_optional(&mut *tx)
    .await?;
    if collection.is_none() {
        exit_with_message(&format!(
            "Unable to find title {} in the database.",
            collection_name
        ));
    }
    let collection = collection.unwrap();
    let rows = sqlx::query_scalar!("UPDATE codes SET is_used = true WHERE code IN (SELECT code FROM codes where collection_id = $1 AND is_used = false LIMIT $2) returning code", collection.id, count).fetch_all(&mut *tx).await?;
    println!("-----------------------------");
    for row in rows.iter() {
        println!("{}", row);
    }
    println!("-----------------------------");

    create_key_export(rows, collection_name).await?;

    tx.commit().await?;

    Ok(())
}

async fn load_codes(database: &PgPool, path: &PathBuf, collection_name: &str) -> Result<()> {
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let mut tx = database.begin().await?;

    // Check if collection exists
    let found_coll = sqlx::query!(
        "select * from collections where title = $1",
        collection_name
    )
    .fetch_optional(&mut *tx)
    .await?;

    let collection_id = match found_coll {
        Some(c) => {
            println!("Found existing title");
            c.id
        }
        None => {
            // Create collection
            println!("New title, creating collection for it.");
            let row = sqlx::query!(
                "INSERT into collections (title) VALUES ($1) returning id",
                collection_name
            )
            .fetch_one(&mut *tx)
            .await?;
            row.id
        }
    };

    let q = "INSERT into codes (code, is_used, collection_id) VALUES ($1, false, $2)";
    let mut count = 0;
    for line in reader.lines() {
        sqlx::query(q)
            .bind(line.unwrap().trim())
            .bind(collection_id)
            .execute(&mut *tx)
            .await?;
        count += 1;
    }
    tx.commit().await?;
    println!("Loaded {} codes into collection {}", count, collection_name);

    Ok(())
}

#[derive(Tabled)]
struct CategoryListing {
    title: String,
    total_keys: i64,
    used_keys: i64,
    unused_keys: i64,
}

async fn list_codes(database: &PgPool) -> Result<()> {
    let mut tx = database.begin().await?;
    // Count
    let availability = sqlx::query!(
        r#"
    SELECT 
    title, 
    COUNT(codes.collection_id) AS total_keys,
    SUM(CASE WHEN codes.is_used = true THEN 1 ELSE 0 END) AS used_keys,
    SUM(CASE WHEN codes.is_used = false THEN 1 ELSE 0 END) AS unused_keys
FROM 
    codes 
LEFT JOIN 
    collections ON codes.collection_id = collections.id 
GROUP BY 
    collection_id, title 
ORDER BY 
    total_keys;
    "#
    )
    .map(|g| CategoryListing {
        title: g.title,
        total_keys: g.total_keys.unwrap_or(0),
        unused_keys: g.unused_keys.unwrap_or(0),
        used_keys: g.used_keys.unwrap_or(0),
    })
    .fetch_all(&mut *tx)
    .await?;

    println!("{}", tabled::Table::new(availability));
    tx.commit().await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let db_url = match std::env::var("DATABASE_URL") {
        Ok(s) => {
            if s.is_empty() {
                exit_with_message("Missing DATABASE_URL environment variable.");
            }
            s
        }
        Err(_) => exit_with_message("Missing DATABASE_URL environment variable."),
    };

    let conn = PgPool::connect(&db_url).await.unwrap();

    let cli = Cli::parse();

    match cli.command {
        Commands::Get { count, title } => get_codes(&conn, &title, count).await?,
        Commands::Load { file, title } => load_codes(&conn, &file, &title).await?,
        Commands::List => list_codes(&conn).await?,
        Commands::Migrate => {
            let migration_status = sqlx::migrate!().run(&conn).await;
            match migration_status {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("Migration error: {err}");
                    exit(1);
                }
            }
        }
    }

    Ok(())
}

fn exit_with_message(message: &str) -> ! {
    eprintln!("{}", message.red());
    exit(1);
}

fn warn(message: &str) {
    println!("{}", message.yellow());
}
