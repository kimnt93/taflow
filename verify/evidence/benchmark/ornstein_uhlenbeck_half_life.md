# OrnsteinUhlenbeckHalfLife benchmark (`rolling OU half life` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.047 | 21.38M | 0.046 | 21.63M | 0.270 | 5.77× | 5.84× |
| 10,000 | 0.436 | 22.92M | 0.437 | 22.90M | 1.552 | 3.56× | 3.55× |
| 100,000 | 4.351 | 22.98M | 4.485 | 22.30M | 17.025 | 3.91× | 3.80× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.093 | 0.132 | 1.41× |
| 1 | 5 | 0.340 | 0.457 | 1.34× |
| 1 | 10 | 0.464 | 0.852 | 1.84× |
| 10 | 1 | 0.051 | 0.087 | 1.69× |
| 10 | 5 | 0.229 | 0.423 | 1.85× |
| 10 | 10 | 0.481 | 0.945 | 1.96× |
| 100 | 1 | 0.061 | 0.254 | 4.17× |
| 100 | 5 | 0.230 | 1.234 | 5.37× |
| 100 | 10 | 0.481 | 2.487 | 5.17× |
| 1,000 | 1 | 0.101 | 0.389 | 3.85× |
| 1,000 | 5 | 0.246 | 1.594 | 6.47× |
| 1,000 | 10 | 0.534 | 3.755 | 7.04× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
