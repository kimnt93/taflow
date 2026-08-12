# RollingVolumeWeightedAveragePrice benchmark (`RollingVWAP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.031 | 32.43M | 0.028 | 36.35M | 0.262 | 8.49× | 9.52× |
| 10,000 | 0.224 | 44.71M | 0.225 | 44.42M | 1.435 | 6.42× | 6.37× |
| 100,000 | 2.194 | 45.59M | 2.247 | 44.51M | 13.090 | 5.97× | 5.83× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.098 | 0.248 | 2.52× |
| 1 | 5 | 0.412 | 1.488 | 3.62× |
| 1 | 10 | 0.550 | 2.446 | 4.45× |
| 10 | 1 | 0.056 | 0.207 | 3.73× |
| 10 | 5 | 0.279 | 1.137 | 4.08× |
| 10 | 10 | 0.586 | 2.596 | 4.43× |
| 100 | 1 | 0.065 | 0.242 | 3.70× |
| 100 | 5 | 0.266 | 1.132 | 4.25× |
| 100 | 10 | 0.600 | 2.595 | 4.32× |
| 1,000 | 1 | 0.080 | 0.346 | 4.33× |
| 1,000 | 5 | 0.300 | 1.709 | 5.70× |
| 1,000 | 10 | 0.606 | 3.932 | 6.49× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
