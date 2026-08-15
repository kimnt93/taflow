# RelativeStrengthIndex benchmark (`RSI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 126.76M | 0.007 | 145.25M | 0.038 | 4.83× | 5.54× |
| 10,000 | 0.066 | 151.32M | 0.063 | 157.52M | 0.087 | 1.31× | 1.36× |
| 100,000 | 0.631 | 158.54M | 0.614 | 162.96M | 0.596 | 0.94× | 0.97× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.075 | 0.104 | 1.40× |
| 1 | 5 | 0.338 | 0.473 | 1.40× |
| 1 | 10 | 0.387 | 0.936 | 2.42× |
| 10 | 1 | 0.042 | 0.090 | 2.15× |
| 10 | 5 | 0.171 | 0.439 | 2.56× |
| 10 | 10 | 0.414 | 0.955 | 2.30× |
| 100 | 1 | 0.045 | 0.091 | 2.00× |
| 100 | 5 | 0.179 | 0.445 | 2.49× |
| 100 | 10 | 0.408 | 0.974 | 2.39× |
| 1,000 | 1 | 0.050 | 0.103 | 2.09× |
| 1,000 | 5 | 0.206 | 0.485 | 2.36× |
| 1,000 | 10 | 0.381 | 0.973 | 2.56× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
