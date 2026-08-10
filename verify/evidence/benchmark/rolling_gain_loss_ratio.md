# RollingGainLossRatio benchmark (`GainLossRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.026 | 39.14M | 0.026 | 38.10M | 0.153 | 5.97× | 5.82× |
| 10,000 | 0.248 | 40.28M | 0.245 | 40.85M | 0.563 | 2.27× | 2.30× |
| 100,000 | 2.492 | 40.13M | 2.493 | 40.12M | 4.607 | 1.85× | 1.85× |
| 1,000,000 | 24.053 | 41.58M | 23.085 | 43.32M | 49.188 | 2.05× | 2.13× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.084 | 0.242 | 2.88× |
| 1 | 5 | 0.332 | 0.956 | 2.88× |
| 1 | 10 | 0.497 | 2.069 | 4.17× |
| 10 | 1 | 0.053 | 0.191 | 3.59× |
| 10 | 5 | 0.214 | 0.939 | 4.40× |
| 10 | 10 | 0.488 | 2.066 | 4.23× |
| 100 | 1 | 0.052 | 0.196 | 3.78× |
| 100 | 5 | 0.242 | 0.962 | 3.98× |
| 100 | 10 | 0.497 | 2.132 | 4.29× |
| 1,000 | 1 | 0.079 | 0.248 | 3.12× |
| 1,000 | 5 | 0.266 | 1.189 | 4.47× |
| 1,000 | 10 | 0.549 | 2.565 | 4.67× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
