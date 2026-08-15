# Retracements benchmark (`causal swing retracements` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.052 | 19.22M | 0.036 | 27.83M | 5.143 | 98.86× | 143.13× |
| 10,000 | 0.404 | 24.74M | 0.395 | 25.31M | 49.007 | 121.26× | 124.05× |
| 100,000 | 4.017 | 24.90M | 3.820 | 26.18M | 477.331 | 118.83× | 124.95× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.093 | 0.137 | 1.47× |
| 1 | 5 | 0.207 | 0.496 | 2.39× |
| 1 | 10 | 0.433 | 0.937 | 2.16× |
| 10 | 1 | 0.047 | 0.099 | 2.13× |
| 10 | 5 | 0.191 | 0.469 | 2.46× |
| 10 | 10 | 0.422 | 0.971 | 2.30× |
| 100 | 1 | 0.052 | 0.554 | 10.65× |
| 100 | 5 | 0.200 | 2.755 | 13.80× |
| 100 | 10 | 0.447 | 5.386 | 12.05× |
| 1,000 | 1 | 0.089 | 5.111 | 57.23× |
| 1,000 | 5 | 0.309 | 26.779 | 86.57× |
| 1,000 | 10 | 0.599 | 73.319 | 122.36× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
