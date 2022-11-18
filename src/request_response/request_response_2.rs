// pub const MEAN: usize = 0;
// pub const SUM: usize = 1;

// const API: &[usize] = &[1, 2, 3, 4, 5];

// pub enum RequestResponse<const N: usize, Req, Res>
// where
//     Req: ApiMessage<N>,
//     Res: ApiMessage<N>,
// {
//     Request(Req),
//     Response(Res),
// }

// pub struct MeanRequest;
// // {
// //     id: usize,
// // }

// pub struct MeanResponse;
// // {
// //     id: usize,
// // }

// impl ApiMessage<MEAN> for MeanResponse {}

// impl ApiMessage<MEAN> for MeanRequest {}

// pub trait ApiMessage<const N: usize> {
//     const MESSAGE_TYPE: usize = N;
//     //fn message_type(&self) -> usize;
// }

// pub fn messing() {
//     let x = RequestResponse::<MEAN, MeanRequest, MeanResponse>::Request(MeanRequest);
// }

// pub trait PoolItemApi: Debug + IdTargeted {
//     fn is_request(&self) -> bool;
//     fn is_response(&self) -> bool {
//         !self.is_request()
//     }
// }
