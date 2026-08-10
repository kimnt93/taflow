# VolumeRelativeStrengthIndex benchmark (`VolumeRsi` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 80.77M | 0.011 | 87.50M | 0.220 | 17.76× | 19.24× |
| 10,000 | 0.104 | 95.89M | 0.095 | 104.91M | 0.843 | 8.08× | 8.84× |
| 100,000 | 0.955 | 104.73M | 0.970 | 103.13M | 10.513 | 11.01× | 10.84× |
| 1,000,000 | 10.058 | 99.42M | 9.144 | 109.36M | 74.794 | 7.44× | 8.18× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.103 | 0.260 | 2.52× |
| 1 | 5 | 0.406 | 1.226 | 3.02× |
| 1 | 10 | 0.462 | 2.305 | 4.99× |
| 10 | 1 | 0.048 | 0.218 | 4.54× |
| 10 | 5 | 0.235 | 1.233 | 5.26× |
| 10 | 10 | 0.475 | 2.282 | 4.81× |
| 100 | 1 | 0.054 | 0.218 | 4.01× |
| 100 | 5 | 0.239 | 1.289 | 5.39× |
| 100 | 10 | 0.473 | 2.919 | 6.18× |
| 1,000 | 1 | 0.067 | 0.313 | 4.66× |
| 1,000 | 5 | 0.310 | 1.900 | 6.12× |
| 1,000 | 10 | 0.560 | 3.260 | 5.82× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
