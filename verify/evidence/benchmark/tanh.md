# MathTanh benchmark (`TANH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.038 | 26.53M | 0.022 | 45.34M | 0.029 | 0.77× | 1.31× |
| 10,000 | 0.153 | 65.46M | 0.140 | 71.40M | 0.053 | 0.35× | 0.38× |
| 100,000 | 1.329 | 75.26M | 1.416 | 70.63M | 0.292 | 0.22× | 0.21× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.140 | 0.120 | 0.85× |
| 1 | 5 | 0.337 | 0.419 | 1.24× |
| 1 | 10 | 0.584 | 0.870 | 1.49× |
| 10 | 1 | 0.062 | 0.087 | 1.40× |
| 10 | 5 | 0.265 | 0.419 | 1.58× |
| 10 | 10 | 0.584 | 0.865 | 1.48× |
| 100 | 1 | 0.064 | 0.082 | 1.27× |
| 100 | 5 | 0.292 | 0.420 | 1.44× |
| 100 | 10 | 0.592 | 0.863 | 1.46× |
| 1,000 | 1 | 0.077 | 0.091 | 1.19× |
| 1,000 | 5 | 0.275 | 0.398 | 1.45× |
| 1,000 | 10 | 0.599 | 0.911 | 1.52× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
