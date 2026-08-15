# ChaikinVolatility benchmark (`ChaikinVolatility` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 118.38M | 0.007 | 137.84M | 0.242 | 28.64× | 33.35× |
| 10,000 | 0.061 | 163.40M | 0.059 | 169.22M | 0.877 | 14.32× | 14.83× |
| 100,000 | 0.621 | 161.10M | 0.631 | 158.60M | 6.987 | 11.26× | 11.08× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.297 | 3.83× |
| 1 | 5 | 0.262 | 1.463 | 5.59× |
| 1 | 10 | 0.393 | 3.007 | 7.66× |
| 10 | 1 | 0.054 | 0.268 | 4.97× |
| 10 | 5 | 0.180 | 1.496 | 8.29× |
| 10 | 10 | 0.471 | 2.840 | 6.03× |
| 100 | 1 | 0.049 | 0.255 | 5.19× |
| 100 | 5 | 0.209 | 1.766 | 8.45× |
| 100 | 10 | 0.435 | 3.008 | 6.92× |
| 1,000 | 1 | 0.057 | 0.326 | 5.67× |
| 1,000 | 5 | 0.202 | 1.957 | 9.68× |
| 1,000 | 10 | 0.437 | 3.426 | 7.84× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
