# torrent-rs


## import
```
$ find . -name *.torrent -exec ./import {} \;
file name:     fb2-573000-575999.zip
creation date: Wed Jan 13 05:58:06 2016
info hash:     5BB44AB52B538546D907185FCBC5313342DD32C0
total length:  1689598959
piece length:  2097152
piece count:   806
Ok(())
file name:     fb2-545000-549999.zip
creation date: Thu Jun 18 12:15:20 2015
info hash:     75E18891DE24FCBC757EC7A8AF282B1A7F53877E
total length:  5190592358
piece length:  4194304
piece count:   1238
Ok(())

```
or
```
$ find . -name *.torrent -exec ./target/release/examples/import {} \;
```