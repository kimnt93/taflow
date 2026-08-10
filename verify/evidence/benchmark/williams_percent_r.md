# WilliamsPercentR benchmark (`WILLR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.027 | 37.51M | 0.025 | 39.77M | 0.036 | 1.34× | 1.42× |
| 10,000 | 0.281 | 35.59M | 0.271 | 36.90M | 0.117 | 0.42× | 0.43× |
| 100,000 | 2.718 | 36.80M | 2.772 | 36.07M | 0.827 | 0.30× | 0.30× |
| 1,000,000 | 29.194 | 34.25M | 27.615 | 36.21M | 8.515 | 0.29× | 0.31× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.108 | 1.24× |
| 1 | 5 | 0.254 | 0.464 | 1.83× |
| 1 | 10 | 0.538 | 0.977 | 1.82× |
| 10 | 1 | 0.060 | 0.086 | 1.44× |
| 10 | 5 | 0.225 | 0.455 | 2.02× |
| 10 | 10 | 0.507 | 0.972 | 1.92× |
| 100 | 1 | 0.054 | 0.098 | 1.81× |
| 100 | 5 | 0.256 | 0.455 | 1.77× |
| 100 | 10 | 0.546 | 0.974 | 1.78× |
| 1,000 | 1 | 0.077 | 0.102 | 1.33× |
| 1,000 | 5 | 0.285 | 0.535 | 1.88× |
| 1,000 | 10 | 0.546 | 1.059 | 1.94× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
