# CandleRiseFallThreeMethods benchmark (`CDLRISEFALL3METHODS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.025 | 40.71M | 0.020 | 50.76M | 0.035 | 1.41× | 1.76× |
| 10,000 | 0.172 | 58.09M | 0.179 | 56.00M | 0.123 | 0.71× | 0.69× |
| 100,000 | 1.793 | 55.79M | 1.761 | 56.80M | 0.901 | 0.50× | 0.51× |
| 1,000,000 | 18.101 | 55.25M | 17.841 | 56.05M | 9.657 | 0.53× | 0.54× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.107 | 1.31× |
| 1 | 5 | 0.385 | 0.480 | 1.25× |
| 1 | 10 | 0.539 | 0.900 | 1.67× |
| 10 | 1 | 0.052 | 0.085 | 1.62× |
| 10 | 5 | 0.241 | 0.419 | 1.74× |
| 10 | 10 | 0.530 | 0.932 | 1.76× |
| 100 | 1 | 0.060 | 0.092 | 1.53× |
| 100 | 5 | 0.259 | 0.420 | 1.62× |
| 100 | 10 | 0.564 | 0.965 | 1.71× |
| 1,000 | 1 | 0.073 | 0.104 | 1.43× |
| 1,000 | 5 | 0.288 | 0.493 | 1.71× |
| 1,000 | 10 | 0.594 | 0.983 | 1.65× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
