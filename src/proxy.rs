struct Proxy {}

impl Proxy {
    fn send<Req, Res>(&self, req:i8)    -> Res
        where Res: Response<Req>,
              Req: ?Sized + Serialize + DeserializeOwned + Debug,
              Res: ?Sized + Serialize + DeserializeOwned + Debug,
    {

    }
}
