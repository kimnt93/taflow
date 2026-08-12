# CandleCounterAttack benchmark (`CDLCOUNTERATTACK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 52.22M | 0.016 | 62.66M | 0.034 | 1.75× | 2.11× |
| 10,000 | 0.175 | 57.18M | 0.164 | 60.86M | 0.131 | 0.75× | 0.79× |
| 100,000 | 1.719 | 58.16M | 1.962 | 50.98M | 1.134 | 0.66× | 0.58× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.119 | 0.146 | 1.23× |
| 1 | 5 | 0.290 | 0.445 | 1.53× |
| 1 | 10 | 0.508 | 0.899 | 1.77× |
| 10 | 1 | 0.052 | 0.089 | 1.69× |
| 10 | 5 | 0.242 | 0.450 | 1.86× |
| 10 | 10 | 0.537 | 0.890 | 1.66× |
| 100 | 1 | 0.057 | 0.087 | 1.53× |
| 100 | 5 | 0.271 | 0.500 | 1.84× |
| 100 | 10 | 0.576 | 0.987 | 1.71× |
| 1,000 | 1 | 0.070 | 0.099 | 1.41× |
| 1,000 | 5 | 0.279 | 0.489 | 1.75× |
| 1,000 | 10 | 0.600 | 1.057 | 1.76× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
