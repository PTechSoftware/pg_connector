use crate::{errors::ConnectorError};
use bb8::{Pool, PooledConnection};
use bb8_postgres::{
    PostgresConnectionManager,
    tokio_postgres::{Socket, tls::MakeTlsConnect, tls::TlsConnect},
};

pub struct PgConnector<W, R>
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
    write: Pool<PostgresConnectionManager<W>>,
    read: Pool<PostgresConnectionManager<R>>,
}

impl<W, R> PgConnector<W, R>
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
    pub fn new(
        write: Pool<PostgresConnectionManager<W>>,
        read: Pool<PostgresConnectionManager<R>>,
    ) -> Self {
        Self { write, read }
    }

    pub fn write(&self) -> &Pool<PostgresConnectionManager<W>> {
        &self.write
    }

    pub fn read(&self) -> &Pool<PostgresConnectionManager<R>> {
        &self.read
    }

    pub async fn get_write(
        &self,
    ) -> Result<bb8::PooledConnection<'_, PostgresConnectionManager<W>>, ConnectorError> {
        self.write.get().await.map_err(ConnectorError::from)
    }

    pub async fn get_read(
        &self,
    ) -> Result<bb8::PooledConnection<'_, PostgresConnectionManager<R>>, ConnectorError> {
        self.read.get().await.map_err(ConnectorError::from)
    }

    pub async fn get_pooled_connection_write(
        &self
    ) -> Result<PooledConnection<'_, PostgresConnectionManager<W>>, ConnectorError> {
        self.get_write().await
    }

    pub async fn get_pooled_connection_read(
        &self
    ) -> Result<PooledConnection<'_, PostgresConnectionManager<R>>, ConnectorError> {
        self.get_read().await
    }
}
