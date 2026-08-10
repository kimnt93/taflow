# DemandIndex benchmark (`DemandIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 73.16M | 0.010 | 98.61M | 0.257 | 18.83× | 25.38× |
| 10,000 | 0.060 | 167.53M | 0.054 | 185.10M | 1.339 | 22.42× | 24.78× |
| 100,000 | 0.518 | 192.90M | 0.490 | 204.14M | 12.742 | 24.58× | 26.01× |
| 1,000,000 | 6.813 | 146.77M | 5.440 | 183.84M | 137.016 | 20.11× | 25.19× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.090 | 0.455 | 5.03× |
| 1 | 5 | 0.410 | 1.211 | 2.95× |
| 1 | 10 | 0.517 | 2.291 | 4.43× |
| 10 | 1 | 0.059 | 0.225 | 3.79× |
| 10 | 5 | 0.261 | 1.247 | 4.78× |
| 10 | 10 | 0.524 | 2.367 | 4.52× |
| 100 | 1 | 0.062 | 0.237 | 3.81× |
| 100 | 5 | 0.254 | 1.322 | 5.21× |
| 100 | 10 | 0.527 | 2.453 | 4.66× |
| 1,000 | 1 | 0.059 | 0.362 | 6.18× |
| 1,000 | 5 | 0.268 | 1.887 | 7.05× |
| 1,000 | 10 | 0.553 | 3.623 | 6.55× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
