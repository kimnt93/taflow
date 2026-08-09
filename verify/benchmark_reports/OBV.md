# OnBalanceVolume benchmark (`OBV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 180.56M | 0.004 | 264.09M | 0.030 | 5.46× | 7.98× |
| 10,000 | 0.040 | 250.16M | 0.038 | 261.54M | 0.062 | 1.56× | 1.63× |
| 100,000 | 0.455 | 219.80M | 0.420 | 237.88M | 0.378 | 0.83× | 0.90× |
| 1,000,000 | 4.788 | 208.87M | 4.206 | 237.78M | 3.498 | 0.73× | 0.83× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.157 | 0.139 | 0.89× |
| 1 | 5 | 0.355 | 0.510 | 1.44× |
| 1 | 10 | 0.464 | 0.936 | 2.02× |
| 10 | 1 | 0.050 | 0.094 | 1.87× |
| 10 | 5 | 0.228 | 0.422 | 1.85× |
| 10 | 10 | 0.474 | 0.942 | 1.99× |
| 100 | 1 | 0.053 | 0.089 | 1.69× |
| 100 | 5 | 0.221 | 0.437 | 1.98× |
| 100 | 10 | 0.481 | 0.937 | 1.95× |
| 1,000 | 1 | 0.057 | 0.099 | 1.75× |
| 1,000 | 5 | 0.239 | 0.476 | 1.99× |
| 1,000 | 10 | 0.516 | 0.993 | 1.93× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
