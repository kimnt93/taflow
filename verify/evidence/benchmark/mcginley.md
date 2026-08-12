# McGinleyDynamic benchmark (`McGinleyDynamic` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 58.57M | 0.016 | 60.88M | 0.197 | 11.54× | 12.00× |
| 10,000 | 0.124 | 80.45M | 0.122 | 82.10M | 0.576 | 4.63× | 4.73× |
| 100,000 | 1.277 | 78.32M | 1.208 | 82.81M | 4.951 | 3.88× | 4.10× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.120 | 0.347 | 2.88× |
| 1 | 5 | 0.293 | 1.371 | 4.68× |
| 1 | 10 | 0.482 | 2.667 | 5.53× |
| 10 | 1 | 0.057 | 0.225 | 3.94× |
| 10 | 5 | 0.230 | 1.683 | 7.31× |
| 10 | 10 | 0.505 | 2.527 | 5.00× |
| 100 | 1 | 0.054 | 0.234 | 4.31× |
| 100 | 5 | 0.281 | 1.615 | 5.75× |
| 100 | 10 | 0.510 | 2.646 | 5.19× |
| 1,000 | 1 | 0.071 | 0.269 | 3.80× |
| 1,000 | 5 | 0.250 | 1.651 | 6.61× |
| 1,000 | 10 | 0.517 | 3.074 | 5.94× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
