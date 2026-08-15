# Amihud benchmark (`AmihudIlliquidity` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 106.79M | 0.008 | 123.12M | 0.541 | 57.79× | 66.63× |
| 10,000 | 0.072 | 139.36M | 0.068 | 146.98M | 3.995 | 55.68× | 58.72× |
| 100,000 | 0.674 | 148.34M | 0.653 | 153.08M | 38.466 | 57.06× | 58.88× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.056 | 0.273 | 4.91× |
| 1 | 5 | 0.239 | 1.069 | 4.48× |
| 1 | 10 | 0.398 | 2.459 | 6.19× |
| 10 | 1 | 0.048 | 0.216 | 4.45× |
| 10 | 5 | 0.191 | 1.060 | 5.55× |
| 10 | 10 | 0.411 | 2.572 | 6.25× |
| 100 | 1 | 0.048 | 0.257 | 5.39× |
| 100 | 5 | 0.196 | 1.239 | 6.33× |
| 100 | 10 | 0.430 | 2.862 | 6.65× |
| 1,000 | 1 | 0.051 | 0.604 | 11.76× |
| 1,000 | 5 | 0.199 | 2.993 | 15.04× |
| 1,000 | 10 | 0.427 | 6.422 | 15.05× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
