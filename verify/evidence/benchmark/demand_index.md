# DemandIndex benchmark (`DemandIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 116.78M | 0.006 | 180.50M | 0.268 | 31.32× | 48.41× |
| 10,000 | 0.049 | 205.49M | 0.044 | 229.68M | 1.466 | 30.12× | 33.66× |
| 100,000 | 0.424 | 236.09M | 0.378 | 264.30M | 12.460 | 29.42× | 32.93× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.061 | 0.244 | 3.98× |
| 1 | 5 | 0.254 | 1.150 | 4.53× |
| 1 | 10 | 0.405 | 2.419 | 5.97× |
| 10 | 1 | 0.050 | 0.257 | 5.19× |
| 10 | 5 | 0.195 | 1.046 | 5.35× |
| 10 | 10 | 0.438 | 2.303 | 5.25× |
| 100 | 1 | 0.052 | 0.228 | 4.35× |
| 100 | 5 | 0.206 | 1.319 | 6.40× |
| 100 | 10 | 0.423 | 2.437 | 5.76× |
| 1,000 | 1 | 0.054 | 0.342 | 6.32× |
| 1,000 | 5 | 0.216 | 1.892 | 8.76× |
| 1,000 | 10 | 0.457 | 3.500 | 7.65× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
