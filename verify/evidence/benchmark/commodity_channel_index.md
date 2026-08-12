# CommodityChannelIndex benchmark (`CCI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 40.96M | 0.023 | 44.29M | 0.051 | 2.08× | 2.25× |
| 10,000 | 0.196 | 50.90M | 0.188 | 53.17M | 0.235 | 1.19× | 1.25× |
| 100,000 | 1.939 | 51.58M | 2.889 | 34.62M | 2.241 | 1.16× | 0.78× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.098 | 0.115 | 1.17× |
| 1 | 5 | 0.287 | 0.522 | 1.82× |
| 1 | 10 | 0.493 | 0.963 | 1.95× |
| 10 | 1 | 0.058 | 0.085 | 1.45× |
| 10 | 5 | 0.266 | 0.456 | 1.71× |
| 10 | 10 | 0.520 | 0.922 | 1.77× |
| 100 | 1 | 0.054 | 0.088 | 1.64× |
| 100 | 5 | 0.237 | 0.460 | 1.94× |
| 100 | 10 | 0.510 | 0.988 | 1.94× |
| 1,000 | 1 | 0.075 | 0.120 | 1.59× |
| 1,000 | 5 | 0.244 | 0.559 | 2.30× |
| 1,000 | 10 | 0.526 | 1.159 | 2.20× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
