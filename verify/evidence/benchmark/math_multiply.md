# MathMultiply benchmark (`MULT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 415.49M | 0.001 | 990.38M | 0.030 | 12.53× | 29.87× |
| 10,000 | 0.007 | 1.43G | 0.004 | 2.60G | 0.035 | 4.99× | 9.11× |
| 100,000 | 0.065 | 1.54G | 0.038 | 2.61G | 0.070 | 1.07× | 1.82× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.051 | 0.109 | 2.11× |
| 1 | 5 | 0.215 | 0.471 | 2.19× |
| 1 | 10 | 0.425 | 1.002 | 2.35× |
| 10 | 1 | 0.038 | 0.087 | 2.30× |
| 10 | 5 | 0.184 | 0.479 | 2.60× |
| 10 | 10 | 0.366 | 0.954 | 2.61× |
| 100 | 1 | 0.045 | 0.086 | 1.91× |
| 100 | 5 | 0.177 | 0.447 | 2.53× |
| 100 | 10 | 0.402 | 0.938 | 2.33× |
| 1,000 | 1 | 0.041 | 0.086 | 2.11× |
| 1,000 | 5 | 0.199 | 0.481 | 2.41× |
| 1,000 | 10 | 0.407 | 0.903 | 2.22× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
