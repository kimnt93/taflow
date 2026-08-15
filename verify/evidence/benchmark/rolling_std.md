# RollingStandardDeviation benchmark (`STDDEV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 220.24M | 0.004 | 278.42M | 0.033 | 7.36× | 9.31× |
| 10,000 | 0.029 | 340.56M | 0.028 | 362.75M | 0.057 | 1.94× | 2.07× |
| 100,000 | 0.321 | 311.79M | 0.287 | 348.15M | 0.301 | 0.94× | 1.05× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.102 | 0.114 | 1.12× |
| 1 | 5 | 0.275 | 0.469 | 1.70× |
| 1 | 10 | 0.401 | 0.952 | 2.38× |
| 10 | 1 | 0.041 | 0.093 | 2.23× |
| 10 | 5 | 0.184 | 0.437 | 2.37× |
| 10 | 10 | 0.386 | 0.928 | 2.41× |
| 100 | 1 | 0.045 | 0.090 | 1.99× |
| 100 | 5 | 0.200 | 0.439 | 2.19× |
| 100 | 10 | 0.402 | 0.913 | 2.27× |
| 1,000 | 1 | 0.053 | 0.093 | 1.76× |
| 1,000 | 5 | 0.190 | 0.456 | 2.40× |
| 1,000 | 10 | 0.434 | 0.968 | 2.23× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
