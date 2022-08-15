Deques are rarely useful in modern code.

Deques have lots of reference cycles and interior mutability. They are pathological for Rust's ownership model. Writing a concurrent deque is the Boss Fight of safe Rust.

std::sync::Arc cycles are not panic safe and std::sync::Weak has been my strategy for avoiding them. I believe this becomes relevant in the context of custom allocators.

The abuse of std::sync::Mutex as a kind of Send RefCell, and the paranoid runtime reference counting mean this is not a practical deque.