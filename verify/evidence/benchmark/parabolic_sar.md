# ParabolicSar benchmark (`SAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.051 | 19.70M | 0.047 | 21.44M | 0.041 | 0.80× | 0.87× |
| 10,000 | 0.364 | 27.47M | 0.362 | 27.61M | 0.091 | 0.25× | 0.25× |
| 100,000 | 3.449 | 28.99M | 3.520 | 28.41M | 0.611 | 0.18× | 0.17× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.086 | 0.110 | 1.27× |
| 1 | 5 | 0.489 | 0.481 | 0.98× |
| 1 | 10 | 0.632 | 0.969 | 1.53× |
| 10 | 1 | 0.072 | 0.099 | 1.38× |
| 10 | 5 | 0.291 | 0.465 | 1.60× |
| 10 | 10 | 0.574 | 0.956 | 1.67× |
| 100 | 1 | 0.075 | 0.093 | 1.24× |
| 100 | 5 | 0.297 | 0.454 | 1.53× |
| 100 | 10 | 0.614 | 0.942 | 1.53× |
| 1,000 | 1 | 0.105 | 0.098 | 0.93× |
| 1,000 | 5 | 0.284 | 0.502 | 1.77× |
| 1,000 | 10 | 0.585 | 1.003 | 1.72× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
