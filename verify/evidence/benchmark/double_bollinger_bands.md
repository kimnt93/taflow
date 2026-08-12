# DoubleBollingerBands benchmark (`DoubleBollinger` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.045 | 22.28M | 0.044 | 22.89M | 0.627 | 13.96× | 14.35× |
| 10,000 | 0.399 | 25.07M | 0.396 | 25.24M | 4.504 | 11.29× | 11.37× |
| 100,000 | 4.096 | 24.41M | 3.952 | 25.31M | 49.745 | 12.14× | 12.59× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.067 | 0.288 | 4.26× |
| 1 | 5 | 0.259 | 1.461 | 5.65× |
| 1 | 10 | 0.465 | 2.865 | 6.16× |
| 10 | 1 | 0.055 | 0.268 | 4.91× |
| 10 | 5 | 0.283 | 1.394 | 4.92× |
| 10 | 10 | 0.472 | 2.912 | 6.16× |
| 100 | 1 | 0.061 | 0.301 | 4.93× |
| 100 | 5 | 0.241 | 1.592 | 6.62× |
| 100 | 10 | 0.560 | 3.334 | 5.96× |
| 1,000 | 1 | 0.095 | 0.879 | 9.29× |
| 1,000 | 5 | 0.244 | 3.881 | 15.91× |
| 1,000 | 10 | 0.578 | 7.964 | 13.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
