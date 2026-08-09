# AroonOscillator benchmark (`AROONOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 79.85M | 0.013 | 77.67M | 0.039 | 3.11× | 3.02× |
| 10,000 | 0.120 | 83.41M | 0.115 | 87.29M | 0.133 | 1.11× | 1.16× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.017 ms**; native kernel **0.016 ms**; TA-Lib 0.041 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.322 | 0.241 | 4.14M | 41.243 | 170.91× | 122.03× |
| 1,500 | 10 | 1.844 | 1.198 | 8.34M | 40.779 | 34.03× | 26.76× |
| 1,500 | 100 | 6.657 | 4.774 | 20.95M | 43.291 | 9.07× | 6.60× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.10M | 14.27M | 1.00× | 836.36K | 1.36M | 1.00× | 8.42M |
| 2 | 18.79M | 12.73M | 0.89× | 1.29M | 1.47M | 1.08× | 7.96M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
