use crate::id_targeted::IdTargeted;

#[derive(Debug, PartialEq, Eq)]
pub enum RequestResponse<Req, Res>
where
    Req: IdTargeted,
    Res: IdTargeted,
{
    Request(Req),
    Response(Res),
}

impl<Req, Res> IdTargeted for RequestResponse<Req, Res>
where
    Req: IdTargeted,
    Res: IdTargeted,
{
    fn id(&self) -> u64 {
        match self {
            RequestResponse::Request(request) => request.id(),
            RequestResponse::Response(response) => response.id(),
        }
    }
}

impl<Req, Res> RequestResponse<Req, Res>
where
    Req: IdTargeted,
    Res: IdTargeted,
{
    pub fn request(&self) -> &Req {
        todo!();
    }
    pub fn response(&self) -> &Res {
        todo!();
    }
    pub fn is_request(&self) -> bool {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn todo() {
        todo!();
    }
}
