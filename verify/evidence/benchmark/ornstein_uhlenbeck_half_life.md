# OrnsteinUhlenbeckHalfLife benchmark (`rolling OU half life` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.047 | 21.14M | 0.049 | 20.58M | 0.271 | 5.73× | 5.58× |
| 10,000 | 0.454 | 22.04M | 0.463 | 21.58M | 1.668 | 3.68× | 3.60× |
| 100,000 | 4.428 | 22.58M | 4.924 | 20.31M | 22.207 | 5.02× | 4.51× |
| 1,000,000 | 45.395 | 22.03M | 46.590 | 21.46M | 194.937 | 4.29× | 4.18× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.103 | 0.133 | 1.29× |
| 1 | 5 | 0.395 | 0.478 | 1.21× |
| 1 | 10 | 0.458 | 0.800 | 1.75× |
| 10 | 1 | 0.048 | 0.081 | 1.69× |
| 10 | 5 | 0.207 | 0.401 | 1.94× |
| 10 | 10 | 0.466 | 0.806 | 1.73× |
| 100 | 1 | 0.059 | 0.254 | 4.29× |
| 100 | 5 | 0.274 | 1.299 | 4.74× |
| 100 | 10 | 0.493 | 2.481 | 5.03× |
| 1,000 | 1 | 0.095 | 0.409 | 4.32× |
| 1,000 | 5 | 0.243 | 1.694 | 6.96× |
| 1,000 | 10 | 0.528 | 3.244 | 6.14× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
