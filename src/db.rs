use crate::result::Result;
use r2d2::PooledConnection;
use r2d2_mysql::MysqlConnectionManager;

#[derive(Clone)]
pub struct MySQLClient(r2d2::Pool<r2d2_mysql::MysqlConnectionManager>);

impl MySQLClient {
    pub fn new(url: &str) -> MySQLClient {
        let opts = mysql::Opts::from_url(&url).unwrap();
        let builder = mysql::OptsBuilder::from_opts(opts);
        let manager = r2d2_mysql::MysqlConnectionManager::new(builder);
        let pool = r2d2::Pool::new(manager).unwrap();
        MySQLClient(pool)
    }
    pub fn transaction<F, U>(&self, mut f: F) -> Result<U>
    where
        F: FnMut(&mut mysql::Transaction) -> Result<U>,
    {
        let mut conn = self.0.get()?;
        let tx_opts = mysql::TxOpts::default();
        tx_opts.set_isolation_level(Some(mysql::IsolationLevel::RepeatableRead));
        let mut tx = conn.start_transaction(tx_opts)?;
        let r = f(&mut tx)?;
        tx.commit()?;
        Ok(r)
    }

    pub fn get(&self) -> Result<PooledConnection<MysqlConnectionManager>> {
        Ok(self.0.get()?)
    }
}
