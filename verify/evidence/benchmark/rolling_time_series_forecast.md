# RollingTimeSeriesForecast benchmark (`TSF` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 57.86M | 0.017 | 59.09M | 0.047 | 2.73× | 2.78× |
| 10,000 | 0.138 | 72.28M | 0.144 | 69.66M | 0.162 | 1.17× | 1.13× |
| 100,000 | 1.403 | 71.29M | 1.376 | 72.70M | 1.385 | 0.99× | 1.01× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.115 | 0.113 | 0.98× |
| 1 | 5 | 0.240 | 0.475 | 1.98× |
| 1 | 10 | 0.517 | 1.006 | 1.95× |
| 10 | 1 | 0.049 | 0.098 | 1.99× |
| 10 | 5 | 0.299 | 0.508 | 1.70× |
| 10 | 10 | 0.491 | 1.012 | 2.06× |
| 100 | 1 | 0.051 | 0.097 | 1.92× |
| 100 | 5 | 0.274 | 0.481 | 1.75× |
| 100 | 10 | 0.535 | 0.987 | 1.85× |
| 1,000 | 1 | 0.067 | 0.103 | 1.53× |
| 1,000 | 5 | 0.246 | 0.575 | 2.34× |
| 1,000 | 10 | 0.522 | 1.128 | 2.16× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
