# Lag benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 189.07M | 0.004 | 239.57M | nan | — | — |
| 10,000 | 0.035 | 289.62M | 0.032 | 317.06M | nan | — | — |
| 100,000 | 0.348 | 287.26M | 0.304 | 329.31M | nan | — | — |
| 1,000,000 | 3.528 | 283.47M | 3.078 | 324.93M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.316 ms**; native kernel **0.289 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.260 | 0.159 | 6.29M | nan | — | — |
| 100,000 | 10 | 1.034 | 0.545 | 18.35M | nan | — | — |
| 100,000 | 1,000 | 4.910 | 6.071 | 164.72M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 217.73M | 267.16M | 1.00× | 2.90M | 3.32M | 1.00× | — |
| 2 | 391.32M | 529.38M | 1.98× | 3.68M | 3.85M | 1.16× | — |
| 4 | 331.91M | 449.07M | 1.68× | 3.55M | 4.03M | 1.21× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
