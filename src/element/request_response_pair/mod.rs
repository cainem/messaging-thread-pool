use crate::id_targeted::IdTargeted;

// pub trait RequesterResponder<Req, Res> {
//     fn request(&self) -> &Req;
//     fn response(&self) -> &Res;
//     fn is_request(&self) -> bool;
// }

#[derive(Debug)]
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

// impl<Req, Res> RequesterResponder<Req, Res> for RequestResponse<Req, Res>
// where
//     Req: IdTargeted,
//     Res: IdTargeted,
// {
//     fn request(&self) -> &Req {
//         todo!()
//     }

//     fn response(&self) -> &Res {
//         todo!()
//     }

//     fn is_request(&self) -> bool {
//         todo!()
//     }
// }

#[cfg(test)]
mod tests {
    #[test]
    fn todo() {
        todo!();
    }
}
