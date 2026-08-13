# TrueStrengthIndex benchmark (`TSI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.077 | 13.00M | 0.071 | 14.05M | 0.190 | 2.47× | 2.67× |
| 10,000 | 0.624 | 16.03M | 0.616 | 16.22M | 0.551 | 0.88× | 0.89× |
| 100,000 | 6.334 | 15.79M | 6.017 | 16.62M | 4.086 | 0.65× | 0.68× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.218 | 0.264 | 1.21× |
| 1 | 5 | 0.566 | 1.450 | 2.56× |
| 1 | 10 | 0.662 | 2.611 | 3.94× |
| 10 | 1 | 0.077 | 0.239 | 3.09× |
| 10 | 5 | 0.323 | 1.336 | 4.14× |
| 10 | 10 | 0.664 | 2.417 | 3.64× |
| 100 | 1 | 0.080 | 0.245 | 3.06× |
| 100 | 5 | 0.321 | 1.362 | 4.24× |
| 100 | 10 | 0.679 | 2.624 | 3.86× |
| 1,000 | 1 | 0.137 | 0.278 | 2.03× |
| 1,000 | 5 | 0.361 | 1.614 | 4.48× |
| 1,000 | 10 | 0.704 | 2.861 | 4.06× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
