# PreviousHighLow benchmark (`previous-session high-low` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 71.27M | 0.011 | 91.33M | 0.583 | 41.53× | 53.22× |
| 10,000 | 0.105 | 95.12M | 0.093 | 107.99M | 5.832 | 55.47× | 62.98× |
| 100,000 | 1.046 | 95.63M | 0.889 | 112.43M | 58.164 | 55.62× | 65.40× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.060 | 0.087 | 1.46× |
| 1 | 5 | 0.235 | 0.336 | 1.43× |
| 1 | 10 | 0.381 | 0.705 | 1.85× |
| 10 | 1 | 0.044 | 0.082 | 1.89× |
| 10 | 5 | 0.190 | 0.350 | 1.84× |
| 10 | 10 | 0.398 | 0.754 | 1.89× |
| 100 | 1 | 0.044 | 0.125 | 2.85× |
| 100 | 5 | 0.211 | 0.656 | 3.11× |
| 100 | 10 | 0.416 | 1.274 | 3.06× |
| 1,000 | 1 | 0.058 | 0.655 | 11.20× |
| 1,000 | 5 | 0.206 | 3.383 | 16.43× |
| 1,000 | 10 | 0.449 | 6.922 | 15.42× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
