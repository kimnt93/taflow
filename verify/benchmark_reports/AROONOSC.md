# AroonOscillator benchmark (`AROONOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 72.85M | 0.012 | 83.15M | 0.036 | 2.66× | 3.03× |
| 10,000 | 0.197 | 50.79M | 0.204 | 49.09M | 0.135 | 0.69× | 0.66× |
| 100,000 | 1.959 | 51.06M | 1.887 | 52.99M | 1.049 | 0.54× | 0.56× |
| 1,000,000 | 21.456 | 46.61M | 21.120 | 47.35M | 10.344 | 0.48× | 0.49× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.948 ms**; native kernel **1.908 ms**; TA-Lib 1.041 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.324 | 0.210 | 4.76M | 1062.139 | 5056.37× | 141.93× |
| 100,000 | 10 | 1.742 | 1.192 | 8.39M | 1050.004 | 881.04× | 24.88× |
| 100,000 | 1,000 | 33.544 | 48.872 | 20.46M | 1055.996 | 21.61× | 0.77× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 44.48M | 46.56M | 1.00× | 2.04M | 2.69M | 1.00× | 73.43M |
| 2 | 76.38M | 82.41M | 1.77× | 1.74M | 2.49M | 0.93× | 78.66M |
| 4 | 113.34M | 123.61M | 2.65× | 2.15M | 2.44M | 0.91× | 80.18M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
