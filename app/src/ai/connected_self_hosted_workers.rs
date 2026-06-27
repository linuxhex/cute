use warpui::{Entity, ModelContext, SingletonEntity};

use crate::server::server_api::ai::ConnectedSelfHostedWorker;
pub const WARP_WORKER_HOST: &str = "warp";

pub enum ConnectedSelfHostedWorkersEvent {
    Changed,
}

pub struct ConnectedSelfHostedWorkersModel {
    workers: Vec<ConnectedSelfHostedWorker>,
}

impl ConnectedSelfHostedWorkersModel {
    pub fn new(ctx: &mut ModelContext<Self>) -> Self {
        // Cute OMJF-11111: 本地模式不连接远程 self-hosted worker
        let _ = ctx;
        Self {
            workers: Vec::new(),
        }
    }

    pub fn refresh(&mut self, ctx: &mut ModelContext<Self>) {
        let _ = ctx;
    }

    pub fn worker_hosts_excluding(&self, excluded: Option<&str>) -> Vec<String> {
        let mut hosts: Vec<String> = self
            .workers
            .iter()
            .map(|worker| worker.worker_host.clone())
            .filter(|host| !host.trim().is_empty())
            .filter(|host| !host.eq_ignore_ascii_case(WARP_WORKER_HOST))
            .filter(|host| match excluded {
                Some(excluded) => !host.eq_ignore_ascii_case(excluded),
                None => true,
            })
            .collect();
        hosts.sort();
        hosts.dedup();
        hosts
    }

    fn clear_workers(&mut self, ctx: &mut ModelContext<Self>) {
        let _ = ctx;
        self.clear_worker_cache();
    }

    fn clear_worker_cache(&mut self) -> bool {
        if self.workers.is_empty() {
            return false;
        }
        self.workers.clear();
        true
    }
}

impl Entity for ConnectedSelfHostedWorkersModel {
    type Event = ConnectedSelfHostedWorkersEvent;
}

impl SingletonEntity for ConnectedSelfHostedWorkersModel {}

#[cfg(test)]
#[path = "connected_self_hosted_workers_tests.rs"]
mod tests;
