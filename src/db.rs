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
    pub fn transaction<F>(&self, mut f: F) -> Result<(), crate::error::Error>
    where
        F: FnMut(&mut mysql::Transaction) -> Result<(), crate::error::Error>,
    {
        let mut conn = self.0.get()?;
        let tx_opts = mysql::TxOpts::default();
        tx_opts.set_isolation_level(Some(mysql::IsolationLevel::RepeatableRead));
        let mut tx = conn.start_transaction(tx_opts)?;
        f(&mut tx)?;
        Ok(tx.commit()?)
    }
}
