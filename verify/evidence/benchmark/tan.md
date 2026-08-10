# MathTan benchmark (`TAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 57.70M | 0.017 | 60.26M | 0.046 | 2.67× | 2.79× |
| 10,000 | 0.193 | 51.76M | 0.204 | 49.04M | 0.217 | 1.13× | 1.07× |
| 100,000 | 1.924 | 51.97M | 1.891 | 52.88M | 1.994 | 1.04× | 1.05× |
| 1,000,000 | 19.982 | 50.04M | 19.466 | 51.37M | 18.975 | 0.95× | 0.97× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.111 | 0.143 | 1.28× |
| 1 | 5 | 0.264 | 0.423 | 1.60× |
| 1 | 10 | 0.446 | 0.859 | 1.93× |
| 10 | 1 | 0.050 | 0.085 | 1.70× |
| 10 | 5 | 0.226 | 0.426 | 1.89× |
| 10 | 10 | 0.523 | 0.942 | 1.80× |
| 100 | 1 | 0.052 | 0.092 | 1.78× |
| 100 | 5 | 0.243 | 0.436 | 1.79× |
| 100 | 10 | 0.473 | 0.881 | 1.86× |
| 1,000 | 1 | 0.066 | 0.110 | 1.67× |
| 1,000 | 5 | 0.244 | 0.555 | 2.28× |
| 1,000 | 10 | 0.543 | 1.167 | 2.15× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
