# DecyclerOscillator benchmark (`DecyclerOscillator` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 90.77M | 0.010 | 97.57M | 0.175 | 15.93× | 17.12× |
| 10,000 | 0.081 | 123.13M | 0.094 | 106.78M | 0.527 | 6.49× | 5.63× |
| 100,000 | 0.762 | 131.20M | 0.738 | 135.57M | 4.860 | 6.38× | 6.59× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | 0.264 | 3.01× |
| 1 | 5 | 0.269 | 1.091 | 4.05× |
| 1 | 10 | 0.497 | 2.354 | 4.73× |
| 10 | 1 | 0.053 | 0.216 | 4.09× |
| 10 | 5 | 0.238 | 1.093 | 4.60× |
| 10 | 10 | 0.527 | 2.319 | 4.40× |
| 100 | 1 | 0.053 | 0.231 | 4.32× |
| 100 | 5 | 0.281 | 1.110 | 3.95× |
| 100 | 10 | 0.493 | 2.396 | 4.86× |
| 1,000 | 1 | 0.058 | 0.254 | 4.41× |
| 1,000 | 5 | 0.223 | 1.259 | 5.65× |
| 1,000 | 10 | 0.521 | 2.769 | 5.31× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
