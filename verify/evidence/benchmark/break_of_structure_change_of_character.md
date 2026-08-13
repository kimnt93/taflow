# BreakOfStructureChangeOfCharacter benchmark (`causal BOS and CHOCH events` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.288 | 3.47M | 0.269 | 3.72M | 4.164 | 14.45× | 15.49× |
| 10,000 | 2.641 | 3.79M | 2.550 | 3.92M | 41.805 | 15.83× | 16.39× |
| 100,000 | 25.307 | 3.95M | 25.042 | 3.99M | 411.784 | 16.27× | 16.44× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.157 | 0.119 | 0.76× |
| 1 | 5 | 0.498 | 0.512 | 1.03× |
| 1 | 10 | 0.671 | 0.975 | 1.45× |
| 10 | 1 | 0.077 | 0.095 | 1.23× |
| 10 | 5 | 0.341 | 0.484 | 1.42× |
| 10 | 10 | 0.716 | 0.975 | 1.36× |
| 100 | 1 | 0.104 | 0.522 | 5.04× |
| 100 | 5 | 0.346 | 2.421 | 7.00× |
| 100 | 10 | 0.707 | 4.969 | 7.03× |
| 1,000 | 1 | 0.360 | 4.427 | 12.30× |
| 1,000 | 5 | 0.855 | 24.504 | 28.66× |
| 1,000 | 10 | 1.123 | 71.645 | 63.82× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
