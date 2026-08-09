# HeikinAshi benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 70.89M | 0.013 | 78.87M | 51.945 | 3682.43× | 4097.09× |
| 10,000 | 0.073 | 136.87M | 0.061 | 163.61M | 468.993 | 6419.00× | 7673.35× |
| 100,000 | 0.707 | 141.41M | 0.621 | 161.10M | 4932.081 | 6974.68× | 7945.52× |
| 1,000,000 | 24.379 | 41.02M | 13.049 | 76.63M | 47473.919 | 1947.31× | 3638.04× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.092 | 2.764 | 30.13× |
| 1 | 5 | 0.329 | 12.738 | 38.75× |
| 1 | 10 | 0.543 | 26.127 | 48.14× |
| 10 | 1 | 0.060 | 2.970 | 49.60× |
| 10 | 5 | 0.285 | 15.676 | 55.08× |
| 10 | 10 | 0.546 | 31.503 | 57.69× |
| 100 | 1 | 0.060 | 7.236 | 120.41× |
| 100 | 5 | 0.389 | 38.575 | 99.08× |
| 100 | 10 | 0.566 | 77.914 | 137.72× |
| 1,000 | 1 | 0.093 | 48.194 | 517.02× |
| 1,000 | 5 | 0.490 | 259.166 | 528.89× |
| 1,000 | 10 | 0.626 | 524.791 | 838.87× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | Reference full µs | vs full |
|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.591 | 0.412 | 2.42M | 4722852.420 | 11450235.39× |
| 100,000 | 10 | 3.889 | 2.142 | 4.67M | 4660698.517 | 2176299.87× |
| 100,000 | 1,000 | 126.741 | 101.432 | 9.86M | 4728897.712 | 46621.42× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | Reference vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 65.84M | 106.11M | 1.00× | 1.32M | 1.27M | 1.00× | 21.08K |
| 5 | 116.98M | 274.72M | 2.59× | 982.27K | 1.07M | 0.85× | 20.42K |
| 10 | 139.00M | 362.79M | 3.42× | 984.66K | 1.09M | 0.86× | 20.21K |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
