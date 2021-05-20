# conch
Toy concurrent hash map in Rust

Basically inspired by how ConcurrentHashmap works on the JVM. Instead of locking the entire structure, partition the keyspace into buckets and then only lock the bucket that the key belongs to on updates. 
