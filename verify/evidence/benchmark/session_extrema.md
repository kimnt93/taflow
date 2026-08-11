# SessionExtrema benchmark (`explicit-session extrema` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 90.40M | 0.009 | 107.32M | 0.515 | 46.54× | 55.25× |
| 10,000 | 0.060 | 167.34M | 0.052 | 192.73M | 4.743 | 79.37× | 91.41× |
| 100,000 | 0.599 | 167.02M | 0.457 | 218.95M | 48.993 | 81.83× | 107.27× |
| 1,000,000 | 5.949 | 168.11M | 5.312 | 188.25M | 470.864 | 79.16× | 88.64× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.105 | 0.100 | 0.95× |
| 1 | 5 | 0.414 | 0.317 | 0.77× |
| 1 | 10 | 0.493 | 0.627 | 1.27× |
| 10 | 1 | 0.050 | 0.076 | 1.51× |
| 10 | 5 | 0.230 | 0.333 | 1.44× |
| 10 | 10 | 0.498 | 0.707 | 1.42× |
| 100 | 1 | 0.055 | 0.109 | 1.97× |
| 100 | 5 | 0.240 | 0.573 | 2.39× |
| 100 | 10 | 0.510 | 1.167 | 2.29× |
| 1,000 | 1 | 0.062 | 0.588 | 9.53× |
| 1,000 | 5 | 0.272 | 3.068 | 11.28× |
| 1,000 | 10 | 0.560 | 5.881 | 10.50× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
