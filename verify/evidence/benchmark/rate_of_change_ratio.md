# RateOfChangeRatio benchmark (`ROCR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 199.30M | 0.004 | 249.04M | 0.036 | 7.24× | 9.04× |
| 10,000 | 0.023 | 438.27M | 0.020 | 502.22M | 0.049 | 2.15× | 2.47× |
| 100,000 | 0.202 | 496.27M | 0.182 | 550.77M | 0.146 | 0.72× | 0.80× |
| 1,000,000 | 2.576 | 388.16M | 1.929 | 518.50M | 1.319 | 0.51× | 0.68× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.061 | 0.094 | 1.53× |
| 1 | 5 | 0.275 | 0.648 | 2.36× |
| 1 | 10 | 0.514 | 1.092 | 2.12× |
| 10 | 1 | 0.059 | 0.103 | 1.76× |
| 10 | 5 | 0.255 | 0.488 | 1.91× |
| 10 | 10 | 0.536 | 1.060 | 1.98× |
| 100 | 1 | 0.056 | 0.087 | 1.56× |
| 100 | 5 | 0.262 | 0.542 | 2.07× |
| 100 | 10 | 0.535 | 1.071 | 2.00× |
| 1,000 | 1 | 0.051 | 0.096 | 1.87× |
| 1,000 | 5 | 0.249 | 0.477 | 1.92× |
| 1,000 | 10 | 0.696 | 1.069 | 1.53× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
