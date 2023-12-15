use std::sync::atomic::Ordering;

use alice_architecture::repository::{DBRepository, MutableRepository, ReadOnlyRepository};
use database_model::{node_instance, node_instance_billing};
use sea_orm::{prelude::Uuid, sea_query::OnConflict, ColumnTrait, QueryFilter};
use sea_orm::{ConnectionTrait, EntityTrait, QueryTrait, Set};

use crate::domain::model::{DbNodeInstanceBilling, NodeInstanceBilling};
use crate::domain::repository::NodeInstanceBillingRepo;
use crate::infrastructure::database::OrmRepo;

use super::RESCALE_TO;

#[async_trait::async_trait]
impl ReadOnlyRepository<NodeInstanceBilling> for OrmRepo {
    async fn get_by_id(&self, uuid: Uuid) -> anyhow::Result<NodeInstanceBilling> {
        let mut model = database_model::node_instance_billing::Entity::find_by_id(uuid)
            .one(self.db.get_connection())
            .await?
            .ok_or(anyhow::anyhow!("there is no such row with key {uuid}"))?;
        model.price.rescale(RESCALE_TO);
        Ok(model.into())
    }
    async fn get_all(&self) -> anyhow::Result<Vec<NodeInstanceBilling>> {
        unimplemented!()
    }
}

#[async_trait::async_trait]
impl MutableRepository<NodeInstanceBilling> for OrmRepo {
    async fn update(&self, entity: DbNodeInstanceBilling) -> anyhow::Result<()> {
        let mut stmts = self.statements.lock().await;
        let active_model = node_instance_billing::ActiveModel {
            id: entity.id.into_active_value(),
            node_instance_id: entity.node_instance_id.into_active_value(),
            flow_instance_id: entity.flow_instance_id.into_active_value(),
            cpu: entity.cpu.into_active_value(),
            memory: entity.memory.into_active_value(),
            storage: entity.storage.into_active_value(),
            cpu_time: entity.cpu_time.into_active_value(),
            wall_time: entity.wall_time.into_active_value(),
            price: entity.price.into_active_value(),
            formula: entity.formula.into_active_value(),
            ..Default::default()
        };
        let stmt = database_model::node_instance_billing::Entity::update(active_model)
            .build(self.db.get_connection().get_database_backend());
        stmts.push(stmt);
        self.can_drop.store(false, Ordering::Relaxed);
        Ok(())
    }
    async fn insert(&self, entity: &NodeInstanceBilling) -> anyhow::Result<Uuid> {
        let active_model = node_instance_billing::ActiveModel {
            id: Set(entity.id),
            node_instance_id: Set(entity.node_instance_id),
            flow_instance_id: Set(entity.flow_instance_id),
            cpu: Set(entity.cpu),
            memory: Set(entity.memory),
            storage: Set(entity.storage),
            cpu_time: Set(entity.cpu_time),
            wall_time: Set(entity.wall_time),
            price: Set(entity.price),
            formula: Set(entity.formula.to_owned()),
            ..Default::default()
        };
        database_model::node_instance_billing::Entity::insert(active_model)
            .on_conflict(
                OnConflict::column(database_model::node_instance::Column::Id)
                    .do_nothing()
                    .to_owned(),
            )
            .exec(self.db.get_connection())
            .await?;
        Ok(entity.id)
    }
    async fn save_changed(&self) -> anyhow::Result<bool> {
        self.save_changed().await
    }
}

impl DBRepository<NodeInstanceBilling> for OrmRepo {}

#[async_trait::async_trait]
impl NodeInstanceBillingRepo for OrmRepo {
    async fn get_all_by_flow_instance_id(
        &self,
        id: Uuid,
    ) -> anyhow::Result<Vec<NodeInstanceBilling>> {
        let res = database_model::node_instance_billing::Entity::find()
            .filter(node_instance::Column::FlowInstanceId.eq(id))
            .all(self.db.get_connection())
            .await?;

        let mut r = vec![];
        for mut el in res.into_iter() {
            el.price.rescale(RESCALE_TO);
            r.push(el.try_into()?);
        }
        Ok(r)
    }
}
