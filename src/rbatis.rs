use crate::ast::lang::py::Py;
use crate::error::RbatisError;
use std::collections::HashMap;
use crate::ast::node::node_type::NodeType;
use crate::engine::runtime::RbatisEngine;
use crate::ast::lang::xml::Xml;
use crate::ast::node::insert_node::InsertNode;
use crate::ast::node::delete_node::DeleteNode;
use crate::ast::node::update_node::UpdateNode;
use crate::ast::node::select_node::SelectNode;
use rbatis_core::mysql::MySqlPool;
use rbatis_core::executor::Executor;
use serde::de::DeserializeOwned;
use rbatis_core::cursor::Cursor;

/// rbatis engine
pub struct Rbatis<'r> {
    pool: Option<MySqlPool>,
    engine: RbatisEngine,
    /// map<mapper_name,map<method_name,NodeType>>
    mapper_node_map: HashMap<&'r str, HashMap<String, NodeType>>,
}


impl<'r> Rbatis<'r> {
    pub async fn new(url: &str) -> Rbatis<'r> {
        let mut pool = Option::None;
        if url.ne("") {
            pool = Some(MySqlPool::new(url).await.unwrap());
        }
        return Rbatis { pool, mapper_node_map: HashMap::new(), engine: RbatisEngine::new() };
    }

    pub fn load_xml(&mut self, mapper_name: &'r str, data: &str) -> Result<(), RbatisError> {
        let xml = Xml::parser(data);
        self.mapper_node_map.insert(mapper_name, xml);
        return Ok(());
    }

    /// fetch result
    pub async fn fetch<T>(&self, sql: &str) -> Result<T, rbatis_core::Error>
        where T: DeserializeOwned {
        let mut conn = self.pool.as_ref().unwrap().acquire().await.unwrap();
        let mut c = conn.fetch(sql);
        return c.decode().await;
    }

    /// exec sql
    pub async fn exec(&self, sql: &str) -> Result<u64, rbatis_core::Error> {
        let mut conn = self.pool.as_ref().unwrap().acquire().await.unwrap();
        return conn.execute(sql).await;
    }
}