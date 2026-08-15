# FlagPennant benchmark (`FlagPennant` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 113.26M | 0.007 | 142.08M | 0.226 | 25.62× | 32.14× |
| 10,000 | 0.089 | 112.07M | 0.077 | 129.34M | 1.357 | 15.21× | 17.55× |
| 100,000 | 0.816 | 122.61M | 0.758 | 131.93M | 12.995 | 15.93× | 17.15× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.105 | 0.219 | 2.09× |
| 1 | 5 | 0.247 | 0.791 | 3.21× |
| 1 | 10 | 0.468 | 1.672 | 3.57× |
| 10 | 1 | 0.057 | 0.159 | 2.81× |
| 10 | 5 | 0.210 | 1.091 | 5.19× |
| 10 | 10 | 0.448 | 1.693 | 3.78× |
| 100 | 1 | 0.046 | 0.178 | 3.91× |
| 100 | 5 | 0.187 | 1.127 | 6.02× |
| 100 | 10 | 0.451 | 1.834 | 4.07× |
| 1,000 | 1 | 0.058 | 0.296 | 5.06× |
| 1,000 | 5 | 0.203 | 1.924 | 9.49× |
| 1,000 | 10 | 0.455 | 3.049 | 6.70× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
