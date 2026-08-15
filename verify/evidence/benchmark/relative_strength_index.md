# RelativeStrengthIndex benchmark (`RSI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 125.48M | 0.007 | 140.33M | 0.036 | 4.46× | 4.99× |
| 10,000 | 0.065 | 154.33M | 0.062 | 160.07M | 0.097 | 1.50× | 1.55× |
| 100,000 | 0.634 | 157.60M | 0.601 | 166.43M | 0.566 | 0.89× | 0.94× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.071 | 0.140 | 1.96× |
| 1 | 5 | 0.257 | 0.532 | 2.07× |
| 1 | 10 | 0.420 | 0.947 | 2.25× |
| 10 | 1 | 0.046 | 0.099 | 2.15× |
| 10 | 5 | 0.194 | 0.455 | 2.35× |
| 10 | 10 | 0.385 | 0.956 | 2.48× |
| 100 | 1 | 0.045 | 0.092 | 2.03× |
| 100 | 5 | 0.192 | 0.452 | 2.36× |
| 100 | 10 | 0.403 | 0.949 | 2.36× |
| 1,000 | 1 | 0.047 | 0.096 | 2.03× |
| 1,000 | 5 | 0.199 | 0.477 | 2.40× |
| 1,000 | 10 | 0.420 | 0.977 | 2.32× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
