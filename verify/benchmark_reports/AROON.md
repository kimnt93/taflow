# Aroon benchmark (`AROON` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 71.56M | 0.012 | 84.38M | 0.042 | 3.02× | 3.56× |
| 10,000 | 0.133 | 75.40M | 0.128 | 77.99M | 0.158 | 1.19× | 1.23× |
| 100,000 | 1.434 | 69.73M | 1.350 | 74.08M | 1.201 | 0.84× | 0.89× |
| 1,000,000 | 14.659 | 68.22M | 14.263 | 70.11M | 12.246 | 0.84× | 0.86× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.111 | 0.133 | 1.21× |
| 1 | 5 | 0.331 | 0.916 | 2.77× |
| 1 | 10 | 0.608 | 1.171 | 1.93× |
| 10 | 1 | 0.053 | 0.100 | 1.90× |
| 10 | 5 | 0.286 | 0.595 | 2.08× |
| 10 | 10 | 0.642 | 1.183 | 1.84× |
| 100 | 1 | 0.058 | 0.104 | 1.79× |
| 100 | 5 | 0.314 | 0.580 | 1.85× |
| 100 | 10 | 0.608 | 1.211 | 1.99× |
| 1,000 | 1 | 0.072 | 0.115 | 1.58× |
| 1,000 | 5 | 0.294 | 0.594 | 2.02× |
| 1,000 | 10 | 0.606 | 1.372 | 2.26× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
