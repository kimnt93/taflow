# AdaptiveCycle benchmark (`AdaptiveCycle` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.064 | 15.52M | 0.063 | 15.82M | 0.193 | 2.99× | 3.05× |
| 10,000 | 0.611 | 16.37M | 0.627 | 15.96M | 0.998 | 1.63× | 1.59× |
| 100,000 | 5.998 | 16.67M | 5.904 | 16.94M | 9.178 | 1.53× | 1.55× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.106 | 0.173 | 1.62× |
| 1 | 5 | 0.314 | 1.129 | 3.60× |
| 1 | 10 | 0.482 | 1.837 | 3.81× |
| 10 | 1 | 0.051 | 0.159 | 3.11× |
| 10 | 5 | 0.220 | 0.874 | 3.98× |
| 10 | 10 | 0.484 | 1.853 | 3.83× |
| 100 | 1 | 0.059 | 0.172 | 2.91× |
| 100 | 5 | 0.232 | 0.916 | 3.95× |
| 100 | 10 | 0.511 | 1.971 | 3.86× |
| 1,000 | 1 | 0.136 | 0.258 | 1.89× |
| 1,000 | 5 | 0.314 | 1.306 | 4.16× |
| 1,000 | 10 | 0.553 | 2.680 | 4.84× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
