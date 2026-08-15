# MinusDirectionalIndicator benchmark (`MINUS_DI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 64.82M | 0.013 | 77.53M | 0.045 | 2.90× | 3.47× |
| 10,000 | 0.122 | 81.82M | 0.115 | 87.15M | 0.101 | 0.83× | 0.88× |
| 100,000 | 1.163 | 86.02M | 1.116 | 89.58M | 0.688 | 0.59× | 0.62× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.115 | 1.48× |
| 1 | 5 | 0.276 | 0.499 | 1.81× |
| 1 | 10 | 0.410 | 0.991 | 2.41× |
| 10 | 1 | 0.050 | 0.101 | 2.02× |
| 10 | 5 | 0.204 | 0.517 | 2.54× |
| 10 | 10 | 0.430 | 1.035 | 2.41× |
| 100 | 1 | 0.047 | 0.091 | 1.93× |
| 100 | 5 | 0.200 | 0.450 | 2.25× |
| 100 | 10 | 0.420 | 1.005 | 2.39× |
| 1,000 | 1 | 0.062 | 0.127 | 2.04× |
| 1,000 | 5 | 0.220 | 0.517 | 2.36× |
| 1,000 | 10 | 0.423 | 1.027 | 2.43× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
