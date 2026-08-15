# SignalDelay benchmark (`signal delay` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 536.68M | 0.001 | 981.90M | 0.026 | 13.97× | 25.55× |
| 10,000 | 0.007 | 1.53G | 0.004 | 2.66G | 0.029 | 4.47× | 7.78× |
| 100,000 | 0.065 | 1.54G | 0.041 | 2.44G | 0.066 | 1.01× | 1.61× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.055 | 0.098 | 1.77× |
| 1 | 5 | 0.189 | 0.461 | 2.43× |
| 1 | 10 | 0.412 | 0.848 | 2.06× |
| 10 | 1 | 0.043 | 0.086 | 2.01× |
| 10 | 5 | 0.179 | 0.411 | 2.29× |
| 10 | 10 | 0.411 | 0.917 | 2.23× |
| 100 | 1 | 0.046 | 0.084 | 1.83× |
| 100 | 5 | 0.189 | 0.419 | 2.22× |
| 100 | 10 | 0.386 | 0.868 | 2.25× |
| 1,000 | 1 | 0.040 | 0.085 | 2.13× |
| 1,000 | 5 | 0.216 | 0.483 | 2.24× |
| 1,000 | 10 | 0.395 | 0.931 | 2.36× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
