# RollingMaximum benchmark (`MAX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 233.12M | 0.003 | 320.34M | 0.037 | 8.65× | 11.88× |
| 10,000 | 0.026 | 381.57M | 0.022 | 455.90M | 0.080 | 3.06× | 3.66× |
| 100,000 | 0.235 | 425.28M | 0.202 | 495.04M | 0.515 | 2.19× | 2.55× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.080 | 0.130 | 1.62× |
| 1 | 5 | 0.275 | 0.503 | 1.83× |
| 1 | 10 | 0.434 | 0.941 | 2.17× |
| 10 | 1 | 0.044 | 0.091 | 2.06× |
| 10 | 5 | 0.202 | 0.450 | 2.23× |
| 10 | 10 | 0.400 | 1.041 | 2.60× |
| 100 | 1 | 0.044 | 0.095 | 2.15× |
| 100 | 5 | 0.184 | 0.446 | 2.42× |
| 100 | 10 | 0.378 | 0.958 | 2.54× |
| 1,000 | 1 | 0.053 | 0.109 | 2.07× |
| 1,000 | 5 | 0.217 | 0.521 | 2.40× |
| 1,000 | 10 | 0.444 | 1.019 | 2.29× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
