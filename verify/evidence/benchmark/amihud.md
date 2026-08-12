# Amihud benchmark (`AmihudIlliquidity` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 88.01M | 0.010 | 96.33M | 0.546 | 48.02× | 52.56× |
| 10,000 | 0.074 | 135.72M | 0.072 | 138.88M | 4.285 | 58.16× | 59.51× |
| 100,000 | 0.683 | 146.35M | 0.660 | 151.62M | 40.919 | 59.88× | 62.04× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.093 | 0.277 | 2.98× |
| 1 | 5 | 0.295 | 1.065 | 3.61× |
| 1 | 10 | 0.469 | 2.513 | 5.36× |
| 10 | 1 | 0.053 | 0.218 | 4.10× |
| 10 | 5 | 0.259 | 1.110 | 4.28× |
| 10 | 10 | 0.504 | 2.480 | 4.92× |
| 100 | 1 | 0.056 | 0.256 | 4.57× |
| 100 | 5 | 0.274 | 1.261 | 4.60× |
| 100 | 10 | 0.497 | 2.852 | 5.74× |
| 1,000 | 1 | 0.061 | 0.642 | 10.48× |
| 1,000 | 5 | 0.247 | 3.114 | 12.62× |
| 1,000 | 10 | 0.505 | 6.605 | 13.07× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
