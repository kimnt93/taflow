# DirectionalMovementIndex benchmark (`DX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 102.01M | 0.007 | 140.38M | 0.042 | 4.31× | 5.93× |
| 10,000 | 0.067 | 148.32M | 0.069 | 145.63M | 0.121 | 1.79× | 1.76× |
| 100,000 | 0.630 | 158.61M | 0.640 | 156.15M | 0.855 | 1.36× | 1.34× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.137 | 0.119 | 0.87× |
| 1 | 5 | 0.312 | 0.501 | 1.61× |
| 1 | 10 | 0.406 | 0.931 | 2.30× |
| 10 | 1 | 0.044 | 0.093 | 2.12× |
| 10 | 5 | 0.189 | 0.458 | 2.43× |
| 10 | 10 | 0.382 | 0.909 | 2.38× |
| 100 | 1 | 0.044 | 0.093 | 2.13× |
| 100 | 5 | 0.181 | 0.434 | 2.40× |
| 100 | 10 | 0.399 | 0.947 | 2.38× |
| 1,000 | 1 | 0.048 | 0.105 | 2.19× |
| 1,000 | 5 | 0.194 | 0.493 | 2.54× |
| 1,000 | 10 | 0.411 | 1.017 | 2.47× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
