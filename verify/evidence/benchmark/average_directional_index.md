# AverageDirectionalIndex benchmark (`ADX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.061 | 16.46M | 0.053 | 18.70M | 0.039 | 0.65× | 0.74× |
| 10,000 | 0.459 | 21.79M | 0.428 | 23.39M | 0.119 | 0.26× | 0.28× |
| 100,000 | 4.379 | 22.84M | 4.265 | 23.45M | 0.951 | 0.22× | 0.22× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.126 | 0.113 | 0.90× |
| 1 | 5 | 0.391 | 0.477 | 1.22× |
| 1 | 10 | 0.727 | 0.983 | 1.35× |
| 10 | 1 | 0.065 | 0.093 | 1.43× |
| 10 | 5 | 0.307 | 0.448 | 1.46× |
| 10 | 10 | 0.632 | 0.956 | 1.51× |
| 100 | 1 | 0.079 | 0.094 | 1.19× |
| 100 | 5 | 0.304 | 0.445 | 1.46× |
| 100 | 10 | 0.635 | 0.966 | 1.52× |
| 1,000 | 1 | 0.118 | 0.108 | 0.91× |
| 1,000 | 5 | 0.317 | 0.502 | 1.59× |
| 1,000 | 10 | 0.680 | 1.041 | 1.53× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
