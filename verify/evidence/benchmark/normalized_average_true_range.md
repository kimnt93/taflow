# NormalizedAverageTrueRange benchmark (`NATR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 72.14M | 0.011 | 87.00M | 0.041 | 2.98× | 3.59× |
| 10,000 | 0.075 | 133.13M | 0.074 | 135.72M | 0.094 | 1.25× | 1.27× |
| 100,000 | 0.774 | 129.24M | 0.858 | 116.61M | 0.649 | 0.84× | 0.76× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.093 | 0.135 | 1.45× |
| 1 | 5 | 0.262 | 0.486 | 1.85× |
| 1 | 10 | 0.490 | 0.941 | 1.92× |
| 10 | 1 | 0.049 | 0.093 | 1.90× |
| 10 | 5 | 0.276 | 0.516 | 1.87× |
| 10 | 10 | 0.496 | 0.960 | 1.93× |
| 100 | 1 | 0.054 | 0.093 | 1.71× |
| 100 | 5 | 0.238 | 0.453 | 1.90× |
| 100 | 10 | 0.561 | 1.033 | 1.84× |
| 1,000 | 1 | 0.063 | 0.104 | 1.63× |
| 1,000 | 5 | 0.247 | 0.522 | 2.11× |
| 1,000 | 10 | 0.559 | 1.101 | 1.97× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
