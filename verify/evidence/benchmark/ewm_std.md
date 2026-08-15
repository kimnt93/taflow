# ExponentiallyWeightedStandardDeviation benchmark (`ewm standard deviation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 142.66M | 0.006 | 170.91M | 1.236 | 176.33× | 211.24× |
| 10,000 | 0.047 | 211.08M | 0.047 | 212.70M | 14.840 | 313.24× | 315.64× |
| 100,000 | 0.631 | 158.49M | 0.565 | 177.01M | 123.780 | 196.18× | 219.10× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.170 | 0.127 | 0.75× |
| 1 | 5 | 0.278 | 0.438 | 1.58× |
| 1 | 10 | 0.393 | 0.863 | 2.20× |
| 10 | 1 | 0.042 | 0.100 | 2.41× |
| 10 | 5 | 0.179 | 0.528 | 2.96× |
| 10 | 10 | 0.422 | 0.987 | 2.34× |
| 100 | 1 | 0.043 | 0.207 | 4.85× |
| 100 | 5 | 0.189 | 1.023 | 5.41× |
| 100 | 10 | 0.411 | 2.156 | 5.24× |
| 1,000 | 1 | 0.047 | 1.338 | 28.51× |
| 1,000 | 5 | 0.212 | 6.760 | 31.82× |
| 1,000 | 10 | 0.423 | 13.614 | 32.20× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
