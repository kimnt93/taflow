# OnBalanceVolume benchmark (`OBV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 229.99M | 0.003 | 332.21M | 0.032 | 7.46× | 10.77× |
| 10,000 | 0.039 | 257.33M | 0.037 | 268.76M | 0.064 | 1.66× | 1.73× |
| 100,000 | 0.441 | 226.71M | 0.415 | 241.19M | 0.407 | 0.92× | 0.98× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.158 | 2.02× |
| 1 | 5 | 0.321 | 0.510 | 1.59× |
| 1 | 10 | 0.393 | 0.908 | 2.31× |
| 10 | 1 | 0.045 | 0.087 | 1.96× |
| 10 | 5 | 0.185 | 0.428 | 2.32× |
| 10 | 10 | 0.407 | 0.938 | 2.30× |
| 100 | 1 | 0.051 | 0.083 | 1.63× |
| 100 | 5 | 0.190 | 0.427 | 2.24× |
| 100 | 10 | 0.394 | 0.909 | 2.31× |
| 1,000 | 1 | 0.050 | 0.123 | 2.46× |
| 1,000 | 5 | 0.214 | 0.440 | 2.06× |
| 1,000 | 10 | 0.425 | 0.954 | 2.24× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
