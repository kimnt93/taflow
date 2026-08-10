# McClellanSummationIndex benchmark (`McClellanSummationIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 108.89M | 0.008 | 125.36M | 7.966 | 867.46× | 998.67× |
| 10,000 | 0.053 | 187.87M | 0.050 | 201.99M | 80.235 | 1507.34× | 1620.68× |
| 100,000 | 0.490 | 204.23M | 0.472 | 211.86M | 826.508 | 1687.96× | 1751.04× |
| 1,000,000 | 5.289 | 189.08M | 5.099 | 196.11M | 8103.627 | 1532.23× | 1589.24× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.124 | 0.257 | 2.07× |
| 1 | 5 | 0.448 | 1.386 | 3.09× |
| 1 | 10 | 0.483 | 2.314 | 4.79× |
| 10 | 1 | 0.051 | 0.288 | 5.67× |
| 10 | 5 | 0.248 | 1.714 | 6.91× |
| 10 | 10 | 0.504 | 3.257 | 6.46× |
| 100 | 1 | 0.053 | 1.057 | 19.78× |
| 100 | 5 | 0.241 | 5.422 | 22.51× |
| 100 | 10 | 0.515 | 11.287 | 21.90× |
| 1,000 | 1 | 0.057 | 8.781 | 153.30× |
| 1,000 | 5 | 0.263 | 46.710 | 177.68× |
| 1,000 | 10 | 0.580 | 96.202 | 165.80× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
