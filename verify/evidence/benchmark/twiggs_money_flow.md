# TwiggsMoneyFlow benchmark (`TwiggsMoneyFlow` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 73.73M | 0.011 | 91.57M | 0.270 | 19.89× | 24.70× |
| 10,000 | 0.105 | 95.60M | 0.101 | 98.63M | 1.424 | 13.62× | 14.05× |
| 100,000 | 0.956 | 104.61M | 0.950 | 105.22M | 13.642 | 14.27× | 14.35× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.063 | 0.263 | 4.17× |
| 1 | 5 | 0.301 | 1.080 | 3.59× |
| 1 | 10 | 0.414 | 2.440 | 5.89× |
| 10 | 1 | 0.047 | 0.216 | 4.59× |
| 10 | 5 | 0.178 | 1.062 | 5.96× |
| 10 | 10 | 0.425 | 2.280 | 5.37× |
| 100 | 1 | 0.048 | 0.227 | 4.69× |
| 100 | 5 | 0.215 | 1.424 | 6.63× |
| 100 | 10 | 0.436 | 2.481 | 5.69× |
| 1,000 | 1 | 0.062 | 0.376 | 6.05× |
| 1,000 | 5 | 0.205 | 1.917 | 9.34× |
| 1,000 | 10 | 0.437 | 3.696 | 8.46× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
