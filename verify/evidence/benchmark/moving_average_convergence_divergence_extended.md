# MovingAverageConvergenceDivergenceExtended benchmark (`MACDEXT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 75.58M | 0.011 | 87.58M | 0.059 | 4.47× | 5.18× |
| 10,000 | 0.111 | 89.97M | 0.110 | 91.19M | 0.124 | 1.12× | 1.13× |
| 100,000 | 2.060 | 48.55M | 1.010 | 99.00M | 1.529 | 0.74× | 1.51× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.160 | 1.96× |
| 1 | 5 | 0.309 | 0.574 | 1.86× |
| 1 | 10 | 0.453 | 1.205 | 2.66× |
| 10 | 1 | 0.054 | 0.114 | 2.12× |
| 10 | 5 | 0.225 | 0.618 | 2.75× |
| 10 | 10 | 0.456 | 1.147 | 2.51× |
| 100 | 1 | 0.046 | 0.116 | 2.54× |
| 100 | 5 | 0.221 | 0.589 | 2.66× |
| 100 | 10 | 0.443 | 1.137 | 2.57× |
| 1,000 | 1 | 0.054 | 0.114 | 2.13× |
| 1,000 | 5 | 0.217 | 0.606 | 2.79× |
| 1,000 | 10 | 0.557 | 1.214 | 2.18× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
