# GarmanKlass benchmark (`GarmanKlassVolatility` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 41.64M | 0.020 | 49.11M | 0.298 | 12.43× | 14.66× |
| 10,000 | 0.179 | 55.84M | 0.155 | 64.42M | 1.559 | 8.70× | 10.04× |
| 100,000 | 1.709 | 58.53M | 1.560 | 64.09M | 13.493 | 7.90× | 8.65× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.086 | 0.281 | 3.27× |
| 1 | 5 | 0.293 | 1.394 | 4.75× |
| 1 | 10 | 0.424 | 2.486 | 5.86× |
| 10 | 1 | 0.048 | 0.238 | 4.93× |
| 10 | 5 | 0.183 | 1.421 | 7.75× |
| 10 | 10 | 0.380 | 2.724 | 7.16× |
| 100 | 1 | 0.051 | 0.259 | 5.09× |
| 100 | 5 | 0.207 | 1.487 | 7.17× |
| 100 | 10 | 0.442 | 2.580 | 5.84× |
| 1,000 | 1 | 0.064 | 0.374 | 5.89× |
| 1,000 | 5 | 0.196 | 2.192 | 11.21× |
| 1,000 | 10 | 0.428 | 4.094 | 9.56× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
