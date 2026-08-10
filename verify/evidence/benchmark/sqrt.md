# MathSqrt benchmark (`SQRT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 225.33M | 0.003 | 294.34M | 0.031 | 7.08× | 9.25× |
| 10,000 | 0.013 | 751.51M | 0.010 | 957.84M | 0.045 | 3.37× | 4.29× |
| 100,000 | 0.104 | 964.32M | 0.081 | 1.23G | 0.182 | 1.75× | 2.25× |
| 1,000,000 | 1.591 | 628.51M | 1.072 | 932.55M | 1.838 | 1.16× | 1.71× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.073 | 0.123 | 1.69× |
| 1 | 5 | 0.250 | 0.423 | 1.69× |
| 1 | 10 | 0.499 | 1.082 | 2.17× |
| 10 | 1 | 0.050 | 0.090 | 1.78× |
| 10 | 5 | 0.275 | 0.451 | 1.64× |
| 10 | 10 | 0.469 | 1.024 | 2.18× |
| 100 | 1 | 0.057 | 0.099 | 1.74× |
| 100 | 5 | 0.256 | 0.491 | 1.92× |
| 100 | 10 | 0.464 | 0.942 | 2.03× |
| 1,000 | 1 | 0.055 | 0.095 | 1.74× |
| 1,000 | 5 | 0.287 | 0.533 | 1.85× |
| 1,000 | 10 | 0.523 | 0.963 | 1.84× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
