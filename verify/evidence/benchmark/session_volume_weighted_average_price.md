# SessionVolumeWeightedAveragePrice benchmark (`SessionVwap` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 54.12M | 0.015 | 66.20M | 0.409 | 22.12× | 27.06× |
| 10,000 | 0.079 | 126.12M | 0.069 | 144.32M | 2.346 | 29.58× | 33.86× |
| 100,000 | 0.635 | 157.57M | 0.601 | 166.28M | 24.435 | 38.50× | 40.63× |
| 1,000,000 | 7.895 | 126.67M | 7.452 | 134.19M | 256.507 | 32.49× | 34.42× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.109 | 0.473 | 4.34× |
| 1 | 5 | 0.442 | 1.226 | 2.77× |
| 1 | 10 | 0.559 | 2.599 | 4.65× |
| 10 | 1 | 0.060 | 0.241 | 4.02× |
| 10 | 5 | 0.283 | 1.173 | 4.15× |
| 10 | 10 | 0.580 | 2.536 | 4.37× |
| 100 | 1 | 0.062 | 0.255 | 4.12× |
| 100 | 5 | 0.304 | 1.446 | 4.76× |
| 100 | 10 | 0.569 | 2.745 | 4.82× |
| 1,000 | 1 | 0.072 | 0.465 | 6.47× |
| 1,000 | 5 | 0.277 | 2.599 | 9.40× |
| 1,000 | 10 | 0.601 | 5.099 | 8.49× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
