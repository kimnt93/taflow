# HilbertTransformPhasor benchmark (`HT_PHASOR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.047 | 21.36M | 0.047 | 21.41M | 0.078 | 1.67× | 1.67× |
| 10,000 | 0.435 | 22.97M | 0.432 | 23.15M | 0.442 | 1.02× | 1.02× |
| 100,000 | 4.465 | 22.40M | 4.348 | 23.00M | 4.186 | 0.94× | 0.96× |
| 1,000,000 | 44.468 | 22.49M | 43.388 | 23.05M | 44.626 | 1.00× | 1.03× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.126 | 0.104 | 0.83× |
| 1 | 5 | 0.301 | 0.481 | 1.60× |
| 1 | 10 | 0.443 | 0.945 | 2.13× |
| 10 | 1 | 0.052 | 0.091 | 1.75× |
| 10 | 5 | 0.239 | 0.463 | 1.94× |
| 10 | 10 | 0.462 | 0.944 | 2.04× |
| 100 | 1 | 0.052 | 0.093 | 1.81× |
| 100 | 5 | 0.240 | 0.466 | 1.94× |
| 100 | 10 | 0.458 | 0.994 | 2.17× |
| 1,000 | 1 | 0.097 | 0.133 | 1.37× |
| 1,000 | 5 | 0.250 | 0.692 | 2.77× |
| 1,000 | 10 | 0.516 | 1.397 | 2.71× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
