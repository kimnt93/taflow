# SuperSmoother benchmark (`SuperSmoother` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 133.38M | 0.007 | 140.67M | 0.150 | 20.03× | 21.12× |
| 10,000 | 0.055 | 181.55M | 0.052 | 193.67M | 0.498 | 9.04× | 9.64× |
| 100,000 | 0.506 | 197.44M | 0.452 | 221.31M | 3.564 | 7.04× | 7.89× |
| 1,000,000 | 6.290 | 158.99M | 4.504 | 222.01M | 37.218 | 5.92× | 8.26× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.058 | 0.249 | 4.27× |
| 1 | 5 | 0.256 | 0.998 | 3.89× |
| 1 | 10 | 0.492 | 2.086 | 4.24× |
| 10 | 1 | 0.050 | 0.191 | 3.78× |
| 10 | 5 | 0.225 | 0.938 | 4.17× |
| 10 | 10 | 0.474 | 2.110 | 4.45× |
| 100 | 1 | 0.053 | 0.195 | 3.70× |
| 100 | 5 | 0.225 | 0.971 | 4.31× |
| 100 | 10 | 0.478 | 2.353 | 4.92× |
| 1,000 | 1 | 0.060 | 0.234 | 3.92× |
| 1,000 | 5 | 0.222 | 1.111 | 5.00× |
| 1,000 | 10 | 0.530 | 2.469 | 4.66× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
