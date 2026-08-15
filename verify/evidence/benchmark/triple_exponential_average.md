# TripleExponentialAverage benchmark (`T3` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 165.15M | 0.005 | 203.37M | 0.040 | 6.58× | 8.10× |
| 10,000 | 0.038 | 264.98M | 0.036 | 276.02M | 0.076 | 2.01× | 2.09× |
| 100,000 | 0.361 | 277.02M | 0.342 | 292.78M | 0.433 | 1.20× | 1.27× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.135 | 0.134 | 0.99× |
| 1 | 5 | 0.281 | 0.501 | 1.78× |
| 1 | 10 | 0.408 | 0.976 | 2.39× |
| 10 | 1 | 0.041 | 0.095 | 2.29× |
| 10 | 5 | 0.180 | 0.447 | 2.49× |
| 10 | 10 | 0.395 | 0.996 | 2.52× |
| 100 | 1 | 0.052 | 0.095 | 1.82× |
| 100 | 5 | 0.182 | 0.460 | 2.52× |
| 100 | 10 | 0.417 | 0.951 | 2.28× |
| 1,000 | 1 | 0.049 | 0.099 | 2.01× |
| 1,000 | 5 | 0.204 | 0.503 | 2.47× |
| 1,000 | 10 | 0.445 | 1.009 | 2.27× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
