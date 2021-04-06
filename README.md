# evtxgrep
regular expression based search in Windows Event Log files

## Example

```shell
evtxgrep --id 4624 --data Administrator Security.evtx
```

## Usage

```
Usage:
  evtxgrep [OPTIONS] EVTXFILE

regular expression based search in Windows Event Log files

Positional arguments:
  evtxfile              name of the evtx file

Optional arguments:
  -h,--help             Show this help message and exit
  -I,--id ID            pattern used to filter event ids
  -D,--data DATA        pattern to search for in the data section
```
