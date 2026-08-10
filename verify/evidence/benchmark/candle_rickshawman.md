# CandleRickshawman benchmark (`CDLRICKSHAWMAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 41.84M | 0.017 | 60.41M | 0.040 | 1.69× | 2.43× |
| 10,000 | 0.128 | 78.07M | 0.202 | 49.50M | 0.138 | 1.08× | 0.68× |
| 100,000 | 1.332 | 75.09M | 1.291 | 77.44M | 1.109 | 0.83× | 0.86× |
| 1,000,000 | 13.252 | 75.46M | 12.934 | 77.31M | 10.604 | 0.80× | 0.82× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.129 | 1.60× |
| 1 | 5 | 0.343 | 0.602 | 1.76× |
| 1 | 10 | 0.592 | 1.110 | 1.87× |
| 10 | 1 | 0.074 | 0.114 | 1.55× |
| 10 | 5 | 0.335 | 0.608 | 1.81× |
| 10 | 10 | 0.625 | 1.101 | 1.76× |
| 100 | 1 | 0.086 | 0.125 | 1.44× |
| 100 | 5 | 0.339 | 0.601 | 1.77× |
| 100 | 10 | 0.645 | 1.186 | 1.84× |
| 1,000 | 1 | 0.080 | 0.114 | 1.43× |
| 1,000 | 5 | 0.391 | 0.636 | 1.63× |
| 1,000 | 10 | 0.703 | 1.326 | 1.89× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
