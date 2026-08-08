# AroonOscillator benchmark (`AROONOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 105.04M | 0.008 | 124.21M | 0.037 | 3.84× | 4.54× |
| 10,000 | 0.112 | 89.26M | 0.108 | 92.77M | 0.129 | 1.15× | 1.19× |
| 100,000 | 1.088 | 91.91M | 1.083 | 92.31M | 1.023 | 0.94× | 0.94× |
| 1,000,000 | 11.497 | 86.98M | 10.850 | 92.17M | 10.109 | 0.88× | 0.93× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.090 ms**; native kernel **1.087 ms**; TA-Lib 1.014 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.295 | 0.215 | 4.66M | 1034.196 | 4820.40× | 140.46× |
| 100,000 | 10 | 1.644 | 0.996 | 10.04M | 1014.376 | 1018.27× | 29.64× |
| 100,000 | 1,000 | 36.471 | 28.662 | 34.89M | 1026.533 | 35.82× | 1.32× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 77.78M | 78.92M | 1.00× | 2.42M | 2.41M | 1.00× | 81.28M |
| 2 | 138.38M | 156.09M | 1.98× | 2.28M | 2.81M | 1.16× | 82.48M |
| 4 | 253.53M | 298.42M | 3.78× | 2.33M | 2.61M | 1.08× | 82.42M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
