# SignedPower benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 48.92M | 0.019 | 53.82M | nan | — | — |
| 10,000 | 0.182 | 55.04M | 0.176 | 56.68M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.029 ms**; native kernel **0.027 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.314 | 0.191 | 5.23M | nan | — | — |
| 1,500 | 10 | 1.184 | 0.685 | 14.59M | nan | — | — |
| 1,500 | 100 | 3.983 | 3.434 | 29.12M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
