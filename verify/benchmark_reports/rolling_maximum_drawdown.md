# RollingMaximumDrawdown benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.050 | 20.16M | 0.050 | 20.20M | 8.544 | 172.24× | 172.57× |
| 10,000 | 0.475 | 21.03M | 0.452 | 22.14M | 84.763 | 178.27× | 187.68× |
| 100,000 | 4.696 | 21.29M | 4.494 | 22.25M | 892.493 | 190.05× | 198.60× |
| 1,000,000 | 46.203 | 21.64M | 46.399 | 21.55M | 8875.772 | 192.10× | 191.29× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
