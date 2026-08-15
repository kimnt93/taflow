# Retracements benchmark (`causal swing retracements` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.042 | 23.68M | 0.038 | 26.23M | 5.206 | 123.25× | 136.55× |
| 10,000 | 0.412 | 24.26M | 0.399 | 25.04M | 49.887 | 121.00× | 124.93× |
| 100,000 | 4.186 | 23.89M | 4.020 | 24.88M | 500.362 | 119.54× | 124.47× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.116 | 0.141 | 1.22× |
| 1 | 5 | 0.273 | 0.471 | 1.73× |
| 1 | 10 | 0.402 | 0.931 | 2.32× |
| 10 | 1 | 0.042 | 0.100 | 2.39× |
| 10 | 5 | 0.200 | 0.501 | 2.50× |
| 10 | 10 | 0.463 | 1.023 | 2.21× |
| 100 | 1 | 0.056 | 0.561 | 9.99× |
| 100 | 5 | 0.219 | 2.739 | 12.53× |
| 100 | 10 | 0.438 | 5.528 | 12.61× |
| 1,000 | 1 | 0.090 | 4.986 | 55.29× |
| 1,000 | 5 | 0.318 | 26.110 | 82.23× |
| 1,000 | 10 | 0.598 | 55.153 | 92.31× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
