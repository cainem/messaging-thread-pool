# Changelog

All notable changes to this project will be documented in this file.

## [1.0.0]

### Changes

* Ditch Element trait in favour of PoolItem trait.\
  The PoolItem trait provides a cleaner and hopefully more intuitive interface with which to communicate with items in
  the pool.

## [2.0.0]

* Add ThreadPoolSenderAndReceiver trait to allow for mocking of the thread pool
* Flatten some namespaces

## [2.0.1]

* Add Default to ThreadPoolMock

## [2.0.2]

* Add Default to AddResponse

## [3.1.0]

* Change PoolItem trait to take ownership of requests rather than references to the request.
* Rename ThreadPoolMock to SenderAndReceiverMock
* Rename ThreadPoolSenderAndReceiver to SenderAndReceiver
* Make SenderAndReceiverMock Send & Sync for use in more scenarios
* Change IdProvider to return usize instead u64
* Remove unnecessary SizedIdProvider; replace with Arc&lt;dyn IdProvider&gt;
* Relax Send constraint on IdTargeted trait.
* Replace RequestResponseMessage with RequestWithResponse trait to simplify use.
* Add more re-exports to simplify exposed modules
* Add custom partial_eq implementation to request_response.
* Make ThreadEndpoint fields private
* Remove unnecessary PhantomData from SenderAndReceiverMock
* Add was_called to SenderAndReceiverMock
* Change send to return a Result and propagate result where necessary
* Make error_message in NewPoolItemError public
* Add SendAndReceiverRawMock to deal with heterogeneous streams of messages
* Change success() to item_existed() in RemovePoolItemResponse
* Change success() to result in AddResponse
* Ignore requests to add and existing key; return appropriate error in response
* Add default implementation of send_and_receive_one to SenderAndReceiver trait
* Add is_complete function to mocks
* Add id to the result of AddResponse
* Implement the insertion of pool item tracing
* tidy up tracing
* Change add_pool_item_tracing to take a reference to the pool item, not just its id
* tidy up add_pool_thread_tracing
* add more logging

## [3.4.0]

* Add a PoolItem function to provide custom id to thread mapping rather that relying on a simple mod
* Add assert_is_complete to SenderAndReceiverMock
* Add Send + Sync constraint on GuardDrop so that it can be used in async functions
* Allow id currently being processed to be accessed with new id_being_processed() function
* Add mimalloc benchmark to show the benefits of changing the default allocator (only tried on windows but the benefits were large)

## [4.0.0]
### These changes were made primarily to allow logging and tracing to be catered for more efficiently

* Add a new associated type *ThreadStartInfo* to PoolItem trait. 
* Add a new *thread_start* function to PoolItem trait that optionally returns a *ThreadStartInfo* when a thread in the thread pool first starts up. This gives the opportunity to create state that is shared across all of the pool items using that thread
* Add a new *pool_item_pre_process* function to PoolItem trait. This gets called prior to the thread switching context to a new pool item. A mutable copy of the shared state *ThreadStartInfo* is passed in.
* Add a new *pool_item_post_process* function to PoolItem trait. This gets called once a message to a pool item has been processed and it is about to switch context to another pool item. A mutable copy of the shared state *ThreadStartInfo* is passed in.
* Remove the function *add_pool_thread_tracing*. This is superseded by the 3 new function calls which give the opportunity to provide whatever logging is required.
* Revert back to using u64 as IDs rather than usize. This was due to interop problems with WASM32.

As started these changes were primarily made for logging/tracing reasons and the examples have been adjusted to show how this might be used. \
The new shared state allows a single tracer to be used for all pool items or individual pool items can be conditionally traced. \
If the functionality is not required *ThreadStartInfo* can just be set to the unit type () and the default behaviour of the three new functions is simply to do nothing.



