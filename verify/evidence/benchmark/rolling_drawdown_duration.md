# RollingDrawdownDuration benchmark (`DrawdownDuration` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 259.77M | 0.003 | 349.99M | 0.123 | 32.01× | 43.12× |
| 10,000 | 0.024 | 414.34M | 0.021 | 465.77M | 0.412 | 17.05× | 19.17× |
| 100,000 | 0.228 | 437.77M | 0.196 | 509.08M | 3.272 | 14.32× | 16.66× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.092 | 0.194 | 2.11× |
| 1 | 5 | 0.333 | 0.879 | 2.64× |
| 1 | 10 | 0.392 | 1.798 | 4.59× |
| 10 | 1 | 0.048 | 0.158 | 3.28× |
| 10 | 5 | 0.227 | 0.818 | 3.61× |
| 10 | 10 | 0.393 | 1.569 | 3.99× |
| 100 | 1 | 0.050 | 0.158 | 3.19× |
| 100 | 5 | 0.203 | 1.122 | 5.53× |
| 100 | 10 | 0.374 | 1.579 | 4.23× |
| 1,000 | 1 | 0.045 | 0.185 | 4.08× |
| 1,000 | 5 | 0.231 | 1.252 | 5.42× |
| 1,000 | 10 | 0.423 | 1.912 | 4.52× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
