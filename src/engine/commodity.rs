use tokio_rusqlite::{params, Connection, Result};
use serde::{Serialize, Deserialize};

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
    async fn init(conn: &Connection) -> Result<bool>{
        conn.call(|con|{con.execute("CREATE TABLE IF NOT EXISTS commodities (id INTEGER PRIMARY KEY, name TEXT NOT NULL, symbol TEXT NOT NULL)", [])?; Ok(true)}).await?;
        Ok(true)
    }
    pub async fn add_update(&self) -> Result<bool>{
        let conn = Connection::open("cxc.db").await?;
        Commodity::init(&conn).await?;
        
        if self.exists().await.unwrap() {
            let s = self.clone();
            conn.call(move|conn|{
                let mut stmt = conn.prepare("UPDATE commodities SET name = ?, symbol = ? WHERE id = ?").unwrap();
                stmt.execute(params![s.name, s.symbol, s.id]).unwrap();
                Ok(true)
            }).await?;
        }
        else{
            let s = self.clone();
            conn.call(move|conn|{
                let mut stmt = conn.prepare("INSERT INTO commodities (id, name, symbol) VALUES (?, ?, ?)").unwrap();
                stmt.execute(params![s.id, s.name, s.symbol]).unwrap();
                Ok(true)
            }).await?;

        }
        conn.close().await?;
        Ok(true)
    }
    pub async fn hydrate(id: i32) -> Result<Option<Commodity>>{
        let conn = Connection::open("cxc.db").await.unwrap();
        Commodity::init(&conn).await?;
        let commodity = conn.call(move |conn|{
            let mut stmt = conn.prepare("SELECT id, name, symbol FROM commodities WHERE id = ?").unwrap();
            let mut rows = stmt.query(params![&id]).unwrap();
            let mut c: Option<Commodity> = None;
            while let Some(row) = rows.next().unwrap(){
                c = Some(Commodity::new(row.get(0).unwrap(), row.get(1).unwrap(), row.get(2).unwrap()));
                break;
            }
            Ok(c)
        }).await.unwrap();
        
        conn.close().await.unwrap();
        Ok(commodity)
    }
    pub async fn exists(&self) -> Result<bool>{
        let conn = Connection::open("cxc.db").await.unwrap();
        Commodity::init(&conn).await?;
        let s = self.clone();
        let does_exist = conn.call(move|conn|{
            let mut stmt = conn.prepare("SELECT id FROM commodities WHERE id = ?").unwrap();
            let mut rows = stmt.query(params![s.id]).unwrap();
            let mut de = false;
            while let Some(_row) = rows.next().unwrap(){
                de = true;
                break;
            }
            Ok(de)
        }).await.unwrap();
        conn.close().await?;
        Ok(does_exist)
    }
    pub async fn get_all() -> Result<Vec<Commodity>>{
        let conn = Connection::open("cxc.db").await.unwrap();
        Commodity::init(&conn).await?;
        let cmds = conn.call(|conn|{
            let mut stmt = conn.prepare("SELECT id, name, symbol FROM commodities").unwrap();
            let mut rows = stmt.query([]).unwrap();
            let mut commodities = Vec::new();
            while let Some(row) = rows.next().unwrap(){
                commodities.push(Commodity::new(row.get(0).unwrap(), row.get(1).unwrap(), row.get(2).unwrap()));
            }
            Ok(commodities)
        }).await.unwrap();
        conn.close().await.unwrap();
        Ok(cmds)
    }
}