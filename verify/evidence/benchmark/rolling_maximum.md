# RollingMaximum benchmark (`MAX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 237.98M | 0.003 | 312.54M | 0.037 | 8.76× | 11.50× |
| 10,000 | 0.025 | 398.35M | 0.022 | 446.75M | 0.091 | 3.64× | 4.08× |
| 100,000 | 0.228 | 437.71M | 0.203 | 493.56M | 0.502 | 2.20× | 2.48× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.096 | 0.120 | 1.25× |
| 1 | 5 | 0.280 | 0.458 | 1.64× |
| 1 | 10 | 0.373 | 0.906 | 2.43× |
| 10 | 1 | 0.039 | 0.093 | 2.36× |
| 10 | 5 | 0.172 | 0.449 | 2.61× |
| 10 | 10 | 0.423 | 0.969 | 2.29× |
| 100 | 1 | 0.044 | 0.091 | 2.07× |
| 100 | 5 | 0.189 | 0.438 | 2.31× |
| 100 | 10 | 0.399 | 1.015 | 2.55× |
| 1,000 | 1 | 0.045 | 0.103 | 2.27× |
| 1,000 | 5 | 0.186 | 0.455 | 2.44× |
| 1,000 | 10 | 0.399 | 1.002 | 2.51× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
