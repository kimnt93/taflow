# McClellanSummationIndex benchmark (`McClellanSummationIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 102.21M | 0.008 | 122.11M | 8.700 | 889.20× | 1062.35× |
| 10,000 | 0.055 | 181.94M | 0.052 | 190.88M | 91.751 | 1669.31× | 1751.31× |
| 100,000 | 0.617 | 162.01M | 0.505 | 198.04M | 872.195 | 1413.08× | 1727.31× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.108 | 0.330 | 3.06× |
| 1 | 5 | 0.321 | 1.420 | 4.42× |
| 1 | 10 | 0.474 | 2.206 | 4.66× |
| 10 | 1 | 0.057 | 0.296 | 5.16× |
| 10 | 5 | 0.224 | 1.760 | 7.85× |
| 10 | 10 | 0.520 | 3.160 | 6.07× |
| 100 | 1 | 0.056 | 1.150 | 20.42× |
| 100 | 5 | 0.337 | 5.906 | 17.51× |
| 100 | 10 | 0.484 | 11.663 | 24.12× |
| 1,000 | 1 | 0.060 | 9.260 | 155.03× |
| 1,000 | 5 | 0.444 | 46.799 | 105.46× |
| 1,000 | 10 | 0.602 | 93.839 | 155.89× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
