//! In order to treat the entity as migrationable it should implement [`Migration`] trait
use anyhow::Result;
use async_trait::async_trait;

use crate::migrator::Env;

#[async_trait]
pub trait Migration: Sync + Send {
    /// Runs a migration.
    async fn up(&self, env: Env) -> Result<()>;

    /// Rollbacks a migration.
    /// Since not every migration could be rollbacked
    /// and it often happens there is a blank implementation
    async fn down(&self, _env: Env) -> Result<()> {
        Ok(())
    }

    /// A status about a migration will be stored in a db collection with the following document id
    /// We can pass an id manually otherwise it will be based on the type name so that uniqueness per project
    /// is guaranteed out of the box
    fn get_id(&self) -> &str {
        std::any::type_name::<Self>()
    }
}
