cut-trailing-bytes
==================
A tool to cut tailing bytes, default cut tailing NULL(0x00 in hex) bytes

# Example

```bash
$ echo "hello" > hello_00

$ dd if=/dev/zero bs=1k count=1 >> hello_00
1+0 records in
1+0 records out
1024 bytes (1.0 kB, 1.0 KiB) copied, 0.0031857 s, 321 kB/s

$ hexdump -C hello_00
00000000  68 65 6c 6c 6f 0a 00 00  00 00 00 00 00 00 00 00  |hello...........|
00000010  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
*
00000406

$ du -b hello_00
1030    hello_00

$ ./cut-trailing-bytes hello_00
cut hello_00 from 1030 to 6

$ hexdump -C hello_00
00000000  68 65 6c 6c 6f 0a                                 |hello.|
00000006

$ du -b hello_00
6       hello_00

$ echo "hello" > hello_ff

$ dd if=/dev/zero bs=1k count=1 | tr '\000' '\377' >> hello_ff
1+0 records in
1+0 records out
1024 bytes (1.0 kB, 1.0 KiB) copied, 0.0070723 s, 145 kB/s

$ hexdump -C hello_ff
00000000  68 65 6c 6c 6f 0a ff ff  ff ff ff ff ff ff ff ff  |hello...........|
00000010  ff ff ff ff ff ff ff ff  ff ff ff ff ff ff ff ff  |................|
*
00000406

$ du -b hello_ff
1030    hello_ff

$ ./cut-trailing-bytes hello_ff -c ff
cut hello_ff from 1030 to 6

$ hexdump -C hello_ff
00000000  68 65 6c 6c 6f 0a                                 |hello.|
00000006

zqb@WSL:~/workspace/rust/cut-trailing-bytes
$ du -b hello_ff
6       hello_ff
```
