# UlcerIndex benchmark (`UlcerIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.123 | 8.14M | 0.118 | 8.49M | 0.180 | 1.46× | 1.53× |
| 10,000 | 1.095 | 9.13M | 1.106 | 9.04M | 0.576 | 0.53× | 0.52× |
| 100,000 | 10.860 | 9.21M | 11.446 | 8.74M | 4.655 | 0.43× | 0.41× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.168 | 0.256 | 1.53× |
| 1 | 5 | 0.452 | 1.442 | 3.19× |
| 1 | 10 | 0.593 | 2.379 | 4.02× |
| 10 | 1 | 0.080 | 0.216 | 2.71× |
| 10 | 5 | 0.299 | 1.335 | 4.46× |
| 10 | 10 | 0.602 | 2.384 | 3.96× |
| 100 | 1 | 0.079 | 0.220 | 2.79× |
| 100 | 5 | 0.305 | 1.336 | 4.38× |
| 100 | 10 | 0.631 | 2.347 | 3.72× |
| 1,000 | 1 | 0.194 | 0.267 | 1.38× |
| 1,000 | 5 | 0.346 | 1.689 | 4.87× |
| 1,000 | 10 | 0.710 | 3.032 | 4.27× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
