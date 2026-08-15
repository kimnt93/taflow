# RollingGrangerCausality benchmark (`GrangerCausality` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 1.908 | 523.97K | 1.893 | 528.21K | 7.793 | 4.08× | 4.12× |
| 10,000 | 20.867 | 479.24K | 20.239 | 494.09K | 83.198 | 3.99× | 4.11× |
| 100,000 | 201.387 | 496.56K | 200.456 | 498.86K | 831.913 | 4.13× | 4.15× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.218 | 0.435 | 2.00× |
| 1 | 5 | 0.265 | 1.320 | 4.98× |
| 1 | 10 | 0.407 | 2.600 | 6.38× |
| 10 | 1 | 0.047 | 0.235 | 5.04× |
| 10 | 5 | 0.230 | 1.369 | 5.95× |
| 10 | 10 | 0.405 | 2.551 | 6.30× |
| 100 | 1 | 0.145 | 0.632 | 4.35× |
| 100 | 5 | 0.275 | 3.328 | 12.12× |
| 100 | 10 | 0.516 | 6.593 | 12.76× |
| 1,000 | 1 | 2.096 | 8.398 | 4.01× |
| 1,000 | 5 | 3.118 | 41.909 | 13.44× |
| 1,000 | 10 | 4.387 | 90.787 | 20.70× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
