# TwiggsMoneyFlow benchmark (`TwiggsMoneyFlow` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 72.46M | 0.011 | 93.30M | 0.278 | 20.17× | 25.98× |
| 10,000 | 0.108 | 92.20M | 0.096 | 104.43M | 1.500 | 13.83× | 15.66× |
| 100,000 | 0.932 | 107.27M | 0.920 | 108.74M | 13.329 | 14.30× | 14.49× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.067 | 0.276 | 4.13× |
| 1 | 5 | 0.228 | 1.135 | 4.98× |
| 1 | 10 | 0.401 | 2.530 | 6.31× |
| 10 | 1 | 0.049 | 0.221 | 4.55× |
| 10 | 5 | 0.191 | 1.012 | 5.29× |
| 10 | 10 | 0.449 | 2.265 | 5.04× |
| 100 | 1 | 0.048 | 0.219 | 4.61× |
| 100 | 5 | 0.206 | 1.303 | 6.34× |
| 100 | 10 | 0.431 | 2.371 | 5.50× |
| 1,000 | 1 | 0.055 | 0.339 | 6.19× |
| 1,000 | 5 | 0.213 | 2.013 | 9.46× |
| 1,000 | 10 | 0.417 | 3.619 | 8.68× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
