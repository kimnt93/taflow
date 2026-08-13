# CandleThreeWhiteSoldiers benchmark (`CDL3WHITESOLDIERS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.189 | 5.29M | 0.184 | 5.44M | 0.042 | 0.22× | 0.23× |
| 10,000 | 1.763 | 5.67M | 1.786 | 5.60M | 0.181 | 0.10× | 0.10× |
| 100,000 | 17.071 | 5.86M | 17.376 | 5.76M | 1.553 | 0.09× | 0.09× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.110 | 0.162 | 1.47× |
| 1 | 5 | 0.401 | 0.478 | 1.19× |
| 1 | 10 | 0.632 | 0.908 | 1.44× |
| 10 | 1 | 0.067 | 0.087 | 1.30× |
| 10 | 5 | 0.309 | 0.418 | 1.36× |
| 10 | 10 | 0.629 | 0.939 | 1.49× |
| 100 | 1 | 0.088 | 0.093 | 1.06× |
| 100 | 5 | 0.311 | 0.436 | 1.40× |
| 100 | 10 | 0.668 | 0.906 | 1.36× |
| 1,000 | 1 | 0.252 | 0.105 | 0.42× |
| 1,000 | 5 | 0.537 | 0.504 | 0.94× |
| 1,000 | 10 | 0.827 | 1.065 | 1.29× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
