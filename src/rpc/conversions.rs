use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use serde_json::Error;

pub fn rpc_req_to_str<Req>(req: &Req) -> String
    where Req: ?Sized + Serialize + DeserializeOwned + Debug {
    let req_json = serde_json::to_string(req).unwrap();
    req_json
}

pub fn rpc_req_from_str<Req>(payload: &str) -> Result<Req, Error>
    where Req: ?Sized + Serialize + DeserializeOwned + Debug {
    let req: Req = serde_json::from_str(payload)?;
    Ok(req)
}

pub fn rpc_res_to_str<Res>(res: &Res) -> String
    where Res: ?Sized + Serialize + DeserializeOwned + Debug {
    let res_json = serde_json::to_string(&res).unwrap();
    res_json
}

pub fn rpc_res_from_str<Res>(res_payload: &String) -> Res
    where Res: ?Sized + Serialize + DeserializeOwned + Debug {
    let res: Res = serde_json::from_str(&res_payload).unwrap();
    res
}