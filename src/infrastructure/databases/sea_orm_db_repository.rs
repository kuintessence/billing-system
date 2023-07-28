use alice_infrastructure::data::db::Database;
use sea_orm::{ConnectionTrait, Statement, TransactionTrait};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct SeaOrmDbRepository {
    pub db: Arc<Database>,
    pub statements: Arc<Mutex<Vec<Statement>>>,
    pub can_drop: AtomicBool,
}

impl SeaOrmDbRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self {
            db,
            statements: Arc::new(Mutex::new(vec![])),
            can_drop: AtomicBool::new(true),
        }
    }

    pub async fn save_changed(&self) -> anyhow::Result<bool> {
        if !self.can_drop.load(Ordering::Relaxed) {
            let mut stmts = self.statements.lock().await;
            let trans = self.db.get_connection().begin().await?;
            for stmt in stmts.iter() {
                if let Err(e) = trans.execute(stmt.clone()).await {
                    trans.rollback().await?;
                    stmts.clear();
                    self.can_drop.store(true, Ordering::Relaxed);
                    anyhow::bail!(e);
                }
            }
            trans.commit().await?;
            self.can_drop.store(true, Ordering::Relaxed);
            stmts.clear();
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl Drop for SeaOrmDbRepository {
    fn drop(&mut self) {
        if !self.can_drop.load(Ordering::Relaxed) {
            log::trace!("{}", self.can_drop.load(Ordering::Relaxed));
            let stmts = self.statements.try_lock().unwrap();
            let sqls = stmts.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("\n");
            log::trace!("Unused sql statements:\n{sqls}")
        }
    }
}
