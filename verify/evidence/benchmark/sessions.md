# Sessions benchmark (`smartmoneyconcepts.smc.sessions` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 87.28M | 0.009 | 111.90M | 89.961 | 7851.50× | 10066.93× |
| 10,000 | 0.082 | 122.30M | 0.072 | 139.13M | 895.932 | 10956.94× | 12465.19× |
| 100,000 | 0.741 | 135.03M | 0.644 | 155.26M | 8866.204 | 11972.04× | 13765.72× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.149 | 1.863 | 12.51× |
| 1 | 5 | 0.313 | 8.524 | 27.24× |
| 1 | 10 | 0.435 | 17.095 | 39.34× |
| 10 | 1 | 0.051 | 2.523 | 49.63× |
| 10 | 5 | 0.200 | 13.010 | 65.12× |
| 10 | 10 | 0.407 | 26.259 | 64.58× |
| 100 | 1 | 0.050 | 11.008 | 221.74× |
| 100 | 5 | 0.238 | 56.430 | 236.77× |
| 100 | 10 | 0.516 | 120.498 | 233.66× |
| 1,000 | 1 | 0.073 | 88.801 | 1223.46× |
| 1,000 | 5 | 0.443 | 527.574 | 1190.66× |
| 1,000 | 10 | 0.440 | 1160.012 | 2638.01× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
