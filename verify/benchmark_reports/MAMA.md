# MesaAdaptiveMovingAverage benchmark (`MAMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.057 | 17.47M | 0.059 | 16.91M | 0.088 | 1.55× | 1.50× |
| 10,000 | 0.560 | 17.86M | 0.583 | 17.15M | 0.555 | 0.99× | 0.95× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.088 ms**; native kernel **0.084 ms**; TA-Lib 0.122 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.438 | 0.258 | 3.87M | 121.488 | 470.34× | 150.17× |
| 1,500 | 10 | 1.668 | 1.349 | 7.41M | 122.177 | 90.54× | 30.25× |
| 1,500 | 100 | 9.519 | 7.845 | 12.75M | 121.178 | 15.45× | 5.74× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 5.02M | 7.10M | 1.00× | 677.55K | 715.71K | 1.00× | 4.31M |
| 2 | 11.13M | 14.12M | 1.99× | 1.16M | 1.35M | 1.89× | 5.98M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
