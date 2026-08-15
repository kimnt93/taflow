# Crossunder benchmark (`causal crossunder` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 384.63M | 0.001 | 855.10M | 0.019 | 7.45× | 16.56× |
| 10,000 | 0.009 | 1.17G | 0.005 | 1.93G | 0.028 | 3.28× | 5.42× |
| 100,000 | 0.068 | 1.48G | 0.048 | 2.10G | 0.144 | 2.13× | 3.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.103 | 0.085 | 0.83× |
| 1 | 5 | 0.223 | 0.345 | 1.55× |
| 1 | 10 | 0.402 | 0.677 | 1.68× |
| 10 | 1 | 0.039 | 0.069 | 1.75× |
| 10 | 5 | 0.213 | 0.349 | 1.64× |
| 10 | 10 | 0.438 | 0.750 | 1.71× |
| 100 | 1 | 0.041 | 0.063 | 1.52× |
| 100 | 5 | 0.245 | 0.392 | 1.60× |
| 100 | 10 | 0.478 | 0.786 | 1.64× |
| 1,000 | 1 | 0.042 | 0.073 | 1.74× |
| 1,000 | 5 | 0.212 | 0.440 | 2.08× |
| 1,000 | 10 | 0.437 | 1.094 | 2.50× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
