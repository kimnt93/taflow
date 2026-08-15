# Drawdown benchmark (`drawdown from cumulative maximum` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 176.08M | 0.005 | 201.25M | 0.023 | 4.00× | 4.57× |
| 10,000 | 0.042 | 240.10M | 0.038 | 262.21M | 0.061 | 1.48× | 1.61× |
| 100,000 | 0.393 | 254.51M | 0.357 | 280.03M | 0.442 | 1.13× | 1.24× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.056 | 0.087 | 1.55× |
| 1 | 5 | 0.258 | 0.369 | 1.43× |
| 1 | 10 | 0.394 | 0.723 | 1.84× |
| 10 | 1 | 0.050 | 0.081 | 1.64× |
| 10 | 5 | 0.195 | 0.356 | 1.82× |
| 10 | 10 | 0.397 | 0.721 | 1.81× |
| 100 | 1 | 0.046 | 0.076 | 1.67× |
| 100 | 5 | 0.187 | 0.354 | 1.89× |
| 100 | 10 | 0.449 | 0.744 | 1.66× |
| 1,000 | 1 | 0.052 | 0.078 | 1.48× |
| 1,000 | 5 | 0.186 | 0.422 | 2.27× |
| 1,000 | 10 | 0.459 | 1.043 | 2.28× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
