# CandleSeparatingLines benchmark (`CDLSEPARATINGLINES` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 92.16M | 0.009 | 114.39M | 0.034 | 3.16× | 3.93× |
| 10,000 | 0.074 | 134.98M | 0.071 | 140.91M | 0.125 | 1.68× | 1.76× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.013 ms**; native kernel **0.011 ms**; TA-Lib 0.040 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.359 | 0.278 | 3.59M | 41.059 | 147.59× | 109.23× |
| 1,500 | 10 | 4.304 | 1.258 | 7.95M | 42.270 | 33.60× | 22.58× |
| 1,500 | 100 | 5.125 | 2.946 | 33.95M | 42.826 | 14.54× | 10.06× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 11.34M | 14.28M | 1.00× | 690.25K | 1.25M | 1.00× | 7.85M |
| 2 | 14.01M | 17.42M | 1.22× | 1.30M | 1.28M | 1.03× | 9.07M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
