# CandleConcealBabySwall benchmark (`CDLCONCEALBABYSWALL` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 48.74M | 0.021 | 47.98M | 0.039 | 1.89× | 1.86× |
| 10,000 | 0.135 | 74.07M | 0.136 | 73.43M | 0.097 | 0.72× | 0.72× |
| 100,000 | 1.303 | 76.75M | 1.246 | 80.23M | 0.700 | 0.54× | 0.56× |
| 1,000,000 | 13.332 | 75.00M | 12.674 | 78.90M | 6.969 | 0.52× | 0.55× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.092 | 0.123 | 1.34× |
| 1 | 5 | 0.310 | 0.471 | 1.52× |
| 1 | 10 | 0.587 | 0.925 | 1.58× |
| 10 | 1 | 0.067 | 0.096 | 1.43× |
| 10 | 5 | 0.256 | 0.433 | 1.69× |
| 10 | 10 | 0.571 | 0.983 | 1.72× |
| 100 | 1 | 0.062 | 0.086 | 1.38× |
| 100 | 5 | 0.261 | 0.444 | 1.70× |
| 100 | 10 | 0.560 | 0.962 | 1.72× |
| 1,000 | 1 | 0.078 | 0.093 | 1.20× |
| 1,000 | 5 | 0.280 | 0.481 | 1.72× |
| 1,000 | 10 | 0.589 | 1.003 | 1.70× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
