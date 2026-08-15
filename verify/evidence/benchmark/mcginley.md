# McGinleyDynamic benchmark (`McGinleyDynamic` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 66.72M | 0.013 | 74.74M | 0.193 | 12.89× | 14.44× |
| 10,000 | 0.121 | 82.47M | 0.121 | 82.82M | 0.545 | 4.50× | 4.52× |
| 100,000 | 1.190 | 84.02M | 1.203 | 83.11M | 4.246 | 3.57× | 3.53× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.067 | 0.267 | 3.99× |
| 1 | 5 | 0.345 | 1.423 | 4.13× |
| 1 | 10 | 0.455 | 2.467 | 5.43× |
| 10 | 1 | 0.046 | 0.219 | 4.77× |
| 10 | 5 | 0.185 | 1.420 | 7.67× |
| 10 | 10 | 0.407 | 2.367 | 5.81× |
| 100 | 1 | 0.045 | 0.221 | 4.92× |
| 100 | 5 | 0.223 | 1.383 | 6.20× |
| 100 | 10 | 0.397 | 2.472 | 6.22× |
| 1,000 | 1 | 0.058 | 0.257 | 4.42× |
| 1,000 | 5 | 0.210 | 1.564 | 7.44× |
| 1,000 | 10 | 0.433 | 2.909 | 6.72× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
