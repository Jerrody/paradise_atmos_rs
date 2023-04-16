# paradise_atmos_rs
An experimental Atmospherics implementation in Rust.

For the base of this code was taken code base of project: [Paradise SS220](https://github.com/ss220-space/Paradise).

# Current Status
Currently, implemented only logic of `gas_mixture`, but in ideal world this should be enough for to gain performance boost per server tick. Also, for the full migrating `gas_mixture` to the Rust code I did interop with `turf` that is still placed in DM code base.

**Implementation isn't tested.**

# Implementation 
In comparison with others implementations of atmos in Rust, this implementation uses technique of SOA - Struct of Array. This means, that for to get `oxygen` field of `gas_mixture` I don't need to load to the CPU memory whole 68 bytes,
I need only to index an array of `oxygen`s and take an `oxygen` of `gas_mixture` by it's id that is internal id in BYOND game instance. As the result, this is ops are very fast and should be very cheap to operate with data per server's tick.

# Features
`profile`- this is an optional feature that enables visual profiling via `Tracy`, you will start see execution time of methods that was migrated to Rust like `fire` proc from DM in profiler.

`profile-proc` - this is an optional feature that enables visual profiling via `Tracy`, you will start see whole execution time from the beginning hook to return from the hook.
