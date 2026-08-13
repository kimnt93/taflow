# MathCos benchmark (`COS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.036 | 27.73M | 0.031 | 32.01M | 0.037 | 1.04× | 1.20× |
| 10,000 | 0.259 | 38.56M | 0.258 | 38.81M | 0.166 | 0.64× | 0.64× |
| 100,000 | 2.537 | 39.41M | 2.562 | 39.02M | 1.429 | 0.56× | 0.56× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.134 | 0.134 | 1.01× |
| 1 | 5 | 0.392 | 0.440 | 1.12× |
| 1 | 10 | 0.579 | 0.859 | 1.48× |
| 10 | 1 | 0.059 | 0.088 | 1.48× |
| 10 | 5 | 0.280 | 0.406 | 1.45× |
| 10 | 10 | 0.592 | 0.911 | 1.54× |
| 100 | 1 | 0.061 | 0.086 | 1.41× |
| 100 | 5 | 0.278 | 0.428 | 1.54× |
| 100 | 10 | 0.597 | 0.886 | 1.48× |
| 1,000 | 1 | 0.088 | 0.101 | 1.14× |
| 1,000 | 5 | 0.281 | 0.514 | 1.83× |
| 1,000 | 10 | 0.584 | 1.032 | 1.77× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
