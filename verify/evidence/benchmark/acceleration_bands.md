# AccelerationBands benchmark (`ACCBANDS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 55.82M | 0.015 | 67.21M | 0.053 | 2.94× | 3.54× |
| 10,000 | 0.100 | 99.61M | 0.089 | 112.19M | 0.116 | 1.15× | 1.30× |
| 100,000 | 1.681 | 59.48M | 1.505 | 66.45M | 1.475 | 0.88× | 0.98× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.099 | 0.134 | 1.35× |
| 1 | 5 | 0.473 | 0.594 | 1.26× |
| 1 | 10 | 0.563 | 1.095 | 1.95× |
| 10 | 1 | 0.059 | 0.101 | 1.72× |
| 10 | 5 | 0.269 | 0.534 | 1.98× |
| 10 | 10 | 0.584 | 1.108 | 1.90× |
| 100 | 1 | 0.053 | 0.116 | 2.21× |
| 100 | 5 | 0.255 | 0.512 | 2.01× |
| 100 | 10 | 0.531 | 1.120 | 2.11× |
| 1,000 | 1 | 0.069 | 0.110 | 1.60× |
| 1,000 | 5 | 0.248 | 0.558 | 2.25× |
| 1,000 | 10 | 0.530 | 1.166 | 2.20× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
