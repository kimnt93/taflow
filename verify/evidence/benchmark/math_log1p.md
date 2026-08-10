# MathLog1p benchmark (`numpy.log1p` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 90.58M | 0.014 | 73.34M | 0.020 | 1.80× | 1.45× |
| 10,000 | 0.080 | 124.57M | 0.077 | 130.19M | 0.090 | 1.13× | 1.18× |
| 100,000 | 0.787 | 127.10M | 0.747 | 133.91M | 0.763 | 0.97× | 1.02× |
| 1,000,000 | 9.899 | 101.02M | 7.366 | 135.76M | 7.588 | 0.77× | 1.03× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.094 | 0.082 | 0.88× |
| 1 | 5 | 0.310 | 0.275 | 0.89× |
| 1 | 10 | 0.433 | 0.558 | 1.29× |
| 10 | 1 | 0.046 | 0.060 | 1.32× |
| 10 | 5 | 0.217 | 0.280 | 1.29× |
| 10 | 10 | 0.460 | 0.555 | 1.20× |
| 100 | 1 | 0.051 | 0.055 | 1.08× |
| 100 | 5 | 0.221 | 0.287 | 1.30× |
| 100 | 10 | 0.477 | 0.574 | 1.20× |
| 1,000 | 1 | 0.055 | 0.075 | 1.36× |
| 1,000 | 5 | 0.233 | 0.344 | 1.48× |
| 1,000 | 10 | 0.526 | 0.759 | 1.44× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
