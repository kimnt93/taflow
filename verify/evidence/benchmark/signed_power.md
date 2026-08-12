# SignedPower benchmark (`numpy.sign/abs/power` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 207.81M | 0.004 | 238.55M | 0.027 | 5.55× | 6.37× |
| 10,000 | 0.022 | 446.39M | 0.020 | 494.74M | 0.043 | 1.92× | 2.13× |
| 100,000 | 0.189 | 529.96M | 0.168 | 596.29M | 0.203 | 1.08× | 1.21× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.093 | 0.125 | 1.35× |
| 1 | 5 | 0.355 | 0.460 | 1.30× |
| 1 | 10 | 0.450 | 0.853 | 1.90× |
| 10 | 1 | 0.047 | 0.087 | 1.84× |
| 10 | 5 | 0.220 | 0.425 | 1.93× |
| 10 | 10 | 0.463 | 0.847 | 1.83× |
| 100 | 1 | 0.050 | 0.103 | 2.05× |
| 100 | 5 | 0.210 | 0.415 | 1.97× |
| 100 | 10 | 0.474 | 0.872 | 1.84× |
| 1,000 | 1 | 0.058 | 0.095 | 1.65× |
| 1,000 | 5 | 0.249 | 0.480 | 1.92× |
| 1,000 | 10 | 0.504 | 1.062 | 2.11× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
