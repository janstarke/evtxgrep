# evtxgrep
XPath based search in Windows Event Log files

## Example

```shell
evtxgrep -event-id 4624 --level 0 Security.evtx
```

## Usage

```
USAGE:
    evtxgrep [FLAGS] [OPTIONS] <EVTXFILE>

FLAGS:
    -h, --help       Prints help information
    -O, --or         combine filters non-inclusively (use OR instead of AND, which is the default) 
    -V, --version    Prints version information

OPTIONS:
        --activity-id <activity_id>                    filter based on ActivityID
        --channel <channel>                            filter based on Channel
        --computer <computer>                          filter based on Computer
        --event-id <event_id>                          filter based on EventID
        --event-record-id <event_record_id>            filter based on EventRecordID
        --keywords <keywords>                          filter based on Keywords
        --level <level>                                filter based on Level
        --opcode <opcode>                              filter based on Opcode
        --process-id <process_id>                      filter based on ProcessID
        --provider <provider>                          filter based on Provider
        --related-activity-id <related_activity_id>    filter based on RelatedActivityID
        --task <task>                                  filter based on Task
        --thread-id <thread_id>                        filter based on ThreadID
        --time-created <time_created>                  filter based on TimeCreated
        --user-id <user_id>                            filter based on UserID

ARGS:
    <EVTXFILE>    name of the evtx file
```
