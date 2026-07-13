use std::collections::HashMap;
use std::sync::Arc;

use futures_util::StreamExt;
use cute_core::features::FeatureFlag;
use cute_multi_agent_api as api;


use super::{ConvertToAPITypeError, RequestParams, ResponseStream};
use crate::server::server_api::ServerApi;
use crate::terminal::model::session::SessionType;

pub async fn generate_multi_agent_output(
    _server_api: Arc<ServerApi>,
    _params: RequestParams,
    _cancellation_rx: futures::channel::oneshot::Receiver<()>,
) -> Result<ResponseStream, ConvertToAPITypeError> {
    unimplemented!("generate_multi_agent_output disabled")
}

#[cfg(test)]
#[path = "impl_tests.rs"]
mod tests;
