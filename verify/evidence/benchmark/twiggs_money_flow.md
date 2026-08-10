# TwiggsMoneyFlow benchmark (`TwiggsMoneyFlow` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 54.96M | 0.015 | 68.03M | 0.271 | 14.89× | 18.43× |
| 10,000 | 0.109 | 91.45M | 0.109 | 91.63M | 1.506 | 13.78× | 13.80× |
| 100,000 | 1.078 | 92.79M | 1.153 | 86.76M | 14.004 | 12.99× | 12.15× |
| 1,000,000 | 10.564 | 94.66M | 10.249 | 97.57M | 144.535 | 13.68× | 14.10× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.112 | 0.479 | 4.27× |
| 1 | 5 | 0.289 | 1.104 | 3.82× |
| 1 | 10 | 0.517 | 2.485 | 4.81× |
| 10 | 1 | 0.059 | 0.232 | 3.91× |
| 10 | 5 | 0.259 | 1.252 | 4.83× |
| 10 | 10 | 0.550 | 2.539 | 4.62× |
| 100 | 1 | 0.060 | 0.239 | 3.96× |
| 100 | 5 | 0.267 | 1.409 | 5.27× |
| 100 | 10 | 0.580 | 2.684 | 4.63× |
| 1,000 | 1 | 0.086 | 0.384 | 4.44× |
| 1,000 | 5 | 0.297 | 1.989 | 6.70× |
| 1,000 | 10 | 0.542 | 3.816 | 7.04× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
