use std::sync::Arc;



use super::{ConvertToAPITypeError, RequestParams, ResponseStream};
use crate::server::server_api::ServerApi;

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
