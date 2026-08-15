# VolumeRelativeStrengthIndex benchmark (`VolumeRsi` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 88.56M | 0.010 | 97.02M | 0.218 | 19.30× | 21.14× |
| 10,000 | 0.097 | 102.84M | 0.099 | 101.51M | 0.842 | 8.66× | 8.55× |
| 100,000 | 0.950 | 105.31M | 0.956 | 104.65M | 7.359 | 7.75× | 7.70× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.106 | 0.411 | 3.88× |
| 1 | 5 | 0.261 | 1.308 | 5.02× |
| 1 | 10 | 0.431 | 2.995 | 6.94× |
| 10 | 1 | 0.063 | 0.244 | 3.88× |
| 10 | 5 | 0.257 | 1.492 | 5.80× |
| 10 | 10 | 0.480 | 2.543 | 5.30× |
| 100 | 1 | 0.045 | 0.220 | 4.83× |
| 100 | 5 | 0.204 | 1.347 | 6.59× |
| 100 | 10 | 0.437 | 2.421 | 5.54× |
| 1,000 | 1 | 0.065 | 0.305 | 4.69× |
| 1,000 | 5 | 0.207 | 1.711 | 8.28× |
| 1,000 | 10 | 0.434 | 3.102 | 7.14× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
