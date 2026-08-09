# ExponentiallyWeightedVariance benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 135.01M | 0.007 | 148.59M | 0.063 | 8.44× | 9.29× |
| 10,000 | 0.043 | 230.64M | 0.040 | 247.88M | 0.169 | 3.89× | 4.18× |
| 100,000 | 0.421 | 237.77M | 0.383 | 260.78M | 1.286 | 3.06× | 3.35× |
| 1,000,000 | 4.466 | 223.92M | 3.944 | 253.58M | 11.675 | 2.61× | 2.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.084 | 0.215 | 2.55× |
| 1 | 5 | 0.255 | 0.673 | 2.64× |
| 1 | 10 | 0.492 | 1.343 | 2.73× |
| 10 | 1 | 0.051 | 0.163 | 3.16× |
| 10 | 5 | 0.249 | 0.615 | 2.47× |
| 10 | 10 | 0.468 | 1.295 | 2.77× |
| 100 | 1 | 0.051 | 0.158 | 3.08× |
| 100 | 5 | 0.218 | 0.596 | 2.73× |
| 100 | 10 | 0.462 | 1.292 | 2.79× |
| 1,000 | 1 | 0.058 | 0.185 | 3.20× |
| 1,000 | 5 | 0.238 | 0.601 | 2.53× |
| 1,000 | 10 | 0.494 | 1.237 | 2.50× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
