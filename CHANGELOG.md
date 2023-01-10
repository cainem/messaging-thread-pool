# Changelog
All notable changes to this project will be documented in this file.

## [1.0.0] 
### Changes
- Ditch Element trait in favour of PoolItem trait.\
The PoolItem trait provides a cleaner and hopefully more intuitive interface with which to communicate with items in the pool.

## [2.0.0] 
- Add ThreadPoolSenderAndReceiver trait to allow for mocking of the thread pool
- Flatten some namespaces

## [2.0.1]
- Add Default to ThreadPoolMock

## [2.0.2]
- Add Default to AddResponse

## [3.0.*-alpha]
- Change PoolItem trait to take ownership of requests rather than references to the request.
- Rename ThreadPoolMock to SenderAndReceiverMock
- Rename ThreadPoolSenderAndReceiver to SenderAndReceiver
- Make SenderAndReceiverMock Send & Sync for use in more scenarios
- Change IdProvider to return usize instead u64
- Remove unnecessary SizedIdProvider; replace with Arc&lt;dyn IdProvider&gt;
- Relax Send constraint on IdTargeted trait.
- Replace RequestResponseMessage with RequestWithResponse trait to simplify use.
- Add more re-exports to simplify exposed modules
- Add custom partial_eq implementation to request_response.
- Make ThreadEndpoint fields private

