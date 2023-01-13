# Vector Heap Allocations

This project plots the memory consumption of the `Vec` type when using the `Vec::push` method.

## TODO

- create formula for memory consumption based on results
- discuss relevant changes to `std::vec::Vec`
- generalize to test many different sizes

## Implementation

```rs
struct RawVec<T> {
    ptr: raw_ptr,
    cap: u64,
}

impl<T> RawVec<T> {
    pub fn new() -> Self {
        Self {
            ptr: alloc::<T>(0),
            cap: 0,
        }
    }

    pub fn with_capacity(capacity: u64) -> Self {
        Self {
            ptr: alloc::<T>(capacity),
            cap: capacity,
        }
    }

    pub fn grow(ref mut self) {
        let new_cap = if self.cap == 0 { 1 } else { 2 * self.cap };

        self.ptr = realloc::<T>(self.ptr, self.cap, new_cap);
        self.cap = new_cap;
    }
}

struct Vec {
    buf: RawVec<T>,
    len: u64,
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        Self {
            buf: RawVec::new(),
            len: 0,
        }
    }

    pub fn with_capacity(capacity: u64) -> Self {
        Self {
            buf: RawVec::with_capacity(capacity),
            len: 0,
        }
    }

    pub fn push(ref mut self, value: T) {
        if self.len == self.buf.capacity() {
            self.buf.grow();
        };
        let end = self.buf.ptr().add::<T>(self.len);
        end.write::<T>(value);
        self.len += 1;
    }
}
```

When creating a vector via `Vec::new`, the buffer is allocated with `0` size. When `Vec::push` is
called, it first checks if the new element fits in the raw vector's capacity and if not, it
allocates a new memory buffer equal to the current buffer times two (unless the capacity is zero,
in which case it becomes one).

## Checking Memory Allocations

We check memory consumption of two vectors using the [MemoryConsoomoor](./src/main.sw) Sway
contract. We begin by calling `Vec::with_capacity` to create the vector with a predefined capacity,
one with `0` and the other with `16`.

> NOTE: By calling `Vec::with_capacity(0)`, it is functionally the same as `Vec::new()`.

We then push `n` elements to each vector and observe their memory consumption by counting the amount
by which the heap pointer decreases (the heap grows downward, so the amount `hp` decreases by is the
amount of memory consumed).

## Results

The results are plotted at the following images:

- [Zero Capacity (`Vec::with_capacity(0)`)](./zero_cap.png)
- [Sixteen Capacity (`Vec::with_capacity(16)`)](./n_cap.png)

## Conclusion

While this is a single example and results may vary on different sized vectors, it is clear that
there are more efficient ways to allocate vectors than the developers may or may not be aware of.

Possible solutions include:

- `Vec::new` begins with a pre-allocated memory greater than zero
- `Vec::with_capacity` is preferred to allocate new vectors
- `RawVec::grow` allocates by a different function
    - if the current capacity is zero, increase it to greater than one
    - use a different formula for scaling the capacity
