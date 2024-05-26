use sycamore::futures::spawn_local;
use sycamore::prelude::*;

use ebobo_shared::Fighter;
use sycamore_router::navigate;

use crate::api::*;

#[component]
pub async fn Dashboard<G: Html>() -> View<G> {
    view!("welcome to the arena")
}