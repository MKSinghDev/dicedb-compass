use log::{debug, warn};
use std::future::Future;
use std::time::Duration;
use tokio::time::sleep;

use super::error::ClientWireError;

#[derive(Clone, Debug)]
pub struct Retrier {
    pub max_retries: u32,
    pub backoff_duration: Duration,
}

impl Retrier {
    pub fn new(max_retries: u32, backoff_duration: Duration) -> Self {
        Self {
            max_retries,
            backoff_duration,
        }
    }

    // Execute an operation with retries for async functions returning a result
    pub async fn execute_async<T, F, Fut, R, RFut>(
        &self,
        operation: F,
        on_retry: R,
    ) -> Result<T, ClientWireError>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<T, ClientWireError>>,
        R: Fn() -> RFut,
        RFut: Future<Output = Result<(), ClientWireError>>,
    {
        let mut attempts = 0;

        loop {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(err) => {
                    attempts += 1;

                    // Check if we should retry
                    if attempts > self.max_retries || !Self::is_retryable(&err) {
                        return Err(err);
                    }

                    warn!(
                        "Operation failed, retrying ({}/{}): {:?}",
                        attempts, self.max_retries, err
                    );

                    // Calculate backoff with exponential increase
                    let backoff = self
                        .backoff_duration
                        .mul_f32(1.5_f32.powi(attempts as i32 - 1));

                    // Try to recover before next attempt
                    if let Err(retry_err) = on_retry().await {
                        warn!("Failed to perform retry operation: {:?}", retry_err);
                        // Continue with retry even if on_retry fails
                    }

                    debug!("Waiting for {:?} before next retry", backoff);
                    sleep(backoff).await;
                }
            }
        }
    }

    // Execute an operation with retries for async functions returning no result
    pub async fn execute_async_void<F, Fut, R, RFut>(
        &self,
        operation: F,
        on_retry: R,
    ) -> Result<(), ClientWireError>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<(), ClientWireError>>,
        R: Fn() -> RFut,
        RFut: Future<Output = Result<(), ClientWireError>>,
    {
        self.execute_async(operation, on_retry).await
    }

    // Check if an error is retryable
    fn is_retryable(err: &ClientWireError) -> bool {
        matches!(
            err,
            ClientWireError::NotEstablished(_)
                | ClientWireError::Terminated(_)
                | ClientWireError::TimeoutError(_)
        )
    }
}
