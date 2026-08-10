# KaufmanAdaptiveMovingAverage benchmark (`KAMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 162.52M | 0.006 | 163.86M | 0.033 | 5.36× | 5.41× |
| 10,000 | 0.033 | 300.46M | 0.031 | 319.06M | 0.061 | 1.84× | 1.96× |
| 100,000 | 0.304 | 328.88M | 0.291 | 344.03M | 0.312 | 1.03× | 1.07× |
| 1,000,000 | 3.435 | 291.12M | 2.919 | 342.58M | 3.141 | 0.91× | 1.08× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.108 | 1.31× |
| 1 | 5 | 0.265 | 0.489 | 1.85× |
| 1 | 10 | 0.531 | 1.054 | 1.98× |
| 10 | 1 | 0.053 | 0.092 | 1.73× |
| 10 | 5 | 0.233 | 0.492 | 2.11× |
| 10 | 10 | 0.479 | 1.017 | 2.12× |
| 100 | 1 | 0.060 | 0.100 | 1.66× |
| 100 | 5 | 0.242 | 0.475 | 1.96× |
| 100 | 10 | 0.459 | 0.930 | 2.03× |
| 1,000 | 1 | 0.050 | 0.095 | 1.89× |
| 1,000 | 5 | 0.241 | 0.480 | 1.99× |
| 1,000 | 10 | 0.489 | 0.979 | 2.00× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
