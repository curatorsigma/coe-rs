# 0.2.1 -> TODO
- Added `is_empy` function and `impl Default` for `Packet`

# 0.2.0 -> 0.2.1
- Added getters for Payloads and Packets.
- Added `iter`, `iter_mut` and `into_iter` for Packets.
- Added `unit_id` getter for Payloads and Values.
- Added `wire_size` to packet to ensure safe serialization.
- Added `serialize_into_vec` on the `alloc` feature to ease serialization.
- Added Conversions `AnalogueCOEValue`->`COEValue` and `DigitalCOEValue`->`COEValue`.

# 0.1.0 -> 0.2.0
Added no_alloc support via the `alloc` feature.

