# MathLog1p benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 101.93M | 0.009 | 113.82M | nan | — | — |
| 10,000 | 0.083 | 121.17M | 0.078 | 127.94M | nan | — | — |
| 100,000 | 0.811 | 123.29M | 0.789 | 126.82M | nan | — | — |
| 1,000,000 | 9.196 | 108.74M | 8.278 | 120.80M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.796 ms**; native kernel **0.795 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.221 | 0.153 | 6.54M | nan | — | — |
| 100,000 | 10 | 0.930 | 0.614 | 16.28M | nan | — | — |
| 100,000 | 1,000 | 10.359 | 9.466 | 105.64M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 98.21M | 102.75M | 1.00× | 2.95M | 2.91M | 1.00× | — |
| 2 | 186.71M | 198.45M | 1.93× | 2.97M | 3.49M | 1.20× | — |
| 4 | 260.20M | 338.22M | 3.29× | 2.84M | 2.81M | 0.97× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
