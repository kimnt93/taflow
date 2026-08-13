# InstantaneousTrendline benchmark (`InstantaneousTrendline` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.043 | 23.32M | 0.037 | 26.85M | 0.145 | 3.38× | 3.90× |
| 10,000 | 0.306 | 32.66M | 0.305 | 32.74M | 0.542 | 1.77× | 1.78× |
| 100,000 | 3.038 | 32.92M | 2.946 | 33.94M | 3.520 | 1.16× | 1.19× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.170 | 0.214 | 1.26× |
| 1 | 5 | 0.426 | 0.978 | 2.30× |
| 1 | 10 | 0.587 | 2.097 | 3.57× |
| 10 | 1 | 0.069 | 0.190 | 2.74× |
| 10 | 5 | 0.294 | 0.963 | 3.28× |
| 10 | 10 | 0.613 | 2.054 | 3.35× |
| 100 | 1 | 0.072 | 0.210 | 2.90× |
| 100 | 5 | 0.301 | 0.953 | 3.17× |
| 100 | 10 | 0.623 | 2.098 | 3.37× |
| 1,000 | 1 | 0.096 | 0.224 | 2.34× |
| 1,000 | 5 | 0.295 | 1.127 | 3.81× |
| 1,000 | 10 | 0.671 | 2.815 | 4.20× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
