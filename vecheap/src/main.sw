contract;

abi MemoryConsoomoor {
    /// Vector Push Method Memory Consumption.
    ///
    /// initial_capacity - Initially allocated memory to contain the elements.
    /// elements_to_push - Number of elements to push to the vector.
    ///
    /// returns amount of memory allocated.
    fn vec_push(initial_capacity: u64, elements_to_push: u64) -> u64;
}

impl MemoryConsoomoor for Contract {
    fn vec_push(initial_capacity: u64, elements_to_push: u64) -> u64 {
        let before_hp = asm() { hp: u64 };

        let mut v = Vec::with_capacity(initial_capacity);
        let element = 1;

        let mut i = 0;
        while i < elements_to_push {
            v.push(element);
            i += 1;
        }

        let after_hp = asm() { hp: u64 };

        // heap grows downward, so `after_hp` is either less than or equal to `before_hp`
        before_hp - after_hp
    }
}
