# MathSinh benchmark (`SINH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 97.11M | 0.009 | 110.11M | 0.033 | 3.20× | 3.63× |
| 10,000 | 0.072 | 138.22M | 0.070 | 143.50M | 0.088 | 1.21× | 1.26× |
| 100,000 | 0.697 | 143.51M | 0.733 | 136.38M | 0.640 | 0.92× | 0.87× |
| 1,000,000 | 7.650 | 130.71M | 7.307 | 136.86M | 6.024 | 0.79× | 0.82× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.108 | 0.114 | 1.05× |
| 1 | 5 | 0.255 | 0.474 | 1.86× |
| 1 | 10 | 0.465 | 0.874 | 1.88× |
| 10 | 1 | 0.047 | 0.088 | 1.86× |
| 10 | 5 | 0.216 | 0.414 | 1.92× |
| 10 | 10 | 0.492 | 0.922 | 1.88× |
| 100 | 1 | 0.050 | 0.085 | 1.70× |
| 100 | 5 | 0.257 | 0.433 | 1.68× |
| 100 | 10 | 0.515 | 0.908 | 1.76× |
| 1,000 | 1 | 0.060 | 0.099 | 1.64× |
| 1,000 | 5 | 0.236 | 0.466 | 1.97× |
| 1,000 | 10 | 0.490 | 0.978 | 2.00× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
