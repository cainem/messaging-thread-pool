// use crate::id_targeted::IdTargeted;

// use self::{
//     mean_response::MeanResponse, randoms_init_response::RandomsInitResponse,
//     sum_response::SumResponse,
// };

pub mod mean_response;
pub mod randoms_init_response;
pub mod sum_response;

// /// This enum defines the full range of responses that can received back from a Randoms request.
// ///
// /// The protocol currently requires that every request provides a response and therefore it makes
// /// sense that the naming of the responses mirrors that of the the requests
// #[derive(Debug, Clone, PartialEq, Eq)]
// pub enum RandomsResponse {
//     Mean(MeanResponse),
//     Sum(SumResponse),
//     Init(RandomsInitResponse),
// }

// impl IdTargeted for RandomsResponse {
//     fn id(&self) -> u64 {
//         match self {
//             RandomsResponse::Mean(do_work) => do_work.id(),
//             RandomsResponse::Init(init) => init.id(),
//             RandomsResponse::Sum(get_state) => get_state.id(),
//         }
//     }
// }
