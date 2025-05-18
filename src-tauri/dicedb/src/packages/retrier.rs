use std::{thread, time::Duration};

use tracing::warn;

use super::wire::{ErrKind, WireError};

#[derive(Clone)]
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

    pub fn execute<T, F, R>(
        &self,
        retryable_error_kinds: &[ErrKind],
        operation: F,
        on_retry: R,
    ) -> Result<T, WireError>
    where
        F: Fn() -> Result<T, WireError>,
        R: Fn() -> Result<(), WireError>,
    {
        let mut attempts = 0;

        loop {
            match operation() {
                Ok(result) => return Ok(result),
                Err(err) => {
                    attempts += 1;

                    if attempts > self.max_retries || !retryable_error_kinds.contains(&err.kind) {
                        return Err(err);
                    }

                    if let Err(retry_err) = on_retry() {
                        return Err(retry_err);
                    }

                    warn!(
                        "Operation failed, retrying ({}/{}): {:?}",
                        attempts, self.max_retries, err
                    );

                    thread::sleep(self.backoff_duration);
                }
            }
        }
    }

    pub fn execute_void<F, R>(
        &self,
        retryable_error_kinds: &[ErrKind],
        operation: F,
        on_retry: R,
    ) -> Result<(), WireError>
    where
        F: Fn() -> Result<(), WireError>,
        R: Fn() -> Result<(), WireError>,
    {
        self.execute(retryable_error_kinds, operation, on_retry)
    }
}
