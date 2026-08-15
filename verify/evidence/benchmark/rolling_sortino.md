# RollingSortino benchmark (`SortinoRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 50.81M | 0.019 | 53.16M | 0.190 | 9.64× | 10.09× |
| 10,000 | 0.185 | 53.94M | 0.232 | 43.15M | 0.785 | 4.24× | 3.39× |
| 100,000 | 1.894 | 52.80M | 1.830 | 54.63M | 9.278 | 4.90× | 5.07× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.096 | 0.304 | 3.15× |
| 1 | 5 | 0.295 | 1.236 | 4.19× |
| 1 | 10 | 0.435 | 2.437 | 5.61× |
| 10 | 1 | 0.058 | 0.232 | 3.99× |
| 10 | 5 | 0.220 | 1.340 | 6.10× |
| 10 | 10 | 0.472 | 2.433 | 5.15× |
| 100 | 1 | 0.056 | 0.243 | 4.33× |
| 100 | 5 | 0.218 | 1.569 | 7.19× |
| 100 | 10 | 0.469 | 2.453 | 5.22× |
| 1,000 | 1 | 0.083 | 0.313 | 3.79× |
| 1,000 | 5 | 0.236 | 1.641 | 6.94× |
| 1,000 | 10 | 0.456 | 3.205 | 7.03× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
