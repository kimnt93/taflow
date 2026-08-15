# MedianPrice benchmark (`MEDPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 426.89M | 0.001 | 955.91M | 0.028 | 11.85× | 26.54× |
| 10,000 | 0.007 | 1.44G | 0.004 | 2.54G | 0.043 | 6.14× | 10.81× |
| 100,000 | 0.062 | 1.61G | 0.037 | 2.67G | 0.069 | 1.11× | 1.85× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.117 | 0.159 | 1.35× |
| 1 | 5 | 0.227 | 0.505 | 2.22× |
| 1 | 10 | 0.456 | 1.082 | 2.37× |
| 10 | 1 | 0.043 | 0.088 | 2.03× |
| 10 | 5 | 0.195 | 0.524 | 2.68× |
| 10 | 10 | 0.460 | 0.998 | 2.17× |
| 100 | 1 | 0.042 | 0.086 | 2.04× |
| 100 | 5 | 0.216 | 0.437 | 2.02× |
| 100 | 10 | 0.428 | 1.025 | 2.40× |
| 1,000 | 1 | 0.050 | 0.088 | 1.75× |
| 1,000 | 5 | 0.212 | 0.440 | 2.07× |
| 1,000 | 10 | 0.441 | 0.917 | 2.08× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
