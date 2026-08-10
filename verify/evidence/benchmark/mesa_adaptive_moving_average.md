# MesaAdaptiveMovingAverage benchmark (`MAMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.057 | 17.53M | 0.056 | 17.88M | 0.088 | 1.54× | 1.57× |
| 10,000 | 0.540 | 18.52M | 0.530 | 18.88M | 0.531 | 0.98× | 1.00× |
| 100,000 | 5.385 | 18.57M | 5.181 | 19.30M | 5.327 | 0.99× | 1.03× |
| 1,000,000 | 56.995 | 17.55M | 52.678 | 18.98M | 50.530 | 0.89× | 0.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.101 | 0.143 | 1.41× |
| 1 | 5 | 0.299 | 0.553 | 1.85× |
| 1 | 10 | 0.521 | 1.052 | 2.02× |
| 10 | 1 | 0.047 | 0.110 | 2.32× |
| 10 | 5 | 0.220 | 0.487 | 2.21× |
| 10 | 10 | 0.480 | 1.045 | 2.18× |
| 100 | 1 | 0.053 | 0.110 | 2.10× |
| 100 | 5 | 0.236 | 0.518 | 2.20× |
| 100 | 10 | 0.498 | 1.073 | 2.16× |
| 1,000 | 1 | 0.105 | 0.163 | 1.56× |
| 1,000 | 5 | 0.259 | 0.778 | 3.01× |
| 1,000 | 10 | 0.509 | 1.572 | 3.09× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
