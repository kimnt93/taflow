# TwiggsMoneyFlow benchmark (`TwiggsMoneyFlow` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 54.42M | 0.015 | 65.24M | 0.291 | 15.86× | 19.01× |
| 10,000 | 0.109 | 91.38M | 0.107 | 93.37M | 1.470 | 13.43× | 13.73× |
| 100,000 | 1.007 | 99.27M | 0.981 | 101.94M | 13.330 | 13.23× | 13.59× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.093 | 0.300 | 3.24× |
| 1 | 5 | 0.307 | 1.132 | 3.69× |
| 1 | 10 | 0.651 | 2.592 | 3.98× |
| 10 | 1 | 0.062 | 0.227 | 3.64× |
| 10 | 5 | 0.271 | 1.168 | 4.31× |
| 10 | 10 | 0.552 | 2.405 | 4.35× |
| 100 | 1 | 0.065 | 0.262 | 4.04× |
| 100 | 5 | 0.265 | 1.371 | 5.18× |
| 100 | 10 | 0.543 | 2.596 | 4.78× |
| 1,000 | 1 | 0.073 | 0.377 | 5.19× |
| 1,000 | 5 | 0.278 | 1.974 | 7.10× |
| 1,000 | 10 | 0.612 | 3.804 | 6.21× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
