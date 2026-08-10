# CommodityChannelIndex benchmark (`CCI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 41.98M | 0.022 | 46.47M | 0.052 | 2.17× | 2.40× |
| 10,000 | 0.186 | 53.83M | 0.190 | 52.71M | 0.230 | 1.24× | 1.21× |
| 100,000 | 1.923 | 52.00M | 1.817 | 55.03M | 1.993 | 1.04× | 1.10× |
| 1,000,000 | 19.675 | 50.83M | 18.597 | 53.77M | 21.787 | 1.11× | 1.17× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.091 | 0.153 | 1.68× |
| 1 | 5 | 0.296 | 0.474 | 1.60× |
| 1 | 10 | 0.492 | 0.933 | 1.90× |
| 10 | 1 | 0.048 | 0.090 | 1.88× |
| 10 | 5 | 0.231 | 0.459 | 1.98× |
| 10 | 10 | 0.466 | 0.917 | 1.97× |
| 100 | 1 | 0.052 | 0.089 | 1.73× |
| 100 | 5 | 0.223 | 0.435 | 1.95× |
| 100 | 10 | 0.524 | 0.917 | 1.75× |
| 1,000 | 1 | 0.068 | 0.113 | 1.66× |
| 1,000 | 5 | 0.245 | 0.540 | 2.20× |
| 1,000 | 10 | 0.513 | 1.140 | 2.22× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
