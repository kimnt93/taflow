# GoldenPocket benchmark (`GoldenPocket` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.087 | 11.50M | 0.078 | 12.82M | 0.467 | 5.37× | 5.99× |
| 10,000 | 0.697 | 14.35M | 0.706 | 14.17M | 3.604 | 5.17× | 5.11× |
| 100,000 | 6.947 | 14.39M | 6.666 | 15.00M | 40.252 | 5.79× | 6.04× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.111 | 0.224 | 2.02× |
| 1 | 5 | 0.459 | 0.851 | 1.85× |
| 1 | 10 | 0.615 | 1.853 | 3.01× |
| 10 | 1 | 0.073 | 0.176 | 2.39× |
| 10 | 5 | 0.295 | 0.833 | 2.82× |
| 10 | 10 | 0.640 | 1.922 | 3.00× |
| 100 | 1 | 0.086 | 0.210 | 2.44× |
| 100 | 5 | 0.315 | 1.008 | 3.20× |
| 100 | 10 | 0.624 | 2.219 | 3.56× |
| 1,000 | 1 | 0.155 | 0.761 | 4.92× |
| 1,000 | 5 | 0.329 | 3.106 | 9.45× |
| 1,000 | 10 | 0.657 | 13.906 | 21.17× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
