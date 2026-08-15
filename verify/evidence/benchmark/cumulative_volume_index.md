# CumulativeVolumeIndex benchmark (`CumulativeVolumeIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 195.40M | 0.004 | 279.11M | 4.391 | 858.01× | 1225.59× |
| 10,000 | 0.030 | 336.13M | 0.025 | 406.25M | 41.069 | 1380.49× | 1668.43× |
| 100,000 | 0.253 | 396.02M | 0.228 | 439.13M | 410.508 | 1625.71× | 1802.65× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.103 | 0.285 | 2.77× |
| 1 | 5 | 0.206 | 1.299 | 6.32× |
| 1 | 10 | 0.408 | 2.189 | 5.36× |
| 10 | 1 | 0.045 | 0.232 | 5.14× |
| 10 | 5 | 0.196 | 1.179 | 6.02× |
| 10 | 10 | 0.389 | 2.694 | 6.92× |
| 100 | 1 | 0.053 | 0.618 | 11.69× |
| 100 | 5 | 0.191 | 3.180 | 16.67× |
| 100 | 10 | 0.409 | 6.496 | 15.89× |
| 1,000 | 1 | 0.053 | 4.479 | 84.25× |
| 1,000 | 5 | 0.449 | 24.537 | 54.67× |
| 1,000 | 10 | 0.486 | 50.541 | 104.04× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
