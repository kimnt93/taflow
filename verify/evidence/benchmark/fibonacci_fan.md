# FibonacciFan benchmark (`FibFan` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 64.31M | 0.013 | 76.35M | 0.488 | 31.40× | 37.27× |
| 10,000 | 0.141 | 70.73M | 0.133 | 75.44M | 4.085 | 28.89× | 30.81× |
| 100,000 | 1.470 | 68.05M | 1.324 | 75.55M | 42.886 | 29.18× | 32.40× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.069 | 0.210 | 3.05× |
| 1 | 5 | 0.314 | 0.867 | 2.76× |
| 1 | 10 | 0.401 | 1.921 | 4.78× |
| 10 | 1 | 0.045 | 0.168 | 3.72× |
| 10 | 5 | 0.193 | 0.831 | 4.30× |
| 10 | 10 | 0.392 | 1.970 | 5.03× |
| 100 | 1 | 0.050 | 0.214 | 4.31× |
| 100 | 5 | 0.201 | 1.029 | 5.13× |
| 100 | 10 | 0.466 | 2.368 | 5.08× |
| 1,000 | 1 | 0.061 | 0.747 | 12.25× |
| 1,000 | 5 | 0.197 | 3.281 | 16.61× |
| 1,000 | 10 | 0.426 | 6.416 | 15.05× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
