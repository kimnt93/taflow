# Lag benchmark (`causal lag` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 580.59M | 0.001 | 1.07G | 0.024 | 13.84× | 25.47× |
| 10,000 | 0.007 | 1.50G | 0.004 | 2.43G | 0.029 | 4.38× | 7.06× |
| 100,000 | 0.071 | 1.42G | 0.046 | 2.17G | 0.064 | 0.90× | 1.39× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.068 | 0.091 | 1.34× |
| 1 | 5 | 0.197 | 0.419 | 2.12× |
| 1 | 10 | 0.406 | 0.836 | 2.06× |
| 10 | 1 | 0.044 | 0.081 | 1.86× |
| 10 | 5 | 0.167 | 0.412 | 2.47× |
| 10 | 10 | 0.375 | 0.856 | 2.28× |
| 100 | 1 | 0.040 | 0.093 | 2.35× |
| 100 | 5 | 0.199 | 0.399 | 2.00× |
| 100 | 10 | 0.369 | 0.852 | 2.31× |
| 1,000 | 1 | 0.040 | 0.078 | 1.94× |
| 1,000 | 5 | 0.173 | 0.410 | 2.37× |
| 1,000 | 10 | 0.381 | 0.892 | 2.34× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
