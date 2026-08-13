# HeadAndShoulders benchmark (`HeadAndShoulders` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.057 | 17.62M | 0.048 | 20.98M | 0.212 | 3.74× | 4.45× |
| 10,000 | 0.379 | 26.37M | 0.389 | 25.68M | 1.297 | 3.42× | 3.33× |
| 100,000 | 3.755 | 26.63M | 3.547 | 28.20M | 11.874 | 3.16× | 3.35× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.123 | 0.240 | 1.95× |
| 1 | 5 | 0.560 | 0.865 | 1.54× |
| 1 | 10 | 0.667 | 1.738 | 2.60× |
| 10 | 1 | 0.079 | 0.171 | 2.17× |
| 10 | 5 | 0.329 | 1.119 | 3.40× |
| 10 | 10 | 0.647 | 1.685 | 2.61× |
| 100 | 1 | 0.074 | 0.181 | 2.44× |
| 100 | 5 | 0.335 | 1.146 | 3.43× |
| 100 | 10 | 0.672 | 1.772 | 2.64× |
| 1,000 | 1 | 0.112 | 0.291 | 2.59× |
| 1,000 | 5 | 0.314 | 1.701 | 5.42× |
| 1,000 | 10 | 0.686 | 2.911 | 4.24× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
