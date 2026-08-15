# RogersSatchell benchmark (`RogersSatchellVolatility` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.034 | 29.34M | 0.030 | 32.84M | 0.345 | 10.12× | 11.33× |
| 10,000 | 0.302 | 33.06M | 0.284 | 35.23M | 1.618 | 5.35× | 5.70× |
| 100,000 | 2.864 | 34.91M | 2.932 | 34.11M | 15.057 | 5.26× | 5.14× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.192 | 0.322 | 1.67× |
| 1 | 5 | 0.284 | 1.410 | 4.97× |
| 1 | 10 | 0.406 | 2.624 | 6.46× |
| 10 | 1 | 0.045 | 0.237 | 5.32× |
| 10 | 5 | 0.202 | 1.577 | 7.79× |
| 10 | 10 | 0.421 | 2.709 | 6.43× |
| 100 | 1 | 0.054 | 0.248 | 4.60× |
| 100 | 5 | 0.201 | 1.562 | 7.79× |
| 100 | 10 | 0.429 | 2.647 | 6.17× |
| 1,000 | 1 | 0.080 | 0.461 | 5.77× |
| 1,000 | 5 | 0.214 | 2.306 | 10.76× |
| 1,000 | 10 | 0.438 | 4.351 | 9.94× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
