# SessionVolumeWeightedAveragePrice benchmark (`SessionVwap` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.033 | 30.48M | 0.014 | 70.92M | 0.373 | 11.37× | 26.45× |
| 10,000 | 0.078 | 128.20M | 0.072 | 138.34M | 2.439 | 31.26× | 33.74× |
| 100,000 | 0.674 | 148.42M | 0.674 | 148.27M | 22.729 | 33.74× | 33.70× |
| 1,000,000 | 6.951 | 143.85M | 6.886 | 145.23M | 256.763 | 36.94× | 37.29× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.100 | 0.481 | 4.82× |
| 1 | 5 | 0.499 | 1.210 | 2.43× |
| 1 | 10 | 0.603 | 2.709 | 4.49× |
| 10 | 1 | 0.062 | 0.240 | 3.84× |
| 10 | 5 | 0.283 | 1.238 | 4.38× |
| 10 | 10 | 0.592 | 2.438 | 4.12× |
| 100 | 1 | 0.070 | 0.254 | 3.64× |
| 100 | 5 | 0.275 | 1.434 | 5.22× |
| 100 | 10 | 0.586 | 2.716 | 4.63× |
| 1,000 | 1 | 0.074 | 0.462 | 6.28× |
| 1,000 | 5 | 0.281 | 2.487 | 8.86× |
| 1,000 | 10 | 0.619 | 4.807 | 7.77× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
