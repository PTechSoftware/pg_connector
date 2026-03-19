use std::time::Duration;

use bb8::{Pool, QueueStrategy};
use bb8_postgres::{
    PostgresConnectionManager,
    tokio_postgres::{
        Socket,
        tls::{MakeTlsConnect, TlsConnect},
    },
};
use crate::errors::ConnectorError;

pub struct PoolManager<TLS>
where
    TLS: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
    <TLS as MakeTlsConnect<Socket>>::Stream: Send + Sync,
    <TLS as MakeTlsConnect<Socket>>::TlsConnect: Send + Sync,
    <<TLS as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
{
    pub first_connection_sleep_time: u64,
    pub max_retries: u32,
    _manager: PostgresConnectionManager<TLS>,
}

impl<TLS> PoolManager<TLS>
where
    TLS: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
    <TLS as MakeTlsConnect<Socket>>::Stream: Send + Sync,
    <TLS as MakeTlsConnect<Socket>>::TlsConnect: Send + Sync,
    <<TLS as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
{
    pub fn new(
        host: &str,
        port: u16,
        user: &str,
        password: &str,
        database: Option<&str>,
        first_connection_sleep_time: u64,
        max_retries: u32,
        tls: TLS,
    ) -> Self {
        let _man = match database {
            Some(db) => PostgresConnectionManager::new_from_stringlike(
                format!(
                    "host={} port={} user={} password={} dbname={}",
                    host, port, user, password, db
                ),
                tls,
            )
            .expect("Failed to create PostgresConnectionManager"),
            None => PostgresConnectionManager::new_from_stringlike(
                format!(
                    "host={} port={} user={} password={}",
                    host, port, user, password
                ),
                tls,
            )
            .expect("Failed to create PostgresConnectionManager"),
        };

        Self {
            first_connection_sleep_time,
            max_retries,
            _manager: _man,
        }
    }

    pub async fn get_pool(
        &self,
        min_connections: u32,
        max_connections: u32,
        retry_until_timeout: bool,
        connection_timeout_millis: u64,
        max_timeout_millis: Option<u64>,
        queue_strategy: QueueStrategy,
    ) -> Result<Pool<PostgresConnectionManager<TLS>>, ConnectorError> {
        tokio::time::sleep(std::time::Duration::from_secs(
            self.first_connection_sleep_time,
        ))
        .await;
        let con_to = Duration::from_millis(connection_timeout_millis);
        let mut retries = 0;
        loop {
            let res = Pool::builder()
                .max_size(max_connections)
                .min_idle(min_connections)
                .queue_strategy(queue_strategy)
                .connection_timeout(con_to)
                .max_lifetime(max_timeout_millis.map(Duration::from_millis))
                .build(self._manager.clone())
                .await;

            match res {
                Ok(p) => return Ok(p),
                Err(e) => {
                    retries += 1;
                    if !retry_until_timeout || retries >= self.max_retries {
                        return Err(ConnectorError::PoolError(format!(
                            "Max retries exceeded while initializing pool (attempted {}): {}",
                            retries, e
                        )));
                    }
                    tokio::time::sleep(Duration::from_millis(250)).await;
                }
            }
        }
    }

}

