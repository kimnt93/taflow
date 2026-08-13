# BreadthThrust benchmark (`BreadthThrust` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.059 | 16.88M | 0.052 | 19.07M | 7.859 | 132.64× | 149.86× |
| 10,000 | 0.498 | 20.09M | 0.451 | 22.18M | 79.969 | 160.68× | 177.40× |
| 100,000 | 4.332 | 23.08M | 4.317 | 23.16M | 782.403 | 180.61× | 181.23× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.180 | 0.328 | 1.82× |
| 1 | 5 | 0.476 | 1.244 | 2.61× |
| 1 | 10 | 0.650 | 2.572 | 3.96× |
| 10 | 1 | 0.069 | 0.323 | 4.68× |
| 10 | 5 | 0.313 | 1.578 | 5.03× |
| 10 | 10 | 0.637 | 3.390 | 5.32× |
| 100 | 1 | 0.075 | 1.083 | 14.35× |
| 100 | 5 | 0.317 | 5.419 | 17.07× |
| 100 | 10 | 0.624 | 11.099 | 17.79× |
| 1,000 | 1 | 0.125 | 8.456 | 67.68× |
| 1,000 | 5 | 0.602 | 52.547 | 87.34× |
| 1,000 | 10 | 0.892 | 89.786 | 100.65× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
