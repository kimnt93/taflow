# Vortex benchmark (`Vortex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.121 | 8.30M | 0.113 | 8.85M | 0.519 | 4.31× | 4.60× |
| 10,000 | 1.023 | 9.78M | 1.030 | 9.71M | 3.927 | 3.84× | 3.81× |
| 100,000 | 9.928 | 10.07M | 9.899 | 10.10M | 41.975 | 4.23× | 4.24× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.106 | 0.261 | 2.45× |
| 1 | 5 | 0.479 | 1.065 | 2.23× |
| 1 | 10 | 0.655 | 2.255 | 3.44× |
| 10 | 1 | 0.087 | 0.214 | 2.47× |
| 10 | 5 | 0.325 | 1.348 | 4.15× |
| 10 | 10 | 0.690 | 2.339 | 3.39× |
| 100 | 1 | 0.093 | 0.261 | 2.80× |
| 100 | 5 | 0.323 | 1.548 | 4.79× |
| 100 | 10 | 0.687 | 2.690 | 3.91× |
| 1,000 | 1 | 0.189 | 0.855 | 4.53× |
| 1,000 | 5 | 0.381 | 3.420 | 8.98× |
| 1,000 | 10 | 0.743 | 6.919 | 9.32× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
