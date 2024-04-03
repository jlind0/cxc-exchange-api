use tokio_rusqlite::{params, Connection, Result};
use serde::{Serialize, Deserialize};

pub trait HydrateableBase {
    async fn init(conn: &Connection) -> Result<bool>;
    async fn add_update(&self) -> Result<bool>;
    async fn exists(&self) -> Result<bool>;
}
impl HydrateableBase for Commodity{
    async fn init(conn: &Connection) -> Result<bool>{
        conn.call(|con|{con.execute("CREATE TABLE IF NOT EXISTS commodities (id INTEGER PRIMARY KEY, name TEXT NOT NULL, symbol TEXT NOT NULL)", [])?; Ok(true)}).await?;
        Ok(true)
    }
    async fn add_update(&self) -> Result<bool>{
        let conn = Connection::open("cxc.db").await?;
        Commodity::init(&conn).await?;
        
        if self.exists().await? {
            let s = self.clone();
            conn.call(move|conn|{
                let mut stmt = conn.prepare("UPDATE commodities SET name = ?, symbol = ? WHERE id = ?")?;
                stmt.execute(params![s.name, s.symbol, s.id])?;
                Ok(true)
            }).await?;
        }
        else{
            let s = self.clone();
            conn.call(move|conn|{
                let mut stmt = conn.prepare("INSERT INTO commodities (id, name, symbol) VALUES (?, ?, ?)")?;
                stmt.execute(params![s.id, s.name, s.symbol])?;
                Ok(true)
            }).await?;

        }
        conn.close().await?;
        Ok(true)
    }
    async fn exists(&self) -> Result<bool>{
        let conn = Connection::open("cxc.db").await?;
        Commodity::init(&conn).await?;
        let s = self.clone();
        let does_exist = conn.call(move|conn|{
            let mut stmt = conn.prepare("SELECT id FROM commodities WHERE id = ?")?;
            let mut rows = stmt.query(params![s.id])?;
            let mut de = false;
            while let Some(_row) = rows.next()?{
                de = true;
                break;
            }
            Ok(de)
        }).await?;
        conn.close().await?;
        Ok(does_exist)
    }
}
pub trait Hydrateable<T> {
    async fn hydrate(id: i32) -> Result<Option<T>>;
    async fn get_all() -> Result<Vec<T>>;
}
impl Hydrateable<Commodity> for Commodity {
   
    
    async fn hydrate(id: i32) -> Result<Option<Commodity>>{
        let conn = Connection::open("cxc.db").await?;
        Commodity::init(&conn).await?;
        let commodity = conn.call(move |conn|{
            let mut stmt = conn.prepare("SELECT id, name, symbol FROM commodities WHERE id = ?")?;
            let mut rows = stmt.query(params![&id])?;
            let mut c: Option<Commodity> = None;
            while let Some(row) = rows.next()?{
                c = Some(Commodity::new(row.get(0)?, row.get(1)?, row.get(2)?));
                break;
            }
            Ok(c)
        }).await?;
        
        conn.close().await?;
        Ok(commodity)
    }
    
    async fn get_all() -> Result<Vec<Commodity>>{
        let conn = Connection::open("cxc.db").await?;
        Commodity::init(&conn).await?;
        let cmds = conn.call(|conn|{
            let mut stmt = conn.prepare("SELECT id, name, symbol FROM commodities")?;
            let mut rows = stmt.query([])?;
            let mut commodities = Vec::new();
            while let Some(row) = rows.next()?{
                commodities.push(Commodity::new(row.get(0)?, row.get(1)?, row.get(2)?));
            }
            Ok(commodities)
        }).await?;
        conn.close().await?;
        Ok(cmds)
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Commodity{
    pub id: i32,
    pub name: String,
    pub symbol: String
}
impl Commodity{
    pub fn new(id: i32, name: String, symbol: String) -> Commodity{
        Commodity{
            id,
            name,
            symbol
        }
    }
    
}