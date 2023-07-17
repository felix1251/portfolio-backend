pub use sea_orm_migration::prelude::*;

mod m20230717_141821_create_blogs_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20230717_141821_create_blogs_table::Migration)]
    }
}
