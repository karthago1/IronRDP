# IronRDP Blocking

Blocking I/O abstraction wrapping the IronRDP state machines conveniently.

This crate is a higher level abstraction for IronRDP state machines using blocking I/O instead of
asynchronous I/O. This results in a simpler API with fewer dependencies that should be used
instead of `ironrdp-async` when concurrency is not a requirement.