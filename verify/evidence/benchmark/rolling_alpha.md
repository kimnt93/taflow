# RollingAlpha benchmark (`Alpha` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.043 | 23.13M | 0.041 | 24.16M | 0.242 | 5.60× | 5.85× |
| 10,000 | 0.413 | 24.22M | 0.387 | 25.82M | 0.957 | 2.32× | 2.47× |
| 100,000 | 4.292 | 23.30M | 3.868 | 25.86M | 7.913 | 1.84× | 2.05× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.278 | 3.41× |
| 1 | 5 | 0.396 | 1.254 | 3.16× |
| 1 | 10 | 0.497 | 2.515 | 5.06× |
| 10 | 1 | 0.054 | 0.230 | 4.30× |
| 10 | 5 | 0.237 | 1.429 | 6.04× |
| 10 | 10 | 0.468 | 2.653 | 5.67× |
| 100 | 1 | 0.059 | 0.247 | 4.17× |
| 100 | 5 | 0.251 | 1.481 | 5.89× |
| 100 | 10 | 0.540 | 2.740 | 5.07× |
| 1,000 | 1 | 0.095 | 0.318 | 3.33× |
| 1,000 | 5 | 0.259 | 1.875 | 7.23× |
| 1,000 | 10 | 0.552 | 3.500 | 6.34× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
