// use crate::id_targeted::IdTargeted;

// use self::{
//     mean_request::MeanRequest, randoms_init_request::RandomsInitRequest, sum_request::SumRequest,
// };

pub mod mean_request;
pub mod randoms_init_request;
pub mod sum_request;

// /// This enum defines the full range of request that a Random struct can be sent
// ///
// /// This in effects defines the api support by the Random.
// ///
// /// Each request is required to produce a response so there is a corresponding
// /// responses enum, which by convention, is similarly named
// #[derive(Debug, PartialEq, Eq, Clone)]
// pub enum RandomsRequest {
//     Mean(MeanRequest),
//     Sum(SumRequest),
//     Init(RandomsInitRequest),
// }

// impl IdTargeted for RandomsRequest {
//     fn id(&self) -> u64 {
//         match self {
//             RandomsRequest::Mean(do_work) => do_work.id(),
//             RandomsRequest::Init(init) => init.id(),
//             RandomsRequest::Sum(get_state) => get_state.id(),
//         }
//     }
// }
