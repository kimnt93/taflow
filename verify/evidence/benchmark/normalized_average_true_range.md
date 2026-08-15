# NormalizedAverageTrueRange benchmark (`NATR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 108.09M | 0.008 | 129.66M | 0.038 | 4.15× | 4.98× |
| 10,000 | 0.071 | 140.49M | 0.066 | 151.76M | 0.093 | 1.31× | 1.42× |
| 100,000 | 0.661 | 151.17M | 0.621 | 160.96M | 0.606 | 0.92× | 0.98× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.175 | 0.173 | 0.99× |
| 1 | 5 | 0.298 | 0.509 | 1.71× |
| 1 | 10 | 0.392 | 0.947 | 2.42× |
| 10 | 1 | 0.040 | 0.095 | 2.35× |
| 10 | 5 | 0.180 | 0.502 | 2.79× |
| 10 | 10 | 0.410 | 0.947 | 2.31× |
| 100 | 1 | 0.045 | 0.090 | 2.00× |
| 100 | 5 | 0.186 | 0.467 | 2.51× |
| 100 | 10 | 0.432 | 0.972 | 2.25× |
| 1,000 | 1 | 0.055 | 0.096 | 1.76× |
| 1,000 | 5 | 0.190 | 0.487 | 2.56× |
| 1,000 | 10 | 0.404 | 1.029 | 2.55× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
