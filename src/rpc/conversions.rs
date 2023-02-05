use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use serde_json::Error;

pub fn rpc_req_to_str<Req>(req: &Req) -> String
    where Req: ?Sized + Serialize + DeserializeOwned + Debug {
    let req_json = serde_json::to_string(req).unwrap();
    req_json
}

pub fn rpc_req_from_str<Req>(payload: &str) -> Result<Req, String>
    where Req: ?Sized + Serialize + DeserializeOwned + Debug {
    let result_req = serde_json::from_str(payload);
    match result_req {
        Ok(req) => { Ok(req) }
        Err(err) => { Err(rpc_error("serde from_str failed")) }
    }
}

pub fn rpc_error(msg: &str) -> String {
    return format!("success=0\n{}", msg);
}

pub fn rpc_success(msg: &str) -> String {
    return format!("success=1\n{}", msg);
}

pub fn rpc_res_to_str<Res>(res: &Res) -> String
    where Res: ?Sized + Serialize + DeserializeOwned + Debug {
    let res_json = serde_json::to_string(&res).unwrap();
    rpc_success(&res_json)
}

pub fn rpc_res_from_str<Res>(res_payload: &String) -> Result<Res, String>
    where Res: ?Sized + Serialize + DeserializeOwned + Debug {
    let mut s = res_payload.splitn(2, "\n");
    let header = s.next().ok_or("Payload split 1")?;
    let body = s.next().ok_or("Payload split 2")?;
    match header {
        "success=0" => { Err(body.to_string()) }
        "success=1" => {
            let res: Res = serde_json::from_str(&body).unwrap();
            Ok(res)
        }
        _ => { Err("unrecognized payload".to_string()) }
    }
}
