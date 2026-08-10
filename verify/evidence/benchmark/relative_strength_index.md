# RelativeStrengthIndex benchmark (`RSI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 91.60M | 0.010 | 99.73M | 0.038 | 3.48× | 3.79× |
| 10,000 | 0.082 | 122.26M | 0.081 | 123.95M | 0.088 | 1.08× | 1.09× |
| 100,000 | 0.803 | 124.61M | 0.789 | 126.71M | 0.601 | 0.75× | 0.76× |
| 1,000,000 | 8.498 | 117.68M | 7.936 | 126.00M | 5.996 | 0.71× | 0.76× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.086 | 0.133 | 1.55× |
| 1 | 5 | 0.345 | 0.473 | 1.37× |
| 1 | 10 | 0.493 | 1.008 | 2.05× |
| 10 | 1 | 0.049 | 0.090 | 1.82× |
| 10 | 5 | 0.214 | 0.440 | 2.05× |
| 10 | 10 | 0.452 | 0.952 | 2.11× |
| 100 | 1 | 0.049 | 0.105 | 2.13× |
| 100 | 5 | 0.232 | 0.475 | 2.05× |
| 100 | 10 | 0.491 | 0.969 | 1.97× |
| 1,000 | 1 | 0.055 | 0.098 | 1.80× |
| 1,000 | 5 | 0.234 | 0.531 | 2.26× |
| 1,000 | 10 | 0.542 | 1.089 | 2.01× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
