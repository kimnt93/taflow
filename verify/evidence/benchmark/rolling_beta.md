# RollingBeta benchmark (`BETA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 91.44M | 0.010 | 103.07M | 0.045 | 4.08× | 4.60× |
| 10,000 | 0.064 | 156.41M | 0.060 | 166.71M | 0.119 | 1.87× | 1.99× |
| 100,000 | 0.983 | 101.71M | 0.566 | 176.81M | 0.740 | 0.75× | 1.31× |
| 1,000,000 | 6.675 | 149.81M | 6.009 | 166.42M | 7.142 | 1.07× | 1.19× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.092 | 0.117 | 1.28× |
| 1 | 5 | 0.315 | 0.676 | 2.15× |
| 1 | 10 | 0.592 | 1.148 | 1.94× |
| 10 | 1 | 0.053 | 0.092 | 1.74× |
| 10 | 5 | 0.278 | 0.596 | 2.14× |
| 10 | 10 | 0.580 | 1.107 | 1.91× |
| 100 | 1 | 0.057 | 0.093 | 1.64× |
| 100 | 5 | 0.244 | 0.529 | 2.17× |
| 100 | 10 | 0.662 | 1.162 | 1.75× |
| 1,000 | 1 | 0.060 | 0.101 | 1.69× |
| 1,000 | 5 | 0.262 | 0.668 | 2.55× |
| 1,000 | 10 | 0.649 | 1.191 | 1.84× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
