# evtxgrep
XPath based search in Windows Event Log files

## Example

```shell
evtxgrep --filter "*/EventID/text()='4624' and */Data[@Name='TargetUserName']/text()='Administrator'" Security.evtx
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
  -F,--filter FILTER    XPath filter condition against which each record is
                        being matched
```
