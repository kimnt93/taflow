# HurstChannel benchmark (`HurstChannel` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.044 | 22.80M | 0.038 | 26.39M | 0.625 | 14.25× | 16.50× |
| 10,000 | 0.366 | 27.35M | 0.368 | 27.20M | 4.533 | 12.40× | 12.33× |
| 100,000 | 3.952 | 25.30M | 3.547 | 28.19M | 49.129 | 12.43× | 13.85× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.392 | 4.86× |
| 1 | 5 | 0.262 | 1.371 | 5.23× |
| 1 | 10 | 0.404 | 2.692 | 6.67× |
| 10 | 1 | 0.050 | 0.255 | 5.14× |
| 10 | 5 | 0.196 | 1.420 | 7.23× |
| 10 | 10 | 0.434 | 2.863 | 6.60× |
| 100 | 1 | 0.061 | 0.303 | 4.97× |
| 100 | 5 | 0.195 | 1.704 | 8.73× |
| 100 | 10 | 0.438 | 3.082 | 7.04× |
| 1,000 | 1 | 0.091 | 0.938 | 10.34× |
| 1,000 | 5 | 0.212 | 4.152 | 19.63× |
| 1,000 | 10 | 0.517 | 14.942 | 28.88× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
