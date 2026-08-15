# DemandIndex benchmark (`DemandIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 120.17M | 0.005 | 186.00M | 0.271 | 32.59× | 50.44× |
| 10,000 | 0.044 | 229.82M | 0.039 | 253.44M | 1.400 | 32.18× | 35.49× |
| 100,000 | 0.403 | 248.09M | 0.382 | 262.08M | 12.838 | 31.85× | 33.64× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.115 | 0.271 | 2.35× |
| 1 | 5 | 0.313 | 1.093 | 3.49× |
| 1 | 10 | 0.430 | 2.455 | 5.71× |
| 10 | 1 | 0.046 | 0.214 | 4.71× |
| 10 | 5 | 0.202 | 1.090 | 5.39× |
| 10 | 10 | 0.441 | 2.334 | 5.29× |
| 100 | 1 | 0.053 | 0.250 | 4.67× |
| 100 | 5 | 0.220 | 1.424 | 6.48× |
| 100 | 10 | 0.415 | 2.461 | 5.93× |
| 1,000 | 1 | 0.049 | 0.343 | 6.99× |
| 1,000 | 5 | 0.213 | 1.889 | 8.87× |
| 1,000 | 10 | 0.451 | 3.691 | 8.19× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
