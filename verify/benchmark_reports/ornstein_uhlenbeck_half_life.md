# OrnsteinUhlenbeckHalfLife benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.099 | 10.13M | 0.097 | 10.27M | nan | — | — |
| 10,000 | 1.020 | 9.81M | 0.987 | 10.13M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.145 ms**; native kernel **0.146 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.368 | 0.276 | 3.63M | nan | — | — |
| 1,500 | 10 | 2.108 | 1.462 | 6.84M | nan | — | — |
| 1,500 | 100 | 11.759 | 10.843 | 9.22M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
