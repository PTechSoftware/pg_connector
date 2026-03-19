use std::ops::{Deref};
use crate::operation::ClusterDefine;
use crate::errors::ConnectorError;
use crate::connector::PgConnector;
use bb8::PooledConnection;
use bb8_postgres::PostgresConnectionManager;
use bb8_postgres::tokio_postgres::{
    types::{ToSql, Type},
    Row, Statement,
    Socket,
    tls::{MakeTlsConnect, TlsConnect},
    Transaction,
};

pub trait SqlExpose {
    async fn execute(&self, cluster: ClusterDefine, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<u64, ConnectorError>;
    async fn query(&self, cluster: ClusterDefine, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, ConnectorError>;
    async fn query_one(&self, cluster: ClusterDefine, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Row, ConnectorError>;
    async fn query_opt(&self, cluster: ClusterDefine, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Option<Row>, ConnectorError>;
    async fn prepare(&self, cluster: ClusterDefine, query: &str) -> Result<Statement, ConnectorError>;
    async fn prepare_typed(&self, cluster: ClusterDefine, query: &str, parameter_types: &[Type]) -> Result<Statement, ConnectorError>;
    async fn batch_execute(&self, cluster: ClusterDefine, query: &str) -> Result<(), ConnectorError>;

    // Versiones mutables
    async fn execute_mut(&mut self, cluster: ClusterDefine, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<u64, ConnectorError>;
    async fn query_mut(&mut self, cluster: ClusterDefine, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, ConnectorError>;
    async fn query_one_mut(&mut self, cluster: ClusterDefine, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Row, ConnectorError>;
    async fn query_opt_mut(&mut self, cluster: ClusterDefine, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Option<Row>, ConnectorError>;
    async fn prepare_mut(&mut self, cluster: ClusterDefine, query: &str) -> Result<Statement, ConnectorError>;
    async fn prepare_typed_mut(&mut self, cluster: ClusterDefine, query: &str, parameter_types: &[Type]) -> Result<Statement, ConnectorError>;
    async fn batch_execute_mut(&mut self, cluster: ClusterDefine, query: &str) -> Result<(), ConnectorError>;
}

// Implementación para PooledConnection
impl<'a, R> SqlExpose for PooledConnection<'a, PostgresConnectionManager<R>>
where
    R: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
    <R as MakeTlsConnect<Socket>>::Stream: Send + Sync,
    <R as MakeTlsConnect<Socket>>::TlsConnect: Send + Sync,
    <<R as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
{
    async fn execute(&self, _cluster: ClusterDefine, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<u64, ConnectorError> {
        self.deref().execute(query, params).await.map_err(ConnectorError::from)
    }

    async fn query(&self, _cluster: ClusterDefine, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, ConnectorError> {
        self.deref().query(query, params).await.map_err(ConnectorError::from)
    }

    async fn query_one(&self, _cluster: ClusterDefine, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Row, ConnectorError> {
        self.deref().query_one(query, params).await.map_err(ConnectorError::from)
    }

    async fn query_opt(&self, _cluster: ClusterDefine, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Option<Row>, ConnectorError> {
        self.deref().query_opt(query, params).await.map_err(ConnectorError::from)
    }

    async fn prepare(&self, _cluster: ClusterDefine, query: &str) -> Result<Statement, ConnectorError> {
        self.deref().prepare(query).await.map_err(ConnectorError::from)
    }

    async fn prepare_typed(&self, _cluster: ClusterDefine, query: &str, parameter_types: &[Type]) -> Result<Statement, ConnectorError> {
        self.deref().prepare_typed(query, parameter_types).await.map_err(ConnectorError::from)
    }

    async fn batch_execute(&self, _cluster: ClusterDefine, query: &str) -> Result<(), ConnectorError> {
        self.deref().batch_execute(query).await.map_err(ConnectorError::from)
    }

    async fn execute_mut(&mut self, cluster: ClusterDefine, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<u64, ConnectorError> {
        SqlExpose::execute(self, cluster, query, params).await
    }
    async fn query_mut(&mut self, cluster: ClusterDefine, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, ConnectorError> {
        SqlExpose::query(self, cluster, query, params).await
    }
    async fn query_one_mut(&mut self, cluster: ClusterDefine, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Row, ConnectorError> {
        SqlExpose::query_one(self, cluster, query, params).await
    }
    async fn query_opt_mut(&mut self, cluster: ClusterDefine, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Option<Row>, ConnectorError> {
        SqlExpose::query_opt(self, cluster, query, params).await
    }
    async fn prepare_mut(&mut self, cluster: ClusterDefine, query: &str) -> Result<Statement, ConnectorError> {
        SqlExpose::prepare(self, cluster, query).await
    }
    async fn prepare_typed_mut(&mut self, cluster: ClusterDefine, query: &str, parameter_types: &[Type]) -> Result<Statement, ConnectorError> {
        SqlExpose::prepare_typed(self, cluster, query, parameter_types).await
    }
    async fn batch_execute_mut(&mut self, cluster: ClusterDefine, query: &str) -> Result<(), ConnectorError> {
        SqlExpose::batch_execute(self, cluster, query).await
    }
}

// Implementación para Transaction (Clave para transacciones asíncronas)
impl<'a> SqlExpose for Transaction<'a> {
    async fn execute(&self, _cluster: ClusterDefine, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<u64, ConnectorError> {
        self.execute(query, params).await.map_err(ConnectorError::from)
    }
    async fn query(&self, _cluster: ClusterDefine, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, ConnectorError> {
        self.query(query, params).await.map_err(ConnectorError::from)
    }
    async fn query_one(&self, _cluster: ClusterDefine, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Row, ConnectorError> {
        self.query_one(query, params).await.map_err(ConnectorError::from)
    }
    async fn query_opt(&self, _cluster: ClusterDefine, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Option<Row>, ConnectorError> {
        self.query_opt(query, params).await.map_err(ConnectorError::from)
    }
    async fn prepare(&self, _cluster: ClusterDefine, query: &str) -> Result<Statement, ConnectorError> {
        self.prepare(query).await.map_err(ConnectorError::from)
    }
    async fn prepare_typed(&self, _cluster: ClusterDefine, query: &str, parameter_types: &[Type]) -> Result<Statement, ConnectorError> {
        self.prepare_typed(query, parameter_types).await.map_err(ConnectorError::from)
    }
    async fn batch_execute(&self, _cluster: ClusterDefine, query: &str) -> Result<(), ConnectorError> {
        self.batch_execute(query).await.map_err(ConnectorError::from)
    }

    async fn execute_mut(&mut self, cluster: ClusterDefine, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<u64, ConnectorError> {
        SqlExpose::execute(self, cluster, query, params).await
    }
    async fn query_mut(&mut self, cluster: ClusterDefine, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, ConnectorError> {
        SqlExpose::query(self, cluster, query, params).await
    }
    async fn query_one_mut(&mut self, cluster: ClusterDefine, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Row, ConnectorError> {
        SqlExpose::query_one(self, cluster, query, params).await
    }
    async fn query_opt_mut(&mut self, cluster: ClusterDefine, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Option<Row>, ConnectorError> {
        SqlExpose::query_opt(self, cluster, query, params).await
    }
    async fn prepare_mut(&mut self, cluster: ClusterDefine, query: &str) -> Result<Statement, ConnectorError> {
        SqlExpose::prepare(self, cluster, query).await
    }
    async fn prepare_typed_mut(&mut self, cluster: ClusterDefine, query: &str, parameter_types: &[Type]) -> Result<Statement, ConnectorError> {
        SqlExpose::prepare_typed(self, cluster, query, parameter_types).await
    }
    async fn batch_execute_mut(&mut self, cluster: ClusterDefine, query: &str) -> Result<(), ConnectorError> {
        SqlExpose::batch_execute(self, cluster, query).await
    }
}

// Implementación para PgConnector
impl<W, R> SqlExpose for PgConnector<W, R>
where
    W: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
    <W as MakeTlsConnect<Socket>>::Stream: Send + Sync,
    <W as MakeTlsConnect<Socket>>::TlsConnect: Send + Sync,
    <<W as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    R: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
    <R as MakeTlsConnect<Socket>>::Stream: Send + Sync,
    <R as MakeTlsConnect<Socket>>::TlsConnect: Send + Sync,
    <<R as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
{
    async fn execute(&self, cluster: ClusterDefine, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<u64, ConnectorError> {
        match cluster {
            ClusterDefine::WRITE => self.get_write().await?.execute(cluster, query, params).await,
            ClusterDefine::READ => self.get_read().await?.execute(cluster, query, params).await,
        }
    }

    async fn query(&self, cluster: ClusterDefine, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, ConnectorError> {
        match cluster {
            ClusterDefine::WRITE => self.get_write().await?.query(cluster, query, params).await,
            ClusterDefine::READ => self.get_read().await?.query(cluster, query, params).await,
        }
    }

    async fn query_one(&self, cluster: ClusterDefine, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Row, ConnectorError> {
        match cluster {
            ClusterDefine::WRITE => self.get_write().await?.query_one(cluster, query, params).await,
            ClusterDefine::READ => self.get_read().await?.query_one(cluster, query, params).await,
        }
    }

    async fn query_opt(&self, cluster: ClusterDefine, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Option<Row>, ConnectorError> {
        match cluster {
            ClusterDefine::WRITE => self.get_write().await?.query_opt(cluster, query, params).await,
            ClusterDefine::READ => self.get_read().await?.query_opt(cluster, query, params).await,
        }
    }

    async fn prepare(&self, cluster: ClusterDefine, query: &str) -> Result<Statement, ConnectorError> {
        match cluster {
            ClusterDefine::WRITE => self.get_write().await?.prepare(cluster, query).await,
            ClusterDefine::READ => self.get_read().await?.prepare(cluster, query).await,
        }
    }

    async fn prepare_typed(&self, cluster: ClusterDefine, query: &str, parameter_types: &[Type]) -> Result<Statement, ConnectorError> {
        match cluster {
            ClusterDefine::WRITE => self.get_write().await?.prepare_typed(cluster, query, parameter_types).await,
            ClusterDefine::READ => self.get_read().await?.prepare_typed(cluster, query, parameter_types).await,
        }
    }

    async fn batch_execute(&self, cluster: ClusterDefine, query: &str) -> Result<(), ConnectorError> {
        match cluster {
            ClusterDefine::WRITE => self.get_write().await?.batch_execute(cluster, query).await,
            ClusterDefine::READ => self.get_read().await?.batch_execute(cluster, query).await,
        }
    }

    async fn execute_mut(&mut self, cluster: ClusterDefine, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<u64, ConnectorError> {
        SqlExpose::execute(self, cluster, query, params).await
    }
    async fn query_mut(&mut self, cluster: ClusterDefine, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, ConnectorError> {
        SqlExpose::query(self, cluster, query, params).await
    }
    async fn query_one_mut(&mut self, cluster: ClusterDefine, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Row, ConnectorError> {
        SqlExpose::query_one(self, cluster, query, params).await
    }
    async fn query_opt_mut(&mut self, cluster: ClusterDefine, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Option<Row>, ConnectorError> {
        SqlExpose::query_opt(self, cluster, query, params).await
    }
    async fn prepare_mut(&mut self, cluster: ClusterDefine, query: &str) -> Result<Statement, ConnectorError> {
        SqlExpose::prepare(self, cluster, query).await
    }
    async fn prepare_typed_mut(&mut self, cluster: ClusterDefine, query: &str, parameter_types: &[Type]) -> Result<Statement, ConnectorError> {
        SqlExpose::prepare_typed(self, cluster, query, parameter_types).await
    }
    async fn batch_execute_mut(&mut self, cluster: ClusterDefine, query: &str) -> Result<(), ConnectorError> {
        SqlExpose::batch_execute(self, cluster, query).await
    }
}
