# RogersSatchell benchmark (`RogersSatchellVolatility` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.035 | 28.97M | 0.030 | 33.41M | 0.307 | 8.88× | 10.25× |
| 10,000 | 0.343 | 29.14M | 0.282 | 35.49M | 1.553 | 4.52× | 5.51× |
| 100,000 | 2.997 | 33.37M | 2.706 | 36.95M | 14.496 | 4.84× | 5.36× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.117 | 0.253 | 2.16× |
| 1 | 5 | 0.286 | 1.453 | 5.09× |
| 1 | 10 | 0.418 | 2.519 | 6.03× |
| 10 | 1 | 0.046 | 0.235 | 5.06× |
| 10 | 5 | 0.190 | 1.506 | 7.92× |
| 10 | 10 | 0.415 | 2.660 | 6.41× |
| 100 | 1 | 0.053 | 0.251 | 4.73× |
| 100 | 5 | 0.206 | 1.473 | 7.16× |
| 100 | 10 | 0.402 | 2.681 | 6.68× |
| 1,000 | 1 | 0.073 | 0.385 | 5.25× |
| 1,000 | 5 | 0.225 | 2.174 | 9.64× |
| 1,000 | 10 | 0.450 | 4.182 | 9.30× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
