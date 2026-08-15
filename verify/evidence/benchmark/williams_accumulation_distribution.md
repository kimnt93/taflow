# WilliamsAccumulationDistribution benchmark (`Wad` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 140.26M | 0.005 | 219.36M | 0.191 | 26.76× | 41.85× |
| 10,000 | 0.065 | 153.26M | 0.061 | 164.94M | 1.073 | 16.45× | 17.71× |
| 100,000 | 0.667 | 150.03M | 0.592 | 168.94M | 9.670 | 14.51× | 16.34× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.188 | 2.28× |
| 1 | 5 | 0.278 | 1.205 | 4.34× |
| 1 | 10 | 0.408 | 1.848 | 4.53× |
| 10 | 1 | 0.046 | 0.158 | 3.42× |
| 10 | 5 | 0.206 | 0.871 | 4.23× |
| 10 | 10 | 0.399 | 1.869 | 4.69× |
| 100 | 1 | 0.044 | 0.172 | 3.89× |
| 100 | 5 | 0.207 | 0.917 | 4.42× |
| 100 | 10 | 0.413 | 1.993 | 4.83× |
| 1,000 | 1 | 0.051 | 0.266 | 5.20× |
| 1,000 | 5 | 0.227 | 1.341 | 5.90× |
| 1,000 | 10 | 0.421 | 2.699 | 6.41× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
