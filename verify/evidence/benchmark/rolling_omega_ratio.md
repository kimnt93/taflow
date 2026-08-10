# RollingOmegaRatio benchmark (`OmegaRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.032 | 30.84M | 0.033 | 30.56M | 0.204 | 6.29× | 6.23× |
| 10,000 | 0.304 | 32.93M | 0.296 | 33.76M | 0.770 | 2.54× | 2.60× |
| 100,000 | 2.982 | 33.53M | 3.021 | 33.10M | 5.610 | 1.88× | 1.86× |
| 1,000,000 | 31.976 | 31.27M | 29.115 | 34.35M | 54.494 | 1.70× | 1.87× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.103 | 0.271 | 2.62× |
| 1 | 5 | 0.408 | 1.346 | 3.30× |
| 1 | 10 | 0.462 | 2.458 | 5.32× |
| 10 | 1 | 0.049 | 0.235 | 4.83× |
| 10 | 5 | 0.230 | 1.332 | 5.80× |
| 10 | 10 | 0.457 | 2.450 | 5.36× |
| 100 | 1 | 0.057 | 0.240 | 4.18× |
| 100 | 5 | 0.248 | 1.359 | 5.49× |
| 100 | 10 | 0.511 | 2.522 | 4.94× |
| 1,000 | 1 | 0.084 | 0.290 | 3.44× |
| 1,000 | 5 | 0.240 | 1.615 | 6.72× |
| 1,000 | 10 | 0.518 | 3.091 | 5.97× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
