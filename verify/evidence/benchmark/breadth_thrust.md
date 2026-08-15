# BreadthThrust benchmark (`BreadthThrust` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 131.33M | 0.006 | 154.32M | 8.562 | 1124.44× | 1321.34× |
| 10,000 | 0.058 | 171.26M | 0.055 | 180.29M | 83.515 | 1430.28× | 1505.69× |
| 100,000 | 0.557 | 179.67M | 0.529 | 189.18M | 838.178 | 1505.99× | 1585.63× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.125 | 0.308 | 2.47× |
| 1 | 5 | 0.221 | 1.185 | 5.37× |
| 1 | 10 | 0.462 | 2.655 | 5.75× |
| 10 | 1 | 0.042 | 0.323 | 7.69× |
| 10 | 5 | 0.191 | 1.707 | 8.95× |
| 10 | 10 | 0.386 | 3.475 | 9.01× |
| 100 | 1 | 0.047 | 1.088 | 23.31× |
| 100 | 5 | 0.197 | 5.602 | 28.43× |
| 100 | 10 | 0.423 | 11.602 | 27.45× |
| 1,000 | 1 | 0.062 | 8.836 | 143.02× |
| 1,000 | 5 | 0.271 | 44.943 | 166.08× |
| 1,000 | 10 | 0.592 | 89.949 | 152.00× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
