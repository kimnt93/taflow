# CandleConcealBabySwall benchmark (`CDLCONCEALBABYSWALL` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.94M | 0.017 | 58.89M | 0.036 | 1.79× | 2.11× |
| 10,000 | 0.130 | 76.82M | 0.130 | 77.09M | 0.096 | 0.73× | 0.74× |
| 100,000 | 1.305 | 76.63M | 1.313 | 76.15M | 0.676 | 0.52× | 0.51× |
| 1,000,000 | 13.314 | 75.11M | 12.785 | 78.21M | 6.883 | 0.52× | 0.54× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.119 | 0.120 | 1.01× |
| 1 | 5 | 0.336 | 0.530 | 1.58× |
| 1 | 10 | 0.617 | 1.155 | 1.87× |
| 10 | 1 | 0.072 | 0.116 | 1.61× |
| 10 | 5 | 0.333 | 0.496 | 1.49× |
| 10 | 10 | 0.643 | 1.103 | 1.72× |
| 100 | 1 | 0.062 | 0.090 | 1.46× |
| 100 | 5 | 0.273 | 0.522 | 1.91× |
| 100 | 10 | 0.631 | 0.992 | 1.57× |
| 1,000 | 1 | 0.073 | 0.095 | 1.29× |
| 1,000 | 5 | 0.281 | 0.473 | 1.68× |
| 1,000 | 10 | 0.605 | 1.094 | 1.81× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
