# TripleExponentialRateOfChange benchmark (`TRIX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 161.04M | 0.007 | 141.54M | 0.042 | 6.81× | 5.99× |
| 10,000 | 0.030 | 328.45M | 0.033 | 306.26M | 0.133 | 4.37× | 4.08× |
| 100,000 | 0.270 | 370.17M | 0.245 | 408.51M | 1.042 | 3.86× | 4.26× |
| 1,000,000 | 3.213 | 311.24M | 2.543 | 393.18M | 11.487 | 3.58× | 4.52× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.099 | 0.133 | 1.34× |
| 1 | 5 | 0.362 | 1.476 | 4.08× |
| 1 | 10 | 0.864 | 1.134 | 1.31× |
| 10 | 1 | 0.061 | 0.094 | 1.55× |
| 10 | 5 | 0.242 | 0.475 | 1.96× |
| 10 | 10 | 0.548 | 0.973 | 1.77× |
| 100 | 1 | 0.050 | 0.092 | 1.84× |
| 100 | 5 | 0.224 | 0.473 | 2.11× |
| 100 | 10 | 0.580 | 1.171 | 2.02× |
| 1,000 | 1 | 0.057 | 0.107 | 1.88× |
| 1,000 | 5 | 0.279 | 0.515 | 1.85× |
| 1,000 | 10 | 0.566 | 1.159 | 2.05× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
