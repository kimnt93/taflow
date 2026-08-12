# StandardErrorBands benchmark (`StandardErrorBands` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.081 | 12.29M | 0.070 | 14.24M | 0.609 | 7.48× | 8.67× |
| 10,000 | 0.709 | 14.10M | 0.670 | 14.92M | 4.228 | 5.96× | 6.31× |
| 100,000 | 7.155 | 13.98M | 6.858 | 14.58M | 45.773 | 6.40× | 6.67× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.109 | 0.344 | 3.15× |
| 1 | 5 | 0.308 | 1.427 | 4.64× |
| 1 | 10 | 0.499 | 2.763 | 5.54× |
| 10 | 1 | 0.052 | 0.254 | 4.88× |
| 10 | 5 | 0.255 | 1.415 | 5.55× |
| 10 | 10 | 0.538 | 2.915 | 5.42× |
| 100 | 1 | 0.058 | 0.292 | 5.06× |
| 100 | 5 | 0.247 | 1.724 | 6.98× |
| 100 | 10 | 0.522 | 3.121 | 5.98× |
| 1,000 | 1 | 0.132 | 0.943 | 7.17× |
| 1,000 | 5 | 0.263 | 3.741 | 14.25× |
| 1,000 | 10 | 0.545 | 7.445 | 13.67× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
