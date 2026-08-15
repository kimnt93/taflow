# UpDownVolumeRatio benchmark (`UpDownVolumeRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 222.78M | 0.003 | 330.57M | 4.979 | 1109.31× | 1646.05× |
| 10,000 | 0.026 | 380.93M | 0.023 | 436.40M | 42.528 | 1620.02× | 1855.93× |
| 100,000 | 0.252 | 397.61M | 0.227 | 441.30M | 416.430 | 1655.77× | 1837.70× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.227 | 2.79× |
| 1 | 5 | 0.235 | 1.239 | 5.28× |
| 1 | 10 | 0.378 | 1.930 | 5.10× |
| 10 | 1 | 0.045 | 0.238 | 5.26× |
| 10 | 5 | 0.195 | 1.438 | 7.39× |
| 10 | 10 | 0.406 | 2.638 | 6.49× |
| 100 | 1 | 0.046 | 0.618 | 13.46× |
| 100 | 5 | 0.198 | 3.233 | 16.37× |
| 100 | 10 | 0.406 | 6.515 | 16.05× |
| 1,000 | 1 | 0.047 | 4.438 | 93.93× |
| 1,000 | 5 | 0.212 | 24.588 | 116.08× |
| 1,000 | 10 | 0.511 | 54.583 | 106.76× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
