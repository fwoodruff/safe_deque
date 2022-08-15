Deques are rarely useful in modern code.

Deques have lots of reference cycles and interior mutability. They are pathological for Rust's ownership model. Writing a concurrent deque is the Boss Fight of safe Rust.

std::sync::Arc cycles are not panic safe and std::sync::Weak has been my strategy for avoiding them. I believe this becomes relevant in the context of custom allocators. This introduces a runtime overhead. I have abused std::sync::Mutex as a Sync RefCell.