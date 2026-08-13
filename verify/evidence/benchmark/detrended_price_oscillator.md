# DetrendedPriceOscillator benchmark (`dpo` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.051 | 19.53M | 0.053 | 18.78M | 0.318 | 6.21× | 5.97× |
| 10,000 | 0.470 | 21.27M | 0.413 | 24.21M | 0.393 | 0.84× | 0.95× |
| 100,000 | 3.942 | 25.36M | 4.124 | 24.25M | 1.437 | 0.36× | 0.35× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.102 | 0.184 | 1.80× |
| 1 | 5 | 0.370 | 0.740 | 2.00× |
| 1 | 10 | 0.603 | 1.453 | 2.41× |
| 10 | 1 | 0.068 | 0.151 | 2.24× |
| 10 | 5 | 0.294 | 0.706 | 2.40× |
| 10 | 10 | 0.605 | 1.484 | 2.45× |
| 100 | 1 | 0.073 | 0.411 | 5.60× |
| 100 | 5 | 0.298 | 1.901 | 6.37× |
| 100 | 10 | 0.624 | 3.792 | 6.07× |
| 1,000 | 1 | 0.113 | 0.386 | 3.40× |
| 1,000 | 5 | 0.290 | 1.960 | 6.75× |
| 1,000 | 10 | 0.620 | 3.977 | 6.41× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
