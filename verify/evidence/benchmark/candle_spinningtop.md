# CandleSpinningTop benchmark (`CDLSPINNINGTOP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 69.89M | 0.011 | 91.02M | 0.032 | 2.21× | 2.88× |
| 10,000 | 0.119 | 84.20M | 0.112 | 89.03M | 0.122 | 1.03× | 1.09× |
| 100,000 | 1.174 | 85.18M | 1.143 | 87.45M | 1.062 | 0.90× | 0.93× |
| 1,000,000 | 11.703 | 85.45M | 11.643 | 85.89M | 10.430 | 0.89× | 0.90× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.096 | 0.137 | 1.42× |
| 1 | 5 | 0.318 | 0.462 | 1.45× |
| 1 | 10 | 0.541 | 0.889 | 1.64× |
| 10 | 1 | 0.051 | 0.085 | 1.66× |
| 10 | 5 | 0.253 | 0.458 | 1.81× |
| 10 | 10 | 0.529 | 0.881 | 1.66× |
| 100 | 1 | 0.057 | 0.092 | 1.61× |
| 100 | 5 | 0.254 | 0.461 | 1.82× |
| 100 | 10 | 0.594 | 0.968 | 1.63× |
| 1,000 | 1 | 0.067 | 0.099 | 1.47× |
| 1,000 | 5 | 0.261 | 0.493 | 1.89× |
| 1,000 | 10 | 0.566 | 1.062 | 1.88× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
