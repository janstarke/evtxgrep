# evtxgrep
filtering in Windows Event Log files

## Example

```shell
evtxgrep --event-id 4624 --data TargetUserName:Administrator Security.evtx
```

## Usage

```
USAGE:
    evtxgrep [FLAGS] [OPTIONS] <EVTXFILE>

FLAGS:
    -h, --help           Prints help information
    -i, --ignore-case    ignore case
    -S, --sorted         sort records based on their event record id
    -O, --or             combine filters non-inclusively (use OR instead of AND, which is the default) 
    -V, --version        Prints version information

OPTIONS:
        --activity-id <ActivityID>                   filter based on ActivityID
        --channel <Channel>                          filter based on Channel
        --computer <Computer>                        filter based on Computer
    -D, --data <DATA:FILTER>...
            key-value pair, separated by colon, to filter based on entries in the data section. For example, to search
            for logins of the user 'Administrator', you would use `--data TargetUserName:Administrator`. This option can
            be used multiple times.
        --event-id <EventID>                         filter based on EventID
        --event-record-id <EventRecordID>            filter based on EventRecordID
        --keywords <Keywords>                        filter based on Keywords
        --level <Level>                              filter based on Level
        --opcode <Opcode>                            filter based on Opcode
        --process-id <ProcessID>                     filter based on ProcessID
        --provider <Provider>                        filter based on Provider
        --related-activity-id <RelatedActivityID>    filter based on RelatedActivityID
        --task <Task>                                filter based on Task
        --thread-id <ThreadID>                       filter based on ThreadID
        --time-created <TimeCreated>                 filter based on TimeCreated
        --user-id <UserID>                           filter based on UserID

ARGS:
    <EVTXFILE>    name of the evtx file
```
