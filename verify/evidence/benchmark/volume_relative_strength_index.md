# VolumeRelativeStrengthIndex benchmark (`VolumeRsi` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 88.19M | 0.010 | 97.53M | 0.212 | 18.68× | 20.66× |
| 10,000 | 0.096 | 104.26M | 0.094 | 106.78M | 0.837 | 8.73× | 8.94× |
| 100,000 | 0.928 | 107.77M | 0.945 | 105.80M | 7.101 | 7.65× | 7.51× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.163 | 0.281 | 1.72× |
| 1 | 5 | 0.288 | 1.186 | 4.11× |
| 1 | 10 | 0.402 | 2.298 | 5.71× |
| 10 | 1 | 0.041 | 0.218 | 5.25× |
| 10 | 5 | 0.206 | 1.373 | 6.66× |
| 10 | 10 | 0.385 | 2.261 | 5.88× |
| 100 | 1 | 0.049 | 0.242 | 4.91× |
| 100 | 5 | 0.191 | 1.353 | 7.07× |
| 100 | 10 | 0.409 | 2.423 | 5.93× |
| 1,000 | 1 | 0.056 | 0.298 | 5.29× |
| 1,000 | 5 | 0.199 | 1.675 | 8.43× |
| 1,000 | 10 | 0.413 | 3.147 | 7.63× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
