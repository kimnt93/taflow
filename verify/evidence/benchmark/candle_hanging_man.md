# CandleHangingMan benchmark (`CDLHANGINGMAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 140.80M | 0.004 | 250.42M | 0.037 | 5.14× | 9.15× |
| 10,000 | 0.083 | 120.97M | 0.075 | 133.50M | 0.163 | 1.97× | 2.17× |
| 100,000 | 1.084 | 92.23M | 1.051 | 95.16M | 1.398 | 1.29× | 1.33× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.111 | 0.120 | 1.08× |
| 1 | 5 | 0.244 | 0.485 | 1.99× |
| 1 | 10 | 0.396 | 0.885 | 2.23× |
| 10 | 1 | 0.045 | 0.087 | 1.95× |
| 10 | 5 | 0.190 | 0.445 | 2.34× |
| 10 | 10 | 0.391 | 0.881 | 2.25× |
| 100 | 1 | 0.045 | 0.091 | 2.00× |
| 100 | 5 | 0.167 | 0.418 | 2.50× |
| 100 | 10 | 0.362 | 0.866 | 2.40× |
| 1,000 | 1 | 0.052 | 0.101 | 1.94× |
| 1,000 | 5 | 0.193 | 0.484 | 2.51× |
| 1,000 | 10 | 0.414 | 1.012 | 2.44× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
