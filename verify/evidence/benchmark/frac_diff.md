# FracDiff benchmark (`fixed-width fractional differencing` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.072 | 13.89M | 0.073 | 13.70M | 0.285 | 3.95× | 3.90× |
| 10,000 | 7.247 | 1.38M | 7.176 | 1.39M | 7.519 | 1.04× | 1.05× |
| 100,000 | 78.958 | 1.27M | 80.422 | 1.24M | 85.160 | 1.08× | 1.06× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.122 | 0.345 | 2.83× |
| 1 | 5 | 0.361 | 1.397 | 3.87× |
| 1 | 10 | 0.548 | 2.770 | 5.06× |
| 10 | 1 | 0.058 | 0.288 | 4.92× |
| 10 | 5 | 0.268 | 1.372 | 5.12× |
| 10 | 10 | 0.566 | 2.781 | 4.91× |
| 100 | 1 | 0.057 | 0.280 | 4.95× |
| 100 | 5 | 0.268 | 1.380 | 5.16× |
| 100 | 10 | 0.549 | 2.800 | 5.10× |
| 1,000 | 1 | 0.126 | 0.379 | 3.00× |
| 1,000 | 5 | 0.297 | 1.885 | 6.34× |
| 1,000 | 10 | 0.643 | 3.809 | 5.92× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
