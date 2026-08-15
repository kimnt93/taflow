# CumulativeVolumeIndex benchmark (`CumulativeVolumeIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 206.21M | 0.003 | 289.26M | 4.206 | 867.37× | 1216.71× |
| 10,000 | 0.029 | 349.95M | 0.025 | 397.88M | 41.565 | 1454.57× | 1653.78× |
| 100,000 | 0.248 | 402.54M | 0.233 | 429.94M | 406.649 | 1636.93× | 1748.33× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.263 | 3.18× |
| 1 | 5 | 0.218 | 1.285 | 5.89× |
| 1 | 10 | 0.409 | 2.092 | 5.11× |
| 10 | 1 | 0.049 | 0.235 | 4.76× |
| 10 | 5 | 0.186 | 1.191 | 6.41× |
| 10 | 10 | 0.381 | 2.642 | 6.93× |
| 100 | 1 | 0.045 | 0.631 | 14.17× |
| 100 | 5 | 0.192 | 3.051 | 15.92× |
| 100 | 10 | 0.384 | 6.717 | 17.51× |
| 1,000 | 1 | 0.052 | 4.429 | 85.72× |
| 1,000 | 5 | 0.280 | 22.671 | 80.95× |
| 1,000 | 10 | 0.537 | 45.119 | 84.04× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
