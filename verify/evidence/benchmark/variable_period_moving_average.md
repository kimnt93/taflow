# VariablePeriodMovingAverage benchmark (`MAVP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.149 | 6.70M | 0.140 | 7.16M | 0.131 | 0.88× | 0.94× |
| 10,000 | 1.391 | 7.19M | 1.337 | 7.48M | 0.889 | 0.64× | 0.66× |
| 100,000 | 14.607 | 6.85M | 12.877 | 7.77M | 9.040 | 0.62× | 0.70× |
| 1,000,000 | 140.641 | 7.11M | 137.479 | 7.27M | 113.520 | 0.81× | 0.83× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.073 | 0.122 | 1.67× |
| 1 | 5 | 0.323 | 0.596 | 1.84× |
| 1 | 10 | 0.539 | 1.091 | 2.02× |
| 10 | 1 | 0.057 | 0.109 | 1.91× |
| 10 | 5 | 0.248 | 0.573 | 2.31× |
| 10 | 10 | 0.618 | 1.048 | 1.70× |
| 100 | 1 | 0.075 | 0.127 | 1.69× |
| 100 | 5 | 0.290 | 0.593 | 2.05× |
| 100 | 10 | 0.681 | 1.175 | 1.73× |
| 1,000 | 1 | 0.198 | 0.198 | 1.00× |
| 1,000 | 5 | 0.393 | 0.922 | 2.35× |
| 1,000 | 10 | 0.756 | 1.881 | 2.49× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
