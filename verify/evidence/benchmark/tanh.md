# MathTanh benchmark (`TANH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 229.35M | 0.003 | 298.42M | 0.033 | 7.58× | 9.86× |
| 10,000 | 0.029 | 339.02M | 0.027 | 367.44M | 0.055 | 1.88× | 2.03× |
| 100,000 | 0.271 | 368.64M | 0.252 | 397.32M | 0.298 | 1.10× | 1.18× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.075 | 0.100 | 1.32× |
| 1 | 5 | 0.251 | 0.424 | 1.69× |
| 1 | 10 | 0.420 | 0.910 | 2.17× |
| 10 | 1 | 0.052 | 0.092 | 1.75× |
| 10 | 5 | 0.177 | 0.437 | 2.47× |
| 10 | 10 | 0.448 | 0.975 | 2.18× |
| 100 | 1 | 0.043 | 0.101 | 2.36× |
| 100 | 5 | 0.203 | 0.435 | 2.14× |
| 100 | 10 | 0.416 | 0.922 | 2.22× |
| 1,000 | 1 | 0.044 | 0.093 | 2.10× |
| 1,000 | 5 | 0.262 | 0.491 | 1.87× |
| 1,000 | 10 | 0.400 | 0.980 | 2.45× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
