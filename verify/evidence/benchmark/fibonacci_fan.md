# FibonacciFan benchmark (`FibFan` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 56.45M | 0.016 | 61.10M | 0.502 | 28.31× | 30.64× |
| 10,000 | 0.139 | 71.81M | 0.130 | 76.80M | 3.820 | 27.43× | 29.33× |
| 100,000 | 1.325 | 75.47M | 1.278 | 78.28M | 39.755 | 30.00× | 31.12× |
| 1,000,000 | 15.023 | 66.56M | 13.015 | 76.83M | 441.768 | 29.41× | 33.94× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.260 | 2.68× |
| 1 | 5 | 0.346 | 0.990 | 2.86× |
| 1 | 10 | 0.543 | 1.905 | 3.51× |
| 10 | 1 | 0.051 | 0.171 | 3.34× |
| 10 | 5 | 0.241 | 0.851 | 3.54× |
| 10 | 10 | 0.476 | 1.924 | 4.04× |
| 100 | 1 | 0.057 | 0.223 | 3.92× |
| 100 | 5 | 0.253 | 1.031 | 4.07× |
| 100 | 10 | 0.555 | 2.484 | 4.48× |
| 1,000 | 1 | 0.076 | 0.756 | 9.95× |
| 1,000 | 5 | 0.284 | 3.140 | 11.04× |
| 1,000 | 10 | 0.526 | 6.206 | 11.79× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
