# EqualHighsLows benchmark (`causal equal pivot levels` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.042 | 23.69M | 0.038 | 26.03M | 4.905 | 116.17× | 127.65× |
| 10,000 | 0.417 | 23.98M | 0.420 | 23.84M | 47.203 | 113.17× | 112.51× |
| 100,000 | 4.115 | 24.30M | 4.087 | 24.47M | 459.051 | 111.55× | 112.32× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.073 | 0.205 | 2.80× |
| 1 | 5 | 0.291 | 0.749 | 2.57× |
| 1 | 10 | 0.392 | 1.427 | 3.64× |
| 10 | 1 | 0.047 | 0.183 | 3.86× |
| 10 | 5 | 0.197 | 0.835 | 4.23× |
| 10 | 10 | 0.415 | 1.693 | 4.08× |
| 100 | 1 | 0.052 | 0.550 | 10.68× |
| 100 | 5 | 0.200 | 2.773 | 13.88× |
| 100 | 10 | 0.439 | 5.687 | 12.94× |
| 1,000 | 1 | 0.096 | 4.837 | 50.64× |
| 1,000 | 5 | 0.280 | 26.475 | 94.53× |
| 1,000 | 10 | 0.768 | 56.995 | 74.19× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
