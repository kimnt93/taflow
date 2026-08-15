# CandleConcealBabySwall benchmark (`CDLCONCEALBABYSWALL` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 132.54M | 0.004 | 247.10M | 0.037 | 4.87× | 9.08× |
| 10,000 | 0.054 | 185.14M | 0.050 | 199.25M | 0.091 | 1.69× | 1.82× |
| 100,000 | 0.615 | 162.57M | 0.591 | 169.19M | 0.651 | 1.06× | 1.10× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.057 | 0.106 | 1.87× |
| 1 | 5 | 0.265 | 0.456 | 1.72× |
| 1 | 10 | 0.388 | 0.892 | 2.30× |
| 10 | 1 | 0.039 | 0.090 | 2.29× |
| 10 | 5 | 0.197 | 0.445 | 2.26× |
| 10 | 10 | 0.399 | 0.923 | 2.31× |
| 100 | 1 | 0.042 | 0.094 | 2.24× |
| 100 | 5 | 0.184 | 0.429 | 2.33× |
| 100 | 10 | 0.438 | 0.934 | 2.13× |
| 1,000 | 1 | 0.052 | 0.095 | 1.83× |
| 1,000 | 5 | 0.196 | 0.470 | 2.40× |
| 1,000 | 10 | 0.435 | 1.021 | 2.35× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
