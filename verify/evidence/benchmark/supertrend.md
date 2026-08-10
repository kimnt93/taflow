# Supertrend benchmark (`supertrend` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 43.36M | 0.021 | 48.77M | 1.717 | 74.47× | 83.75× |
| 10,000 | 0.177 | 56.38M | 0.174 | 57.57M | 2.691 | 15.17× | 15.49× |
| 100,000 | 1.881 | 53.15M | 1.743 | 57.36M | 14.464 | 7.69× | 8.30× |
| 1,000,000 | 37.750 | 26.49M | 18.501 | 54.05M | 165.120 | 4.37× | 8.92× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.093 | 0.295 | 3.16× |
| 1 | 5 | 0.293 | 1.203 | 4.11× |
| 1 | 10 | 0.554 | 2.448 | 4.42× |
| 10 | 1 | 0.056 | 1.784 | 31.95× |
| 10 | 5 | 0.247 | 8.534 | 34.53× |
| 10 | 10 | 0.514 | 17.069 | 33.23× |
| 100 | 1 | 0.083 | 1.685 | 20.24× |
| 100 | 5 | 0.267 | 8.925 | 33.39× |
| 100 | 10 | 0.545 | 17.606 | 32.30× |
| 1,000 | 1 | 0.081 | 1.840 | 22.81× |
| 1,000 | 5 | 0.264 | 9.577 | 36.22× |
| 1,000 | 10 | 0.573 | 19.627 | 34.23× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
