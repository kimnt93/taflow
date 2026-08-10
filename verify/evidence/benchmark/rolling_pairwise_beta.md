# RollingPairwiseBeta benchmark (`PairwiseBeta` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.036 | 27.67M | 0.035 | 28.85M | 0.237 | 6.55× | 6.83× |
| 10,000 | 0.308 | 32.49M | 0.304 | 32.86M | 1.196 | 3.89× | 3.93× |
| 100,000 | 3.162 | 31.63M | 3.075 | 32.52M | 10.026 | 3.17× | 3.26× |
| 1,000,000 | 33.140 | 30.17M | 30.708 | 32.56M | 97.802 | 2.95× | 3.18× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.093 | 0.248 | 2.68× |
| 1 | 5 | 0.287 | 1.421 | 4.94× |
| 1 | 10 | 0.571 | 2.611 | 4.57× |
| 10 | 1 | 0.078 | 0.319 | 4.12× |
| 10 | 5 | 0.327 | 1.438 | 4.40× |
| 10 | 10 | 0.554 | 5.831 | 10.52× |
| 100 | 1 | 0.108 | 0.430 | 3.99× |
| 100 | 5 | 1.163 | 2.287 | 1.97× |
| 100 | 10 | 1.961 | 3.417 | 1.74× |
| 1,000 | 1 | 0.104 | 0.395 | 3.79× |
| 1,000 | 5 | 0.315 | 1.826 | 5.80× |
| 1,000 | 10 | 0.598 | 3.822 | 6.39× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
