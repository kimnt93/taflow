# RollingCorrelation benchmark (`CORREL` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 89.45M | 0.009 | 108.19M | 0.044 | 3.97× | 4.80× |
| 10,000 | 0.058 | 172.33M | 0.053 | 187.11M | 0.099 | 1.71× | 1.85× |
| 100,000 | 0.545 | 183.42M | 0.555 | 180.21M | 0.627 | 1.15× | 1.13× |
| 1,000,000 | 5.519 | 181.21M | 5.084 | 196.70M | 6.033 | 1.09× | 1.19× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.136 | 0.149 | 1.10× |
| 1 | 5 | 0.314 | 0.634 | 2.02× |
| 1 | 10 | 0.544 | 1.076 | 1.98× |
| 10 | 1 | 0.062 | 0.108 | 1.75× |
| 10 | 5 | 0.288 | 0.571 | 1.98× |
| 10 | 10 | 0.542 | 1.133 | 2.09× |
| 100 | 1 | 0.055 | 0.103 | 1.87× |
| 100 | 5 | 0.276 | 0.614 | 2.23× |
| 100 | 10 | 0.587 | 1.165 | 1.98× |
| 1,000 | 1 | 0.057 | 0.110 | 1.94× |
| 1,000 | 5 | 0.295 | 0.591 | 2.00× |
| 1,000 | 10 | 0.639 | 1.247 | 1.95× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
