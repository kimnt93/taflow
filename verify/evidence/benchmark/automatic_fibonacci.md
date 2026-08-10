# AutomaticFibonacci benchmark (`AutoFib` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.027 | 36.68M | 0.022 | 44.83M | 0.636 | 23.32× | 28.50× |
| 10,000 | 0.234 | 42.77M | 0.213 | 46.91M | 5.232 | 22.38× | 24.54× |
| 100,000 | 2.433 | 41.10M | 2.028 | 49.32M | 61.962 | 25.47× | 30.56× |
| 1,000,000 | 26.293 | 38.03M | 22.509 | 44.43M | 649.112 | 24.69× | 28.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.091 | 0.207 | 2.29× |
| 1 | 5 | 0.261 | 0.840 | 3.22× |
| 1 | 10 | 0.481 | 1.807 | 3.76× |
| 10 | 1 | 0.051 | 0.169 | 3.29× |
| 10 | 5 | 0.240 | 0.853 | 3.55× |
| 10 | 10 | 0.487 | 1.883 | 3.87× |
| 100 | 1 | 0.053 | 0.230 | 4.38× |
| 100 | 5 | 0.256 | 1.123 | 4.39× |
| 100 | 10 | 0.514 | 2.447 | 4.76× |
| 1,000 | 1 | 0.079 | 0.959 | 12.18× |
| 1,000 | 5 | 0.257 | 4.115 | 15.98× |
| 1,000 | 10 | 0.530 | 8.250 | 15.57× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
