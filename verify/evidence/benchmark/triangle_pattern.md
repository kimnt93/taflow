# TrianglePattern benchmark (`Triangle` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 98.81M | 0.007 | 135.84M | 0.227 | 22.41× | 30.81× |
| 10,000 | 0.111 | 90.08M | 0.094 | 106.17M | 1.443 | 13.00× | 15.33× |
| 100,000 | 0.985 | 101.52M | 0.943 | 106.03M | 12.940 | 13.14× | 13.72× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.060 | 0.257 | 4.30× |
| 1 | 5 | 0.240 | 0.819 | 3.41× |
| 1 | 10 | 0.414 | 1.696 | 4.10× |
| 10 | 1 | 0.047 | 0.164 | 3.51× |
| 10 | 5 | 0.193 | 1.101 | 5.69× |
| 10 | 10 | 0.410 | 1.713 | 4.18× |
| 100 | 1 | 0.050 | 0.175 | 3.52× |
| 100 | 5 | 0.217 | 1.154 | 5.32× |
| 100 | 10 | 0.434 | 1.831 | 4.21× |
| 1,000 | 1 | 0.055 | 0.294 | 5.33× |
| 1,000 | 5 | 0.201 | 1.815 | 9.05× |
| 1,000 | 10 | 0.427 | 3.002 | 7.02× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
