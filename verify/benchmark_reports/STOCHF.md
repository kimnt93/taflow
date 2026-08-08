# FastStochasticOscillator benchmark (`STOCHF` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.200 | 4.99M | 0.048 | 20.65M | 0.047 | 0.23× | 0.96× |
| 10,000 | 2.188 | 4.57M | 0.518 | 19.30M | 0.148 | 0.07× | 0.29× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.300 ms**; native kernel **0.074 ms**; TA-Lib 0.051 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.445 | 0.616 | 1.62M | 48.620 | 78.93× | 64.20× |
| 1,500 | 10 | 4.283 | 1.704 | 5.87M | 52.041 | 30.54× | 31.02× |
| 1,500 | 100 | 19.499 | 7.817 | 12.79M | 54.262 | 6.94× | 5.33× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
