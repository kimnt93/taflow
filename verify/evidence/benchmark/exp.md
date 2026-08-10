# MathExp benchmark (`EXP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 119.31M | 0.007 | 133.66M | 0.033 | 3.93× | 4.40× |
| 10,000 | 0.057 | 174.86M | 0.054 | 184.33M | 0.081 | 1.42× | 1.49× |
| 100,000 | 0.534 | 187.25M | 0.525 | 190.41M | 0.523 | 0.98× | 1.00× |
| 1,000,000 | 5.677 | 176.15M | 5.228 | 191.26M | 4.954 | 0.87× | 0.95× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.145 | 0.104 | 0.71× |
| 1 | 5 | 0.255 | 0.429 | 1.69× |
| 1 | 10 | 0.447 | 0.834 | 1.87× |
| 10 | 1 | 0.049 | 0.085 | 1.73× |
| 10 | 5 | 0.219 | 0.433 | 1.98× |
| 10 | 10 | 0.499 | 0.862 | 1.73× |
| 100 | 1 | 0.046 | 0.088 | 1.92× |
| 100 | 5 | 0.211 | 0.398 | 1.89× |
| 100 | 10 | 0.489 | 0.944 | 1.93× |
| 1,000 | 1 | 0.056 | 0.086 | 1.53× |
| 1,000 | 5 | 0.251 | 0.447 | 1.78× |
| 1,000 | 10 | 0.484 | 0.961 | 1.98× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
