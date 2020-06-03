use std::sync::atomic::{AtomicUsize, Ordering};

use actix_web::{Responder};

static GLOBAL_REQUEST_COUNTER: AtomicUsize = AtomicUsize::new(1);

pub async fn hello_world() -> impl Responder {
    let request_number = GLOBAL_REQUEST_COUNTER.fetch_add(1, Ordering::SeqCst);

    format!("Request # {}", request_number)
}
