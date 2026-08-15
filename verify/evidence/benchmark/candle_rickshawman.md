# CandleRickshawman benchmark (`CDLRICKSHAWMAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 80.13M | 0.009 | 107.76M | 0.040 | 3.19× | 4.28× |
| 10,000 | 0.114 | 87.78M | 0.111 | 90.39M | 0.130 | 1.14× | 1.18× |
| 100,000 | 1.144 | 87.43M | 1.139 | 87.79M | 0.990 | 0.87× | 0.87× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.061 | 0.105 | 1.72× |
| 1 | 5 | 0.262 | 0.438 | 1.67× |
| 1 | 10 | 0.416 | 0.957 | 2.30× |
| 10 | 1 | 0.043 | 0.086 | 2.01× |
| 10 | 5 | 0.186 | 0.434 | 2.33× |
| 10 | 10 | 0.367 | 0.943 | 2.57× |
| 100 | 1 | 0.057 | 0.102 | 1.79× |
| 100 | 5 | 0.213 | 0.444 | 2.08× |
| 100 | 10 | 0.406 | 0.947 | 2.33× |
| 1,000 | 1 | 0.058 | 0.099 | 1.70× |
| 1,000 | 5 | 0.206 | 0.551 | 2.67× |
| 1,000 | 10 | 0.448 | 1.039 | 2.32× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
