# CumulativeSumControlChart benchmark (`CUSUM event filter` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 149.16M | 0.006 | 166.44M | 0.525 | 78.27× | 87.34× |
| 10,000 | 0.042 | 240.69M | 0.037 | 269.33M | 5.056 | 121.69× | 136.17× |
| 100,000 | 0.411 | 243.42M | 0.377 | 265.05M | 51.589 | 125.58× | 136.74× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.122 | 1.47× |
| 1 | 5 | 0.236 | 0.412 | 1.74× |
| 1 | 10 | 0.442 | 0.849 | 1.92× |
| 10 | 1 | 0.049 | 0.090 | 1.85× |
| 10 | 5 | 0.211 | 0.438 | 2.08× |
| 10 | 10 | 0.502 | 0.930 | 1.86× |
| 100 | 1 | 0.054 | 0.135 | 2.51× |
| 100 | 5 | 0.264 | 0.688 | 2.60× |
| 100 | 10 | 0.474 | 1.414 | 2.98× |
| 1,000 | 1 | 0.058 | 0.603 | 10.31× |
| 1,000 | 5 | 0.240 | 2.999 | 12.47× |
| 1,000 | 10 | 0.492 | 6.126 | 12.44× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
