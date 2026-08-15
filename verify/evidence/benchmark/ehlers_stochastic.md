# EhlersStochastic benchmark (`EhlersStochastic` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.033 | 30.68M | 0.032 | 31.30M | 0.185 | 5.68× | 5.80× |
| 10,000 | 0.313 | 31.95M | 0.311 | 32.18M | 0.772 | 2.47× | 2.48× |
| 100,000 | 3.260 | 30.67M | 3.080 | 32.47M | 6.907 | 2.12× | 2.24× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.069 | 0.208 | 3.04× |
| 1 | 5 | 0.291 | 1.041 | 3.57× |
| 1 | 10 | 0.399 | 2.143 | 5.37× |
| 10 | 1 | 0.045 | 0.233 | 5.17× |
| 10 | 5 | 0.227 | 0.945 | 4.16× |
| 10 | 10 | 0.413 | 2.183 | 5.28× |
| 100 | 1 | 0.051 | 0.205 | 4.02× |
| 100 | 5 | 0.203 | 0.954 | 4.71× |
| 100 | 10 | 0.402 | 2.186 | 5.44× |
| 1,000 | 1 | 0.077 | 0.260 | 3.37× |
| 1,000 | 5 | 0.242 | 1.268 | 5.23× |
| 1,000 | 10 | 0.424 | 2.847 | 6.72× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
