# Parkinson benchmark (`ParkinsonVolatility` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 62.53M | 0.014 | 69.11M | 0.224 | 13.99× | 15.46× |
| 10,000 | 0.138 | 72.36M | 0.134 | 74.55M | 0.879 | 6.36× | 6.55× |
| 100,000 | 1.324 | 75.54M | 1.292 | 77.42M | 7.726 | 5.84× | 5.98× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.114 | 0.289 | 2.54× |
| 1 | 5 | 0.195 | 1.435 | 7.35× |
| 1 | 10 | 0.389 | 2.589 | 6.65× |
| 10 | 1 | 0.047 | 0.235 | 5.00× |
| 10 | 5 | 0.196 | 1.414 | 7.23× |
| 10 | 10 | 0.399 | 2.448 | 6.14× |
| 100 | 1 | 0.049 | 0.238 | 4.87× |
| 100 | 5 | 0.197 | 1.484 | 7.54× |
| 100 | 10 | 0.422 | 2.782 | 6.59× |
| 1,000 | 1 | 0.057 | 0.309 | 5.39× |
| 1,000 | 5 | 0.194 | 1.880 | 9.72× |
| 1,000 | 10 | 0.426 | 3.243 | 7.61× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
