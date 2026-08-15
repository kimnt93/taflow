# Sessions benchmark (`smartmoneyconcepts.smc.sessions` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 89.76M | 0.009 | 111.62M | 89.402 | 8025.15× | 9979.26× |
| 10,000 | 0.076 | 131.00M | 0.067 | 150.08M | 868.799 | 11380.93× | 13039.01× |
| 100,000 | 0.683 | 146.38M | 0.614 | 162.75M | 8507.247 | 12453.25× | 13845.93× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.108 | 1.729 | 16.03× |
| 1 | 5 | 0.270 | 8.374 | 31.04× |
| 1 | 10 | 0.418 | 17.099 | 40.86× |
| 10 | 1 | 0.048 | 2.446 | 50.82× |
| 10 | 5 | 0.203 | 12.626 | 62.20× |
| 10 | 10 | 0.407 | 25.544 | 62.80× |
| 100 | 1 | 0.050 | 10.600 | 213.20× |
| 100 | 5 | 0.334 | 56.476 | 169.12× |
| 100 | 10 | 0.468 | 129.907 | 277.74× |
| 1,000 | 1 | 0.064 | 87.876 | 1372.80× |
| 1,000 | 5 | 0.528 | 538.766 | 1021.35× |
| 1,000 | 10 | 0.628 | 1155.203 | 1838.33× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
