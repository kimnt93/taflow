# VolumeRelativeStrengthIndex benchmark (`VolumeRsi` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 81.58M | 0.011 | 87.17M | 0.216 | 17.62× | 18.83× |
| 10,000 | 0.104 | 96.34M | 0.095 | 105.29M | 0.803 | 7.74× | 8.46× |
| 100,000 | 0.910 | 109.91M | 0.883 | 113.29M | 6.545 | 7.19× | 7.41× |
| 1,000,000 | 9.306 | 107.45M | 9.139 | 109.42M | 77.120 | 8.29× | 8.44× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.281 | 3.40× |
| 1 | 5 | 0.337 | 1.231 | 3.65× |
| 1 | 10 | 0.471 | 2.270 | 4.82× |
| 10 | 1 | 0.055 | 0.214 | 3.87× |
| 10 | 5 | 0.215 | 1.230 | 5.72× |
| 10 | 10 | 0.478 | 2.413 | 5.04× |
| 100 | 1 | 0.051 | 0.233 | 4.56× |
| 100 | 5 | 0.248 | 1.325 | 5.35× |
| 100 | 10 | 0.501 | 2.346 | 4.68× |
| 1,000 | 1 | 0.058 | 0.302 | 5.18× |
| 1,000 | 5 | 0.227 | 1.617 | 7.11× |
| 1,000 | 10 | 0.512 | 3.050 | 5.96× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
