# RollingKellyCriterion benchmark (`KellyCriterion` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.025 | 39.69M | 0.032 | 31.30M | 0.203 | 8.07× | 6.37× |
| 10,000 | 0.209 | 47.82M | 0.254 | 39.44M | 0.642 | 3.07× | 2.53× |
| 100,000 | 1.951 | 51.25M | 1.901 | 52.60M | 5.152 | 2.64× | 2.71× |
| 1,000,000 | 19.549 | 51.15M | 19.218 | 52.04M | 53.114 | 2.72× | 2.76× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.089 | 0.251 | 2.82× |
| 1 | 5 | 0.336 | 1.007 | 2.99× |
| 1 | 10 | 0.467 | 2.094 | 4.49× |
| 10 | 1 | 0.053 | 0.188 | 3.52× |
| 10 | 5 | 0.234 | 0.932 | 3.98× |
| 10 | 10 | 0.490 | 2.133 | 4.35× |
| 100 | 1 | 0.049 | 0.190 | 3.84× |
| 100 | 5 | 0.218 | 0.931 | 4.28× |
| 100 | 10 | 0.488 | 2.169 | 4.44× |
| 1,000 | 1 | 0.079 | 0.245 | 3.09× |
| 1,000 | 5 | 0.249 | 1.241 | 4.98× |
| 1,000 | 10 | 0.518 | 2.690 | 5.19× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
