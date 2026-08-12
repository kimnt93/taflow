# SessionExtrema benchmark (`explicit-session extrema` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 86.19M | 0.010 | 100.45M | 0.511 | 44.04× | 51.33× |
| 10,000 | 0.057 | 176.82M | 0.050 | 201.36M | 4.835 | 85.49× | 97.36× |
| 100,000 | 0.468 | 213.68M | 0.422 | 237.22M | 48.043 | 102.66× | 113.97× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.109 | 0.070 | 0.64× |
| 1 | 5 | 0.301 | 0.310 | 1.03× |
| 1 | 10 | 0.510 | 0.634 | 1.24× |
| 10 | 1 | 0.053 | 0.070 | 1.32× |
| 10 | 5 | 0.249 | 0.350 | 1.41× |
| 10 | 10 | 0.582 | 0.741 | 1.27× |
| 100 | 1 | 0.053 | 0.122 | 2.31× |
| 100 | 5 | 0.254 | 0.567 | 2.23× |
| 100 | 10 | 0.545 | 1.234 | 2.27× |
| 1,000 | 1 | 0.061 | 0.578 | 9.55× |
| 1,000 | 5 | 0.279 | 2.884 | 10.34× |
| 1,000 | 10 | 0.549 | 5.789 | 10.54× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
