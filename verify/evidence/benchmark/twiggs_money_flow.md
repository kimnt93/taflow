# TwiggsMoneyFlow benchmark (`TwiggsMoneyFlow` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 53.95M | 0.016 | 63.62M | 0.292 | 15.76× | 18.59× |
| 10,000 | 0.109 | 91.50M | 0.107 | 93.77M | 1.857 | 17.00× | 17.42× |
| 100,000 | 1.133 | 88.24M | 1.075 | 93.01M | 16.532 | 14.59× | 15.38× |
| 1,000,000 | 11.836 | 84.49M | 11.502 | 86.94M | 140.746 | 11.89× | 12.24× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.120 | 0.527 | 4.40× |
| 1 | 5 | 0.318 | 1.083 | 3.40× |
| 1 | 10 | 0.567 | 2.475 | 4.37× |
| 10 | 1 | 0.060 | 0.221 | 3.66× |
| 10 | 5 | 0.276 | 1.325 | 4.80× |
| 10 | 10 | 0.542 | 2.418 | 4.46× |
| 100 | 1 | 0.064 | 0.244 | 3.80× |
| 100 | 5 | 0.254 | 1.378 | 5.43× |
| 100 | 10 | 0.563 | 2.483 | 4.41× |
| 1,000 | 1 | 0.071 | 0.361 | 5.08× |
| 1,000 | 5 | 0.291 | 1.980 | 6.80× |
| 1,000 | 10 | 0.557 | 3.880 | 6.96× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
