# AccumulationDistributionOscillator benchmark (`ADOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 67.31M | 0.013 | 78.69M | 0.038 | 2.59× | 3.03× |
| 10,000 | 0.082 | 121.88M | 0.078 | 127.68M | 0.061 | 0.74× | 0.77× |
| 100,000 | 0.773 | 129.33M | 0.714 | 140.07M | 0.278 | 0.36× | 0.39× |
| 1,000,000 | 7.850 | 127.39M | 7.263 | 137.69M | 2.844 | 0.36× | 0.39× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.115 | 1.18× |
| 1 | 5 | 0.339 | 0.514 | 1.52× |
| 1 | 10 | 0.569 | 0.964 | 1.70× |
| 10 | 1 | 0.054 | 0.097 | 1.80× |
| 10 | 5 | 0.247 | 0.458 | 1.85× |
| 10 | 10 | 0.528 | 0.993 | 1.88× |
| 100 | 1 | 0.055 | 0.098 | 1.78× |
| 100 | 5 | 0.248 | 0.463 | 1.86× |
| 100 | 10 | 0.504 | 0.996 | 1.98× |
| 1,000 | 1 | 0.062 | 0.102 | 1.66× |
| 1,000 | 5 | 0.260 | 0.474 | 1.82× |
| 1,000 | 10 | 0.545 | 1.028 | 1.89× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
