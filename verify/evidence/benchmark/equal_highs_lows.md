# EqualHighsLows benchmark (`causal equal pivot levels` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.042 | 23.83M | 0.039 | 25.43M | 4.642 | 110.60× | 118.06× |
| 10,000 | 0.433 | 23.08M | 0.411 | 24.32M | 46.957 | 108.36× | 114.18× |
| 100,000 | 4.338 | 23.05M | 4.092 | 24.44M | 470.336 | 108.43× | 114.95× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | 0.186 | 2.12× |
| 1 | 5 | 0.227 | 0.760 | 3.34× |
| 1 | 10 | 0.389 | 1.384 | 3.56× |
| 10 | 1 | 0.048 | 0.175 | 3.67× |
| 10 | 5 | 0.195 | 0.903 | 4.64× |
| 10 | 10 | 0.403 | 1.686 | 4.19× |
| 100 | 1 | 0.057 | 0.565 | 9.88× |
| 100 | 5 | 0.221 | 2.839 | 12.86× |
| 100 | 10 | 0.432 | 5.674 | 13.14× |
| 1,000 | 1 | 0.093 | 4.684 | 50.49× |
| 1,000 | 5 | 0.264 | 24.612 | 93.35× |
| 1,000 | 10 | 0.612 | 50.432 | 82.36× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
