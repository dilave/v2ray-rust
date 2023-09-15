#[cfg(feature = "enable_useless")]
use crate::api::v2ray_rust_api::{GetStatsRequest, GetStatsResponse};
use crate::config::COUNTER_MAP;

use std::sync::atomic::Ordering::Relaxed;
#[cfg(feature = "enable_useless")]
use tonic::{Request, Response, Status};

pub mod v2ray_rust_api {
    #[cfg(feature = "enable_useless")]
    tonic::include_proto!("v2ray.core.app.stats.command");
}
#[cfg(feature = "enable_useless")]
use v2ray_rust_api::stats_service_server::{StatsService, StatsServiceServer};

#[derive(Default)]
pub struct ApiServer;

impl ApiServer {
    #[cfg(feature = "enable_useless")]
    pub(crate) fn new_server() -> StatsServiceServer<Self> {
        StatsServiceServer::new(Self)
    }
}
#[cfg(feature = "enable_useless")]
#[tonic::async_trait]
impl StatsService for ApiServer {
    async fn get_stats(
        &self,
        mut request: Request<GetStatsRequest>,
    ) -> Result<Response<GetStatsResponse>, Status> {
        let name = &request.get_ref().name;
        let reset = request.get_ref().reset;
        let ret_v;
        if let Some(v) = COUNTER_MAP.get().unwrap().get(name) {
            if reset {
                ret_v = v.swap(0, Relaxed);
            } else {
                ret_v = v.load(Relaxed);
            }
        } else {
            return Err(Status::new(tonic::Code::InvalidArgument, "name is invalid"));
        }
        Ok(Response::new(v2ray_rust_api::GetStatsResponse {
            stat: Some(v2ray_rust_api::Stat {
                name: std::mem::take(&mut request.get_mut().name),
                value: ret_v as i64,
            }),
        }))
    }
}
