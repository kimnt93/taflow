# BreakOfStructureChangeOfCharacter benchmark (`causal BOS and CHOCH events` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.053 | 18.89M | 0.044 | 22.59M | 4.241 | 80.12× | 95.81× |
| 10,000 | 0.451 | 22.15M | 0.431 | 23.21M | 43.890 | 97.24× | 101.88× |
| 100,000 | 4.491 | 22.27M | 4.747 | 21.07M | 480.418 | 106.98× | 101.21× |
| 1,000,000 | 70.326 | 14.22M | 49.219 | 20.32M | 4442.795 | 63.17× | 90.27× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.120 | 1.44× |
| 1 | 5 | 0.319 | 0.507 | 1.59× |
| 1 | 10 | 0.509 | 1.014 | 1.99× |
| 10 | 1 | 0.051 | 0.102 | 2.00× |
| 10 | 5 | 0.245 | 0.521 | 2.13× |
| 10 | 10 | 0.543 | 1.008 | 1.86× |
| 100 | 1 | 0.065 | 0.502 | 7.66× |
| 100 | 5 | 0.252 | 2.662 | 10.58× |
| 100 | 10 | 0.559 | 5.291 | 9.47× |
| 1,000 | 1 | 0.107 | 4.570 | 42.60× |
| 1,000 | 5 | 0.292 | 22.147 | 75.78× |
| 1,000 | 10 | 0.663 | 48.153 | 72.63× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
