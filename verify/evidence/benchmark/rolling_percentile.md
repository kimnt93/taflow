# RollingPercentile benchmark (`rolling percentile` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.039 | 25.45M | 0.040 | 25.11M | 0.373 | 9.50× | 9.38× |
| 10,000 | 0.435 | 23.00M | 0.432 | 23.16M | 2.243 | 5.16× | 5.20× |
| 100,000 | 4.232 | 23.63M | 4.378 | 22.84M | 22.393 | 5.29× | 5.12× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.049 | 0.280 | 5.65× |
| 1 | 5 | 0.257 | 1.055 | 4.10× |
| 1 | 10 | 0.403 | 2.243 | 5.57× |
| 10 | 1 | 0.045 | 0.189 | 4.17× |
| 10 | 5 | 0.188 | 0.999 | 5.32× |
| 10 | 10 | 0.405 | 2.209 | 5.46× |
| 100 | 1 | 0.053 | 0.240 | 4.54× |
| 100 | 5 | 0.204 | 1.272 | 6.23× |
| 100 | 10 | 0.484 | 2.700 | 5.58× |
| 1,000 | 1 | 0.097 | 0.441 | 4.54× |
| 1,000 | 5 | 0.200 | 1.545 | 7.74× |
| 1,000 | 10 | 0.456 | 3.061 | 6.71× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
