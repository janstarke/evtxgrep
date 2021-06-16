# evtxgrep
XPath based search in Windows Event Log files

## Example

```shell
evtxgrep -event-id 4624 --level 0 Security.evtx
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
  -O,--or               combine filters non-inclusively (use OR instead of AND,
                        which is the default)
  --provider PROVIDER   filter based on Provider
  --event-id EVENT_ID   filter based on EventID
  --level LEVEL         filter based on Level
  --task TASK           filter based on Task
  --opcode OPCODE       filter based on Opcode
  --keywords KEYWORDS   filter based on Keywords
  --time-created TIME_CREATED
                        filter based on TimeCreated
  --event-record-id EVENT_RECORD_ID
                        filter based on EventRecordID
  --activity-id ACTIVITY_ID
                        filter based on ActivityID
  --related-activity-id RELATED_ACTIVITY_ID
                        filter based on RelatedActivityID
  --process-id PROCESS_ID
                        filter based on ProcessID
  --thread-id THREAD_ID filter based on ThreadID
  --channel CHANNEL     filter based on Channel
  --computer COMPUTER   filter based on Computer
  --user-id USER_ID     filter based on UserID
```
