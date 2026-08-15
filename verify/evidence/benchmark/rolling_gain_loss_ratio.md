# RollingGainLossRatio benchmark (`GainLossRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 42.92M | 0.022 | 45.24M | 0.163 | 7.00× | 7.37× |
| 10,000 | 0.223 | 44.85M | 0.218 | 45.95M | 0.540 | 2.42× | 2.48× |
| 100,000 | 2.228 | 44.88M | 2.135 | 46.83M | 4.466 | 2.00× | 2.09× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.073 | 0.237 | 3.24× |
| 1 | 5 | 0.267 | 0.977 | 3.66× |
| 1 | 10 | 0.391 | 2.050 | 5.25× |
| 10 | 1 | 0.046 | 0.208 | 4.53× |
| 10 | 5 | 0.212 | 0.973 | 4.59× |
| 10 | 10 | 0.386 | 2.125 | 5.50× |
| 100 | 1 | 0.062 | 0.203 | 3.25× |
| 100 | 5 | 0.202 | 0.940 | 4.65× |
| 100 | 10 | 0.412 | 2.168 | 5.26× |
| 1,000 | 1 | 0.079 | 0.245 | 3.11× |
| 1,000 | 5 | 0.218 | 1.208 | 5.54× |
| 1,000 | 10 | 0.434 | 2.618 | 6.03× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
