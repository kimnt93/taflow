# BreakOfStructureChangeOfCharacter benchmark (`causal BOS and CHOCH events` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.051 | 19.57M | 0.039 | 25.87M | 4.436 | 86.82× | 114.76× |
| 10,000 | 0.430 | 23.27M | 0.425 | 23.51M | 45.479 | 105.83× | 106.93× |
| 100,000 | 4.627 | 21.61M | 4.248 | 23.54M | 449.053 | 97.05× | 105.72× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.132 | 0.153 | 1.15× |
| 1 | 5 | 0.305 | 0.545 | 1.78× |
| 1 | 10 | 0.427 | 0.974 | 2.28× |
| 10 | 1 | 0.046 | 0.097 | 2.09× |
| 10 | 5 | 0.196 | 0.478 | 2.44× |
| 10 | 10 | 0.444 | 1.042 | 2.35× |
| 100 | 1 | 0.056 | 0.508 | 9.16× |
| 100 | 5 | 0.204 | 2.565 | 12.57× |
| 100 | 10 | 0.458 | 5.031 | 10.99× |
| 1,000 | 1 | 0.091 | 4.526 | 49.65× |
| 1,000 | 5 | 0.238 | 34.268 | 143.96× |
| 1,000 | 10 | 1.081 | 52.210 | 48.30× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
