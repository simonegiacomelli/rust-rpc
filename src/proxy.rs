use std::fmt::Debug;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::rpc::Response;

pub struct Proxy {}

impl Proxy {
    pub fn send<Req, Res>(&self, req: Req) -> Res
        where Res: Response<Req>,
              Req: ?Sized + Serialize + DeserializeOwned + Debug,
              Res: ?Sized + Serialize + DeserializeOwned + Debug,
    {
        todo!()
    }
}
