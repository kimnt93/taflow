# GarmanKlass benchmark (`GarmanKlassVolatility` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 43.45M | 0.020 | 49.38M | 0.312 | 13.56× | 15.41× |
| 10,000 | 0.158 | 63.16M | 0.169 | 59.13M | 1.570 | 9.91× | 9.28× |
| 100,000 | 1.586 | 63.05M | 1.532 | 65.28M | 13.554 | 8.55× | 8.85× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | 0.268 | 3.05× |
| 1 | 5 | 0.330 | 1.385 | 4.20× |
| 1 | 10 | 0.521 | 2.608 | 5.00× |
| 10 | 1 | 0.056 | 0.243 | 4.36× |
| 10 | 5 | 0.246 | 1.496 | 6.09× |
| 10 | 10 | 0.532 | 2.915 | 5.47× |
| 100 | 1 | 0.064 | 0.262 | 4.07× |
| 100 | 5 | 0.264 | 1.549 | 5.86× |
| 100 | 10 | 0.552 | 2.745 | 4.98× |
| 1,000 | 1 | 0.074 | 0.396 | 5.37× |
| 1,000 | 5 | 0.263 | 3.091 | 11.76× |
| 1,000 | 10 | 0.655 | 4.334 | 6.61× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
