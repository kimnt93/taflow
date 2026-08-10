# Sessions benchmark (`smartmoneyconcepts.smc.sessions` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 68.01M | 0.012 | 81.50M | 88.130 | 5993.48× | 7182.29× |
| 10,000 | 0.084 | 118.46M | 0.078 | 128.15M | 867.997 | 10282.17× | 11123.61× |
| 100,000 | 0.893 | 112.01M | 0.779 | 128.37M | 8998.626 | 10079.72× | 11551.42× |
| 1,000,000 | 23.130 | 43.23M | 7.956 | 125.70M | 85545.925 | 3698.55× | 10753.04× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.121 | 1.628 | 13.48× |
| 1 | 5 | 0.307 | 8.413 | 27.44× |
| 1 | 10 | 0.534 | 16.807 | 31.48× |
| 10 | 1 | 0.059 | 2.462 | 41.83× |
| 10 | 5 | 0.243 | 12.915 | 53.19× |
| 10 | 10 | 0.498 | 27.535 | 55.25× |
| 100 | 1 | 0.061 | 10.458 | 170.68× |
| 100 | 5 | 0.441 | 58.687 | 133.08× |
| 100 | 10 | 0.585 | 113.318 | 193.79× |
| 1,000 | 1 | 0.089 | 88.251 | 990.68× |
| 1,000 | 5 | 0.540 | 513.395 | 951.08× |
| 1,000 | 10 | 0.566 | 1146.873 | 2024.69× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
