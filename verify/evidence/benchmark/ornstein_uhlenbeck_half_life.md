# OrnsteinUhlenbeckHalfLife benchmark (`rolling OU half life` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.047 | 21.50M | 0.048 | 20.94M | 0.291 | 6.25× | 6.09× |
| 10,000 | 0.459 | 21.80M | 0.459 | 21.79M | 1.659 | 3.61× | 3.61× |
| 100,000 | 4.624 | 21.62M | 4.689 | 21.33M | 18.854 | 4.08× | 4.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.080 | 0.118 | 1.49× |
| 1 | 5 | 0.270 | 0.415 | 1.54× |
| 1 | 10 | 0.416 | 0.870 | 2.09× |
| 10 | 1 | 0.048 | 0.084 | 1.75× |
| 10 | 5 | 0.200 | 0.421 | 2.10× |
| 10 | 10 | 0.394 | 0.820 | 2.08× |
| 100 | 1 | 0.049 | 0.237 | 4.89× |
| 100 | 5 | 0.197 | 1.287 | 6.52× |
| 100 | 10 | 0.430 | 2.620 | 6.09× |
| 1,000 | 1 | 0.115 | 0.405 | 3.51× |
| 1,000 | 5 | 0.209 | 1.552 | 7.43× |
| 1,000 | 10 | 0.461 | 3.365 | 7.31× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
