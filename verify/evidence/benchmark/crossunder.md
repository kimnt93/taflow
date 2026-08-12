# Crossunder benchmark (`causal crossunder` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 128.92M | 0.006 | 155.17M | 0.017 | 2.18× | 2.63× |
| 10,000 | 0.037 | 269.58M | 0.035 | 289.23M | 0.031 | 0.83× | 0.89× |
| 100,000 | 0.323 | 309.77M | 0.314 | 318.46M | 0.140 | 0.43× | 0.45× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.102 | 0.097 | 0.94× |
| 1 | 5 | 0.374 | 0.330 | 0.88× |
| 1 | 10 | 0.446 | 0.674 | 1.51× |
| 10 | 1 | 0.056 | 0.071 | 1.26× |
| 10 | 5 | 0.222 | 0.359 | 1.62× |
| 10 | 10 | 0.471 | 0.732 | 1.55× |
| 100 | 1 | 0.048 | 0.062 | 1.28× |
| 100 | 5 | 0.232 | 0.322 | 1.39× |
| 100 | 10 | 0.518 | 0.731 | 1.41× |
| 1,000 | 1 | 0.052 | 0.071 | 1.35× |
| 1,000 | 5 | 0.258 | 0.409 | 1.59× |
| 1,000 | 10 | 0.515 | 0.933 | 1.81× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
