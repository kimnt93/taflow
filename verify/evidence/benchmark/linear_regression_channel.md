# LinearRegressionChannel benchmark (`LinRegChannel` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.072 | 13.97M | 0.084 | 11.86M | 0.605 | 8.46× | 7.18× |
| 10,000 | 0.690 | 14.50M | 0.709 | 14.11M | 4.354 | 6.31× | 6.14× |
| 100,000 | 7.824 | 12.78M | 6.941 | 14.41M | 48.868 | 6.25× | 7.04× |
| 1,000,000 | 74.250 | 13.47M | 77.021 | 12.98M | 532.728 | 7.17× | 6.92× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.061 | 0.296 | 4.85× |
| 1 | 5 | 0.252 | 1.490 | 5.90× |
| 1 | 10 | 0.469 | 2.627 | 5.61× |
| 10 | 1 | 0.074 | 0.292 | 3.96× |
| 10 | 5 | 0.257 | 1.556 | 6.06× |
| 10 | 10 | 0.522 | 2.933 | 5.62× |
| 100 | 1 | 0.066 | 0.306 | 4.61× |
| 100 | 5 | 0.289 | 1.933 | 6.69× |
| 100 | 10 | 0.621 | 3.247 | 5.23× |
| 1,000 | 1 | 0.140 | 0.903 | 6.45× |
| 1,000 | 5 | 0.309 | 4.001 | 12.94× |
| 1,000 | 10 | 0.637 | 13.887 | 21.80× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
