# AccelerationBands benchmark (`ACCBANDS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 62.53M | 0.013 | 76.17M | 0.046 | 2.90× | 3.53× |
| 10,000 | 0.099 | 100.59M | 0.087 | 115.03M | 0.118 | 1.19× | 1.36× |
| 100,000 | 0.964 | 103.71M | 0.843 | 118.69M | 0.752 | 0.78× | 0.89× |
| 1,000,000 | 21.548 | 46.41M | 16.416 | 60.92M | 11.581 | 0.54× | 0.71× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.141 | 0.150 | 1.06× |
| 1 | 5 | 0.368 | 0.567 | 1.54× |
| 1 | 10 | 0.512 | 1.056 | 2.06× |
| 10 | 1 | 0.052 | 0.099 | 1.89× |
| 10 | 5 | 0.251 | 0.491 | 1.95× |
| 10 | 10 | 0.510 | 1.075 | 2.11× |
| 100 | 1 | 0.061 | 0.103 | 1.69× |
| 100 | 5 | 0.262 | 0.518 | 1.98× |
| 100 | 10 | 0.577 | 1.033 | 1.79× |
| 1,000 | 1 | 0.076 | 0.129 | 1.70× |
| 1,000 | 5 | 0.262 | 0.538 | 2.05× |
| 1,000 | 10 | 0.531 | 1.155 | 2.18× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
