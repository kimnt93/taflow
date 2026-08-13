# CandleConcealBabySwall benchmark (`CDLCONCEALBABYSWALL` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.118 | 8.44M | 0.110 | 9.07M | 0.061 | 0.52× | 0.56× |
| 10,000 | 1.267 | 7.89M | 0.940 | 10.64M | 0.089 | 0.07× | 0.09× |
| 100,000 | 8.780 | 11.39M | 8.957 | 11.16M | 0.623 | 0.07× | 0.07× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.189 | 0.108 | 0.57× |
| 1 | 5 | 0.526 | 0.474 | 0.90× |
| 1 | 10 | 0.650 | 0.930 | 1.43× |
| 10 | 1 | 0.072 | 0.084 | 1.17× |
| 10 | 5 | 0.309 | 0.483 | 1.56× |
| 10 | 10 | 1.083 | 0.961 | 0.89× |
| 100 | 1 | 0.085 | 0.089 | 1.04× |
| 100 | 5 | 0.335 | 0.461 | 1.38× |
| 100 | 10 | 0.680 | 0.916 | 1.35× |
| 1,000 | 1 | 0.164 | 0.092 | 0.56× |
| 1,000 | 5 | 0.369 | 0.467 | 1.27× |
| 1,000 | 10 | 0.701 | 0.968 | 1.38× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
