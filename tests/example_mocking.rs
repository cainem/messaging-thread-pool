use messaging_thread_pool::{
    samples::{MeanRequest, MeanResponse, Randoms, RandomsAddRequest},
    thread_pool_sender_and_receiver::ThreadPoolSenderAndReceiver,
    thread_request_response::AddResponse,
};

struct Complex<T>
where
    T: ThreadPoolSenderAndReceiver<Randoms>,
{
    contained_ids: Vec<usize>,
    contained_thread_pool: T,
}

impl<T> Complex<T>
where
    T: ThreadPoolSenderAndReceiver<Randoms>,
{
    fn new(contained_thread_pool: T, ids: impl Iterator<Item = usize>) -> Self {
        let ids: Vec<_> = ids.collect();

        let _: Box<dyn Iterator<Item = AddResponse>> =
            contained_thread_pool.send_and_receive(ids.iter().map(|id| RandomsAddRequest(*id)));

        Self {
            contained_ids: ids,
            contained_thread_pool,
        }
    }

    // This is a function that uses the internal thread pool to get the sum of the means
    // of the passed in ids.
    fn sum_of_means(&self) -> u128 {
        let result: u128 = self
            .contained_thread_pool
            .send_and_receive(self.contained_ids.iter().map(|id| MeanRequest(*id)))
            .map(|res: MeanResponse| res.mean)
            .sum();

        result
    }
}

mod tests {
    use messaging_thread_pool::{
        samples::{MeanRequest, MeanResponse, Randoms, RandomsAddRequest},
        thread_pool_sender_and_receiver::ThreadPoolMock,
        thread_request_response::{AddResponse, ThreadRequestResponse},
        ThreadPool,
    };

    use crate::Complex;

    #[test]
    pub fn example_complex_type_with_contained_thread_pool_that_needs_mocking_mocked_example() {
        let expected_requests: Vec<ThreadRequestResponse<Randoms>> = vec![
            RandomsAddRequest(1).into(),
            RandomsAddRequest(2).into(),
            MeanRequest(1).into(),
            MeanRequest(2).into(),
        ];
        let responses: Vec<ThreadRequestResponse<Randoms>> = vec![
            AddResponse::new(1, true, None).into(),
            AddResponse::new(2, true, None).into(),
            MeanResponse { id: 1, mean: 3 }.into(),
            MeanResponse { id: 2, mean: 5 }.into(),
        ];

        // create a mock thread pool
        // this defines the expected requests and a vec of responses to return
        let mock = ThreadPoolMock::new_with_expected_requests(expected_requests, responses);

        // create the complex type with the mock thread pool
        let target = Complex::new(mock, [1, 2].into_iter());

        let result = target.sum_of_means();

        assert_eq!(8, result);
    }

    #[test]
    pub fn example_complex_type_with_contained_thread_pool_that_needs_mocking_concrete_example() {
        // create a real thread pool
        let thread_pool = ThreadPool::new(5);

        // create the complex type with the mock thread pool
        let target = Complex::new(thread_pool, [1, 2].into_iter());

        // result here is a sum of 2 random numbers
        let result = target.sum_of_means();

        dbg!(result);
    }
}
