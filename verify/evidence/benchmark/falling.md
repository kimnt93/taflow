# Falling benchmark (`period-over-period falling` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 506.00M | 0.001 | 909.77M | 0.033 | 16.94× | 30.46× |
| 10,000 | 0.007 | 1.45G | 0.004 | 2.27G | 0.042 | 6.08× | 9.48× |
| 100,000 | 0.078 | 1.27G | 0.055 | 1.81G | 0.139 | 1.77× | 2.51× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.077 | 0.145 | 1.87× |
| 1 | 5 | 0.250 | 0.462 | 1.85× |
| 1 | 10 | 0.416 | 1.023 | 2.46× |
| 10 | 1 | 0.053 | 0.102 | 1.92× |
| 10 | 5 | 0.219 | 0.512 | 2.34× |
| 10 | 10 | 0.421 | 0.994 | 2.36× |
| 100 | 1 | 0.046 | 0.087 | 1.90× |
| 100 | 5 | 0.198 | 0.488 | 2.47× |
| 100 | 10 | 0.423 | 0.936 | 2.21× |
| 1,000 | 1 | 0.044 | 0.097 | 2.17× |
| 1,000 | 5 | 0.202 | 0.494 | 2.45× |
| 1,000 | 10 | 0.389 | 1.154 | 2.96× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
