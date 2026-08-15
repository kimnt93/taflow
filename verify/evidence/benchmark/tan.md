# MathTan benchmark (`TAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 60.15M | 0.015 | 65.84M | 0.050 | 3.02× | 3.30× |
| 10,000 | 0.206 | 48.57M | 0.210 | 47.68M | 0.226 | 1.10× | 1.08× |
| 100,000 | 2.106 | 47.48M | 2.145 | 46.63M | 1.967 | 0.93× | 0.92× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.142 | 0.127 | 0.90× |
| 1 | 5 | 0.251 | 0.448 | 1.78× |
| 1 | 10 | 0.390 | 0.869 | 2.23× |
| 10 | 1 | 0.043 | 0.088 | 2.07× |
| 10 | 5 | 0.203 | 0.448 | 2.21× |
| 10 | 10 | 0.412 | 0.940 | 2.28× |
| 100 | 1 | 0.054 | 0.095 | 1.77× |
| 100 | 5 | 0.218 | 0.438 | 2.00× |
| 100 | 10 | 0.496 | 0.899 | 1.81× |
| 1,000 | 1 | 0.066 | 0.107 | 1.62× |
| 1,000 | 5 | 0.212 | 0.542 | 2.56× |
| 1,000 | 10 | 0.426 | 1.140 | 2.68× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
