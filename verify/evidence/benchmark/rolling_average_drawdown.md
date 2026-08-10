# RollingAverageDrawdown benchmark (`AverageDrawdown` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.047 | 21.41M | 0.047 | 21.11M | 0.203 | 4.35× | 4.29× |
| 10,000 | 0.488 | 20.49M | 0.520 | 19.24M | 1.023 | 2.10× | 1.97× |
| 100,000 | 4.948 | 20.21M | 4.883 | 20.48M | 8.775 | 1.77× | 1.80× |
| 1,000,000 | 49.158 | 20.34M | 47.557 | 21.03M | 86.578 | 1.76× | 1.82× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.103 | 0.343 | 3.33× |
| 1 | 5 | 0.408 | 1.006 | 2.46× |
| 1 | 10 | 0.487 | 2.061 | 4.23× |
| 10 | 1 | 0.046 | 0.184 | 3.98× |
| 10 | 5 | 0.215 | 0.917 | 4.27× |
| 10 | 10 | 0.463 | 2.084 | 4.50× |
| 100 | 1 | 0.051 | 0.204 | 3.96× |
| 100 | 5 | 0.246 | 1.026 | 4.17× |
| 100 | 10 | 0.545 | 2.271 | 4.16× |
| 1,000 | 1 | 0.105 | 0.292 | 2.78× |
| 1,000 | 5 | 0.294 | 1.494 | 5.08× |
| 1,000 | 10 | 0.564 | 3.317 | 5.88× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
