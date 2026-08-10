# FisherTransform benchmark (`fisher` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.042 | 23.87M | 0.038 | 26.45M | 1.326 | 31.65× | 35.07× |
| 10,000 | 0.579 | 17.26M | 0.462 | 21.63M | 1.775 | 3.06× | 3.84× |
| 100,000 | 3.933 | 25.43M | 3.621 | 27.62M | 6.403 | 1.63× | 1.77× |
| 1,000,000 | 39.657 | 25.22M | 37.280 | 26.82M | 69.273 | 1.75× | 1.86× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.196 | 0.269 | 1.37× |
| 1 | 5 | 0.331 | 0.919 | 2.78× |
| 1 | 10 | 0.513 | 2.064 | 4.03× |
| 10 | 1 | 0.064 | 1.432 | 22.38× |
| 10 | 5 | 0.248 | 6.593 | 26.57× |
| 10 | 10 | 0.509 | 13.807 | 27.14× |
| 100 | 1 | 0.060 | 1.296 | 21.62× |
| 100 | 5 | 0.249 | 6.417 | 25.81× |
| 100 | 10 | 0.655 | 13.442 | 20.51× |
| 1,000 | 1 | 0.094 | 1.370 | 14.53× |
| 1,000 | 5 | 0.261 | 7.003 | 26.85× |
| 1,000 | 10 | 0.528 | 25.415 | 48.16× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
