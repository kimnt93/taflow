# Parkinson benchmark (`ParkinsonVolatility` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 54.04M | 0.017 | 58.64M | 0.228 | 12.30× | 13.35× |
| 10,000 | 0.137 | 72.99M | 0.133 | 75.19M | 0.841 | 6.14× | 6.33× |
| 100,000 | 1.274 | 78.47M | 1.298 | 77.01M | 6.979 | 5.48× | 5.37× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.120 | 0.273 | 2.28× |
| 1 | 5 | 0.273 | 1.397 | 5.11× |
| 1 | 10 | 0.484 | 2.675 | 5.52× |
| 10 | 1 | 0.053 | 0.234 | 4.42× |
| 10 | 5 | 0.238 | 1.425 | 5.98× |
| 10 | 10 | 0.494 | 2.496 | 5.05× |
| 100 | 1 | 0.053 | 0.246 | 4.62× |
| 100 | 5 | 0.244 | 1.495 | 6.12× |
| 100 | 10 | 0.515 | 2.900 | 5.63× |
| 1,000 | 1 | 0.068 | 0.313 | 4.60× |
| 1,000 | 5 | 0.247 | 1.842 | 7.46× |
| 1,000 | 10 | 0.520 | 3.238 | 6.23× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
