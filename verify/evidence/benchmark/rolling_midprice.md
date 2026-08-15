# RollingMidprice benchmark (`MIDPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 152.50M | 0.005 | 186.34M | 0.040 | 6.14× | 7.50× |
| 10,000 | 0.049 | 205.93M | 0.043 | 231.87M | 0.110 | 2.26× | 2.54× |
| 100,000 | 0.435 | 230.01M | 0.420 | 238.35M | 0.746 | 1.72× | 1.78× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.075 | 0.180 | 2.39× |
| 1 | 5 | 0.286 | 0.541 | 1.89× |
| 1 | 10 | 0.414 | 0.953 | 2.30× |
| 10 | 1 | 0.044 | 0.093 | 2.12× |
| 10 | 5 | 0.184 | 0.450 | 2.44× |
| 10 | 10 | 0.396 | 1.001 | 2.53× |
| 100 | 1 | 0.048 | 0.093 | 1.97× |
| 100 | 5 | 0.188 | 0.434 | 2.30× |
| 100 | 10 | 0.417 | 0.945 | 2.27× |
| 1,000 | 1 | 0.048 | 0.123 | 2.58× |
| 1,000 | 5 | 0.213 | 0.501 | 2.36× |
| 1,000 | 10 | 0.413 | 1.015 | 2.46× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
