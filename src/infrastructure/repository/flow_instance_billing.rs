use std::sync::atomic::Ordering;

use alice_architecture::repository::{DBRepository, MutableRepository, ReadOnlyRepository};
use database_model::flow_instance_billing;
use sea_orm::{prelude::Uuid, sea_query::OnConflict, ColumnTrait, QueryFilter};
use sea_orm::{ConnectionTrait, EntityTrait, QueryTrait, Set};

use crate::domain::model::{DbFlowInstanceBilling, FlowInstanceBilling};
use crate::domain::repository::FlowInstanceBillingRepo;
use crate::infrastructure::database::OrmRepo;

use super::RESCALE_TO;

#[async_trait::async_trait]
impl ReadOnlyRepository<FlowInstanceBilling> for OrmRepo {
    async fn get_by_id(&self, uuid: Uuid) -> anyhow::Result<FlowInstanceBilling> {
        let mut entity = database_model::prelude::FlowInstanceBilling::find_by_id(uuid)
            .one(self.db.get_connection())
            .await?
            .ok_or(anyhow::anyhow!("there is no such row with key {uuid}"))?;
        entity.total_price.rescale(RESCALE_TO);
        Ok(entity.into())
    }
    async fn get_all(&self) -> anyhow::Result<Vec<FlowInstanceBilling>> {
        unimplemented!()
    }
}

#[async_trait::async_trait]
impl MutableRepository<FlowInstanceBilling> for OrmRepo {
    async fn update(&self, entity: DbFlowInstanceBilling) -> anyhow::Result<()> {
        let mut stmts = self.statements.lock().await;
        let active_model = flow_instance_billing::ActiveModel {
            id: entity.id.into_active_value(),
            flow_instance_id: entity.flow_instance_id.into_active_value(),
            cpu: entity.cpu.into_active_value(),
            memory: entity.memory.into_active_value(),
            storage: entity.storage.into_active_value(),
            wall_time: entity.wall_time.into_active_value(),
            total_price: entity.total_price.into_active_value(),
            user_id: entity.user_id.into_active_value(),
            ..Default::default()
        };
        let stmt = database_model::prelude::FlowInstanceBilling::update(active_model)
            .build(self.db.get_connection().get_database_backend());
        stmts.push(stmt);
        self.can_drop.store(false, Ordering::Relaxed);
        Ok(())
    }
    async fn insert(&self, entity: &FlowInstanceBilling) -> anyhow::Result<Uuid> {
        let active_model = flow_instance_billing::ActiveModel {
            id: Set(entity.id),
            flow_instance_id: Set(entity.flow_instance_id),
            cpu: Set(entity.cpu),
            memory: Set(entity.memory),
            storage: Set(entity.storage),
            wall_time: Set(entity.wall_time),
            total_price: Set(entity.total_price),
            user_id: Set(entity.user_id),
            ..Default::default()
        };
        database_model::prelude::FlowInstanceBilling::insert(active_model)
            .on_conflict(
                OnConflict::column(flow_instance_billing::Column::FlowInstanceId)
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

impl DBRepository<FlowInstanceBilling> for OrmRepo {}

#[async_trait::async_trait]
impl FlowInstanceBillingRepo for OrmRepo {
    async fn get_by_flow_instance_id(&self, id: Uuid) -> anyhow::Result<FlowInstanceBilling> {
        let mut model = database_model::prelude::FlowInstanceBilling::find()
            .filter(flow_instance_billing::Column::FlowInstanceId.eq(id))
            .one(self.db.get_connection())
            .await?
            .ok_or(anyhow::anyhow!("No such Flow Instence"))?;
        model.total_price.rescale(RESCALE_TO);
        Ok(model.into())
    }
    async fn insert_or_update(&self, entity: FlowInstanceBilling) -> anyhow::Result<()> {
        let mut stmts = self.statements.lock().await;
        let active_model = flow_instance_billing::ActiveModel {
            id: Set(entity.id),
            flow_instance_id: Set(entity.flow_instance_id),
            cpu: Set(entity.cpu),
            memory: Set(entity.memory),
            storage: Set(entity.storage),
            wall_time: Set(entity.wall_time),
            total_price: Set(entity.total_price),
            user_id: Set(entity.user_id),
            ..Default::default()
        };
        let stmt = database_model::prelude::FlowInstanceBilling::insert(active_model)
            .on_conflict(
                OnConflict::column(flow_instance_billing::Column::Id)
                    .update_columns([
                        flow_instance_billing::Column::Cpu,
                        flow_instance_billing::Column::Memory,
                        flow_instance_billing::Column::Storage,
                        flow_instance_billing::Column::TotalPrice,
                        flow_instance_billing::Column::WallTime,
                    ])
                    .to_owned(),
            )
            .build(self.db.get_connection().get_database_backend());
        stmts.push(stmt);
        self.can_drop.store(false, Ordering::Relaxed);
        Ok(())
    }
}
