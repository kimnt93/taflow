# TrueStrengthIndex benchmark (`TSI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 112.83M | 0.008 | 127.30M | 0.209 | 23.63× | 26.66× |
| 10,000 | 0.066 | 150.78M | 0.065 | 154.71M | 0.596 | 8.98× | 9.21× |
| 100,000 | 0.639 | 156.51M | 0.613 | 163.11M | 4.560 | 7.14× | 7.44× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.100 | 0.259 | 2.59× |
| 1 | 5 | 0.214 | 1.380 | 6.43× |
| 1 | 10 | 0.392 | 2.761 | 7.04× |
| 10 | 1 | 0.054 | 0.251 | 4.69× |
| 10 | 5 | 0.191 | 1.407 | 7.38× |
| 10 | 10 | 0.410 | 2.589 | 6.32× |
| 100 | 1 | 0.049 | 0.250 | 5.11× |
| 100 | 5 | 0.199 | 1.388 | 6.97× |
| 100 | 10 | 0.431 | 2.777 | 6.44× |
| 1,000 | 1 | 0.054 | 0.284 | 5.23× |
| 1,000 | 5 | 0.205 | 1.649 | 8.06× |
| 1,000 | 10 | 0.427 | 2.965 | 6.95× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
