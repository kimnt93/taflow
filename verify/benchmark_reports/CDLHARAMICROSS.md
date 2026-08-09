# CandleHaramiCross benchmark (`CDLHARAMICROSS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 99.83M | 0.008 | 122.18M | 0.035 | 3.50× | 4.28× |
| 10,000 | 0.093 | 107.99M | 0.090 | 110.51M | 0.139 | 1.50× | 1.54× |
| 100,000 | 1.079 | 92.67M | 1.052 | 95.03M | 1.111 | 1.03× | 1.06× |
| 1,000,000 | 10.958 | 91.26M | 11.133 | 89.82M | 10.814 | 0.99× | 0.97× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.123 | 1.27× |
| 1 | 5 | 0.343 | 0.478 | 1.39× |
| 1 | 10 | 0.485 | 0.881 | 1.82× |
| 10 | 1 | 0.054 | 0.093 | 1.70× |
| 10 | 5 | 0.226 | 0.429 | 1.90× |
| 10 | 10 | 0.501 | 0.914 | 1.82× |
| 100 | 1 | 0.063 | 0.103 | 1.63× |
| 100 | 5 | 0.274 | 0.456 | 1.67× |
| 100 | 10 | 0.538 | 0.934 | 1.74× |
| 1,000 | 1 | 0.063 | 0.101 | 1.61× |
| 1,000 | 5 | 0.254 | 0.496 | 1.95× |
| 1,000 | 10 | 0.544 | 1.062 | 1.95× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
