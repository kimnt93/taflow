# RollingMaximum benchmark (`MAX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 151.28M | 0.005 | 183.34M | 0.038 | 5.76× | 6.98× |
| 10,000 | 0.037 | 266.68M | 0.036 | 279.67M | 0.088 | 2.33× | 2.45× |
| 100,000 | 0.378 | 264.81M | 0.376 | 266.22M | 0.546 | 1.44× | 1.45× |
| 1,000,000 | 4.804 | 208.15M | 4.333 | 230.79M | 5.267 | 1.10× | 1.22× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.062 | 0.121 | 1.93× |
| 1 | 5 | 0.258 | 0.481 | 1.87× |
| 1 | 10 | 0.535 | 1.140 | 2.13× |
| 10 | 1 | 0.057 | 0.104 | 1.83× |
| 10 | 5 | 0.242 | 0.487 | 2.02× |
| 10 | 10 | 0.502 | 1.188 | 2.37× |
| 100 | 1 | 0.065 | 0.115 | 1.77× |
| 100 | 5 | 0.288 | 0.543 | 1.88× |
| 100 | 10 | 0.574 | 1.138 | 1.98× |
| 1,000 | 1 | 0.057 | 0.117 | 2.03× |
| 1,000 | 5 | 0.290 | 0.521 | 1.80× |
| 1,000 | 10 | 0.604 | 1.091 | 1.81× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
