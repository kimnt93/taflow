# CandleUniqueThreeRiver benchmark (`CDLUNIQUE3RIVER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 149.15M | 0.003 | 353.95M | 0.031 | 4.64× | 11.02× |
| 10,000 | 0.057 | 176.98M | 0.050 | 199.56M | 0.075 | 1.33× | 1.50× |
| 100,000 | 0.764 | 130.84M | 0.757 | 132.15M | 0.600 | 0.79× | 0.79× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.079 | 0.115 | 1.46× |
| 1 | 5 | 0.197 | 0.435 | 2.21× |
| 1 | 10 | 0.411 | 0.892 | 2.17× |
| 10 | 1 | 0.045 | 0.088 | 1.96× |
| 10 | 5 | 0.184 | 0.396 | 2.16× |
| 10 | 10 | 0.354 | 0.899 | 2.54× |
| 100 | 1 | 0.045 | 0.088 | 1.98× |
| 100 | 5 | 0.184 | 0.401 | 2.18× |
| 100 | 10 | 0.376 | 0.850 | 2.26× |
| 1,000 | 1 | 0.051 | 0.092 | 1.80× |
| 1,000 | 5 | 0.193 | 0.433 | 2.25× |
| 1,000 | 10 | 0.400 | 0.964 | 2.41× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
