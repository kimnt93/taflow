# MathRadians benchmark (`numpy.radians` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 301.91M | 0.003 | 392.43M | 0.013 | 4.02× | 5.22× |
| 10,000 | 0.007 | 1.35G | 0.005 | 2.17G | 0.023 | 3.16× | 5.07× |
| 100,000 | 0.053 | 1.89G | 0.029 | 3.40G | 0.123 | 2.32× | 4.17× |
| 1,000,000 | 0.839 | 1.19G | 0.487 | 2.05G | 1.268 | 1.51× | 2.60× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.065 | 0.79× |
| 1 | 5 | 0.282 | 0.269 | 0.96× |
| 1 | 10 | 0.467 | 0.563 | 1.20× |
| 10 | 1 | 0.043 | 0.053 | 1.23× |
| 10 | 5 | 0.221 | 0.273 | 1.23× |
| 10 | 10 | 0.450 | 0.570 | 1.27× |
| 100 | 1 | 0.046 | 0.056 | 1.21× |
| 100 | 5 | 0.215 | 0.274 | 1.28× |
| 100 | 10 | 0.454 | 0.574 | 1.26× |
| 1,000 | 1 | 0.047 | 0.055 | 1.19× |
| 1,000 | 5 | 0.241 | 0.280 | 1.16× |
| 1,000 | 10 | 0.513 | 0.611 | 1.19× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
