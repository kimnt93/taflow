# VolumeWeightedMovingAverageConvergenceDivergence benchmark (`VolumeWeightedMacd` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.025 | 39.91M | 0.023 | 43.58M | 0.663 | 26.47× | 28.90× |
| 10,000 | 0.191 | 52.38M | 0.187 | 53.50M | 4.567 | 23.92× | 24.43× |
| 100,000 | 1.958 | 51.07M | 1.839 | 54.37M | 47.657 | 24.34× | 25.91× |
| 1,000,000 | 20.547 | 48.67M | 18.899 | 52.91M | 518.034 | 25.21× | 27.41× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.079 | 0.359 | 4.57× |
| 1 | 5 | 0.303 | 1.654 | 5.45× |
| 1 | 10 | 0.500 | 3.474 | 6.95× |
| 10 | 1 | 0.056 | 0.306 | 5.43× |
| 10 | 5 | 0.280 | 1.634 | 5.84× |
| 10 | 10 | 0.563 | 3.430 | 6.10× |
| 100 | 1 | 0.057 | 0.333 | 5.82× |
| 100 | 5 | 0.268 | 1.978 | 7.37× |
| 100 | 10 | 0.603 | 12.667 | 21.01× |
| 1,000 | 1 | 0.089 | 0.842 | 9.45× |
| 1,000 | 5 | 0.326 | 4.362 | 13.39× |
| 1,000 | 10 | 0.669 | 8.415 | 12.57× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
