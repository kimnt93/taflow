# DirectionalMovementIndex benchmark (`DX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.097 | 10.29M | 0.086 | 11.67M | 0.039 | 0.40× | 0.46× |
| 10,000 | 0.730 | 13.70M | 0.726 | 13.78M | 0.112 | 0.15× | 0.15× |
| 100,000 | 7.311 | 13.68M | 7.201 | 13.89M | 0.854 | 0.12× | 0.12× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.194 | 0.131 | 0.67× |
| 1 | 5 | 0.395 | 0.468 | 1.18× |
| 1 | 10 | 0.628 | 0.964 | 1.54× |
| 10 | 1 | 0.070 | 0.091 | 1.29× |
| 10 | 5 | 0.310 | 0.462 | 1.49× |
| 10 | 10 | 0.620 | 0.913 | 1.47× |
| 100 | 1 | 0.074 | 0.091 | 1.23× |
| 100 | 5 | 0.295 | 0.469 | 1.59× |
| 100 | 10 | 0.642 | 0.934 | 1.46× |
| 1,000 | 1 | 0.141 | 0.107 | 0.76× |
| 1,000 | 5 | 0.330 | 0.493 | 1.50× |
| 1,000 | 10 | 0.697 | 1.025 | 1.47× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
