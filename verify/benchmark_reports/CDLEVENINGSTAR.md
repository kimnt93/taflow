# CandleEveningStar benchmark (`CDLEVENINGSTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 104.77M | 0.008 | 121.96M | 0.038 | 3.95× | 4.60× |
| 10,000 | 0.083 | 120.44M | 0.087 | 114.51M | 0.110 | 1.32× | 1.26× |
| 100,000 | 0.883 | 113.22M | 0.881 | 113.57M | 0.856 | 0.97× | 0.97× |
| 1,000,000 | 9.116 | 109.69M | 9.346 | 107.00M | 8.391 | 0.92× | 0.90× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.147 | 0.138 | 0.94× |
| 1 | 5 | 0.261 | 0.491 | 1.88× |
| 1 | 10 | 0.533 | 1.019 | 1.91× |
| 10 | 1 | 0.054 | 0.099 | 1.84× |
| 10 | 5 | 0.253 | 0.475 | 1.88× |
| 10 | 10 | 0.542 | 1.012 | 1.87× |
| 100 | 1 | 0.058 | 0.098 | 1.69× |
| 100 | 5 | 0.273 | 0.484 | 1.77× |
| 100 | 10 | 0.555 | 0.996 | 1.79× |
| 1,000 | 1 | 0.063 | 0.106 | 1.70× |
| 1,000 | 5 | 0.244 | 0.507 | 2.08× |
| 1,000 | 10 | 0.552 | 1.064 | 1.93× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
