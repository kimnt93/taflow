# NormalizedAverageTrueRange benchmark (`NATR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.049 | 20.59M | 0.008 | 127.50M | 0.038 | 0.78× | 4.85× |
| 10,000 | 0.468 | 21.38M | 0.067 | 148.93M | 0.093 | 0.20× | 1.38× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.071 ms**; native kernel **0.011 ms**; TA-Lib 0.042 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.433 | 0.245 | 4.08M | 41.262 | 168.50× | 126.34× |
| 1,500 | 10 | 2.676 | 1.072 | 9.33M | 41.710 | 38.92× | 31.31× |
| 1,500 | 100 | 10.224 | 3.322 | 30.10M | 40.730 | 12.26× | 9.98× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
